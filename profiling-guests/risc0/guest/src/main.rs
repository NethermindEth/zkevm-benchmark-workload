use risc0_zkvm::guest::env;
use revm::ExecuteEvm;
use revm::{
    context::Context,
    context::TxEnv,
    MainContext, MainBuilder,
};
use serde::{Serialize, Deserialize};
use profile::track_cycles;
use alloy_primitives::{U256 as AlloyU256, Address as AlloyAddress};
use state_serializer::SerializableState;

#[derive(Serialize, Deserialize)]
struct ExecutionOutput {
    success: bool,
    gas_used: u64,
    output: Option<Vec<u8>>,
    error: Option<String>,
    cycles_used: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BlockEnvironment {
    beneficiary: AlloyAddress,
    gas_limit: u64,
    number: u64,
    timestamp: u64,
    difficulty: u64,
}

fn main() {
    // Read inputs from host
    let tx_data: TxEnv = env::read();
    let block_env: BlockEnvironment = env::read();
    let serializable_state: SerializableState = env::read();

    // Convert SerializableState to revm State
    let mut state = serializable_state.into_state();

    // Create EVM context with pre-configured block environment
    let mut evm = Context::mainnet()
        .with_db(&mut state)
        .modify_block_chained(|block| {
            block.beneficiary = block_env.beneficiary;
            block.gas_limit = block_env.gas_limit;
            block.number = AlloyU256::from(block_env.number);
            block.timestamp = AlloyU256::from(block_env.timestamp);
            block.difficulty = AlloyU256::from(block_env.difficulty);
        })
        .build_mainnet();

    // Execute transaction and capture cycles
    let (result, cycles_used) = track_cycles!("evm_transact_one", {
        evm.transact(tx_data)
    });

    // Create output
    let output = match result {
        Ok(execution_result) => {
            ExecutionOutput {
                success: true,
                gas_used: execution_result.result.gas_used(),
                output: execution_result.result.output().map(|o| o.to_vec()),
                error: None,
                cycles_used,
            }
        }
        Err(evm_error) => {
            ExecutionOutput {
                success: false,
                gas_used: 0,
                output: None,
                error: Some(format!("{:?}", evm_error)),
                cycles_used,
            }
        }
    };
    
    // Send result back to host
    env::commit(&output);
}