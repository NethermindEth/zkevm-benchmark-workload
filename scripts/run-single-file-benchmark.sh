#!/usr/bin/env bash
#
# run-single-file-benchmark.sh
#
# Runs ere-hosts benchmark on a single fixture file instead of a folder.
# This script utilizes the new --input-file option to benchmark individual fixtures.
#
# Usage:
#   ./scripts/run-single-file-benchmark.sh <FIXTURE_FILE> [OPTIONS]
#
# Arguments:
#   FIXTURE_FILE         Path to the fixture file to benchmark
#
# Options:
#   --dry-run           Show what would be executed without actually running
#   --help, -h          Show this help message
#   --force-rerun       Force rerun of benchmarks (default: true)
#   --no-force-rerun    Disable force rerun
#   --action <ACTION>   Benchmark action to run (default: prove)
#   --resource <RESOURCE> Resource type to use (default: gpu)
#   --guest <GUEST>     Guest program type (default: stateless-executor)
#   --zkvm <ZKVM>       zkVM implementation to use (default: risc0)
#   --execution-client <CLIENT> Execution client to use (default: reth)
#   --output-dir <DIR>  Output directory for metrics (default: ./zkevm-metrics-single)
#   --memory-tracking   Enable memory tracking (default: false)
#
# Examples:
#   # Run benchmark on a single fixture file with defaults
#   ./scripts/run-single-file-benchmark.sh ./fixtures/block_12345.json
#   
#   # Run with custom action and resource
#   ./scripts/run-single-file-benchmark.sh ./fixtures/block_12345.json --action execute --resource cpu
#   
#   # Run with specific zkVM and execution client
#   ./scripts/run-single-file-benchmark.sh ./fixtures/block_12345.json --zkvm sp1 --execution-client ethrex
#   
#   # Run with custom output directory
#   ./scripts/run-single-file-benchmark.sh ./fixtures/block_12345.json --output-dir ./my-results
#   
#   # Preview what would be executed
#   ./scripts/run-single-file-benchmark.sh ./fixtures/block_12345.json --dry-run
#
# Available zkVM Features:
#   - risc0: RISC0 zkVM implementation (default)
#   - sp1: SP1 zkVM implementation
#   - openvm: OpenVM zkVM implementation
#   - pico: Pico zkVM implementation
#   - zisk: Zisk zkVM implementation
#
# Available Execution Clients:
#   - reth: Reth execution client (default)
#   - ethrex: Ethrex execution client
#

set -euo pipefail

# Default values
DRY_RUN=false
FORCE_RERUN=true
ACTION="prove"
RESOURCE="gpu"
GUEST="stateless-executor"
ZKVM="risc0"
EXECUTION_CLIENT="reth"
OUTPUT_DIR="./zkevm-metrics-single"
MEMORY_TRACKING=false

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    local color="$1"
    local message="$2"
    echo -e "${color}${message}${NC}"
}

# Function to show help
show_help() {
    echo "Usage: $0 <FIXTURE_FILE> [OPTIONS]"
    echo ""
    echo "Runs ere-hosts benchmark on a single fixture file instead of a folder."
    echo "This script utilizes the new --input-file option to benchmark individual fixtures."
    echo ""
    echo "Arguments:"
    echo "  FIXTURE_FILE         Path to the fixture file to benchmark"
    echo ""
    echo "Options:"
    echo "  --dry-run           Show what would be executed without actually running"
    echo "  --help, -h          Show this help message"
    echo "  --force-rerun       Force rerun of benchmarks (default: true)"
    echo "  --no-force-rerun    Disable force rerun"
    echo "  --action <ACTION>   Benchmark action to run (default: prove)"
    echo "  --resource <RESOURCE> Resource type to use (default: gpu)"
    echo "  --guest <GUEST>     Guest program type (default: stateless-executor)"
    echo "  --zkvm <ZKVM>       zkVM implementation to use (default: risc0)"
    echo "  --execution-client <CLIENT> Execution client to use (default: reth)"
    echo "  --output-dir <DIR>  Output directory for metrics (default: ./zkevm-metrics-single)"
    echo "  --memory-tracking   Enable memory tracking (default: false)"
    echo ""
    echo "Available zkVM Features:"
    echo "  - risc0: RISC0 zkVM implementation (default)"
    echo "  - sp1: SP1 zkVM implementation"
    echo "  - openvm: OpenVM zkVM implementation"
    echo "  - pico: Pico zkVM implementation"
    echo "  - zisk: Zisk zkVM implementation"
    echo ""
    echo "Available Execution Clients:"
    echo "  - reth: Reth execution client (default)"
    echo "  - ethrex: Ethrex execution client"
    echo ""
    echo "Examples:"
    echo "  $0 ./fixtures/block_12345.json                                    # Run with defaults"
    echo "  $0 ./fixtures/block_12345.json --action execute --resource cpu    # Run with custom action and resource"
    echo "  $0 ./fixtures/block_12345.json --zkvm sp1 --execution-client ethrex # Run with specific zkVM and client"
    echo "  $0 ./fixtures/block_12345.json --output-dir ./my-results          # Run with custom output directory"
    echo "  $0 ./fixtures/block_12345.json --dry-run                          # Show what would be executed"
    exit 0
}

# Parse command line arguments
FIXTURE_FILE=""
while [[ $# -gt 0 ]]; do
    case $1 in
        --help|-h)
            show_help
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --force-rerun)
            FORCE_RERUN=true
            shift
            ;;
        --no-force-rerun)
            FORCE_RERUN=false
            shift
            ;;
        --action)
            ACTION="$2"
            shift 2
            ;;
        --resource)
            RESOURCE="$2"
            shift 2
            ;;
        --guest)
            GUEST="$2"
            shift 2
            ;;
        --zkvm)
            ZKVM="$2"
            shift 2
            ;;
        --execution-client)
            EXECUTION_CLIENT="$2"
            shift 2
            ;;
        --output-dir)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        --memory-tracking)
            MEMORY_TRACKING=true
            shift
            ;;
        -*)
            echo "Unknown option: $1"
            show_help
            ;;
        *)
            if [ -z "$FIXTURE_FILE" ]; then
                FIXTURE_FILE="$1"
            else
                echo "Error: Multiple fixture files specified. Only one file is allowed."
                show_help
            fi
            shift
            ;;
    esac
done

# Check if fixture file is provided
if [ -z "$FIXTURE_FILE" ]; then
    print_status "$RED" "❌ Error: Fixture file path is required"
    echo ""
    show_help
fi

# Function to check if cargo is available
check_cargo() {
    if ! command -v cargo &> /dev/null; then
        print_status "$RED" "❌ Error: cargo is not installed or not in PATH"
        exit 1
    fi
}

# Function to check if we're in the right directory
check_workspace() {
    if [ ! -f "Cargo.toml" ]; then
        print_status "$RED" "❌ Error: Cargo.toml not found. This script must be run from the project root directory"
        exit 1
    fi
    
    if [ ! -d "crates/ere-hosts" ]; then
        print_status "$RED" "❌ Error: ere-hosts crate not found. This script must be run from the project root directory"
        exit 1
    fi
    
    print_status "$GREEN" "✅ Project structure verified"
}

# Function to validate fixture file
validate_fixture_file() {
    if [ ! -f "$FIXTURE_FILE" ]; then
        print_status "$RED" "❌ Error: Fixture file '$FIXTURE_FILE' does not exist"
        exit 1
    fi
    
    # Check if it's a JSON file
    if [[ ! "$FIXTURE_FILE" =~ \.json$ ]]; then
        print_status "$YELLOW" "⚠️  Warning: File '$FIXTURE_FILE' does not have .json extension"
    fi
    
    # Try to validate JSON syntax
    if ! python3 -m json.tool "$FIXTURE_FILE" > /dev/null 2>&1; then
        print_status "$YELLOW" "⚠️  Warning: File '$FIXTURE_FILE' may not be valid JSON"
    fi
    
    print_status "$GREEN" "✅ Fixture file validated: $FIXTURE_FILE"
}

# Function to build the project if needed
build_project() {
    print_status "$BLUE" "🔨 Building ere-hosts with $ZKVM feature..."
    cargo build --release --bin ere-hosts
    if [ $? -eq 0 ]; then
        print_status "$GREEN" "✅ Build successful"
    else
        print_status "$RED" "❌ Build failed"
        exit 1
    fi
}

# Function to run benchmark on single file
run_benchmark() {
    print_status "$BLUE" "🚀 Running benchmark on single fixture file"
    print_status "$BLUE" "📄 Fixture file: $FIXTURE_FILE"
    print_status "$BLUE" "📊 Output directory: $OUTPUT_DIR"
    
    # Create output directory if it doesn't exist
    mkdir -p "$OUTPUT_DIR"
    
    # Build force-rerun argument
    local force_arg=""
    if [ "$FORCE_RERUN" = true ]; then
        force_arg="--force-rerun"
    fi
    
    # Build memory-tracking argument
    local memory_arg=""
    if [ "$MEMORY_TRACKING" = true ]; then
        memory_arg="--memory-tracking"
    fi
    
    # Run the benchmark
    if cargo run --release --bin ere-hosts -- --zkvms "$ZKVM" -a "$ACTION" -r "$RESOURCE" $force_arg $memory_arg -o "$OUTPUT_DIR" "$GUEST" --input-file "$FIXTURE_FILE" --execution-client "$EXECUTION_CLIENT"; then
        print_status "$GREEN" "✅ Successfully completed benchmark for $FIXTURE_FILE"
        
        # Count the generated metric files
        local file_count=$(find "$OUTPUT_DIR" -type f 2>/dev/null | wc -l)
        print_status "$GREEN" "📊 Generated $file_count metric files in $OUTPUT_DIR"
        
        # Show the generated files
        if [ $file_count -gt 0 ]; then
            print_status "$BLUE" "📁 Generated files:"
            find "$OUTPUT_DIR" -type f -exec basename {} \; | sort | while read -r file; do
                print_status "$BLUE" "  - $file"
            done
        fi
    else
        print_status "$RED" "❌ Failed to complete benchmark for $FIXTURE_FILE"
        return 1
    fi
}

# Function to show summary
show_summary() {
    print_status "$GREEN" "\n🎉 Single file benchmark execution completed!"
    print_status "$BLUE" "\n📊 Summary:"
    print_status "$GREEN" "  ✅ Fixture file: $FIXTURE_FILE"
    print_status "$GREEN" "  ✅ Output directory: $OUTPUT_DIR"
    
    if [ -d "$OUTPUT_DIR" ]; then
        local metric_file_count=$(find "$OUTPUT_DIR" -type f 2>/dev/null | wc -l)
        print_status "$GREEN" "  ✅ Generated $metric_file_count metric files"
    else
        print_status "$RED" "  ❌ No output directory found"
    fi
    
    print_status "$BLUE" "\n📁 All metrics are located in: $OUTPUT_DIR"
}

# Main execution
main() {
    if [ "$DRY_RUN" = true ]; then
        print_status "$YELLOW" "🔍 DRY RUN MODE - No actual execution will occur"
        print_status "$BLUE" "🚀 Would start ere-hosts benchmark for single file..."
        print_status "$BLUE" "📄 Fixture file: $FIXTURE_FILE"
        print_status "$BLUE" "📊 Action: $ACTION"
        print_status "$BLUE" "🖥️  Resource: $RESOURCE"
        print_status "$BLUE" "🎯 Guest: $GUEST"
        print_status "$BLUE" "🔧 zkVM: $ZKVM"
        print_status "$BLUE" "⚙️  Execution Client: $EXECUTION_CLIENT"
        print_status "$BLUE" "📁 Output Directory: $OUTPUT_DIR"
        print_status "$BLUE" "🔄 Force Rerun: $FORCE_RERUN"
        print_status "$BLUE" "🧠 Memory Tracking: $MEMORY_TRACKING"
        print_status "$BLUE" "\n📋 Would execute the following command:"
        
        local force_arg=""
        if [ "$FORCE_RERUN" = true ]; then
            force_arg="--force-rerun"
        fi
        
        local memory_arg=""
        if [ "$MEMORY_TRACKING" = true ]; then
            memory_arg="--memory-tracking"
        fi
        
        print_status "$BLUE" "  cargo run --release --bin ere-hosts -- --zkvms $ZKVM -a $ACTION -r $RESOURCE $force_arg $memory_arg -o \"$OUTPUT_DIR\" $GUEST --input-file \"$FIXTURE_FILE\" --execution-client $EXECUTION_CLIENT"
        
        print_status "$GREEN" "\n✅ Dry run completed. Use without --dry-run to execute."
        exit 0
    fi
    
    print_status "$BLUE" "🚀 Starting ere-hosts benchmark for single file..."
    print_status "$BLUE" "📄 Fixture file: $FIXTURE_FILE"
    print_status "$BLUE" "📊 Action: $ACTION"
    print_status "$BLUE" "🖥️  Resource: $RESOURCE"
    print_status "$BLUE" "🎯 Guest: $GUEST"
    print_status "$BLUE" "🔧 zkVM: $ZKVM"
    print_status "$BLUE" "⚙️  Execution Client: $EXECUTION_CLIENT"
    print_status "$BLUE" "📁 Output Directory: $OUTPUT_DIR"
    print_status "$BLUE" "🔄 Force Rerun: $FORCE_RERUN"
    print_status "$BLUE" "🧠 Memory Tracking: $MEMORY_TRACKING"
    
    # Pre-flight checks
    check_cargo
    check_workspace
    validate_fixture_file
    
    # Build the project
    build_project
    
    # Run benchmark
    if run_benchmark; then
        print_status "$GREEN" "✅ Benchmark completed successfully"
    else
        print_status "$RED" "❌ Benchmark failed"
        exit 1
    fi
    
    # Show summary
    show_summary
    
    print_status "$GREEN" "\n🎯 Single file benchmark completed successfully!"
}

# Run main function
main "$@"
