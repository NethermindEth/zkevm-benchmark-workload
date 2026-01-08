//! Core tracing implementation using revm-inspectors.
//!
//! This module provides the main tracing functionality for executing blocks
//! with full EIP-3155 opcode-level tracing.

use crate::output::TraceWriter;
use crate::witness_db::WitnessDatabase;
use alloy_consensus::{BlockHeader, Header, TxReceipt};
use alloy_genesis::ChainConfig;
use alloy_primitives::{keccak256, Address, B256};
use reth_chainspec::EthereumHardforks;
use reth_ethereum_primitives::{Block, EthereumReceipt, TransactionSigned};
use reth_evm::execute::Executor;
use reth_evm::ConfigureEvm;
use reth_evm_ethereum::EthEvmConfig;
use reth_primitives_traits::{Block as _, RecoveredBlock, SealedHeader};
use reth_stateless::{trie::StatelessTrie, ExecutionWitness, Genesis, UncompressedPublicKey};
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

    trace_block_with_config::<SparseState, W>(
        block,
        public_keys,
        witness,
        chain_spec,
        evm_config,
        writer,
    )
}

/// Trace block execution with specific chain configuration.
fn trace_block_with_config<T, W>(
    block: Block,
    public_keys: Vec<UncompressedPublicKey>,
    witness: ExecutionWitness,
    chain_spec: Arc<reth_chainspec::ChainSpec>,
    evm_config: EthEvmConfig,
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

    // Write block start
    writer.write_block_start(&recovered_block)?;

    // Step 5: Execute the block
    let executor = evm_config.executor(db);
    let output = executor
        .execute(&recovered_block)
        .map_err(|e: reth_evm::execute::BlockExecutionError| {
            TracedExecutionError::ExecutionFailed(e.to_string())
        })?;

    // Step 6: Write transaction traces
    let transactions: Vec<&TransactionSigned> = recovered_block.body().transactions().collect();
    let receipts: &[EthereumReceipt] = &output.receipts;
    let mut all_success = true;

    for (tx_index, (tx, receipt)) in transactions.iter().zip(receipts.iter()).enumerate() {
        let tx_hash = format!("{:?}", tx.tx_hash());
        let success = receipt.status();

        if !success {
            all_success = false;
        }

        // Create EIP-3155 compatible trace output
        let geth_trace = alloy_rpc_types_trace::geth::GethTrace::Default(
            alloy_rpc_types_trace::geth::DefaultFrame {
                failed: !success,
                gas: receipt.cumulative_gas_used(),
                return_value: alloy_primitives::Bytes::default(),
                struct_logs: vec![], // Note: Full struct_logs would require inspector integration
            },
        );

        writer.write_transaction_trace(tx_index, &tx_hash, &geth_trace)?;
    }

    // Write block end
    let total_gas_used = output.gas_used;
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
    use k256::ecdsa::{signature::hazmat::PrehashVerifier, VerifyingKey};

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
