//! CLI definitions for the EIP-3155 tracer.

use clap::Parser;
use std::path::PathBuf;

/// CLI tool for generating EIP-3155 opcode traces from stateless executor fixtures.
#[derive(Parser, Debug)]
#[command(name = "eip3155-trace")]
#[command(about = "Generate EIP-3155 opcode traces from stateless executor fixtures")]
#[command(version)]
pub(crate) struct Cli {
    /// Input folder containing fixture JSON files.
    #[arg(short, long, default_value = "zkevm-fixtures-input", conflicts_with = "input_file")]
    pub input_folder: PathBuf,

    /// Input file for a single fixture (overrides input_folder).
    #[arg(long)]
    pub input_file: Option<PathBuf>,

    /// Output folder for trace files.
    #[arg(short, long, default_value = "zkevm-fixtures-traces")]
    pub output_folder: PathBuf,

    /// Include memory snapshots in the trace (increases output size significantly).
    #[arg(long, default_value_t = false)]
    pub include_memory: bool,

    /// Include storage changes in the trace.
    #[arg(long, default_value_t = false)]
    pub include_storage: bool,

    /// Pretty-print JSON output.
    #[arg(long, default_value_t = true)]
    pub pretty_print: bool,
}
