# Benchmark Results

This directory contains generated benchmark results from zkGas profiling runs. The results are organized by gas categories and zkVM implementations for easy analysis and comparison.

## Directory Structure

```
benchmark-results/
├── README.md                           # This file
├── gas-categorized/                    # Results organized by gas categories
│   ├── 1M/                            # 1 million gas limit results
│   ├── 10M/                           # 10 million gas limit results
│   ├── 30M/                           # 30 million gas limit results
│   ├── 45M/                           # 45 million gas limit results
│   ├── 60M/                           # 60 million gas limit results
│   ├── 100M/                          # 100 million gas limit results
│   └── 500M/                          # 500 million gas limit results
├── zkvm-comparisons/                   # Results comparing different zkVM implementations
│   ├── risc0/                         # RISC0 zkVM results
│   ├── sp1/                           # SP1 zkVM results
│   ├── openvm/                        # OpenVM zkVM results
│   ├── pico/                          # Pico zkVM results
│   └── zisk/                          # Zisk zkVM results
├── execution-clients/                  # Results organized by execution client
│   ├── reth/                          # Reth execution client results
│   └── ethrex/                        # Ethrex execution client results
├── markdown-reports/                   # Generated markdown analysis reports
│   ├── latest/                        # Most recent analysis reports
│   ├── comparisons/                   # Comparison reports between different runs
│   └── statistics/                    # Statistical analysis reports
└── archived/                          # Archived results from previous runs
    ├── 2024-01/                       # Results from January 2024
    ├── 2024-02/                       # Results from February 2024
    └── ...
```

## File Types

### JSON Metrics Files
- **Location**: `gas-categorized/{GAS_CATEGORY}/` and `zkvm-comparisons/{ZKVM}/`
- **Format**: JSON files containing detailed profiling metrics
- **Content**: Execution cycles, proving time, memory usage, and metadata

### Markdown Reports
- **Location**: `markdown-reports/`
- **Format**: Markdown files with formatted tables and analysis
- **Content**: Human-readable analysis of profiling results

### Comparison Reports
- **Location**: `markdown-reports/comparisons/`
- **Format**: Markdown files comparing different profiling runs
- **Content**: Side-by-side comparisons of resource requirements

## Usage

### Viewing Results

1. **Browse by Gas Category**:
   ```bash
   ls benchmark-results/gas-categorized/1M/
   ```

2. **Browse by zkVM Implementation**:
   ```bash
   ls benchmark-results/zkvm-comparisons/risc0/
   ```

3. **View Markdown Reports**:
   ```bash
   cat benchmark-results/markdown-reports/latest/profiling-results.md
   ```

### Generating New Results

1. **Run Profiling**:
   ```bash
   ./scripts/run-gas-categorized-benchmarks.sh
   ```

2. **Generate Markdown Tables**:
   ```bash
   ./scripts/generate_results.sh --compare --statistics --output benchmark-results/markdown-reports/latest/profiling-results.md
   ```

3. **Archive Previous Results**:
   ```bash
   mv benchmark-results/markdown-reports/latest benchmark-results/archived/$(date +%Y-%m)
   ```

## Naming Conventions

### Gas Category Directories
- Format: `{GAS_VALUE}` (e.g., `1M`, `10M`, `100M`)
- Contains: Results for that specific gas limit

### zkVM Comparison Directories
- Format: `{ZKVM_NAME}` (e.g., `risc0`, `sp1`, `openvm`)
- Contains: Results for that specific zkVM implementation

### Markdown Report Files
- Format: `{DESCRIPTION}-{TIMESTAMP}.md` (e.g., `profiling-results-2024-01-15.md`)
- Contains: Formatted analysis of profiling results

## Best Practices

### Organization
1. **Use Descriptive Names**: Name files and directories clearly
2. **Include Timestamps**: Add timestamps to report files for versioning
3. **Archive Old Results**: Move old results to the archived directory
4. **Document Changes**: Update this README when adding new result types

### Analysis
1. **Compare Across Gas Categories**: Look for patterns in resource usage
2. **Compare Across zkVM Implementations**: Identify performance differences
3. **Use Statistical Analysis**: Include statistical summaries in reports
4. **Document Findings**: Add notes about significant discoveries

### Maintenance
1. **Regular Cleanup**: Archive old results to keep the directory organized
2. **Backup Important Results**: Keep copies of significant findings
3. **Update Documentation**: Keep this README current with the directory structure
4. **Version Control**: Consider versioning important result sets

## Integration with Documentation

The results in this directory are referenced in the main documentation:

- [Gas Categorized Benchmarks](/gas-categorized-benchmarks) - How to generate these results
- [Markdown Tables](/markdown-tables) - How to create analysis reports
- [Getting Started](/getting-started) - Overview of the profiling workflow

## Examples

### Viewing a Specific Gas Category
```bash
# List all results for 10M gas category
ls benchmark-results/gas-categorized/10M/

# View a specific result file
cat benchmark-results/gas-categorized/10M/block_001.json
```

### Comparing zkVM Implementations
```bash
# Compare RISC0 and SP1 results
diff benchmark-results/zkvm-comparisons/risc0/ benchmark-results/zkvm-comparisons/sp1/
```

### Generating a New Report
```bash
# Generate a comprehensive report
./scripts/generate_results.sh --compare --statistics \
  --output benchmark-results/markdown-reports/latest/comprehensive-analysis-$(date +%Y-%m-%d).md
```

## Troubleshooting

### Missing Results
- Ensure profiling has been run: `./scripts/run-gas-categorized-benchmarks.sh`
- Check if fixtures exist: `ls zkevm-fixtures-input-*`
- Verify script permissions: `chmod +x scripts/*.sh`

### Permission Issues
```bash
# Make sure the directory is writable
chmod 755 benchmark-results/
chmod 755 benchmark-results/*/
```

### Disk Space
```bash
# Check available space
df -h benchmark-results/

# Clean up old results if needed
rm -rf benchmark-results/archived/old-results/
```
