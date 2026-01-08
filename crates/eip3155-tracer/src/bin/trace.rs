//! CLI binary for running EIP-3155 traces on stateless executor fixtures.

#![allow(unused_crate_dependencies)]

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use eip3155_tracer::{trace_block, TraceOutput, TraceWriter};
use guest_libs::senders::recover_signers;
use std::fs::File;
use std::io::{self, BufWriter};
use std::path::PathBuf;
use witness_generator::StatelessExecutorFixture;

/// EIP-3155 tracer CLI - generate opcode-level execution traces from fixtures.
#[derive(Parser, Debug)]
#[command(name = "eip3155-trace")]
#[command(about = "Generate EIP-3155 compliant execution traces from stateless executor fixtures")]
struct Args {
    /// Path to the fixture JSON file.
    #[arg(short, long)]
    fixture: PathBuf,

    /// Output file path. If not specified, outputs to stdout.
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Include memory snapshots in the trace (increases output size significantly).
    #[arg(long, default_value = "false")]
    memory: bool,

    /// Include storage changes in the trace.
    #[arg(long, default_value = "true")]
    storage: bool,

    /// Pretty-print JSON output.
    #[arg(long, default_value = "false")]
    pretty: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Load fixture
    let fixture = load_fixture(&args.fixture)?;
    eprintln!("Loaded fixture: {}", fixture.name);
    eprintln!(
        "Block #{} with {} transactions",
        fixture.stateless_input.block.number,
        fixture.stateless_input.block.body.transactions.len()
    );

    // Recover public keys from transactions
    let public_keys = recover_signers(&fixture.stateless_input.block.body.transactions)
        .context("Failed to recover signers from transactions")?;

    // Build trace configuration
    let mut config = TraceOutput::default();
    if args.memory {
        config = config.with_memory();
    }
    if args.storage {
        config = config.with_storage();
    }
    if args.pretty {
        config = config.with_pretty_print();
    }

    // Run the tracer
    let result = match &args.output {
        Some(path) => {
            let file = File::create(path)
                .with_context(|| format!("Failed to create output file: {}", path.display()))?;
            let mut writer = TraceWriter::new(BufWriter::new(file), config);
            trace_block(
                fixture.stateless_input.block.clone().into(),
                public_keys,
                fixture.stateless_input.witness.clone().into(),
                fixture.stateless_input.chain_config.clone(),
                &mut writer,
            )
        }
        None => {
            let stdout = io::stdout();
            let mut writer = TraceWriter::new(BufWriter::new(stdout.lock()), config);
            trace_block(
                fixture.stateless_input.block.clone().into(),
                public_keys,
                fixture.stateless_input.witness.clone().into(),
                fixture.stateless_input.chain_config.clone(),
                &mut writer,
            )
        }
    };

    match result {
        Ok(execution) => {
            eprintln!("\nExecution complete:");
            eprintln!("  Transactions: {}", execution.transaction_count);
            eprintln!("  Total gas used: {}", execution.gas_used);
            eprintln!("  Success: {}", execution.success);
            Ok(())
        }
        Err(e) => {
            eprintln!("\nExecution failed: {e}");
            Err(e.into())
        }
    }
}

/// Load a single fixture from a JSON file.
fn load_fixture(path: &PathBuf) -> Result<StatelessExecutorFixture> {
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read fixture file: {}", path.display()))?;

    // Try parsing as single fixture first, then as array
    serde_json::from_str::<StatelessExecutorFixture>(&contents)
        .or_else(|_| {
            // Try parsing as array and take first element
            let fixtures: Vec<StatelessExecutorFixture> = serde_json::from_str(&contents)?;
            fixtures
                .into_iter()
                .next()
                .ok_or_else(|| anyhow!("Empty fixture array"))
                .map_err(|e| serde_json::Error::io(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))
        })
        .with_context(|| format!("Failed to parse fixture: {}", path.display()))
}
