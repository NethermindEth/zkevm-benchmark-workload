To run the profiling benchmarks, you need to have the following:

1. Run the script file `run_profiling_benchmarks.sh`
```bash
./scripts/download-and-extract-fixtures.sh
```

2. Then run the command

```bash
cargo run --bin profile-runner -- ./zkevm-fixtures/fixtures/state_tests/
```

3. The results will be saved in the `results/` directory


Commands to run the benchmarks:

Run Execution (Default)
cargo run --release --bin profile-runner-cli tests

Run Proving
cargo run --release --bin profile-runner-cli tests --action prove

Custom Output Directory
cargo run --release --bin profile-runner-cli tests --output-dir my_results

Force Rerun
cargo run --release --bin profile-runner-cli tests --force-rerun
