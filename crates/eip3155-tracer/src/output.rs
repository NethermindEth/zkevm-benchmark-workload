//! Trace output formatting for EIP-3155 compliance.
//!
//! This module provides types and utilities for writing EIP-3155 compliant
//! execution traces in JSONL format.

use alloy_consensus::BlockHeader;
use alloy_rpc_types_trace::geth::GethTrace;
use reth_ethereum_primitives::Block;
use reth_primitives_traits::RecoveredBlock;
use serde::Serialize;
use std::io::Write;

/// Trace output configuration.
#[derive(Debug, Clone)]
pub struct TraceOutput {
    /// Include memory snapshots in trace.
    pub include_memory: bool,
    /// Include storage changes in trace.
    pub include_storage: bool,
    /// Pretty-print JSON output.
    pub pretty_print: bool,
}

impl Default for TraceOutput {
    fn default() -> Self {
        Self {
            include_memory: false,
            include_storage: true,
            pretty_print: false,
        }
    }
}

impl TraceOutput {
    /// Create a new trace output configuration with memory snapshots enabled.
    #[must_use]
    pub const fn with_memory(mut self) -> Self {
        self.include_memory = true;
        self
    }

    /// Create a new trace output configuration with storage changes enabled.
    #[must_use]
    pub const fn with_storage(mut self) -> Self {
        self.include_storage = true;
        self
    }

    /// Create a new trace output configuration with pretty printing enabled.
    #[must_use]
    pub const fn with_pretty_print(mut self) -> Self {
        self.pretty_print = true;
        self
    }
}

/// Writer for EIP-3155 trace output.
///
/// This struct wraps an underlying writer and provides methods for writing
/// structured trace output in JSONL format.
#[derive(Debug)]
pub struct TraceWriter<W: Write> {
    writer: W,
    config: TraceOutput,
}

impl<W: Write> TraceWriter<W> {
    /// Create a new trace writer.
    pub const fn new(writer: W, config: TraceOutput) -> Self {
        Self { writer, config }
    }

    /// Get a reference to the underlying writer.
    pub const fn inner(&self) -> &W {
        &self.writer
    }

    /// Get a mutable reference to the underlying writer.
    pub fn inner_mut(&mut self) -> &mut W {
        &mut self.writer
    }

    /// Consume the writer and return the underlying writer.
    pub fn into_inner(self) -> W {
        self.writer
    }

    /// Get a reference to the trace output configuration.
    pub const fn config(&self) -> &TraceOutput {
        &self.config
    }

    /// Write block start marker with header information.
    pub fn write_block_start(&mut self, block: &RecoveredBlock<Block>) -> std::io::Result<()> {
        #[derive(Serialize)]
        struct BlockStartData {
            #[serde(rename = "type")]
            type_: &'static str,
            number: u64,
            hash: String,
            parent_hash: String,
            timestamp: u64,
            gas_limit: u64,
            beneficiary: String,
        }

        let header = BlockStartData {
            type_: "block_start",
            number: block.header().number(),
            hash: format!("{:?}", block.hash()),
            parent_hash: format!("{:?}", block.header().parent_hash()),
            timestamp: block.header().timestamp(),
            gas_limit: block.header().gas_limit(),
            beneficiary: format!("{:?}", block.header().beneficiary()),
        };
        self.write_json(&header)
    }

    /// Write a full transaction trace in EIP-3155 format.
    pub fn write_transaction_trace(
        &mut self,
        tx_index: usize,
        tx_hash: &str,
        trace: &GethTrace,
    ) -> std::io::Result<()> {
        #[derive(Serialize)]
        struct TransactionTrace<'a> {
            #[serde(rename = "type")]
            type_: &'static str,
            tx_index: usize,
            tx_hash: &'a str,
            trace: &'a GethTrace,
        }

        let output = TransactionTrace {
            type_: "transaction_trace",
            tx_index,
            tx_hash,
            trace,
        };
        self.write_json(&output)
    }

    /// Write a transaction error.
    pub fn write_transaction_error(
        &mut self,
        tx_index: usize,
        tx_hash: &str,
        error: &str,
    ) -> std::io::Result<()> {
        #[derive(Serialize)]
        struct TransactionError<'a> {
            #[serde(rename = "type")]
            type_: &'static str,
            tx_index: usize,
            tx_hash: &'a str,
            error: &'a str,
        }

        let output = TransactionError {
            type_: "transaction_error",
            tx_index,
            tx_hash,
            error,
        };
        self.write_json(&output)
    }

    /// Write block end marker with summary.
    pub fn write_block_end(&mut self, gas_used: u64, tx_count: usize) -> std::io::Result<()> {
        #[derive(Serialize)]
        struct BlockEnd {
            #[serde(rename = "type")]
            type_: &'static str,
            total_gas_used: u64,
            transaction_count: usize,
        }

        let summary = BlockEnd {
            type_: "block_end",
            total_gas_used: gas_used,
            transaction_count: tx_count,
        };
        self.write_json(&summary)
    }

    /// Write a raw JSON value.
    fn write_json<T: Serialize>(&mut self, value: &T) -> std::io::Result<()> {
        let json = if self.config.pretty_print {
            serde_json::to_string_pretty(value)
        } else {
            serde_json::to_string(value)
        }
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        writeln!(self.writer, "{json}")
    }

    /// Flush the underlying writer.
    pub fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
