//! Trace file parsing and querying functionality.
//!
//! This module provides structures and methods for loading, parsing, and querying
//! EVM trace files in JSONL format.
//!
//! # Example
//!
//! ```ignore
//! use eip3155_tracer::query::TraceFile;
//! use std::path::Path;
//!
//! let trace = TraceFile::load(Path::new("trace.jsonl"))?;
//!
//! // Query struct_logs for first transaction
//! if let Some(logs) = trace.struct_logs(0) {
//!     for log in logs {
//!         println!("{}: {} (gas: {})", log.pc, log.op_name, log.gas_cost);
//!     }
//! }
//!
//! // Filter by opcode
//! let calls = trace.filter_by_opcode("STATICCALL");
//! println!("Found {} STATICCALL operations", calls.len());
//! ```

use crate::opcodes::{get_opcode_name, get_precompile_name};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Errors that can occur during trace file operations.
#[derive(Debug, thiserror::Error)]
pub enum TraceQueryError {
    /// IO error during file operations.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON parsing error.
    #[error("JSON parse error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Missing required field in trace.
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Invalid trace format.
    #[error("Invalid trace format: {0}")]
    InvalidFormat(String),

    /// Block start not found.
    #[error("Block start record not found")]
    MissingBlockStart,

    /// Block end not found.
    #[error("Block end record not found")]
    MissingBlockEnd,
}

/// Block start information from trace file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockStart {
    /// Block number.
    pub number: u64,
    /// Block hash.
    pub hash: String,
    /// Parent block hash.
    pub parent_hash: String,
    /// Block timestamp.
    pub timestamp: u64,
    /// Block gas limit.
    pub gas_limit: u64,
    /// Fee recipient (coinbase) address.
    pub beneficiary: String,
}

/// Block end information from trace file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockEnd {
    /// Total gas used in the block.
    pub total_gas_used: u64,
    /// Number of transactions in the block.
    pub transaction_count: usize,
}

/// A single struct log entry with resolved opcode name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructLogEntry {
    /// Program counter.
    pub pc: u64,
    /// Opcode byte value.
    pub op: u8,
    /// Human-readable opcode name.
    pub op_name: String,
    /// Gas remaining before this operation.
    pub gas: u64,
    /// Gas cost of this operation.
    pub gas_cost: u64,
    /// Call depth.
    pub depth: u64,
    /// Stack contents (optional).
    #[serde(default)]
    pub stack: Vec<String>,
    /// Memory contents (optional).
    #[serde(default)]
    pub memory: Vec<String>,
    /// Gas refund counter (optional).
    #[serde(default)]
    pub refund: u64,
}

/// Opcode usage statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpcodeUsage {
    /// Opcode name.
    pub name: String,
    /// Number of times executed.
    pub count: u64,
    /// Total gas cost.
    pub total_gas: u64,
    /// Average gas cost per execution.
    pub avg_gas: f64,
}

/// Precompile usage statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecompileUsage {
    /// Precompile address.
    pub address: u8,
    /// Precompile name.
    pub name: String,
    /// Number of times called.
    pub count: u64,
    /// Total gas cost.
    pub total_gas: f64,
    /// Average gas cost per call.
    pub avg_gas: f64,
}

/// Transaction summary from trace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceSummary {
    /// Total number of opcodes executed.
    pub total_opcodes: u64,
    /// Total gas cost for all opcodes.
    pub total_gas_cost: f64,
    /// Breakdown by opcode.
    pub opcode_breakdown: Vec<OpcodeUsage>,
    /// Total number of precompile calls.
    pub total_precompiles: u64,
    /// Total gas cost for precompiles.
    pub total_precompile_gas: f64,
    /// Breakdown by precompile.
    pub precompile_breakdown: Vec<PrecompileUsage>,
}

/// Accessed storage slots information.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccessedSlots {
    /// Storage reads by address and slot.
    #[serde(default)]
    pub reads: HashMap<String, HashMap<String, String>>,
    /// Storage writes by address and slot.
    #[serde(default)]
    pub writes: HashMap<String, HashMap<String, String>>,
    /// Transient storage reads.
    #[serde(default, rename = "transientReads")]
    pub transient_reads: HashMap<String, HashMap<String, String>>,
    /// Transient storage writes.
    #[serde(default, rename = "transientWrites")]
    pub transient_writes: HashMap<String, HashMap<String, String>>,
}

/// Transaction trace data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionTrace {
    /// Transaction index in block.
    pub tx_index: usize,
    /// Transaction hash.
    pub tx_hash: String,
    /// Sender address.
    pub from: String,
    /// Recipient address (if any).
    pub to: Option<String>,
    /// Gas provided.
    pub gas: u64,
    /// Gas used.
    pub gas_used: u64,
    /// Input data.
    pub input: String,
    /// Output data.
    pub output: String,
    /// ETH value transferred.
    pub value: String,
    /// Call type (CALL, CREATE, etc.).
    pub call_type: String,
    /// Whether execution ran out of gas.
    pub out_of_gas: bool,
    /// Error message (if any).
    pub error: Option<String>,
    /// Opcode-level execution logs.
    pub struct_logs: Vec<StructLogEntry>,
    /// Used opcodes summary.
    pub used_opcodes: HashMap<String, u64>,
    /// Accessed storage slots.
    pub accessed_slots: AccessedSlots,
    /// Transaction summary statistics.
    pub summary: Option<TraceSummary>,
}

/// Parsed trace file containing all queryable data.
#[derive(Debug, Clone, Serialize)]
pub struct TraceFile {
    /// Block start information.
    pub block_start: BlockStart,
    /// Transaction traces.
    pub transactions: Vec<TransactionTrace>,
    /// Block end information.
    pub block_end: BlockEnd,
}

/// Internal struct for parsing raw JSON records.
#[derive(Debug, Deserialize)]
struct RawRecord {
    #[serde(rename = "type")]
    record_type: String,
    #[serde(flatten)]
    data: serde_json::Value,
}

/// Internal struct for parsing block start.
#[derive(Debug, Deserialize)]
struct RawBlockStart {
    number: u64,
    hash: String,
    parent_hash: String,
    timestamp: u64,
    gas_limit: u64,
    beneficiary: String,
}

/// Internal struct for parsing block end.
#[derive(Debug, Deserialize)]
struct RawBlockEnd {
    total_gas_used: u64,
    transaction_count: usize,
}

/// Internal struct for parsing transaction trace.
#[derive(Debug, Deserialize)]
struct RawTransactionTrace {
    tx_index: usize,
    tx_hash: String,
    trace: serde_json::Value,
    summary: Option<serde_json::Value>,
}

impl TraceFile {
    /// Load and parse a trace file from the given path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the JSONL trace file.
    ///
    /// # Returns
    ///
    /// Returns a parsed `TraceFile` on success, or an error if parsing fails.
    pub fn load(path: &Path) -> Result<Self, TraceQueryError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut block_start: Option<BlockStart> = None;
        let mut transactions: Vec<TransactionTrace> = Vec::new();
        let mut block_end: Option<BlockEnd> = None;

        for line_result in reader.lines() {
            let line = line_result?;
            if line.trim().is_empty() {
                continue;
            }

            let record: RawRecord = serde_json::from_str(&line)?;

            match record.record_type.as_str() {
                "block_start" => {
                    let raw: RawBlockStart = serde_json::from_value(record.data)?;
                    block_start = Some(BlockStart {
                        number: raw.number,
                        hash: raw.hash,
                        parent_hash: raw.parent_hash,
                        timestamp: raw.timestamp,
                        gas_limit: raw.gas_limit,
                        beneficiary: raw.beneficiary,
                    });
                }
                "transaction_trace" => {
                    let raw: RawTransactionTrace = serde_json::from_value(record.data)?;
                    let tx = Self::parse_transaction_trace(raw)?;
                    transactions.push(tx);
                }
                "block_end" => {
                    let raw: RawBlockEnd = serde_json::from_value(record.data)?;
                    block_end = Some(BlockEnd {
                        total_gas_used: raw.total_gas_used,
                        transaction_count: raw.transaction_count,
                    });
                }
                "transaction_error" => {
                    // Handle transaction errors - skip for now or create a minimal entry
                    continue;
                }
                _ => {
                    // Unknown record type, skip
                    continue;
                }
            }
        }

        let block_start = block_start.ok_or(TraceQueryError::MissingBlockStart)?;
        let block_end = block_end.ok_or(TraceQueryError::MissingBlockEnd)?;

        Ok(TraceFile {
            block_start,
            transactions,
            block_end,
        })
    }

    /// Parse a raw transaction trace into structured format.
    fn parse_transaction_trace(
        raw: RawTransactionTrace,
    ) -> Result<TransactionTrace, TraceQueryError> {
        let trace = &raw.trace;

        // Extract basic fields from trace
        let from = trace
            .get("from")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let to = trace
            .get("to")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let gas = Self::parse_hex_u64(trace.get("gas"))?;
        let gas_used = Self::parse_hex_u64(trace.get("gasUsed"))?;

        let input = trace
            .get("input")
            .and_then(|v| v.as_str())
            .unwrap_or("0x")
            .to_string();

        let output = trace
            .get("output")
            .and_then(|v| v.as_str())
            .unwrap_or("0x")
            .to_string();

        let value = trace
            .get("value")
            .and_then(|v| v.as_str())
            .unwrap_or("0x0")
            .to_string();

        let call_type = trace
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("CALL")
            .to_string();

        let out_of_gas = trace
            .get("outOfGas")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let error = trace
            .get("error")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Parse struct_logs
        let struct_logs = Self::parse_struct_logs(trace.get("struct_logs"))?;

        // Parse usedOpcodes
        let used_opcodes = Self::parse_used_opcodes(trace.get("usedOpcodes"))?;

        // Parse accessedSlots
        let accessed_slots = Self::parse_accessed_slots(trace.get("accessedSlots"))?;

        // Parse summary
        let summary = raw.summary.map(|s| Self::parse_summary(&s)).transpose()?;

        Ok(TransactionTrace {
            tx_index: raw.tx_index,
            tx_hash: raw.tx_hash,
            from,
            to,
            gas,
            gas_used,
            input,
            output,
            value,
            call_type,
            out_of_gas,
            error,
            struct_logs,
            used_opcodes,
            accessed_slots,
            summary,
        })
    }

    /// Parse struct_logs array.
    fn parse_struct_logs(
        value: Option<&serde_json::Value>,
    ) -> Result<Vec<StructLogEntry>, TraceQueryError> {
        let arr = match value {
            Some(serde_json::Value::Array(arr)) => arr,
            _ => return Ok(Vec::new()),
        };

        let mut logs = Vec::with_capacity(arr.len());
        for item in arr {
            let pc = item.get("pc").and_then(|v| v.as_u64()).unwrap_or(0);

            // Parse opcode - can be string name or number
            let (op, op_name) = if let Some(op_val) = item.get("op") {
                if let Some(op_str) = op_val.as_str() {
                    // Try to look up the opcode value from name
                    if let Some(op_byte) = crate::opcodes::get_opcode_value(op_str) {
                        (op_byte, op_str.to_string())
                    } else {
                        // Unknown opcode name
                        (0xff, op_str.to_string())
                    }
                } else if let Some(op_num) = op_val.as_u64() {
                    let op_byte = op_num as u8;
                    (op_byte, get_opcode_name(op_byte).to_string())
                } else {
                    (0xff, "UNKNOWN".to_string())
                }
            } else {
                (0xff, "UNKNOWN".to_string())
            };

            let gas = item.get("gas").and_then(|v| v.as_u64()).unwrap_or(0);

            let gas_cost = item.get("gasCost").and_then(|v| v.as_u64()).unwrap_or(0);

            let depth = item.get("depth").and_then(|v| v.as_u64()).unwrap_or(1);

            let stack = item
                .get("stack")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default();

            let memory = item
                .get("memory")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default();

            let refund = item.get("refund").and_then(|v| v.as_u64()).unwrap_or(0);

            logs.push(StructLogEntry {
                pc,
                op,
                op_name,
                gas,
                gas_cost,
                depth,
                stack,
                memory,
                refund,
            });
        }

        Ok(logs)
    }

    /// Parse usedOpcodes map.
    fn parse_used_opcodes(
        value: Option<&serde_json::Value>,
    ) -> Result<HashMap<String, u64>, TraceQueryError> {
        let obj = match value {
            Some(serde_json::Value::Object(obj)) => obj,
            _ => return Ok(HashMap::new()),
        };

        let mut result = HashMap::new();
        for (key, val) in obj {
            // Key can be hex string like "0x60" or decimal
            let opcode_byte = if key.starts_with("0x") {
                u8::from_str_radix(&key[2..], 16).unwrap_or(0xff)
            } else {
                key.parse::<u8>().unwrap_or(0xff)
            };

            let opcode_name = get_opcode_name(opcode_byte).to_string();
            let count = val.as_u64().unwrap_or(0);

            result.insert(opcode_name, count);
        }

        Ok(result)
    }

    /// Parse accessedSlots.
    fn parse_accessed_slots(
        value: Option<&serde_json::Value>,
    ) -> Result<AccessedSlots, TraceQueryError> {
        match value {
            Some(v) => Ok(serde_json::from_value(v.clone()).unwrap_or_default()),
            None => Ok(AccessedSlots::default()),
        }
    }

    /// Parse summary.
    fn parse_summary(value: &serde_json::Value) -> Result<TraceSummary, TraceQueryError> {
        let total_opcodes = value
            .get("total_opcodes")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        let total_gas_cost = value
            .get("total_gas_cost")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let total_precompiles = value
            .get("total_precompiles")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        let total_precompile_gas = value
            .get("total_precompile_gas")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        // Parse opcode breakdown
        let opcode_breakdown = value
            .get("opcode_breakdown")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| {
                        let opcode = item.get("opcode")?.as_str()?;
                        // Convert opcode string to name if it's numeric
                        let name = if opcode.starts_with("0x")
                            || opcode.chars().all(|c| c.is_ascii_digit())
                        {
                            let byte = if opcode.starts_with("0x") {
                                u8::from_str_radix(&opcode[2..], 16).unwrap_or(0xff)
                            } else {
                                opcode.parse::<u8>().unwrap_or(0xff)
                            };
                            get_opcode_name(byte).to_string()
                        } else {
                            opcode.to_string()
                        };

                        Some(OpcodeUsage {
                            name,
                            count: item.get("count")?.as_u64()?,
                            total_gas: item.get("total_gas_cost")?.as_f64()? as u64,
                            avg_gas: item.get("average_gas_cost")?.as_f64()?,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        // Parse precompile breakdown
        let precompile_breakdown = value
            .get("precompile_breakdown")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| {
                        let address = item.get("address")?.as_u64()? as u8;
                        let name = item
                            .get("name")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| get_precompile_name(address).to_string());

                        Some(PrecompileUsage {
                            address,
                            name,
                            count: item.get("count")?.as_u64()?,
                            total_gas: item.get("total_gas_cost")?.as_f64()?,
                            avg_gas: item.get("average_gas_cost")?.as_f64()?,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(TraceSummary {
            total_opcodes,
            total_gas_cost,
            opcode_breakdown,
            total_precompiles,
            total_precompile_gas,
            precompile_breakdown,
        })
    }

    /// Parse hex string to u64.
    fn parse_hex_u64(value: Option<&serde_json::Value>) -> Result<u64, TraceQueryError> {
        match value {
            Some(serde_json::Value::String(s)) => {
                if s.starts_with("0x") {
                    u64::from_str_radix(&s[2..], 16).map_err(|_| {
                        TraceQueryError::InvalidFormat(format!("Invalid hex value: {}", s))
                    })
                } else {
                    s.parse::<u64>().map_err(|_| {
                        TraceQueryError::InvalidFormat(format!("Invalid numeric value: {}", s))
                    })
                }
            }
            Some(serde_json::Value::Number(n)) => Ok(n.as_u64().unwrap_or(0)),
            _ => Ok(0),
        }
    }

    // =========================================================================
    // Query Methods
    // =========================================================================

    /// Get block start information.
    pub fn block_info(&self) -> &BlockStart {
        &self.block_start
    }

    /// Get block end information.
    pub fn block_summary(&self) -> &BlockEnd {
        &self.block_end
    }

    /// Get the number of transactions in the trace.
    pub fn transaction_count(&self) -> usize {
        self.transactions.len()
    }

    /// Get a transaction by index.
    pub fn transaction(&self, tx_index: usize) -> Option<&TransactionTrace> {
        self.transactions.get(tx_index)
    }

    /// Get struct_logs for a specific transaction.
    pub fn struct_logs(&self, tx_index: usize) -> Option<&[StructLogEntry]> {
        self.transactions
            .get(tx_index)
            .map(|tx| tx.struct_logs.as_slice())
    }

    /// Get used opcodes for a specific transaction.
    pub fn used_opcodes(&self, tx_index: usize) -> Option<&HashMap<String, u64>> {
        self.transactions.get(tx_index).map(|tx| &tx.used_opcodes)
    }

    /// Get summary for a specific transaction.
    pub fn summary(&self, tx_index: usize) -> Option<&TraceSummary> {
        self.transactions
            .get(tx_index)
            .and_then(|tx| tx.summary.as_ref())
    }

    /// Get accessed slots for a specific transaction.
    pub fn accessed_slots(&self, tx_index: usize) -> Option<&AccessedSlots> {
        self.transactions.get(tx_index).map(|tx| &tx.accessed_slots)
    }

    /// Filter struct_logs by opcode name across all transactions.
    pub fn filter_by_opcode(&self, opcode_name: &str) -> Vec<(usize, &StructLogEntry)> {
        let target = opcode_name.to_uppercase();
        self.transactions
            .iter()
            .enumerate()
            .flat_map(|(tx_idx, tx)| {
                tx.struct_logs
                    .iter()
                    .filter(|log| log.op_name.to_uppercase() == target)
                    .map(move |log| (tx_idx, log))
            })
            .collect()
    }

    /// Filter struct_logs by gas cost range across all transactions.
    pub fn filter_by_gas_cost(&self, min: u64, max: u64) -> Vec<(usize, &StructLogEntry)> {
        self.transactions
            .iter()
            .enumerate()
            .flat_map(|(tx_idx, tx)| {
                tx.struct_logs
                    .iter()
                    .filter(|log| log.gas_cost >= min && log.gas_cost <= max)
                    .map(move |log| (tx_idx, log))
            })
            .collect()
    }

    /// Filter struct_logs by call depth.
    pub fn filter_by_depth(&self, depth: u64) -> Vec<(usize, &StructLogEntry)> {
        self.transactions
            .iter()
            .enumerate()
            .flat_map(|(tx_idx, tx)| {
                tx.struct_logs
                    .iter()
                    .filter(|log| log.depth == depth)
                    .map(move |log| (tx_idx, log))
            })
            .collect()
    }

    /// Get all CALL-type operations (CALL, STATICCALL, DELEGATECALL, CALLCODE).
    pub fn get_call_operations(&self) -> Vec<(usize, &StructLogEntry)> {
        self.transactions
            .iter()
            .enumerate()
            .flat_map(|(tx_idx, tx)| {
                tx.struct_logs
                    .iter()
                    .filter(|log| crate::opcodes::is_call_opcode(log.op))
                    .map(move |log| (tx_idx, log))
            })
            .collect()
    }

    /// Get aggregate statistics across all transactions.
    pub fn aggregate_stats(&self) -> AggregateStats {
        let mut total_opcodes = 0u64;
        let mut total_gas = 0.0f64;
        let mut opcode_counts: HashMap<String, u64> = HashMap::new();
        let mut opcode_gas: HashMap<String, f64> = HashMap::new();

        for tx in &self.transactions {
            for log in &tx.struct_logs {
                total_opcodes += 1;
                total_gas += log.gas_cost as f64;

                *opcode_counts.entry(log.op_name.clone()).or_insert(0) += 1;
                *opcode_gas.entry(log.op_name.clone()).or_insert(0.0) += log.gas_cost as f64;
            }
        }

        let opcode_breakdown: Vec<OpcodeUsage> = opcode_counts
            .into_iter()
            .map(|(name, count)| {
                let gas = opcode_gas.get(&name).copied().unwrap_or(0.0);
                OpcodeUsage {
                    name,
                    count,
                    total_gas: gas as u64,
                    avg_gas: if count > 0 { gas / count as f64 } else { 0.0 },
                }
            })
            .collect();

        AggregateStats {
            total_transactions: self.transactions.len(),
            total_opcodes,
            total_gas,
            opcode_breakdown,
        }
    }

    /// Export the trace file to JSON.
    pub fn to_json(&self) -> Result<String, TraceQueryError> {
        serde_json::to_string_pretty(self).map_err(TraceQueryError::from)
    }
}

/// Aggregate statistics across all transactions.
#[derive(Debug, Clone, Serialize)]
pub struct AggregateStats {
    /// Total number of transactions.
    pub total_transactions: usize,
    /// Total opcode executions.
    pub total_opcodes: u64,
    /// Total gas used by opcodes.
    pub total_gas: f64,
    /// Breakdown by opcode.
    pub opcode_breakdown: Vec<OpcodeUsage>,
}

/// Query result wrapper with metadata.
#[derive(Debug, Clone, Serialize)]
pub struct QueryResult<T> {
    /// The query result data.
    pub data: T,
    /// Query metadata.
    pub metadata: QueryMetadata,
}

/// Metadata about a query execution.
#[derive(Debug, Clone, Serialize)]
pub struct QueryMetadata {
    /// Type of query performed.
    pub query_type: String,
    /// Number of records returned.
    pub record_count: usize,
}

impl<T> QueryResult<T> {
    /// Create a new query result.
    pub fn new(data: T, query_type: &str, record_count: usize) -> Self {
        QueryResult {
            data,
            metadata: QueryMetadata {
                query_type: query_type.to_string(),
                record_count,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_u64() {
        assert_eq!(
            TraceFile::parse_hex_u64(Some(&serde_json::json!("0x186a0"))).unwrap(),
            100000
        );
        assert_eq!(
            TraceFile::parse_hex_u64(Some(&serde_json::json!(100000))).unwrap(),
            100000
        );
        assert_eq!(
            TraceFile::parse_hex_u64(Some(&serde_json::json!("100000"))).unwrap(),
            100000
        );
    }
}
