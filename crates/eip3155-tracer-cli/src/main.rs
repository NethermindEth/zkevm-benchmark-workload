//! CLI tool for generating EIP-3155 opcode traces from stateless executor fixtures.
//!
//! This binary provides a standalone tool for tracing EVM execution at the opcode level,
//! producing EIP-3155 compliant output in JSONL format.

#![cfg_attr(not(test), warn(unused_crate_dependencies))]

mod cli;
mod tracer;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use tracing::info;
use tracing_subscriber::EnvFilter;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    info!(
        "Tracing fixtures from: {}",
        cli.input_file
            .as_ref()
            .unwrap_or(&cli.input_folder)
            .display()
    );
    info!("Output folder: {}", cli.output_folder.display());

    let config = if cli.full {
        tracer::TraceConfig::full()
    } else {
        tracer::TraceConfig {
            include_stack: cli.include_stack,
            include_memory: cli.include_memory,
            include_storage: cli.include_storage,
            include_return_data: cli.include_return_data,
            include_gas: cli.include_gas,
            include_depth: cli.include_depth,
            include_refund: cli.include_refund,
            include_summary: cli.include_summary,
            summary_only: cli.summary_only,
            minimal: cli.minimal,
            pretty_print: cli.pretty_print,
        }
    };

    let results = tracer::trace_fixtures(
        &cli.input_folder,
        cli.input_file.as_deref(),
        &cli.output_folder,
        &config,
    )?;

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
