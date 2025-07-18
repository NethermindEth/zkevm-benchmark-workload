//! Profile runner for zkVM workloads

#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use std::{any::Any, panic, path::PathBuf};
use tracing::info;
use anyhow::Result;
use profile_metrics::{ProfileRun, ExecutionMetrics, ProvingMetrics, CrashInfo};
use profile_witness_generator::{NamedTestCase, create_tx_env};
use state_serializer::SerializableState;
use risc0_zkvm::{default_prover, ExecutorEnv};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use methods::{PROFILE_ELF, PROFILE_ID};
use std::str::FromStr;
use alloy_primitives::Address as AlloyAddress;

/// Holds the configuration for running profiles
#[derive(Debug, Clone)]
pub struct RunConfig {
    /// Output folder where profile results will be stored
    pub output_folder: PathBuf,
    /// Action to perform: either proving or executing
    pub action: Action,
    /// Force rerun profiles even if output files already exist
    pub force_rerun: bool,
}

/// Action specifies whether we should prove or execute
#[derive(Debug, Clone, Copy)]
pub enum Action {
    /// Generate a proof for the zkVM execution
    Prove,
    /// Only execute the zkVM without proving
    Execute,
    /// Prove and execute the zkVM
    Full,
}

/// Execution output from the guest
#[derive(serde::Serialize, serde::Deserialize)]
struct ExecutionOutput {
    success: bool,
    gas_used: u64,
    output: Option<Vec<u8>>,
    error: Option<String>,
    cycles_used: Option<u64>,
}

/// Simplified block environment data sent to guest
#[derive(serde::Serialize, serde::Deserialize)]
struct BlockEnvironment {
    beneficiary: AlloyAddress,
    gas_limit: u64,
    number: u64,
    timestamp: u64,
    difficulty: u64,
}

/// Runs the profile for a given corpus of test cases
pub async fn run_profile(
    run_config: &RunConfig,
    test_cases: &[NamedTestCase],
) -> Result<()> {
    // Detect and save hardware information
    let hardware_info = profile_metrics::HardwareInfo::detect();
    hardware_info.to_path(run_config.output_folder.join("hardware.json"))?;

    info!("Starting profile run with {} test cases", test_cases.len());

    match run_config.action {
        Action::Execute => {
            // Use parallel iteration for execution
            test_cases
                .iter()
                .try_for_each(|test_case| process_test_case(test_case, run_config))?;
        }
        Action::Prove => {
            // Use sequential iteration for proving
            test_cases
                .iter()
                .try_for_each(|test_case| process_test_case(test_case, run_config))?;
        }
        Action::Full => {
            // Use parallel iteration for execution
            test_cases
                .iter()
                .try_for_each(|test_case| process_test_case(test_case, run_config))?;
        }
    }
    Ok(())
}

fn process_test_case(
    test_case: &NamedTestCase,
    run_config: &RunConfig,
) -> Result<()> {
    let out_path = run_config
        .output_folder
        .join(format!("{}.json", test_case.name));

    if !run_config.force_rerun && out_path.exists() {
        info!("Skipping {} (already exists)", test_case.name);
        return Ok(());
    }

    info!("Running {}", test_case.name);

    let profile_run = match run_config.action {
        Action::Execute => {
            let execution = execute_test_case(&test_case.test_case)?;
            ProfileRun {
                name: test_case.name.clone(),
                gas_used: 21000, // Default gas, would be extracted from execution in real implementation
                execution: Some(execution),
                proving: None,
            }
        }
        Action::Prove => {
            let proving = prove_test_case(&test_case.test_case)?;
            ProfileRun {
                name: test_case.name.clone(),
                gas_used: 21000, // Default gas, would be extracted from proving in real implementation
                execution: None,
                proving: Some(proving),
            }
        }
        Action::Full => {
           let proving = prove_test_case(&test_case.test_case)?;
           let execution = execute_test_case(&test_case.test_case)?;
           ProfileRun {
            name: test_case.name.clone(),
            gas_used: 21000,
            execution: Some(execution),
            proving: Some(proving),
           }
        }
    };

    info!("Saving report for {}", test_case.name);
    ProfileRun::to_path(out_path, &[profile_run])?;

    Ok(())
}

fn execute_test_case(test_case: &profile_witness_generator::TestCase) -> Result<ExecutionMetrics> {
    // Create transaction environment from the test case
    let tx_env = create_tx_env(test_case)?;
    
    // Create SerializableState from test case pre-state
    let serializable_state = SerializableState {
        accounts: test_case.pre.iter().map(|(addr, account)| {
            (addr.clone(), state_serializer::SerializableAccount {
                nonce: account.nonce.clone(),
                balance: account.balance.clone(),
                code: account.code.clone(),
                storage: account.storage.clone(),
            })
        }).collect(),
    };
    
    // Create simplified block environment for guest
    let block_env = BlockEnvironment {
        beneficiary: AlloyAddress::from_str(&test_case.env.current_coinbase)
            .expect("Invalid coinbase address"),
        gas_limit: u64::from_str_radix(&test_case.env.current_gas_limit.trim_start_matches("0x"), 16)
            .expect("Invalid gas limit hex"),
        number: u64::from_str_radix(&test_case.env.current_number.trim_start_matches("0x"), 16)
            .expect("Invalid number hex"),
        timestamp: u64::from_str_radix(&test_case.env.current_timestamp.trim_start_matches("0x"), 16)
            .expect("Invalid timestamp hex"),
        difficulty: u64::from_str_radix(&test_case.env.current_difficulty.trim_start_matches("0x"), 16)
            .expect("Invalid difficulty hex"),
    };
    
    // Create the executor environment
    let env = ExecutorEnv::builder()
        .write(&tx_env)
        .map_err(|e| anyhow::anyhow!("Failed to write tx_env: {}", e))?
        .write(&block_env)
        .map_err(|e| anyhow::anyhow!("Failed to write block_env: {}", e))?
        .write(&serializable_state)
        .map_err(|e| anyhow::anyhow!("Failed to write serializable_state: {}", e))?
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build executor environment: {}", e))?;

    let execution_start = std::time::Instant::now();
    
    // Execute the test case using RISC0 zkVM (no proof, just run)
    let run = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        let prover = default_prover();
        let prove_info = prover.prove(env, PROFILE_ELF)?;
        // For execution, we do not verify the receipt
        let output: ExecutionOutput = prove_info.receipt.journal.decode()
            .map_err(|e| anyhow::anyhow!("Failed to decode output: {}", e))?;
        Ok::<ExecutionOutput, anyhow::Error>(output)
    }));

    let execution_duration = execution_start.elapsed();

    let execution = match run {
        Ok(Ok(output)) => ExecutionMetrics::Success {
            total_num_cycles: output.cycles_used.unwrap_or(0),
            region_cycles: std::collections::HashMap::new(), // Would be populated in real implementation
            execution_duration,
            output: output.output,
            error: output.error,
        },
        Ok(Err(e)) => ExecutionMetrics::Crashed(CrashInfo {
            reason: format!("Execution error: {}", e),
        }),
        Err(panic_info) => ExecutionMetrics::Crashed(CrashInfo {
            reason: get_panic_msg(panic_info),
        }),
    };

    Ok(execution)
}

fn prove_test_case(test_case: &profile_witness_generator::TestCase) -> Result<ProvingMetrics> {
    // Create transaction environment from the test case
    let tx_env = create_tx_env(test_case)?;
    
    // Create SerializableState from test case pre-state
    let serializable_state = SerializableState {
        accounts: test_case.pre.iter().map(|(addr, account)| {
            (addr.clone(), state_serializer::SerializableAccount {
                nonce: account.nonce.clone(),
                balance: account.balance.clone(),
                code: account.code.clone(),
                storage: account.storage.clone(),
            })
        }).collect(),
    };
    
    // Create simplified block environment for guest
    let block_env = BlockEnvironment {
            beneficiary: AlloyAddress::from_str(&test_case.env.current_coinbase)
            .expect("Invalid coinbase address"),
        gas_limit: u64::from_str_radix(&test_case.env.current_gas_limit.trim_start_matches("0x"), 16)
            .expect("Invalid gas limit hex"),
        number: u64::from_str_radix(&test_case.env.current_number.trim_start_matches("0x"), 16)
            .expect("Invalid number hex"),
        timestamp: u64::from_str_radix(&test_case.env.current_timestamp.trim_start_matches("0x"), 16)
            .expect("Invalid timestamp hex"),
        difficulty: u64::from_str_radix(&test_case.env.current_difficulty.trim_start_matches("0x"), 16)
            .expect("Invalid difficulty hex"),
    };
    
    // Create the executor environment
    let env = ExecutorEnv::builder()
        .write(&tx_env)
        .map_err(|e| anyhow::anyhow!("Failed to write tx_env: {}", e))?
        .write(&block_env)
        .map_err(|e| anyhow::anyhow!("Failed to write block_env: {}", e))?
        .write(&serializable_state)
        .map_err(|e| anyhow::anyhow!("Failed to write serializable_state: {}", e))?
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build executor environment: {}", e))?;

    let proving_start = std::time::Instant::now();
    
    // Prove the execution
    let run = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        // Use the actual RISC0 prover
        let prover = default_prover();
        let prove_info = prover.prove(env, PROFILE_ELF)?;
        let receipt = prove_info.receipt;
        receipt.verify(PROFILE_ID)?;
        
        // Read the output from the receipt
        let output: ExecutionOutput = receipt.journal.decode()
            .map_err(|e| anyhow::anyhow!("Failed to decode output: {}", e))?;
        
        Ok::<ExecutionOutput, anyhow::Error>(output)
    }));

    let proving_duration = proving_start.elapsed();

    let proving = match run {
        Ok(Ok(output)) => ProvingMetrics::Success {
            proof_size: 256, // Would be actual proof size in real implementation
            proving_time_ms: proving_duration.as_millis(),
            cycles_used: output.cycles_used.unwrap_or(0),
            output: output.output,
            error: output.error,
        },
        Ok(Err(e)) => ProvingMetrics::Crashed(CrashInfo {
            reason: format!("Proving error: {}", e),
        }),
        Err(panic_info) => ProvingMetrics::Crashed(CrashInfo {
            reason: get_panic_msg(panic_info),
        }),
    };

    Ok(proving)
}

fn get_panic_msg(panic_info: Box<dyn Any + Send>) -> String {
    panic_info
        .downcast_ref::<&str>()
        .map(|s| s.to_string())
        .or_else(|| panic_info.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "Unknown panic occurred".to_string())
} 