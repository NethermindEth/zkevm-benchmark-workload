# Single File Benchmark Usage Examples

This document provides examples of how to use the new `run-single-file-benchmark.sh` script to run benchmarks on individual fixture files.

## Prerequisites

1. Ensure you have fixture files available (either generated or downloaded)
2. Make sure the script is executable: `chmod +x scripts/run-single-file-benchmark.sh`

## Basic Usage

### Run a benchmark on a single fixture file

```bash
# Basic usage with defaults (prove action, gpu resource, risc0 zkVM, reth execution client)
./scripts/run-single-file-benchmark.sh ./tests/assets/eest-empty-block/fixtures/blockchain_tests/empty_block.json
```

### Preview what would be executed (dry run)

```bash
# See what command would be executed without actually running it
./scripts/run-single-file-benchmark.sh ./tests/assets/eest-empty-block/fixtures/blockchain_tests/empty_block.json --dry-run
```

## Advanced Usage Examples

### Different Actions and Resources

```bash
# Run execution only (no proving) with CPU resource
./scripts/run-single-file-benchmark.sh ./fixtures/block_12345.json --action execute --resource cpu

# Run proving with GPU resource (default)
./scripts/run-single-file-benchmark.sh ./fixtures/block_12345.json --action prove --resource gpu
```

### Different zkVM Implementations

```bash
# Use SP1 zkVM
./scripts/run-single-file-benchmark.sh ./fixtures/block_12345.json --zkvm sp1

# Use RISC0 zkVM (default)
./scripts/run-single-file-benchmark.sh ./fixtures/block_12345.json --zkvm risc0

# Use OpenVM zkVM
./scripts/run-single-file-benchmark.sh ./fixtures/block_12345.json --zkvm openvm
```

### Different Execution Clients

```bash
# Use Reth execution client (default)
./scripts/run-single-file-benchmark.sh ./fixtures/block_12345.json --execution-client reth

# Use Ethrex execution client
./scripts/run-single-file-benchmark.sh ./fixtures/block_12345.json --execution-client ethrex
```

### Different Guest Programs

```bash
# Run stateless executor (default)
./scripts/run-single-file-benchmark.sh ./fixtures/block_12345.json --guest stateless-executor

# Run stateless validator
./scripts/run-single-file-benchmark.sh ./fixtures/block_12345.json --guest stateless-validator
```

### Custom Output Directory

```bash
# Specify custom output directory
./scripts/run-single-file-benchmark.sh ./fixtures/block_12345.json --output-dir ./my-custom-results

# Default output directory is ./zkevm-metrics-single
```

### Memory Tracking

```bash
# Enable memory tracking during proving
./scripts/run-single-file-benchmark.sh ./fixtures/block_12345.json --memory-tracking
```

### Force Rerun Control

```bash
# Force rerun even if results already exist (default)
./scripts/run-single-file-benchmark.sh ./fixtures/block_12345.json --force-rerun

# Skip if results already exist
./scripts/run-single-file-benchmark.sh ./fixtures/block_12345.json --no-force-rerun
```

## Complete Example

Here's a complete example that combines multiple options:

```bash
# Run a comprehensive benchmark with custom settings
./scripts/run-single-file-benchmark.sh \
    ./fixtures/block_12345.json \
    --action prove \
    --resource gpu \
    --zkvm sp1 \
    --execution-client ethrex \
    --guest stateless-executor \
    --output-dir ./results-sp1-ethrex \
    --memory-tracking \
    --force-rerun
```

## Output

The script will:

1. Validate the fixture file exists and is valid JSON
2. Build the ere-hosts binary with the specified zkVM feature
3. Run the benchmark using the new `--input-file` option
4. Generate metrics in the specified output directory
5. Show a summary of the results

Example output:
```
🚀 Starting ere-hosts benchmark for single file...
📄 Fixture file: ./fixtures/block_12345.json
📊 Action: prove
🖥️  Resource: gpu
🎯 Guest: stateless-executor
🔧 zkVM: sp1
⚙️  Execution Client: ethrex
📁 Output Directory: ./results-sp1-ethrex
🔄 Force Rerun: true
🧠 Memory Tracking: false

✅ Project structure verified
✅ Fixture file validated: ./fixtures/block_12345.json
🔨 Building ere-hosts with sp1 feature...
✅ Build successful
🚀 Running benchmark on single fixture file
📄 Fixture file: ./fixtures/block_12345.json
📊 Output directory: ./results-sp1-ethrex
✅ Successfully completed benchmark for ./fixtures/block_12345.json
📊 Generated 2 metric files in ./results-sp1-ethrex
📁 Generated files:
  - hardware.json
  - sp1-v5.1.0/block_12345.json

🎉 Single file benchmark execution completed!

📊 Summary:
  ✅ Fixture file: ./fixtures/block_12345.json
  ✅ Output directory: ./results-sp1-ethrex
  ✅ Generated 2 metric files

📁 All metrics are located in: ./results-sp1-ethrex

🎯 Single file benchmark completed successfully!
```

## Troubleshooting

### Common Issues

1. **File not found**: Ensure the fixture file path is correct and the file exists
2. **Invalid JSON**: The script will warn if the file is not valid JSON
3. **Build failures**: Make sure you have the required dependencies and are in the project root
4. **Permission denied**: Make sure the script is executable (`chmod +x scripts/run-single-file-benchmark.sh`)

### Getting Help

```bash
# Show help and all available options
./scripts/run-single-file-benchmark.sh --help
```

## Comparison with Folder-based Benchmarks

| Feature | Single File Script | Gas Categorized Script |
|---------|-------------------|------------------------|
| Input | Single fixture file | Multiple fixture folders |
| Use Case | Testing individual fixtures | Batch processing |
| Output | Single metrics directory | Multiple metrics directories |
| Flexibility | High (any single file) | Medium (predefined categories) |
| Speed | Fast (single file) | Slower (multiple files) |
