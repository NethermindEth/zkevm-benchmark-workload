# Profile Witness Generator

This crate provides test case generation and processing functionality for the profile runner.

## Features

- Test case data structures for EVM profiling
- JSON file parsing and processing
- Transaction environment creation
- Hex string parsing utilities
- Async test case generation

## Usage

```rust
use profile_witness_generator::{JsonTestCaseGenerator, TestCaseGenerator, NamedTestCase};

// Create a JSON generator
let generator = JsonTestCaseGenerator::new("tests".to_string());

// Generate test cases
let test_cases = generator.generate().await?;

// Process test cases
for test_case in test_cases {
    let tx_env = create_tx_env(&test_case.test_case)?;
    // Use tx_env for execution...
}
```

## Test Case Format

Test cases are expected to be in JSON format with the following structure:

```json
{
  "test_name": {
    "env": {
      "currentCoinbase": "0x...",
      "currentGasLimit": "0x...",
      "currentNumber": "0x...",
      "currentTimestamp": "0x...",
      "currentDifficulty": "0x..."
    },
    "pre": {
      "0x...": {
        "nonce": "0x...",
        "balance": "0x...",
        "code": "0x...",
        "storage": {}
      }
    },
    "transaction": {
      "nonce": "0x...",
      "gasPrice": "0x...",
      "gasLimit": ["0x..."],
      "to": "0x...",
      "value": ["0x..."],
      "data": ["0x..."],
      "sender": "0x...",
      "secretKey": "0x..."
    },
    "post": {},
    "config": {
      "chainid": "0x..."
    }
  }
}
``` 