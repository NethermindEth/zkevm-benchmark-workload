//! EIP-3155 opcode-level tracing for stateless executor fixtures.
//!
//! This module provides functionality to generate detailed opcode-level traces
//! from stateless executor fixtures using the [`eip3155_tracer`] crate.

use anyhow::{Context, Result};
use eip3155_tracer::{TraceOutput, TraceWriter, trace_block};
use guest_libs::senders::recover_signers;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::Path;
use tracing::info;
use walkdir::WalkDir;
use witness_generator::StatelessExecutorFixture;

/// Configuration for opcode tracing.
#[derive(Debug, Clone)]
pub struct TraceConfig {
    /// Include memory snapshots in the trace (increases output size significantly).
    pub include_memory: bool,
    /// Include storage changes in the trace.
    pub include_storage: bool,
    /// Pretty-print JSON output.
    pub pretty_print: bool,
}

impl Default for TraceConfig {
    fn default() -> Self {
        Self {
            include_memory: false,
            include_storage: false,
            pretty_print: true,
        }
    }
}

/// Result of tracing a single fixture.
#[derive(Debug)]
pub struct TraceResult {
    /// Name of the traced fixture.
    pub name: String,
    /// Number of transactions traced.
    pub transaction_count: usize,
    /// Total gas used across all transactions.
    pub gas_used: u64,
    /// Whether all transactions executed successfully.
    pub success: bool,
}

/// Traces all fixtures in a folder, writing output to the specified directory.
///
/// Each fixture is traced and output to `{output_folder}/{fixture_name}.jsonl`.
///
/// # Arguments
///
/// * `input_folder` - Path to folder containing fixture JSON files
/// * `output_folder` - Path to folder where trace files will be written
/// * `config` - Configuration for the trace output
///
/// # Returns
///
/// A vector of trace results for each fixture processed.
pub fn trace_fixtures_folder(
    input_folder: &Path,
    output_folder: &Path,
    config: &TraceConfig,
) -> Result<Vec<TraceResult>> {
    trace_fixtures(input_folder, None, output_folder, config)
}

/// Traces fixtures from either a folder or a single file.
///
/// # Arguments
///
/// * `input_folder` - Path to folder containing fixture JSON files (used if `input_file` is None)
/// * `input_file` - Optional path to a single fixture file
/// * `output_folder` - Path to folder where trace files will be written
/// * `config` - Configuration for the trace output
///
/// # Returns
///
/// A vector of trace results for each fixture processed.
pub fn trace_fixtures(
    input_folder: &Path,
    input_file: Option<&Path>,
    output_folder: &Path,
    config: &TraceConfig,
) -> Result<Vec<TraceResult>> {
    // Ensure output directory exists
    fs::create_dir_all(output_folder)
        .with_context(|| format!("Failed to create output directory: {}", output_folder.display()))?;

    let fixtures = match input_file {
        Some(file) => vec![read_fixture_file(file)?],
        None => read_fixtures_folder(input_folder)?,
    };

    info!("Tracing {} fixtures", fixtures.len());

    // Process fixtures in parallel
    fixtures
        .into_par_iter()
        .map(|fixture| trace_single_fixture(&fixture, output_folder, config))
        .collect()
}

/// Traces a single fixture and writes the output to a file.
fn trace_single_fixture(
    fixture: &StatelessExecutorFixture,
    output_folder: &Path,
    config: &TraceConfig,
) -> Result<TraceResult> {
    let output_path = output_folder.join(format!("{}.jsonl", fixture.name));

    // Skip if output already exists
    if output_path.exists() {
        info!("Skipping {} (trace already exists)", fixture.name);
        // Return a placeholder result - we could also read the existing file
        return Ok(TraceResult {
            name: fixture.name.clone(),
            transaction_count: fixture.stateless_input.block.body.transactions.len(),
            gas_used: 0,
            success: true,
        });
    }

    info!("Tracing {}", fixture.name);

    // Recover public keys from transactions
    let public_keys = recover_signers(&fixture.stateless_input.block.body.transactions)
        .with_context(|| format!("Failed to recover signers for fixture: {}", fixture.name))?;

    // Create trace output configuration
    let trace_output = TraceOutput {
        include_memory: config.include_memory,
        include_storage: config.include_storage,
        pretty_print: config.pretty_print,
    };

    // Create output file
    let file = File::create(&output_path)
        .with_context(|| format!("Failed to create output file: {}", output_path.display()))?;
    let mut writer = TraceWriter::new(BufWriter::new(file), trace_output);

    // Run the tracer
    let result = trace_block(
        fixture.stateless_input.block.clone().into(),
        public_keys,
        fixture.stateless_input.witness.clone().into(),
        fixture.stateless_input.chain_config.clone(),
        &mut writer,
    )
    .with_context(|| format!("Failed to trace fixture: {}", fixture.name))?;

    info!(
        "Traced {} ({} txs, {} gas)",
        fixture.name, result.transaction_count, result.gas_used
    );

    Ok(TraceResult {
        name: fixture.name.clone(),
        transaction_count: result.transaction_count,
        gas_used: result.gas_used,
        success: result.success,
    })
}

/// Reads a single fixture file.
fn read_fixture_file(path: &Path) -> Result<StatelessExecutorFixture> {
    let content =
        fs::read(path).with_context(|| format!("Failed to read file: {}", path.display()))?;
    serde_json::from_slice(&content).with_context(|| format!("Failed to parse {}", path.display()))
}

/// Reads all fixture files from a folder.
fn read_fixtures_folder(path: &Path) -> Result<Vec<StatelessExecutorFixture>> {
    WalkDir::new(path)
        .min_depth(1)
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?
        .into_par_iter()
        .filter_map(|entry| {
            if entry.file_type().is_file()
                && entry.path().extension().is_some_and(|ext| ext == "json")
            {
                Some(entry)
            } else {
                None
            }
        })
        .map(|entry| {
            let content = fs::read(entry.path())?;
            let fixture: StatelessExecutorFixture = serde_json::from_slice(&content)
                .map_err(|e| anyhow::anyhow!("Failed to parse {}: {}", entry.path().display(), e))?;
            Ok(fixture)
        })
        .collect()
}
