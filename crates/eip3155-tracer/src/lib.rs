//! EIP-3155 compliant EVM execution tracer.
//!
//! This crate provides full EIP-3155 tracing for stateless block execution,
//! outputting detailed opcode-level traces suitable for debugging and analysis.
//!
//! # Overview
//!
//! EIP-3155 defines a standard trace output format that includes detailed information
//! for each EVM opcode executed:
//! - `pc` - Program counter
//! - `op` - Opcode number
//! - `opName` - Opcode name
//! - `gas` - Gas remaining
//! - `gasCost` - Gas cost of this operation
//! - `stack` - Current stack state
//! - `memory` - Memory contents (optional)
//! - `storage` - Storage changes (optional)
//! - `depth` - Call depth
//! - `refund` - Gas refund counter
//! - `error` - Error message if any
//!
//! # Usage
//!
//! ```ignore
//! use eip3155_tracer::{trace_block, TraceOutput, TraceWriter};
//! use std::fs::File;
//! use std::io::BufWriter;
//!
//! let output_file = File::create("trace.jsonl")?;
//! let config = TraceOutput::default();
//! let mut writer = TraceWriter::new(BufWriter::new(output_file), config);
//!
//! let result = trace_block(
//!     block,
//!     public_keys,
//!     witness,
//!     chain_spec,
//!     &mut writer,
//! )?;
//!
//! println!("Executed {} transactions, gas used: {}",
//!          result.transaction_count, result.gas_used);
//! ```

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/paradigmxyz/reth/main/assets/reth-docs.png",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/97369466?s=256"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

mod output;
mod tracer;
mod witness_db;

pub use output::{OpcodeSummary, TraceOutput, TraceWriter, TransactionSummary};
pub use tracer::{TracedExecution, TracedExecutionError, trace_block};

// Re-export types that users will need
pub use reth_stateless::{ExecutionWitness, Genesis, StatelessInput, UncompressedPublicKey};
