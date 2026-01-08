# EIP-3155 Tracer

EIP-3155 compliant EVM execution tracer for stateless block execution.

## Overview

This crate provides EIP-3155 opcode-level tracing for Ethereum block execution using reth's stateless execution infrastructure. It's designed for host-side debugging and analysis, separate from zkVM guest programs.

**Default output**: By default, only minimal fields are included: `pc` (program counter), `op` (opcode), and `gasCost`. Use the builder methods to enable additional fields.

## Features

- **EIP-3155 compliance** - Outputs traces in the standard format
- **Opcode-level tracing** - Records every EVM instruction executed
- **Geth-compatible output** - Produces traces compatible with geth's `debug_traceTransaction`
- **Configurable output** - Control which fields are included (stack, memory, storage, etc.)
- **JSONL format** - Easy to parse, stream, and process

## Usage

```rust
use eip3155_tracer::{trace_block, TraceOutput, TraceWriter};
use std::fs::File;
use std::io::BufWriter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load your block, witness, and public keys...
    let block = /* ... */;
    let witness = /* ... */;
    let public_keys = /* ... */;
    let chain_config = /* ... */;

    // Create trace output file
    let output_file = File::create("trace.jsonl")?;
    
    // Minimal output (default): only pc, op, gasCost
    let config = TraceOutput::default();
    
    // Or include additional fields:
    let config = TraceOutput::default()
        .with_stack()    // Include stack snapshots
        .with_storage()  // Include storage changes
        .with_memory();  // Include memory snapshots
    
    // Or include all fields:
    let config = TraceOutput::full();

    let mut writer = TraceWriter::new(BufWriter::new(output_file), config);

    // Execute with tracing
    let result = trace_block(
        block,
        public_keys,
        witness,
        chain_config,
        &mut writer,
    )?;

    println!("Executed {} transactions", result.transaction_count);
    println!("Total gas used: {}", result.gas_used);
    println!("All successful: {}", result.success);

    Ok(())
}
```

## Output Format

The tracer outputs JSONL (JSON Lines) format, with one JSON object per line:

### Block Start

```json
{"type":"block_start","number":12345,"hash":"0x...","parent_hash":"0x...","timestamp":1234567890,"gas_limit":30000000,"beneficiary":"0x..."}
```

### Transaction Trace

```json
{"type":"transaction_trace","tx_index":0,"tx_hash":"0x...","trace":{"gas":21000,"failed":false,"returnValue":"0x","structLogs":[...]}}
```

Each `structLogs` entry contains:

```json
{
  "pc": 0,
  "op": "PUSH1",
  "gas": 29978,
  "gasCost": 3,
  "depth": 1,
  "stack": ["0x60"],
  "memory": "0x...",
  "storage": {"0x0": "0x1"}
}
```

### Transaction Error

```json
{"type":"transaction_error","tx_index":1,"tx_hash":"0x...","error":"out of gas"}
```

### Block End

```json
{"type":"block_end","total_gas_used":42000,"transaction_count":2}
```

## EIP-3155 Specification

This implementation follows [EIP-3155](https://eips.ethereum.org/EIPS/eip-3155) which defines a standard trace output format for the EVM.

Key fields per step:

| Field | Description |
|-------|-------------|
| `pc` | Program counter |
| `op` | Opcode name |
| `gas` | Remaining gas |
| `gasCost` | Gas cost of operation |
| `depth` | Call depth |
| `stack` | Stack contents |
| `memory` | Memory contents (optional) |
| `storage` | Storage changes (optional) |
| `refund` | Gas refund counter |
| `error` | Error message if any |

## Configuration

### TraceOutput

Configure what information to include in traces. By default, only `pc`, `op`, and `gasCost` are included:

```rust
// Minimal output (default)
let config = TraceOutput::default();

// Add specific fields
let config = TraceOutput::default()
    .with_stack()        // Include stack snapshots
    .with_memory()       // Include memory snapshots
    .with_storage()      // Include storage changes
    .with_return_data()  // Include return data
    .with_gas()          // Include gas remaining
    .with_depth()        // Include call depth
    .with_refund()       // Include gas refund counter
    .with_pretty_print(); // Pretty-print JSON output

// Include all fields
let config = TraceOutput::full();
```

### TraceWriter

The `TraceWriter` wraps any `std::io::Write` implementation:

```rust
// Write to file
let writer = TraceWriter::new(File::create("trace.jsonl")?, config);

// Write to stdout
let writer = TraceWriter::new(std::io::stdout(), config);

// Write to buffer
let writer = TraceWriter::new(Vec::new(), config);
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
