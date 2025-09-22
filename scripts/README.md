# Scripts Directory

This directory contains various scripts for running benchmarks and analyzing results.

## Benchmark Execution Scripts

### `run-gas-categorized-benchmarks.sh`
Runs ere-hosts benchmarks on each gas-categorized fixtures folder and outputs results to metrics folders.

**Usage:**
```bash
./scripts/run-gas-categorized-benchmarks.sh [OPTIONS]
```

**Key Options:**
- `--dry-run`: Show what would be executed without running
- `--action <ACTION>`: Benchmark action (default: prove)
- `--resource <RESOURCE>`: Resource type (default: gpu)
- `--zkvm <ZKVM>`: zkVM implementation (default: risc0)
- `--execution-client <CLIENT>`: Execution client (default: reth)

**Example:**
```bash
# Run all gas categories with default settings
./scripts/run-gas-categorized-benchmarks.sh

# Run with specific zkVM and execution client
./scripts/run-gas-categorized-benchmarks.sh --zkvm sp1 --execution-client ethrex
```

## Results Analysis Scripts

### `generate_markdown_tables.py`
Generates markdown tables from benchmark results stored in JSON files.

**Usage:**
```bash
python3 scripts/generate_markdown_tables.py [OPTIONS] <metrics_folder> [<metrics_folder2> ...]
```

**Key Options:**
- `--output, -o <file>`: Output file (default: benchmark_results.md)
- `--compare`: Compare metrics between multiple folders
- `--execution-only`: Only show execution metrics
- `--proving-only`: Only show proving metrics
- `--statistics`: Include statistical analysis
- `--gas-categories`: Group results by gas categories

**Examples:**
```bash
# Generate tables from single metrics folder
python3 scripts/generate_markdown_tables.py zkevm-metrics-1M

# Compare multiple gas categories
python3 scripts/generate_markdown_tables.py --compare --gas-categories \
  zkevm-metrics-1M zkevm-metrics-10M zkevm-metrics-100M

# Generate with statistics
python3 scripts/generate_markdown_tables.py --statistics --output results.md zkevm-metrics-1M
```

### `generate_results.sh`
Convenient wrapper script for generating markdown tables with common use cases.

**Usage:**
```bash
./scripts/generate_results.sh [OPTIONS]
```

**Key Options:**
- `--all`: Generate tables for all available gas categories
- `--compare`: Compare all available gas categories
- `--output <file>`: Output file (default: benchmark_results.md)
- `--execution-only`: Only show execution metrics
- `--proving-only`: Only show proving metrics
- `--statistics`: Include statistical analysis
- `--open`: Open the generated file after creation

**Examples:**
```bash
# Generate tables for all categories
./scripts/generate_results.sh --all

# Compare all with statistics
./scripts/generate_results.sh --compare --statistics

# Generate execution-only and open
./scripts/generate_results.sh --all --execution-only --open
```

## Comparison Scripts

### `compare_executions.py`
Compares execution metrics between baseline and optimized runs.

**Usage:**
```bash
python3 scripts/compare_executions.py <baseline_folder> <optimized_folder>
```

### `compare_provings.py`
Compares proving time metrics between baseline and optimized runs.

**Usage:**
```bash
python3 scripts/compare_provings.py <baseline_folder> <optimized_folder>
```

## Fixture Generation Scripts

### `generate-gas-categorized-fixtures.sh`
Generates gas-categorized fixtures for benchmarking.

### `download-and-extract-fixtures.sh`
Downloads and extracts test fixtures.

## Workflow Examples

### Complete Benchmark and Analysis Workflow

1. **Generate fixtures:**
   ```bash
   ./scripts/generate-gas-categorized-fixtures.sh
   ```

2. **Run benchmarks:**
   ```bash
   ./scripts/run-gas-categorized-benchmarks.sh
   ```

3. **Generate markdown tables:**
   ```bash
   ./scripts/generate_results.sh --compare --statistics --open
   ```

### Performance Comparison Workflow

1. **Run baseline benchmarks:**
   ```bash
   ./scripts/run-gas-categorized-benchmarks.sh --zkvm risc0
   ```

2. **Run optimized benchmarks:**
   ```bash
   ./scripts/run-gas-categorized-benchmarks.sh --zkvm sp1
   ```

3. **Compare results:**
   ```bash
   python3 scripts/compare_executions.py zkevm-metrics-1M zkevm-metrics-1M-sp1
   python3 scripts/compare_provings.py zkevm-metrics-1M zkevm-metrics-1M-sp1
   ```

## Output Formats

The markdown table generator supports multiple output formats:

- **Markdown** (default): Well-formatted tables for GitHub, documentation
- **HTML**: For web viewing
- **CSV**: For spreadsheet analysis

## Gas Categories

The system supports the following gas categories:
- 1M: 1 million gas limit
- 10M: 10 million gas limit  
- 30M: 30 million gas limit
- 45M: 45 million gas limit
- 60M: 60 million gas limit
- 100M: 100 million gas limit
- 500M: 500 million gas limit

## Metrics Structure

The benchmark system generates JSON files containing:

### Execution Metrics
- `total_num_cycles`: Total execution cycles
- `region_cycles`: Cycles by region (setup, compute, teardown, etc.)
- `execution_duration`: Execution time

### Proving Metrics
- `proof_size`: Proof size in bytes
- `proving_time_ms`: Proving time in milliseconds

### Metadata
- `name`: Benchmark name
- `timestamp_completed`: Completion timestamp
- `metadata`: Additional benchmark-specific data
