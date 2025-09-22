# Markdown Table Generator Usage Examples

This document provides examples of how to use the `generate_markdown_tables.py` script to present benchmark results in tabular form.

## Basic Usage

### Generate tables from a single metrics folder
```bash
python3 scripts/generate_markdown_tables.py zkevm-metrics-1M
```

### Generate tables and save to a file
```bash
python3 scripts/generate_markdown_tables.py --output results.md zkevm-metrics-1M
```

### Show only execution metrics
```bash
python3 scripts/generate_markdown_tables.py --execution-only zkevm-metrics-1M
```

### Show only proving metrics
```bash
python3 scripts/generate_markdown_tables.py --proving-only zkevm-metrics-1M
```

## Comparison Mode

### Compare multiple gas categories
```bash
python3 scripts/generate_markdown_tables.py --compare --gas-categories \
  zkevm-metrics-1M zkevm-metrics-10M zkevm-metrics-100M
```

### Compare with statistics
```bash
python3 scripts/generate_markdown_tables.py --compare --statistics \
  --output comparison_results.md \
  zkevm-metrics-1M zkevm-metrics-10M zkevm-metrics-100M
```

## Advanced Usage

### Generate CSV output
```bash
python3 scripts/generate_markdown_tables.py --format csv --output results.csv zkevm-metrics-1M
```

### Generate HTML output
```bash
python3 scripts/generate_markdown_tables.py --format html --output results.html zkevm-metrics-1M
```

## Example Output

The script generates markdown tables like this:

### Execution Metrics
| Benchmark | Gas Category | Total Cycles | Duration (ms) | Setup | Compute | Teardown |
|-----------|--------------|--------------|---------------|-------|---------|----------|
| block_1 | 1M | 1,234,567 | 150.5 | 100,000 | 1,000,000 | 134,567 |
| block_2 | 1M | 2,345,678 | 280.2 | 200,000 | 2,000,000 | 145,678 |

### Proving Metrics
| Benchmark | Gas Category | Proof Size (bytes) | Proving Time (ms) | Proving Time (s) |
|-----------|--------------|-------------------|-------------------|------------------|
| block_1 | 1M | 256 | 2,000 | 2.00 |
| block_2 | 1M | 512 | 4,500 | 4.50 |

### Summary by Gas Category
| Gas Category | Total Benchmarks | Execution | Proving | Avg Cycles | Avg Duration (ms) | Avg Proof Size (bytes) | Avg Proving Time (ms) |
|--------------|------------------|-----------|---------|------------|-------------------|------------------------|----------------------|
| 1M | 10 | 10 | 8 | 1,500,000 | 200.5 | 384 | 3,250 |
| 10M | 15 | 15 | 12 | 8,500,000 | 1,200.8 | 512 | 8,500 |

## Integration with Benchmark Script

You can integrate this with the existing benchmark script:

```bash
# Run benchmarks
./scripts/run-gas-categorized-benchmarks.sh

# Generate markdown tables
python3 scripts/generate_markdown_tables.py --compare --statistics \
  --output benchmark_analysis.md \
  zkevm-metrics-1M zkevm-metrics-10M zkevm-metrics-100M
```

## Features

- **Multiple Output Formats**: Markdown, HTML, CSV
- **Flexible Filtering**: Show only execution or proving metrics
- **Comparison Mode**: Compare results across multiple gas categories
- **Statistical Analysis**: Min, max, average values
- **Region Analysis**: Detailed breakdown of execution cycles by region
- **Gas Category Grouping**: Automatic detection and grouping by gas limits
- **Summary Tables**: High-level overview of all metrics

## Requirements

- Python 3.6+
- No external dependencies (uses only standard library)
