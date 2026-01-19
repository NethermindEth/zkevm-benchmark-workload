//! Core tracing implementation using revm-inspectors.
//!
//! This module provides the main tracing functionality for executing blocks
//! with full EIP-3155 opcode-level tracing using `TracingInspector`.

use crate::output::{SummaryAccumulator, TraceWriter};
use crate::witness_db::WitnessDatabase;
use alloy_consensus::{BlockHeader, Header};
use alloy_genesis::ChainConfig;
use alloy_primitives::{Address, B256, keccak256, map::HashMap};
use alloy_rpc_types_trace::geth::GethTrace;
use alloy_rpc_types_trace::geth::StructLog;
use alloy_rpc_types_trace::geth::erc7562::{AccessedSlots, CallFrameType, Erc7562Frame};
use reth_chainspec::EthereumHardforks;
use serde::{Deserialize, Serialize};

/// Custom ERC-7562 frame that includes opcode-level structLogs for detailed tracing.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomErc7562Frame {
    #[serde(flatten)]
    pub base: Erc7562Frame,
    pub struct_logs: Vec<StructLog>,
}
use reth_ethereum_primitives::{Block, TransactionSigned};
use reth_evm::ConfigureEvm;
use reth_evm_ethereum::EthEvmConfig;
use reth_primitives_traits::{Block as _, Recovered, RecoveredBlock, SealedHeader};
use reth_revm::{DatabaseCommit, InspectEvm, MainBuilder, MainContext, State};
use reth_stateless::{ExecutionWitness, Genesis, UncompressedPublicKey, trie::StatelessTrie};
use revm_inspectors::tracing::{StackSnapshotType, TracingInspector, TracingInspectorConfig};
use serde_json;
use sparsestate::SparseState;
use std::{collections::BTreeMap, io::Write, sync::Arc};

/// Errors that can occur during traced execution.
#[derive(Debug, thiserror::Error)]
pub enum TracedExecutionError {
    /// Error during execution.
    #[error("execution failed: {0}")]
    ExecutionFailed(String),

    /// Error during signer recovery.
    #[error("signer recovery failed")]
    SignerRecovery,

    /// Error when signature has non-normalized s value in homestead block.
    #[error("signature s value not normalized for homestead block")]
    HomesteadSignatureNotNormalized,

    /// Error when public key count doesn't match transaction count.
    #[error("public key count mismatch: {keys} keys vs {txs} transactions")]
    PublicKeyCountMismatch {
        /// Number of public keys provided.
        keys: usize,
        /// Number of transactions in the block.
        txs: usize,
    },

    /// Error when building state from witness.
    #[error("failed to build state from witness")]
    WitnessBuildFailed,

    /// Error deserializing ancestor headers.
    #[error("failed to deserialize ancestor headers")]
    HeaderDeserializationFailed,

    /// Error when no ancestor headers provided.
    #[error("missing ancestor headers")]
    MissingAncestorHeader,

    /// IO error.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Serialization error.
    #[error("serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// EVM configuration error.
    #[error("EVM configuration error: {0}")]
    EvmConfigError(String),
}

/// Result of a traced block execution.
#[derive(Debug, Clone)]
pub struct TracedExecution {
    /// Total gas used by the block.
    pub gas_used: u64,
    /// Number of transactions executed.
    pub transaction_count: usize,
    /// Whether all transactions succeeded.
    pub success: bool,
}

/// Trace a block execution with full EIP-3155 output.
///
/// This function executes all transactions in a block while collecting
/// detailed opcode-level traces conforming to EIP-3155.
///
/// # Arguments
///
/// * `block` - The block to execute
/// * `public_keys` - Public keys for transaction signature verification
/// * `witness` - The execution witness containing state data
/// * `chain_config` - The chain configuration from the witness
/// * `writer` - The trace writer for output
///
/// # Returns
///
/// Returns a [`TracedExecution`] containing execution statistics on success.
pub fn trace_block<W: Write>(
    block: Block,
    public_keys: Vec<UncompressedPublicKey>,
    witness: ExecutionWitness,
    chain_config: ChainConfig,
    writer: &mut TraceWriter<W>,
) -> Result<TracedExecution, TracedExecutionError> {
    // Create chain spec from config
    let genesis = Genesis {
        config: chain_config,
        ..Default::default()
    };
    let chain_spec: Arc<reth_chainspec::ChainSpec> = Arc::new(genesis.into());
    let evm_config = EthEvmConfig::new(chain_spec.clone());

    // Get the trace config from the writer
    let trace_config = writer.config().clone();

    trace_block_with_config::<SparseState, W>(
        block,
        public_keys,
        witness,
        chain_spec,
        evm_config,
        trace_config,
        writer,
    )
}

/// Trace block execution with specific chain configuration.
///
/// This function manually executes each transaction with a `TracingInspector`
/// to capture full opcode-level traces.
fn trace_block_with_config<T, W>(
    block: Block,
    public_keys: Vec<UncompressedPublicKey>,
    witness: ExecutionWitness,
    chain_spec: Arc<reth_chainspec::ChainSpec>,
    evm_config: EthEvmConfig,
    trace_config: crate::output::TraceOutput,
    writer: &mut TraceWriter<W>,
) -> Result<TracedExecution, TracedExecutionError>
where
    T: StatelessTrie,
    W: Write,
{
    // Step 1: Recover signers from public keys
    let recovered_block = recover_block_with_public_keys(block, public_keys, &*chain_spec)?;

    // Step 2: Build ancestor hashes from witness headers
    let (ancestor_hashes, parent_state_root) = build_ancestor_hashes(&recovered_block, &witness)?;

    // Step 3: Build state from witness
    let (trie, bytecode) = T::new(&witness, parent_state_root)
        .map_err(|_| TracedExecutionError::WitnessBuildFailed)?;

    // Step 4: Create witness database
    let db = WitnessDatabase::new(&trie, bytecode, ancestor_hashes);

    // Step 5: Create state with the database
    let mut state = State::builder().with_database(db).build();

    // Write block start
    writer.write_block_start(&recovered_block)?;

    // Step 6: Get EVM environment from block header using ConfigureEvm
    let evm_env = evm_config
        .evm_env(recovered_block.header())
        .expect("failed to create EVM environment");

    // Step 7: Execute each transaction with tracing
    let mut total_gas_used = 0u64;
    let mut all_success = true;
    let mut summary_accumulator = SummaryAccumulator::new();

    for (tx_index, (sender, tx)) in recovered_block.transactions_with_sender().enumerate() {
        let tx_hash = tx.tx_hash();

        // Reset summary accumulator for each transaction
        if trace_config.include_summary {
            summary_accumulator.reset();
        }

        // Create tracing inspector based on config
        let stack_snapshots = if trace_config.include_stack {
            StackSnapshotType::Full
        } else {
            StackSnapshotType::None
        };

        let inspector_config = TracingInspectorConfig::default_geth()
            .set_memory_snapshots(trace_config.include_memory)
            .set_stack_snapshots(stack_snapshots);

        let mut inspector = TracingInspector::new(inspector_config);

        // Create a recovered transaction for tx_env
        let recovered_tx = Recovered::new_unchecked(tx.clone(), *sender);

        // Get transaction environment
        let tx_env = evm_config.tx_env(&recovered_tx);
        let tx_env_clone = tx_env.clone();

        // Build the EVM context with our state
        let ctx = reth_revm::Context::mainnet()
            .with_db(&mut state)
            .with_block(evm_env.block_env.clone())
            .with_cfg(evm_env.cfg_env.clone());

        // Build and execute EVM with inspector using MainBuilder trait
        // This properly integrates the inspector into the execution flow
        let mut evm = ctx.build_mainnet_with_inspector(&mut inspector);

        // Execute the transaction with inspector (enables step-level tracing)
        let result = InspectEvm::inspect_tx(&mut evm, tx_env_clone);

        match result {
            Ok(result_and_state) => {
                let gas_used = result_and_state.result.gas_used();
                total_gas_used += gas_used;

                // Get return value from execution result
                let return_value = result_and_state
                    .result
                    .output()
                    .cloned()
                    .unwrap_or_default();

                // Build default trace for structLogs
                let geth_trace_opts =
                    alloy_rpc_types_trace::geth::GethDefaultTracingOptions::default()
                        .with_enable_memory(trace_config.include_memory)
                        .with_disable_stack(!trace_config.include_stack)
                        .with_disable_storage(!trace_config.include_storage)
                        .with_enable_return_data(trace_config.include_return_data);

                let default_frame = inspector.into_geth_builder().geth_traces(
                    gas_used,
                    return_value.clone(),
                    geth_trace_opts,
                );

                // Build ERC-7562 trace with configured options for opcode details
                let erc_frame = Erc7562Frame {
                    call_frame_type: CallFrameType::Call,
                    from: *sender,
                    gas: tx_env.gas_limit,
                    gas_used,
                    to: match tx_env.kind {
                        reth_revm::primitives::TxKind::Call(to) => Some(to),
                        _ => None,
                    },
                    input: tx_env.data.clone(),
                    output: Some(return_value),
                    error: None,
                    revert_reason: None,
                    logs: vec![],
                    value: Some(tx_env.value),
                    accessed_slots: AccessedSlots::default(),
                    ext_code_access_info: vec![],
                    used_opcodes: HashMap::default(),
                    contract_size: HashMap::default(),
                    out_of_gas: false,
                    keccak: vec![],
                    calls: vec![],
                };

                let custom_frame = CustomErc7562Frame {
                    base: erc_frame,
                    struct_logs: default_frame.struct_logs,
                };

                // Generate summary if requested
                let summary = if trace_config.include_summary {
                    summary_accumulator.process_trace(&custom_frame.struct_logs);
                    Some(summary_accumulator.generate_summary())
                } else {
                    None
                };

                // Convert to GethTrace for output
                let geth_trace = GethTrace::JS(serde_json::to_value(&custom_frame).unwrap());

                writer.write_transaction_trace(tx_index, &tx_hash, &geth_trace, summary)?;

                // Commit state changes
                state.commit(result_and_state.state);

                if !result_and_state.result.is_success() {
                    all_success = false;
                }
            }
            Err(e) => {
                writer.write_transaction_error(tx_index, &tx_hash, &format!("{e:?}"))?;
                all_success = false;
            }
        }
    }

    // Write block end
    let tx_count = recovered_block.body().transactions.len();
    writer.write_block_end(total_gas_used, tx_count)?;
    writer.flush()?;

    Ok(TracedExecution {
        gas_used: total_gas_used,
        transaction_count: tx_count,
        success: all_success,
    })
}

/// Build ancestor hashes from witness headers.
fn build_ancestor_hashes(
    current_block: &RecoveredBlock<Block>,
    witness: &ExecutionWitness,
) -> Result<(BTreeMap<u64, B256>, B256), TracedExecutionError> {
    let mut ancestor_headers: Vec<_> = witness
        .headers
        .iter()
        .map(|bytes| {
            let hash = keccak256(bytes);
            alloy_rlp::decode_exact::<Header>(bytes)
                .map(|h| SealedHeader::new(h, hash))
                .map_err(|_| TracedExecutionError::HeaderDeserializationFailed)
        })
        .collect::<Result<_, _>>()?;

    // Sort headers by block number (ascending)
    ancestor_headers.sort_by_key(|header| header.number());

    // Get the parent header (last in sorted list = highest block number = parent)
    let parent = ancestor_headers
        .last()
        .ok_or(TracedExecutionError::MissingAncestorHeader)?;
    let parent_state_root = parent.state_root;

    let mut ancestor_hashes = BTreeMap::new();
    let mut child_header = current_block.sealed_header();

    // Build the hash map
    for parent_header in ancestor_headers.iter().rev() {
        let parent_hash = child_header.parent_hash();
        ancestor_hashes.insert(parent_header.number, parent_hash);
        child_header = parent_header;
    }

    Ok((ancestor_hashes, parent_state_root))
}

/// Recover block with verified public keys.
fn recover_block_with_public_keys(
    block: Block,
    public_keys: Vec<UncompressedPublicKey>,
    chain_spec: &reth_chainspec::ChainSpec,
) -> Result<RecoveredBlock<Block>, TracedExecutionError> {
    if block.body().transactions.len() != public_keys.len() {
        return Err(TracedExecutionError::PublicKeyCountMismatch {
            keys: public_keys.len(),
            txs: block.body().transactions.len(),
        });
    }

    // Determine if we're in the Homestead fork
    let is_homestead = chain_spec.is_homestead_active_at_block(block.header().number());

    // Verify each transaction signature
    let senders = public_keys
        .iter()
        .zip(block.body().transactions())
        .map(|(vk, tx)| verify_and_compute_sender(vk, tx, is_homestead))
        .collect::<Result<Vec<_>, _>>()?;

    // Create RecoveredBlock with verified senders
    let block_hash = block.hash_slow();
    Ok(RecoveredBlock::new(block, senders, block_hash))
}

/// Verify transaction signature and compute sender address.
fn verify_and_compute_sender(
    vk: &UncompressedPublicKey,
    tx: &TransactionSigned,
    is_homestead: bool,
) -> Result<Address, TracedExecutionError> {
    use k256::ecdsa::{VerifyingKey, signature::hazmat::PrehashVerifier};

    let sig = tx.signature();

    // Non-normalized signatures are only valid pre-homestead
    let sig_is_normalized = sig.normalize_s().is_none();
    if is_homestead && !sig_is_normalized {
        return Err(TracedExecutionError::HomesteadSignatureNotNormalized);
    }

    let sig_hash = tx.signature_hash();

    let vk = VerifyingKey::from_sec1_bytes(vk).map_err(|_| TracedExecutionError::SignerRecovery)?;

    sig.to_k256()
        .and_then(|sig| vk.verify_prehash(sig_hash.as_slice(), &sig))
        .map_err(|_| TracedExecutionError::SignerRecovery)?;

    Ok(Address::from_public_key(&vk))
}
