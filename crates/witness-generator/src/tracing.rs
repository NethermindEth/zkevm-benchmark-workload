//! EVM instruction tracing for generating EIP-3155 compatible traces.
//!
//! This module provides functionality to trace EVM execution and produce
//! EIP-3155 formatted traces containing opcodes and precompile calls.

use std::{collections::HashMap, fs, path::Path, sync::Arc};

use alloy_genesis::Genesis;
use alloy_primitives::{Address, Bytes, B256, U256};
use reth_chainspec::ChainSpec;
use reth_evm_ethereum::EthEvmConfig;
use reth_stateless::StatelessInput;
use revm_inspectors::tracing::{TracingInspector, TracingInspectorConfig};
use serde::{Deserialize, Serialize};

use crate::{Result, WGError};

/// EIP-3155 compatible execution trace for a block.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionTrace {
    /// The name of the fixture this trace belongs to.
    pub fixture_name: String,
    /// Block number that was traced.
    pub block_number: u64,
    /// Transaction traces within the block.
    pub transactions: Vec<TransactionTrace>,
}

/// Trace for a single transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionTrace {
    /// Transaction hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<B256>,
    /// Transaction index in the block.
    pub index: usize,
    /// Whether the transaction succeeded.
    pub success: bool,
    /// Gas used by the transaction.
    pub gas_used: u64,
    /// Structured execution logs (EIP-3155 format).
    pub struct_logs: Vec<StructLog>,
    /// Precompile calls made during execution.
    pub precompile_calls: Vec<PrecompileCall>,
}

/// A single step in the execution trace (EIP-3155 format).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructLog {
    /// Program counter.
    pub pc: u64,
    /// Opcode name.
    pub op: String,
    /// Gas remaining before this operation.
    pub gas: u64,
    /// Gas cost of this operation.
    pub gas_cost: u64,
    /// Call depth.
    pub depth: u64,
    /// Stack state (optional, can be large).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<Vec<U256>>,
    /// Memory state (optional, can be very large).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<Vec<String>>,
    /// Storage changes (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<HashMap<U256, U256>>,
    /// Return data (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_data: Option<Bytes>,
    /// Error message if the step failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Record of a precompile call.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrecompileCall {
    /// Precompile contract address (0x01 - 0x09 for standard precompiles).
    pub address: Address,
    /// Input data to the precompile.
    pub input: Bytes,
    /// Output data from the precompile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Bytes>,
    /// Gas used by the precompile.
    pub gas_used: u64,
    /// Whether the precompile call succeeded.
    pub success: bool,
}

impl ExecutionTrace {
    /// Writes the trace to a JSON file at the specified path.
    ///
    /// The file will be named `{fixture_name}.trace.json`.
    pub fn write_to_path(&self, dir: &Path) -> Result<()> {
        let file_path = dir.join(format!("{}.trace.json", self.fixture_name));
        let json = serde_json::to_string_pretty(self).map_err(|e| WGError::Other(Box::new(e)))?;
        fs::write(&file_path, json).map_err(|e| WGError::FixtureWriteError {
            path: file_path.display().to_string(),
            source: e,
        })?;
        Ok(())
    }
}

/// Configuration for trace generation.
#[derive(Debug, Clone, Default)]
pub struct TraceConfig {
    /// Include stack in trace output.
    pub include_stack: bool,
    /// Include memory in trace output (can be very large).
    pub include_memory: bool,
    /// Include storage changes in trace output.
    pub include_storage: bool,
}

impl TraceConfig {
    /// Creates a minimal trace config (opcodes and gas only).
    pub const fn minimal() -> Self {
        Self {
            include_stack: false,
            include_memory: false,
            include_storage: false,
        }
    }

    /// Creates a full trace config with all details.
    pub const fn full() -> Self {
        Self {
            include_stack: true,
            include_memory: true,
            include_storage: true,
        }
    }
}

/// Generates an EIP-3155 compatible execution trace from a stateless input.
///
/// This function re-executes the block using the witness data and captures
/// a detailed trace of all EVM operations.
///
/// # Arguments
///
/// * `input` - The stateless input containing block, witness, and chain config
/// * `fixture_name` - Name to associate with this trace
/// * `config` - Configuration for what details to include in the trace
///
/// # Returns
///
/// Returns an `ExecutionTrace` containing EIP-3155 formatted logs for each transaction.
pub fn generate_trace(
    input: &StatelessInput,
    fixture_name: &str,
    _config: &TraceConfig,
) -> Result<ExecutionTrace> {
    // Build chain spec from config
    let genesis = Genesis {
        config: input.chain_config.clone(),
        ..Default::default()
    };
    let chain_spec: Arc<ChainSpec> = Arc::new(genesis.into());
    let _evm_config = EthEvmConfig::new(chain_spec.clone());

    // Create tracing inspector with EIP-3155 config
    let _inspector_config = TracingInspectorConfig::default_geth();
    let _inspector = TracingInspector::new(_inspector_config);

    // TODO: Re-execute the block with the tracing inspector
    // This requires building the witness database and executing with the inspector.
    // For now, we return a placeholder trace indicating the structure.
    //
    // The full implementation would:
    // 1. Build state from witness using reth_stateless utilities
    // 2. Create an executor with the tracing inspector
    // 3. Execute each transaction and capture the trace
    // 4. Convert the trace to our EIP-3155 format

    let transactions = input
        .block
        .body
        .transactions()
        .enumerate()
        .map(|(index, tx)| {
            TransactionTrace {
                hash: Some(*tx.tx_hash()),
                index,
                success: true, // Placeholder
                gas_used: 0,   // Placeholder
                struct_logs: Vec::new(),
                precompile_calls: Vec::new(),
            }
        })
        .collect();

    Ok(ExecutionTrace {
        fixture_name: fixture_name.to_string(),
        block_number: input.block.header.number,
        transactions,
    })
}

/// Standard Ethereum precompile addresses.
pub mod precompiles {
    use alloy_primitives::Address;

    /// ECRECOVER precompile (0x01).
    pub const ECRECOVER: Address = Address::new([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ]);
    /// SHA256 precompile (0x02).
    pub const SHA256: Address = Address::new([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
    ]);
    /// RIPEMD160 precompile (0x03).
    pub const RIPEMD160: Address = Address::new([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3,
    ]);
    /// Identity precompile (0x04).
    pub const IDENTITY: Address = Address::new([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    ]);
    /// MODEXP precompile (0x05).
    pub const MODEXP: Address = Address::new([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5,
    ]);
    /// BN_ADD precompile (0x06).
    pub const BN_ADD: Address = Address::new([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6,
    ]);
    /// BN_MUL precompile (0x07).
    pub const BN_MUL: Address = Address::new([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7,
    ]);
    /// BN_PAIRING precompile (0x08).
    pub const BN_PAIRING: Address = Address::new([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8,
    ]);
    /// BLAKE2F precompile (0x09).
    pub const BLAKE2F: Address = Address::new([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9,
    ]);
    /// Point evaluation precompile (0x0a) - EIP-4844.
    pub const POINT_EVALUATION: Address = Address::new([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10,
    ]);

    /// Check if an address is a standard precompile.
    pub fn is_precompile(address: &Address) -> bool {
        let bytes = address.as_slice();
        // Check if first 19 bytes are zero and last byte is 1-10
        bytes[..19].iter().all(|&b| b == 0) && bytes[19] >= 1 && bytes[19] <= 10
    }

    /// Get the name of a precompile from its address.
    pub fn name(address: &Address) -> Option<&'static str> {
        if !is_precompile(address) {
            return None;
        }
        match address.as_slice()[19] {
            1 => Some("ecrecover"),
            2 => Some("sha256"),
            3 => Some("ripemd160"),
            4 => Some("identity"),
            5 => Some("modexp"),
            6 => Some("bn_add"),
            7 => Some("bn_mul"),
            8 => Some("bn_pairing"),
            9 => Some("blake2f"),
            10 => Some("point_evaluation"),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precompile_detection() {
        assert!(precompiles::is_precompile(&precompiles::ECRECOVER));
        assert!(precompiles::is_precompile(&precompiles::MODEXP));
        assert!(!precompiles::is_precompile(&Address::ZERO));
    }

    #[test]
    fn test_precompile_names() {
        assert_eq!(precompiles::name(&precompiles::ECRECOVER), Some("ecrecover"));
        assert_eq!(precompiles::name(&precompiles::SHA256), Some("sha256"));
        assert_eq!(precompiles::name(&Address::ZERO), None);
    }
}

