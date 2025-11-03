# Quick Start Guide - Opcode Test Name Formatter

## What Does This Do?

Converts long pytest test names like:
```
test_worst_compute.py::test_worst_binop_simple[fork_Prague-benchmark-gas-value_10M-opcode_EXP]
```

Into readable format:
```
EXP (Prague, 10M gas)
```

## Quick Commands

### Format a Single Test Name
```bash
python scripts/test_name_formatter.py "test_worst_compute.py::test_worst_binop_simple[fork_Prague-benchmark-gas-value_10M-opcode_EXP]"
```

### Format Multiple Test Names
```bash
# From files
ls zkevm-metrics-sp1-1M/sp1-v5.1.0/*.json | \
  xargs -I {} basename {} .json | \
  python scripts/test_name_formatter.py

# Show with original names
python scripts/test_name_formatter.py --show-original "test_name_here"

# JSON output
python scripts/test_name_formatter.py --json "test_name_here"
```

### Generate Markdown Tables with Opcode Format
```bash
# Single folder
python scripts/generate_markdown_tables.py --name-format opcode \
  --output results.md zkevm-metrics-sp1-1M

# Compare multiple folders
python scripts/generate_markdown_tables.py --compare --name-format opcode \
  --output comparison.md zkevm-metrics-sp1-1M zkevm-metrics-risc0-1M
```

## Name Format Options

Use with `--name-format` flag in `generate_markdown_tables.py`:

- `original` - Original long test names (default)
- `display` - Human-readable from test_name_parser
- `simplified` - Shortened names  
- `category` - Categorized names
- **`opcode`** - **NEW!** Opcode-focused: `EXP (Prague, 10M gas)`

## Examples

```bash
# EXP opcode test
Input:  test_worst_compute.py::test_worst_binop_simple[fork_Prague-benchmark-gas-value_10M-opcode_EXP]
Output: EXP (Prague, 10M gas)

# Precompile test
Input:  test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_120M-bls12_pairing_check]
Output: PRECOMPILE_BLS12_PAIRING, CALL, STATICCALL (Prague, 120M gas)

# Memory operation
Input:  test_worst_memory.py::test_worst_mcopy[fork_Cancun-benchmark-gas-value_10M-0 bytes]
Output: MCOPY (Cancun, 10M gas, 0 bytes)

# Stack operation
Input:  test_worst_compute.py::test_worst_dup[fork_Prague-benchmark-gas-value_10M-opcode_DUP5]
Output: DUP5 (Prague, 10M gas)
```

## Integration in Python Scripts

```python
from test_name_formatter import TestNameFormatter

formatter = TestNameFormatter()
display_name = formatter.format_test_name(test_filename)
print(display_name)
```

## Files

- `opcode_test_parser.py` - Opcode extraction engine
- `test_name_formatter.py` - Name formatting with opcode priority
- `generate_markdown_tables.py` - **Updated** with `--name-format opcode` option

## Documentation

- **Full Documentation**: `TEST_NAME_FORMATTING.md`
- **Integration Details**: `OPCODE_FORMATTER_INTEGRATION.md`
- **This Guide**: `QUICK_START.md`

## All Supported Opcodes & Precompiles

✅ All EVM opcodes (Frontier through Osaka/Prague)
✅ All precompiles (0x01-0x14 including BLS12, P256VERIFY)

See `TEST_NAME_FORMATTING.md` for complete list.

