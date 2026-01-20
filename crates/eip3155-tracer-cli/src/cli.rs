//! CLI definitions for the EIP-3155 tracer.

use clap::Parser;
use std::path::PathBuf;

/// CLI tool for generating EIP-3155 opcode traces from stateless executor fixtures.
///
/// By default, only minimal fields are included in the trace output:
/// `pc` (program counter), `op` (opcode), and `gasCost`.
///
/// Use the various `--include-*` flags to enable additional fields.
#[derive(Parser, Debug)]
#[command(name = "eip3155-trace")]
#[command(about = "Generate EIP-3155 opcode traces from stateless executor fixtures")]
#[command(version)]
pub(crate) struct Cli {
    /// Input folder containing fixture JSON files.
    #[arg(
        short,
        long,
        default_value = "zkevm-fixtures-input",
        conflicts_with = "input_file"
    )]
    pub input_folder: PathBuf,

    /// Input file for a single fixture (overrides input_folder).
    #[arg(long)]
    pub input_file: Option<PathBuf>,

    /// Output folder for trace files.
    #[arg(short, long, default_value = "zkevm-fixtures-traces")]
    pub output_folder: PathBuf,

    // === Trace Field Options ===
    // By default only pc, op (opcode), and gasCost are included.
    // Use these flags to enable additional fields.
    /// Include stack snapshots in the trace.
    #[arg(long)]
    pub include_stack: bool,

    /// Include memory snapshots in the trace (increases output size significantly).
    #[arg(long)]
    pub include_memory: bool,

    /// Include storage changes in the trace.
    #[arg(long)]
    pub include_storage: bool,

    /// Include return data in the trace.
    #[arg(long)]
    pub include_return_data: bool,

    /// Include gas remaining in the trace (not just gasCost).
    #[arg(long)]
    pub include_gas: bool,

    /// Include call depth in the trace.
    #[arg(long)]
    pub include_depth: bool,

    /// Include gas refund counter in the trace.
    #[arg(long)]
    pub include_refund: bool,

    /// Include summary statistics in trace.
    #[arg(long)]
    pub include_summary: bool,

    /// Generate only summary statistics (no structLogs).
    #[arg(long)]
    pub summary_only: bool,

    /// Minimal output: only write the summary object (no transaction wrapper, block markers, or traces).
    #[arg(long)]
    pub minimal: bool,

    /// Include all optional fields in the trace.
    #[arg(long, conflicts_with_all = ["include_stack", "include_memory", "include_storage", "include_return_data", "include_gas", "include_depth", "include_refund", "include_summary", "summary_only"])]
    pub full: bool,

    // === Output Format Options ===
    /// Pretty-print JSON output.
    #[arg(long)]
    pub pretty_print: bool,
}
