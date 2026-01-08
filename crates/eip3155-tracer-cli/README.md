# EIP-3155 Tracer CLI

A standalone CLI tool for generating EIP-3155 compliant opcode traces from stateless executor fixtures.

## Overview

This tool traces EVM execution at the opcode level, producing detailed traces conforming to [EIP-3155](https://eips.ethereum.org/EIPS/eip-3155). The output is written in JSONL format (one JSON object per line).

**Default output**: Only minimal fields are included by default: `pc` (program counter), `op` (opcode), and `gasCost`. Use the `--include-*` flags to enable additional fields.

## Installation

Build the binary from the workspace root:

```bash
cargo build --release -p eip3155-tracer-cli
```

The binary will be available at `target/release/eip3155-trace`.

## Usage

### Basic Usage (Minimal Output)

Trace all fixtures in a folder with minimal output (pc, op, gasCost):

```bash
eip3155-trace -i zkevm-fixtures-input -o zkevm-fixtures-traces
```

### Trace with Additional Fields

Include specific fields:

```bash
# Include stack and storage
eip3155-trace -i input/ -o output/ --include-stack --include-storage

# Include all fields
eip3155-trace -i input/ -o output/ --full
```

### Trace a Single Fixture

```bash
eip3155-trace --input-file path/to/fixture.json -o zkevm-fixtures-traces
```

### Options

```
Generate EIP-3155 opcode traces from stateless executor fixtures

By default, only minimal fields are included in the trace output:
`pc` (program counter), `op` (opcode), and `gasCost`.

Use the various `--include-*` flags to enable additional fields.

Usage: eip3155-trace [OPTIONS]

Options:
  -i, --input-folder <INPUT_FOLDER>
          Input folder containing fixture JSON files [default: zkevm-fixtures-input]

      --input-file <INPUT_FILE>
          Input file for a single fixture (overrides input_folder)

  -o, --output-folder <OUTPUT_FOLDER>
          Output folder for trace files [default: zkevm-fixtures-traces]

      --include-stack
          Include stack snapshots in the trace

      --include-memory
          Include memory snapshots in the trace (increases output size significantly)

      --include-storage
          Include storage changes in the trace

      --include-return-data
          Include return data in the trace

      --include-gas
          Include gas remaining in the trace (not just gasCost)

      --include-depth
          Include call depth in the trace

      --include-refund
          Include gas refund counter in the trace

      --full
          Include all optional fields in the trace

      --pretty-print
          Pretty-print JSON output

  -h, --help
          Print help

  -V, --version
          Print version
```

### Output Format

Each fixture produces a JSONL file containing:

1. **Block start marker** - Block metadata (number, hash, timestamp, etc.)
2. **Transaction traces** - EIP-3155 traces for each transaction
3. **Block end marker** - Summary with total gas used

Example output structure (minimal, default):

```json
{"type":"block_start","number":12345,"hash":"0x...","parent_hash":"0x...","timestamp":1234567890,"gas_limit":30000000,"beneficiary":"0x..."}
{"type":"transaction_trace","tx_index":0,"tx_hash":"0x...","trace":{"gas":21000,"failed":false,"returnValue":"0x","structLogs":[{"pc":0,"op":96,"gasCost":3},...]}}
{"type":"block_end","total_gas_used":21000,"transaction_count":1}
```

With `--full` flag, each structLog entry includes all fields:

```json
{"pc":0,"op":96,"opName":"PUSH1","gas":100000,"gasCost":3,"depth":1,"stack":[],"memory":"0x","storage":{}}
```

## Related Crates

- `eip3155-tracer` - Core tracing library with the EVM execution logic
- `witness-generator` - Generates stateless executor fixtures from blocks
