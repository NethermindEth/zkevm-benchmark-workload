//! Trace output formatting for EIP-3155 compliance.
//!
//! This module provides types and utilities for writing EIP-3155 compliant
//! execution traces in JSONL format.

use alloy_consensus::BlockHeader;
use alloy_primitives::B256;

use alloy_rpc_types_trace::geth::GethTrace;
use alloy_rpc_types_trace::geth::erc7562::Erc7562Frame;
use reth_ethereum_primitives::Block;
use reth_primitives_traits::RecoveredBlock;
use serde::Serialize;
use std::collections::HashMap;
use std::io::Write;

/// Trace output configuration.
///
/// By default, only minimal fields are included: `pc`, `op` (opcode), and `gasCost`.
/// All other fields can be enabled as needed.
#[derive(Debug, Clone)]
pub struct TraceOutput {
    /// Include stack snapshots in trace.
    pub include_stack: bool,
    /// Include memory snapshots in trace.
    pub include_memory: bool,
    /// Include storage changes in trace.
    pub include_storage: bool,
    /// Include return data in trace.
    pub include_return_data: bool,
    /// Include gas remaining in trace (not just gasCost).
    pub include_gas: bool,
    /// Include call depth in trace.
    pub include_depth: bool,
    /// Include gas refund counter in trace.
    pub include_refund: bool,
    /// Include summary statistics in trace.
    pub include_summary: bool,
    /// Generate only summary statistics (no structLogs).
    pub summary_only: bool,
    /// Pretty-print JSON output.
    pub pretty_print: bool,
}

impl Default for TraceOutput {
    fn default() -> Self {
        Self {
            // Default: only pc, op, and gasCost are included
            include_stack: false,
            include_memory: false,
            include_storage: false,
            include_return_data: false,
            include_gas: false,
            include_depth: false,
            include_refund: false,
            include_summary: false,
            summary_only: false,
            pretty_print: true,
        }
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
        tx_hash: &B256,
        trace: &GethTrace,
        summary: Option<TransactionSummary>,
    ) -> std::io::Result<()> {
        #[derive(Serialize)]
        struct TransactionTrace<'a> {
            #[serde(rename = "type")]
            type_: &'static str,
            tx_index: usize,
            tx_hash: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            trace: Option<&'a GethTrace>,
            #[serde(skip_serializing_if = "Option::is_none")]
            summary: Option<&'a TransactionSummary>,
        }

        let trace_ref = if self.config.summary_only {
            None
        } else {
            Some(trace)
        };

        let output = TransactionTrace {
            type_: "transaction_trace",
            tx_index,
            tx_hash: format!("{:?}", tx_hash),
            trace: trace_ref,
            summary: summary.as_ref(),
        };
        self.write_json(&output)
    }

    /// Write a transaction error.
    pub fn write_transaction_error(
        &mut self,
        tx_index: usize,
        tx_hash: &B256,
        error: &str,
    ) -> std::io::Result<()> {
        #[derive(Serialize)]
        struct TransactionError<'a> {
            #[serde(rename = "type")]
            type_: &'static str,
            tx_index: usize,
            tx_hash: String,
            error: &'a str,
        }

        let output = TransactionError {
            type_: "transaction_error",
            tx_index,
            tx_hash: format!("{:?}", tx_hash),
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

/// Summary statistics for opcode execution.
#[derive(Debug, Clone, Serialize)]
pub struct OpcodeSummary {
    /// Opcode name (e.g., "PUSH1", "SSTORE").
    pub opcode: String,
    /// Number of times this opcode was executed.
    pub count: u64,
    /// Total gas cost for all executions of this opcode.
    pub total_gas_cost: u64,
    /// Average gas cost per execution.
    pub average_gas_cost: f64,
}

/// Summary statistics for a transaction.
#[derive(Debug, Clone, Serialize)]
pub struct TransactionSummary {
    /// Total number of opcodes executed.
    pub total_opcodes: u64,
    /// Total gas cost for all opcodes.
    pub total_gas_cost: u64,
    /// Breakdown by opcode.
    pub opcode_breakdown: Vec<OpcodeSummary>,
}

/// Accumulator for generating summary statistics.
#[derive(Debug, Default)]
pub struct SummaryAccumulator {
    /// Map of opcode to (count, total_gas_cost).
    opcode_data: HashMap<String, (u64, u64)>,
    /// Total number of opcodes executed.
    total_opcodes: u64,
    /// Total gas cost for all opcodes.
    total_gas_cost: u64,
}

impl SummaryAccumulator {
    /// Create a new summary accumulator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Process a Geth trace and accumulate statistics.
    pub fn process_trace(&mut self, frames: &[Erc7562Frame]) {
        for frame in frames {
            for (&opcode, &count) in &frame.used_opcodes {
                let opcode_name = format!("0x{:x}", opcode);
                self.opcode_data.entry(opcode_name).or_insert((0, 0)).0 += count;
                self.total_opcodes += count as u64;
                // gas_cost not available, so total_gas_cost remains 0
            }
        }
    }

    /// Process used opcodes from an Erc7562Frame and accumulate statistics.
    pub fn process_used_opcodes(&mut self, used_opcodes: &alloy_primitives::map::HashMap<u8, u64>) {
        for (&opcode, &count) in used_opcodes {
            let opcode_name = format!("0x{:x}", opcode);
            self.opcode_data.entry(opcode_name).or_insert((0, 0)).0 += count;
            self.total_opcodes += count as u64;
        }
    }

    /// Accumulate data for a single opcode execution.
    fn accumulate_opcode(&mut self, opcode: &str, gas_cost: u64) {
        let (count, total_cost) = self.opcode_data.entry(opcode.to_string()).or_insert((0, 0));

        *count += 1;
        *total_cost += gas_cost;
        self.total_opcodes += 1;
        self.total_gas_cost += gas_cost;
    }

    /// Generate the final summary.
    pub fn generate_summary(&self) -> TransactionSummary {
        let mut opcode_breakdown: Vec<OpcodeSummary> = self
            .opcode_data
            .iter()
            .map(|(opcode, &(count, total_cost))| OpcodeSummary {
                opcode: opcode.clone(),
                count,
                total_gas_cost: total_cost,
                average_gas_cost: if count > 0 {
                    total_cost as f64 / count as f64
                } else {
                    0.0
                },
            })
            .collect();

        // Sort by total gas cost descending (most expensive opcodes first)
        opcode_breakdown.sort_by(|a, b| b.total_gas_cost.cmp(&a.total_gas_cost));

        TransactionSummary {
            total_opcodes: self.total_opcodes,
            total_gas_cost: self.total_gas_cost,
            opcode_breakdown,
        }
    }

    /// Reset the accumulator for a new transaction.
    pub fn reset(&mut self) {
        self.opcode_data.clear();
        self.total_opcodes = 0;
        self.total_gas_cost = 0;
    }
}
