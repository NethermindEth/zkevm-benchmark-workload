# Profile Runner Workspace

This workspace contains a modular profiling system for zkEVM workloads, following the patterns established in the main `crates` directory.

## Architecture

The profiling system is split into several crates, each with a specific responsibility:

### `profile-metrics`
Handles metrics collection and output writing. Provides:
- Hardware information detection
- Execution and proving metrics structures
- JSON serialization/deserialization
- File I/O operations for metrics storage

### `profile-witness-generator`
Handles test case loading and processing. Provides:
- Test case data structures for EVM profiling
- JSON file parsing and processing
- Transaction environment creation
- Hex string parsing utilities
- Async test case generation

### `profile-runner`
Core execution logic. Provides:
- Profile execution orchestration
- Test case processing
- Execution and proving workflows
- Integration with RISC0 zkVM

### `profile-runner-cli`
Command-line interface. Provides:
- CLI argument parsing
- Configuration management
- User-friendly interface for running profiles

## Usage

### Building the workspace

```bash
cd profiling-crates
cargo build
```

### Running profiles

```bash
# Run execution profiles
cargo run -p profile-runner-cli -- tests --action execute

# Run proving profiles
cargo run -p profile-runner-cli -- tests --action prove

# Use custom output directory
cargo run -p profile-runner-cli -- --output-folder my-results tests

# Force rerun existing results
cargo run -p profile-runner-cli -- --force-rerun tests
```

### Using as a library

```rust
use profile_runner::{run_profile, RunConfig, Action};
use profile_witness_generator::{JsonTestCaseGenerator, TestCaseGenerator};

// Create configuration
let config = RunConfig {
    output_folder: PathBuf::from("results"),
    action: Action::Execute,
    force_rerun: false,
};

// Generate test cases
let generator = JsonTestCaseGenerator::new("tests".to_string());
let test_cases = generator.generate().await?;

// Run profiles
run_profile(&config, &test_cases).await?;
```

## Design Patterns

This workspace follows the same patterns as the main `crates` directory:

1. **Separation of Concerns**: Each crate has a single, well-defined responsibility
2. **Trait-based Interfaces**: Uses traits like `TestCaseGenerator` for extensibility
3. **Error Handling**: Consistent error types and handling patterns
4. **Async Support**: Full async/await support for I/O operations
5. **Configuration**: Structured configuration management
6. **CLI Design**: Consistent CLI patterns with clap

## Extending the System

To add new functionality:

1. **New Metrics**: Extend `profile-metrics` with new metric types
2. **New Test Sources**: Implement `TestCaseGenerator` trait in `profile-witness-generator`
3. **New Execution Modes**: Extend `Action` enum in `profile-runner`
4. **New CLI Commands**: Add subcommands to `SourceCommand` in `profile-runner-cli` 