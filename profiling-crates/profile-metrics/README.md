# Profile Metrics

This crate provides metrics collection and output writing functionality for the profile runner.

## Features

- Hardware information detection
- Execution and proving metrics collection
- JSON serialization/deserialization
- File I/O operations for metrics storage

## Usage

```rust
use profile_metrics::{ProfileRun, ExecutionMetrics, ProvingMetrics, HardwareInfo};

// Detect hardware information
let hardware = HardwareInfo::detect();
hardware.to_path("hardware.json")?;

// Create a profile run
let run = ProfileRun {
    name: "test_case".to_string(),
    gas_used: 12345,
    execution: Some(ExecutionMetrics::Success { /* ... */ }),
    proving: Some(ProvingMetrics::Success { /* ... */ }),
};

// Save to file
ProfileRun::to_path("results.json", &[run])?;
``` 