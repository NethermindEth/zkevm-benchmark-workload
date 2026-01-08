//! Binary for benchmarking different Ere compatible zkVMs

#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use anyhow::{Context, Result, bail};
use benchmark_runner::{
    block_encoding_length_program, empty_program,
    runner::{Action, RunConfig, get_zkvm_instances, run_benchmark},
    stateless_executor, stateless_validator,
    tracer::{TraceConfig, trace_fixtures},
};

use clap::Parser;
use ere_dockerized::zkVMKind;
use ere_zkvm_interface::ProverResourceType;
use std::path::{Path, PathBuf};
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::cli::{
    BenchmarkAction, Cli, GuestProgramCommand, Resource, StatelessExecutorClient,
    StatelessValidatorClient,
};

pub mod cli;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    // Handle trace-only action separately (no zkVM required)
    if cli.action == BenchmarkAction::TraceOnly {
        return run_trace_only(&cli);
    }

    // Validate that zkVMs are specified for non-trace-only actions
    if cli.zkvms.is_empty() {
        bail!("At least one zkVM must be specified with --zkvms for execute/prove actions");
    }

    // Validate that network proving is only used with SP1
    if matches!(cli.resource, Resource::Network | Resource::Cluster)
        && cli.zkvms.iter().any(|z| *z != zkVMKind::SP1)
    {
        bail!("Network or cluster proving is only supported for SP1. Use --zkvms sp1");
    }

    let resource: ProverResourceType = cli.resource.into();
    let action: Action = cli.action.into();
    info!(
        "Running benchmarks with resource={:?} and action={:?}",
        resource, action
    );

    let workspace_dir = workspace_root().join("ere-guests");
    match cli.guest_program {
        GuestProgramCommand::StatelessExecutor {
            input_folder,
            input_file,
            execution_client,
        } => {
            let input_display = input_file.as_ref().unwrap_or(&input_folder);
            info!(
                "Running stateless-executor benchmark for input: {}",
                input_display.display()
            );

            // Run opcode tracing if requested
            if cli.trace_opcode {
                run_opcode_tracing(&input_folder, input_file.as_deref(), &cli.trace_output)?;
            }

            let el = execution_client.into();
            let guest_io = stateless_executor::stateless_executor_inputs_from(
                input_folder.as_path(),
                input_file.as_deref(),
                el,
            )?;
            let guest_relative = execution_client
                .guest_rel_path()
                .context("Failed to get guest relative path")?;
            let apply_patches = matches!(execution_client, StatelessExecutorClient::Reth);
            let zkvms = get_zkvm_instances(
                &cli.zkvms,
                &workspace_dir,
                &guest_relative,
                resource,
                apply_patches,
            )?;
            let config = RunConfig {
                output_folder: cli.output_folder,
                sub_folder: Some(el.as_ref().to_lowercase()),
                action,
                force_rerun: cli.force_rerun,
                dump_inputs_folder: cli.dump_inputs.clone(),
            };
            for zkvm in zkvms {
                run_benchmark(&zkvm, &config, &guest_io)?;
            }
        }
        GuestProgramCommand::StatelessValidator {
            input_folder,
            input_file,
            execution_client,
        } => {
            let input_display = input_file.as_ref().unwrap_or(&input_folder);
            info!(
                "Running stateless-validator benchmark for input: {}",
                input_display.display()
            );

            // Run opcode tracing if requested
            if cli.trace_opcode {
                run_opcode_tracing(&input_folder, input_file.as_deref(), &cli.trace_output)?;
            }

            let el = execution_client.into();
            let guest_io = stateless_validator::stateless_validator_inputs_from(
                input_folder.as_path(),
                input_file.as_deref(),
                el,
            )?;
            let guest_relative = execution_client
                .guest_rel_path()
                .context("Failed to get guest relative path")?;
            let apply_patches = matches!(execution_client, StatelessValidatorClient::Reth);
            let zkvms = get_zkvm_instances(
                &cli.zkvms,
                &workspace_dir,
                &guest_relative,
                resource,
                apply_patches,
            )?;
            let config = RunConfig {
                output_folder: cli.output_folder,
                sub_folder: Some(el.as_ref().to_lowercase()),
                action,
                force_rerun: cli.force_rerun,
                dump_inputs_folder: cli.dump_inputs.clone(),
            };
            for zkvm in zkvms {
                run_benchmark(&zkvm, &config, &guest_io)?;
            }
        }
        GuestProgramCommand::EmptyProgram => {
            if cli.trace_opcode {
                bail!("Opcode tracing is not supported for empty-program");
            }
            info!("Running empty-program benchmarks");
            let guest_io = empty_program::empty_program_input()
                .context("Failed to create empty program input")?;
            let zkvms = get_zkvm_instances(
                &cli.zkvms,
                &workspace_dir,
                Path::new("empty-program"),
                resource,
                true,
            )?;
            let config = RunConfig {
                output_folder: cli.output_folder,
                sub_folder: None,
                action,
                force_rerun: cli.force_rerun,
                dump_inputs_folder: cli.dump_inputs.clone(),
            };
            for zkvm in zkvms {
                run_benchmark(&zkvm, &config, [&guest_io])?;
            }
        }
        GuestProgramCommand::BlockEncodingLength {
            input_folder,
            loop_count,
            format,
        } => {
            if cli.trace_opcode {
                bail!("Opcode tracing is not supported for block-encoding-length");
            }
            info!(
                "Running {:?}-encoding-length benchmarks for input folder {} and loop count {}",
                format,
                input_folder.display(),
                loop_count
            );
            let guest_io = block_encoding_length_program::block_encoding_length_inputs(
                input_folder.as_path(),
                loop_count,
                format.into(),
            )?;
            let zkvms = get_zkvm_instances(
                &cli.zkvms,
                &workspace_dir,
                Path::new("block-encoding-length"),
                resource,
                true,
            )?;
            let config = RunConfig {
                output_folder: cli.output_folder,
                sub_folder: None,
                action,
                force_rerun: cli.force_rerun,
                dump_inputs_folder: cli.dump_inputs.clone(),
            };
            for zkvm in zkvms {
                run_benchmark(&zkvm, &config, &guest_io)?;
            }
        }
    }

    Ok(())
}

/// Handles the trace-only action (no zkVM execution).
fn run_trace_only(cli: &Cli) -> Result<()> {
    match &cli.guest_program {
        GuestProgramCommand::StatelessExecutor {
            input_folder,
            input_file,
            ..
        }
        | GuestProgramCommand::StatelessValidator {
            input_folder,
            input_file,
            ..
        } => {
            let input_display = input_file.as_ref().unwrap_or(input_folder);
            info!(
                "Running trace-only for input: {}",
                input_display.display()
            );
            run_opcode_tracing(input_folder, input_file.as_deref(), &cli.trace_output)?;
            Ok(())
        }
        GuestProgramCommand::EmptyProgram => {
            bail!("trace-only action is not supported for empty-program")
        }
        GuestProgramCommand::BlockEncodingLength { .. } => {
            bail!("trace-only action is not supported for block-encoding-length")
        }
    }
}

/// Runs opcode-level tracing on fixtures.
fn run_opcode_tracing(
    input_folder: &Path,
    input_file: Option<&Path>,
    output_folder: &Path,
) -> Result<()> {
    info!("Running opcode tracing, output to: {}", output_folder.display());

    let config = TraceConfig::default();
    let results = trace_fixtures(input_folder, input_file, output_folder, &config)?;

    let success_count = results.iter().filter(|r| r.success).count();
    let total_gas: u64 = results.iter().map(|r| r.gas_used).sum();

    info!(
        "Tracing complete: {} fixtures traced, {} succeeded, {} total gas",
        results.len(),
        success_count,
        total_gas
    );

    Ok(())
}

/// Repository root (assumes `ere-hosts` lives in `<root>/crates/ere-hosts`).
fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}
