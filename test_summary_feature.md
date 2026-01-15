# EIP-3155 Summary Feature Test

This document demonstrates the new summary statistics feature.

## Summary Configuration

The tracer now supports generating summary statistics that aggregate opcode and gas cost information.

### New Configuration Options

- `include_summary`: Include summary statistics alongside structLogs
- `summary_only`: Generate only summary statistics (no structLogs)

### CLI Flags

- `--include-summary`: Include summary statistics in trace
- `--summary-only`: Generate only summary statistics (no structLogs)

## Output Format

### With Summary (`--include-summary`)
```json
{
  "type": "transaction_trace",
  "tx_index": 0,
  "tx_hash": "0x...",
  "trace": {
    "gas": 21000,
    "failed": false,
    "returnValue": "0x",
    "structLogs": [...]
  },
  "summary": {
    "total_opcodes": 150,
    "total_gas_cost": 21000,
    "opcode_breakdown": [
      {
        "opcode": "PUSH1",
        "count": 45,
        "total_gas_cost": 135,
        "average_gas_cost": 3.0
      }
    ]
  }
}
```

### Summary Only (`--summary-only`)
```json
{
  "type": "transaction_trace",
  "tx_index": 0,
  "tx_hash": "0x...",
  "trace": {
    "gas": 21000,
    "failed": false,
    "returnValue": "0x",
    "structLogs": []
  },
  "summary": {
    "total_opcodes": 150,
    "total_gas_cost": 21000,
    "opcode_breakdown": [...]
  }
}
```

## Implementation Details

### SummaryAccumulator
- Processes GethTrace frames to accumulate opcode statistics
- Tracks count and total gas cost per opcode
- Generates sorted breakdown by total gas cost

### Performance
- O(n) processing time over structLogs
- Minimal memory overhead when disabled
- Efficient HashMap for aggregation

### Backward Compatibility
- Existing trace formats unchanged when summary disabled
- Full `--full` mode now includes summary by default
- All existing CLI flags work as before