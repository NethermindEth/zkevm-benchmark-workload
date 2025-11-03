# Test Name Formatting Scripts

This directory contains two Python scripts for parsing and formatting zkEVM benchmark test names into human-readable formats.

## Scripts

### 1. `opcode_test_parser.py`

Parses pytest test names and extracts information about which EVM opcodes or precompiles are being tested.

**Features:**
- Extracts opcode names from test parameters
- Identifies precompiles by address or name
- Maps test function names to their tested opcodes
- Handles all EVM opcodes including Prague and Osaka upgrades
- Supports BLS12, P256VERIFY, and other precompiles

**Usage:**

```bash
# Parse a single test name
python opcode_test_parser.py "test_worst_compute.py::test_worst_binop_simple[Prague-benchmark-gas-value_10M-opcode_EXP]"
# Output: EXP

# Verbose output
python opcode_test_parser.py -v "test_worst_compute.py::test_worst_push[opcode_PUSH32]"
# Output:
# Test: test_worst_compute.py::test_worst_push
# Module: test_worst_compute
# Parameters: opcode_PUSH32
# Opcodes: PUSH32

# Parse multiple tests from a file
python opcode_test_parser.py --batch tests.txt

# Parse from stdin
ls *.json | xargs -I {} basename {} .json | python opcode_test_parser.py --stdin

# JSON output
python opcode_test_parser.py --json "test_worst_compute.py::test_worst_binop_simple[Prague-benchmark-gas-value_10M-opcode_EXP]"
```

### 2. `test_name_formatter.py`

Formats pytest test names into human-readable display names with the format: `OPCODE/PRECOMPILE (parameters)`

**Features:**
- Extracts and displays opcodes first
- Shows relevant parameters (gas amount, fork name, sizes, etc.)
- Filters out noise parameters
- Handles precompiles with readable names
- Supports multiple output formats

**Examples:**

```bash
# Simple format
python test_name_formatter.py "test_worst_compute.py::test_worst_binop_simple[fork_Prague-benchmark-gas-value_10M-opcode_EXP]"
# Output: EXP (Prague, 10M gas)

# Precompile test
python test_name_formatter.py "test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_120M-bls12_pairing_check]"
# Output: PRECOMPILE_BLS12_PAIRING, CALL, STATICCALL (Prague, 120M gas)

# Memory operation
python test_name_formatter.py "test_worst_memory.py::test_worst_mcopy[fork_Cancun-benchmark-gas-value_10M-0 bytes]"
# Output: MCOPY (Cancun, 10M gas, 0 bytes)

# Show original alongside display name
python test_name_formatter.py --show-original "test_worst_compute.py::test_worst_binop_simple[fork_Prague-benchmark-gas-value_10M-opcode_EXP]"
# Output:
# test_worst_compute.py::test_worst_binop_simple[fork_Prague-benchmark-gas-value_10M-opcode_EXP]
#   → EXP (Prague, 10M gas)

# JSON output
python test_name_formatter.py --json "test_worst_compute.py::test_worst_binop_simple[fork_Prague-benchmark-gas-value_10M-opcode_EXP]"
# Output: [{"original": "...", "display": "EXP (Prague, 10M gas)"}]

# Format multiple test names
python test_name_formatter.py test1 test2 test3

# Read from stdin
cat test_names.txt | python test_name_formatter.py
```

## Integration with Benchmark Results

### Format benchmark result filenames

```bash
# Format all test names from benchmark results
cd /path/to/zkevm-benchmark-workload
ls zkevm-metrics-sp1-1M/sp1-v5.1.0/*.json | \
  xargs -I {} basename {} .json | \
  python scripts/test_name_formatter.py
```

### Use in markdown table generation

The formatter can be integrated into `generate_markdown_tables.py` to provide more readable test names in the output tables:

```python
from test_name_formatter import TestNameFormatter

formatter = TestNameFormatter()

# In your table generation code:
for test_file in test_files:
    display_name = formatter.format_test_name(test_file)
    # Use display_name in your table
```

## Output Format

The formatter prioritizes information in this order:

1. **Opcode/Precompile** - Always shown first (e.g., `EXP`, `MCOPY`, `PRECOMPILE_BLS12_PAIRING`)
2. **Fork** - The Ethereum fork being tested (e.g., `Prague`, `Cancun`)
3. **Gas** - Gas limit for the test (e.g., `10M gas`, `1M gas`)
4. **Other Parameters** - Sizes, counts, flags, etc. (e.g., `0 bytes`, `big memory expansion: True`)

## Opcode Coverage

The parser recognizes all EVM opcodes including:

- **Arithmetic**: ADD, MUL, SUB, DIV, MOD, EXP, ADDMOD, MULMOD, etc.
- **Comparison & Bitwise**: LT, GT, EQ, AND, OR, XOR, NOT, SHL, SHR, SAR
- **Stack Operations**: PUSH0-PUSH32, DUP1-DUP16, SWAP1-SWAP16, POP
- **Memory**: MLOAD, MSTORE, MSTORE8, MCOPY, MSIZE
- **Storage**: SLOAD, SSTORE, TLOAD, TSTORE
- **Control Flow**: JUMP, JUMPI, JUMPDEST, PC
- **Environmental**: ADDRESS, BALANCE, CALLER, CALLVALUE, etc.
- **Crypto**: SHA3, KECCAK256
- **Contract Operations**: CALL, DELEGATECALL, STATICCALL, CREATE, CREATE2
- **Logging**: LOG0-LOG4
- **System**: RETURN, REVERT, SELFDESTRUCT
- **EIP-4844**: BLOBHASH, BLOBBASEFEE
- **Osaka**: CLZ

## Precompile Coverage

All precompiles are recognized including:

- **Classic**: ECRECOVER (0x01), SHA2-256 (0x02), RIPEMD-160 (0x03), IDENTITY (0x04)
- **Byzantium**: MODEXP (0x05), EC_ADD (0x06), EC_MUL (0x07), EC_PAIRING (0x08)
- **Istanbul**: BLAKE2F (0x09)
- **Cancun**: POINT_EVALUATION (0x0a)
- **Prague BLS12**: G1ADD (0x0b), G1MSM (0x0c), G2ADD (0x0d), G2MSM (0x0e), PAIRING (0x0f), MAP_FP_TO_G1 (0x10), MAP_FP2_TO_G2 (0x11)
- **P256VERIFY** (0x14)

## Requirements

- Python 3.7+
- No external dependencies (uses only standard library)

## File Dependencies

- `test_name_formatter.py` depends on `opcode_test_parser.py`
- Both scripts can be used standalone or imported as modules

