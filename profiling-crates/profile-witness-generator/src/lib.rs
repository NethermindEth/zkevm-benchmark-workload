#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use std::{fs, io, path::Path};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use revm::context::TxEnv;
use revm::primitives::{U256, Address, Bytes, TxKind};

/// JSON generator for test cases
pub mod json_generator;

/// Represents a test case for profiling.
#[derive(Debug, Serialize, Deserialize)]
pub struct TestCase {
    pub env: Environment,
    pub pre: HashMap<String, Account>,
    pub transaction: Transaction,
    pub post: HashMap<String, Vec<PostState>>,
    pub config: Config,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Environment {
    #[serde(rename = "currentCoinbase")]
    pub current_coinbase: String,
    #[serde(rename = "currentGasLimit")]
    pub current_gas_limit: String,
    #[serde(rename = "currentNumber")]
    pub current_number: String,
    #[serde(rename = "currentTimestamp")]
    pub current_timestamp: String,
    #[serde(rename = "currentDifficulty")]
    pub current_difficulty: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub nonce: String,
    pub balance: String,
    pub code: String,
    pub storage: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub nonce: String,
    #[serde(rename = "gasPrice")]
    pub gas_price: String,
    #[serde(rename = "gasLimit")]
    pub gas_limit: Vec<String>,
    pub to: String,
    pub value: Vec<String>,
    pub data: Vec<String>,
    pub sender: String,
    #[serde(rename = "secretKey")]
    pub secret_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostState {
    pub hash: String,
    pub logs: String,
    pub txbytes: String,
    pub indexes: HashMap<String, u64>,
    pub state: HashMap<String, Account>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub chainid: String,
}

/// Represents a named test case for profiling.
#[derive(Debug, Serialize, Deserialize)]
pub struct NamedTestCase {
    /// Name of the test case.
    pub name: String,
    /// The test case data.
    pub test_case: TestCase,
}

/// Errors that can occur during test case processing.
#[derive(Error, Debug)]
pub enum TestCaseError {
    /// Serde JSON (de)serialization error.
    #[error("serde JSON (de)serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    /// Error during file system I/O operations.
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// Error during hex parsing.
    #[error("hex parsing error: {0}")]
    Hex(#[from] hex::FromHexError),

    /// Error during hex string parsing.
    #[error("invalid hex string: {0}")]
    InvalidHex(String),
}

impl NamedTestCase {
    /// Serializes a list of `NamedTestCase` to a JSON pretty-printed string.
    ///
    /// # Errors
    ///
    /// Returns `TestCaseError::Serde` if JSON serialization fails.
    pub fn to_json(items: &[Self]) -> Result<String, TestCaseError> {
        serde_json::to_string_pretty(items).map_err(TestCaseError::from)
    }

    /// Deserializes a list of `NamedTestCase` from a JSON string.
    ///
    /// # Errors
    ///
    /// Returns `TestCaseError::Serde` if JSON deserialization fails.
    pub fn from_json(json: &str) -> Result<Vec<Self>, TestCaseError> {
        serde_json::from_str(json).map_err(TestCaseError::from)
    }

    /// Serializes `items` to pretty-printed JSON and writes them to `path`.
    ///
    /// The file is created if it does not exist and truncated if it does.
    /// Parent directories are created if they are missing.
    ///
    /// # Errors
    ///
    /// Returns `TestCaseError::Io` if any filesystem operation fails.
    /// Returns `TestCaseError::Serde` if JSON serialization fails.
    pub fn to_path<P: AsRef<Path>>(path: P, items: &[Self]) -> Result<(), TestCaseError> {
        let path = path.as_ref();
        ensure_parent_dirs(path)?;
        let json = Self::to_json(items)?;
        fs::write(path, json).map_err(TestCaseError::Io)?;
        Ok(())
    }

    /// Reads the file at `path` and deserializes a `Vec<NamedTestCase>` from its JSON content.
    ///
    /// # Errors
    ///
    /// Returns `TestCaseError::Io` if reading the file fails.
    /// Returns `TestCaseError::Serde` if JSON deserialization fails.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Vec<Self>, TestCaseError> {
        let path = path.as_ref();
        let contents = fs::read_to_string(path).map_err(TestCaseError::Io)?;
        Self::from_json(&contents)
    }
}

/// Trait for generating test cases.
///
/// Implementors of this trait provide different strategies for generating
/// test cases, such as from JSON files or other sources.
#[async_trait]
pub trait TestCaseGenerator {
    /// Generates test case fixtures.
    ///
    /// # Errors
    ///
    /// Returns an error if the generation process fails, including file I/O problems,
    /// or data processing errors.
    async fn generate(&self) -> Result<Vec<NamedTestCase>>;

    /// Generates test case fixtures and writes them to the specified path.
    ///
    /// # Arguments
    /// * `path` - The directory path where fixture files will be written
    ///
    /// # Returns
    /// The number of fixture files successfully generated and written
    ///
    /// # Errors
    ///
    /// Returns an error if the generation fails or if writing to the path fails.
    async fn generate_to_path(&self, path: &Path) -> Result<usize>;
}

/// Helper function to parse hex string to U256
pub fn parse_hex_to_u256(hex_str: &str) -> Result<U256, TestCaseError> {
    let clean_hex = if hex_str.starts_with("0x") {
        &hex_str[2..]
    } else {
        hex_str
    };
    U256::from_str_radix(clean_hex, 16)
        .map_err(|_| TestCaseError::InvalidHex(hex_str.to_string()))
}

/// Helper function to parse hex string to u64
pub fn parse_hex_to_u64(hex_str: &str) -> Result<u64, TestCaseError> {
    let clean_hex = if hex_str.starts_with("0x") {
        &hex_str[2..]
    } else {
        hex_str
    };
    u64::from_str_radix(clean_hex, 16)
        .map_err(|_| TestCaseError::InvalidHex(hex_str.to_string()))
}

/// Helper function to parse hex string to u128
pub fn parse_hex_to_u128(hex_str: &str) -> Result<u128, TestCaseError> {
    let clean_hex = if hex_str.starts_with("0x") {
        &hex_str[2..]
    } else {
        hex_str
    };
    u128::from_str_radix(clean_hex, 16)
        .map_err(|_| TestCaseError::InvalidHex(hex_str.to_string()))
}

/// Helper function to parse hex string to Address
pub fn parse_hex_to_address(hex_str: &str) -> Result<Address, TestCaseError> {
    let clean_hex = if hex_str.starts_with("0x") {
        &hex_str[2..]
    } else {
        hex_str
    };
    
    let bytes = hex::decode(clean_hex)?;
    let mut addr_bytes = [0u8; 20];
    if bytes.len() <= 20 {
        addr_bytes[20 - bytes.len()..].copy_from_slice(&bytes);
    } else {
        addr_bytes.copy_from_slice(&bytes[bytes.len() - 20..]);
    }
    
    Ok(Address::from(addr_bytes))
}

/// Helper function to parse hex string to bytes
pub fn parse_hex_to_bytes(hex_str: &str) -> Result<Vec<u8>, TestCaseError> {
    let clean_hex = if hex_str.starts_with("0x") {
        &hex_str[2..]
    } else {
        hex_str
    };
    hex::decode(clean_hex).map_err(TestCaseError::from)
}

/// Creates a transaction environment from a test case.
pub fn create_tx_env(test_case: &TestCase) -> Result<TxEnv, TestCaseError> {
    let to = if test_case.transaction.to.is_empty() {
        None
    } else {
        Some(parse_hex_to_address(&test_case.transaction.to)?)
    };

    let data = if test_case.transaction.data.is_empty() {
        Bytes::new()
    } else {
        Bytes::from(parse_hex_to_bytes(&test_case.transaction.data[0])?)
    };

    let value = if test_case.transaction.value.is_empty() {
        U256::ZERO
    } else {
        parse_hex_to_u256(&test_case.transaction.value[0])?
    };

    let gas_limit = if test_case.transaction.gas_limit.is_empty() {
        0
    } else {
        parse_hex_to_u64(&test_case.transaction.gas_limit[0])?
    };

    let gas_price = parse_hex_to_u128(&test_case.transaction.gas_price)?;

    let mut tx_env = TxEnv::default();
    tx_env.caller = parse_hex_to_address(&test_case.transaction.sender)?;
    tx_env.gas_limit = gas_limit;
    tx_env.gas_price = gas_price;
    tx_env.kind = TxKind::Call(to.unwrap_or(parse_hex_to_address(&test_case.transaction.sender)?));
    tx_env.value = value;
    tx_env.data = data;
    tx_env.chain_id = Some(parse_hex_to_u64(&test_case.config.chainid)?);
    tx_env.nonce = parse_hex_to_u64(&test_case.transaction.nonce)?;
    
    Ok(tx_env)
}

fn ensure_parent_dirs<P: AsRef<Path>>(path: P) -> Result<(), io::Error> {
    if let Some(parent) = path.as_ref().parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
}

// Re-export commonly used types
pub use std::collections::HashMap;
pub use json_generator::JsonTestCaseGenerator; 