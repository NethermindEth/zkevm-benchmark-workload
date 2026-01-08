# EIP-3155 Tracer CLI

A standalone CLI tool for generating EIP-3155 compliant opcode traces from stateless executor fixtures.

## Overview

This tool traces EVM execution at the opcode level, producing detailed traces conforming to [EIP-3155](https://eips.ethereum.org/EIPS/eip-3155). The output is written in JSONL format (one JSON object per line).

## Installation

Build the binary from the workspace root:

```bash
cargo build --release -p eip3155-tracer-cli
```

The binary will be available at `target/release/eip3155-trace`.

## Usage

### Basic Usage

Trace all fixtures in a folder:

```bash
eip3155-trace --input-folder zkevm-fixtures-input --output-folder zkevm-fixtures-traces
```

### Trace a Single Fixture

```bash
eip3155-trace --input-file path/to/fixture.json --output-folder zkevm-fixtures-traces
```

### Options

```
Generate EIP-3155 opcode traces from stateless executor fixtures

Usage: eip3155-trace [OPTIONS]

Options:
  -i, --input-folder <INPUT_FOLDER>
          Input folder containing fixture JSON files [default: zkevm-fixtures-input]

      --input-file <INPUT_FILE>
          Input file for a single fixture (overrides input_folder)

  -o, --output-folder <OUTPUT_FOLDER>
          Output folder for trace files [default: zkevm-fixtures-traces]

      --include-memory
          Include memory snapshots in the trace (increases output size significantly)

      --include-storage
          Include storage changes in the trace

      --pretty-print
          Pretty-print JSON output [default: true]

  -h, --help
          Print help

  -V, --version
          Print version
```

### Output Format

Each fixture produces a JSONL file containing:

1. **Block start marker** - Block metadata (number, hash, timestamp, etc.)
2. **Transaction traces** - Full EIP-3155 traces for each transaction
3. **Block end marker** - Summary with total gas used

Example output structure:

```json
{"type":"block_start","number":12345,"hash":"0x...","parent_hash":"0x...","timestamp":1234567890,"gas_limit":30000000,"beneficiary":"0x..."}
{"type":"transaction_trace","tx_index":0,"tx_hash":"0x...","trace":{"gas":21000,"failed":false,"returnValue":"0x","structLogs":[...]}}
{"type":"block_end","total_gas_used":21000,"transaction_count":1}
```

## Related Crates

- `eip3155-tracer` - Core tracing library with the EVM execution logic
- `witness-generator` - Generates stateless executor fixtures from blocks
