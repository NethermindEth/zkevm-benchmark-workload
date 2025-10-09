#!/usr/bin/env bash
#
# generate_results.sh
#
# Wrapper script to easily generate markdown tables from benchmark results.
# This script provides convenient shortcuts for common use cases.
#
# Usage:
#   ./scripts/generate_results.sh [OPTIONS]
#
# Options:
#   --help, -h              Show this help message
#   --all                   Generate tables for all available gas categories
#   --compare               Compare all available gas categories
#   --output <file>         Output file (default: benchmark_results.md)
#   --execution-only        Only show execution metrics
#   --proving-only          Only show proving metrics
#   --statistics            Include statistical analysis
#   --name-format <format>  Format for benchmark names (default: display)
#   --open                  Open the generated file after creation
#
# Examples:
#   # Generate tables for all available gas categories (with display names)
#   ./scripts/generate_results.sh --all
#
#   # Compare all gas categories with statistics
#   ./scripts/generate_results.sh --compare --statistics
#
#   # Generate execution-only results and open them
#   ./scripts/generate_results.sh --all --execution-only --open
#
#   # Use simplified names instead of display names
#   ./scripts/generate_results.sh --all --name-format simplified
#
#   # Use original complex names
#   ./scripts/generate_results.sh --all --name-format original
#

set -euo pipefail

# Default values
SHOW_ALL=false
COMPARE=false
OUTPUT_FILE="benchmark_results.md"
EXECUTION_ONLY=false
PROVING_ONLY=false
INCLUDE_STATISTICS=false
OPEN_FILE=false
NAME_FORMAT="display"
BASE_METRICS_DIR="./zkevm-metrics"

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
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Wrapper script to easily generate markdown tables from benchmark results."
    echo "This script provides convenient shortcuts for common use cases."
    echo ""
    echo "Options:"
    echo "  --help, -h              Show this help message"
    echo "  --all                   Generate tables for all available gas categories"
    echo "  --compare               Compare all available gas categories"
    echo "  --output <file>         Output file (default: benchmark_results.md)"
    echo "  --execution-only        Only show execution metrics"
    echo "  --proving-only          Only show proving metrics"
    echo "  --statistics            Include statistical analysis"
    echo "  --name-format <format>  Format for benchmark names (default: display)"
    echo "  --open                  Open the generated file after creation"
    echo ""
    echo "Examples:"
    echo "  $0 --all                                    # Generate tables for all categories (display names)"
    echo "  $0 --compare --statistics                   # Compare all with statistics"
    echo "  $0 --all --execution-only --open            # Generate execution-only and open"
    echo "  $0 --all --name-format simplified           # Use simplified names"
    echo "  $0 --all --name-format original             # Use original complex names"
    echo ""
    echo "Available gas categories:"
    echo "  - 1M: 1 million gas limit"
    echo "  - 10M: 10 million gas limit"
    echo "  - 30M: 30 million gas limit"
    echo "  - 45M: 45 million gas limit"
    echo "  - 60M: 60 million gas limit"
    echo "  - 100M: 100 million gas limit"
    echo "  - 500M: 500 million gas limit"
    exit 0
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --help|-h)
            show_help
            ;;
        --all)
            SHOW_ALL=true
            shift
            ;;
        --compare)
            COMPARE=true
            shift
            ;;
        --output)
            OUTPUT_FILE="$2"
            shift 2
            ;;
        --execution-only)
            EXECUTION_ONLY=true
            shift
            ;;
        --proving-only)
            PROVING_ONLY=true
            shift
            ;;
        --statistics)
            INCLUDE_STATISTICS=true
            shift
            ;;
        --name-format)
            NAME_FORMAT="$2"
            shift 2
            ;;
        --open)
            OPEN_FILE=true
            shift
            ;;
        *)
            echo "Unknown option: $1"
            show_help
            ;;
    esac
done

# Function to check if we're in the right directory
check_workspace() {
    if [ ! -f "Cargo.toml" ]; then
        print_status "$RED" "❌ Error: Cargo.toml not found. This script must be run from the project root directory"
        exit 1
    fi
    
    if [ ! -f "scripts/generate_markdown_tables.py" ]; then
        print_status "$RED" "❌ Error: generate_markdown_tables.py not found. This script must be run from the project root directory"
        exit 1
    fi
    
    print_status "$GREEN" "✅ Project structure verified"
}

# Function to validate name format
validate_name_format() {
    case "$NAME_FORMAT" in
        original|display|simplified|category)
            return 0
            ;;
        *)
            print_status "$RED" "❌ Error: Invalid name format '$NAME_FORMAT'"
            print_status "$RED" "   Valid formats: original, display, simplified, category"
            exit 1
            ;;
    esac
}

# Function to find available gas categories
find_available_categories() {
    local categories=()
    
    # Look for directories with pattern: zkevm-metrics-{zkvm}-{gas_value}
    for metrics_dir in "${BASE_METRICS_DIR}"-*; do
        if [ -d "$metrics_dir" ]; then
            # Check if directory has any JSON files
            if find "$metrics_dir" -name "*.json" -type f | grep -q .; then
                categories+=("$metrics_dir")
            fi
        fi
    done
    
    echo "${categories[@]}"
}

# Function to run the markdown generator
run_generator() {
    local args=()
    
    # Add output file
    args+=("--output" "$OUTPUT_FILE")
    
    # Add execution/proving filters
    if [ "$EXECUTION_ONLY" = true ]; then
        args+=("--execution-only")
    elif [ "$PROVING_ONLY" = true ]; then
        args+=("--proving-only")
    fi
    
    # Add statistics
    if [ "$INCLUDE_STATISTICS" = true ]; then
        args+=("--statistics")
    fi
    
    # Add name format
    args+=("--name-format" "$NAME_FORMAT")
    
    # Add comparison mode
    if [ "$COMPARE" = true ]; then
        args+=("--compare")
    fi
    
    # Add metrics folders
    args+=("$@")
    
    print_status "$BLUE" "🚀 Generating markdown tables..."
    print_status "$BLUE" "📊 Command: python3 scripts/generate_markdown_tables.py ${args[*]}"
    
    if python3 scripts/generate_markdown_tables.py "${args[@]}"; then
        print_status "$GREEN" "✅ Successfully generated $OUTPUT_FILE"
        return 0
    else
        print_status "$RED" "❌ Failed to generate markdown tables"
        return 1
    fi
}

# Function to open the generated file
open_generated_file() {
    if [ -f "$OUTPUT_FILE" ]; then
        print_status "$BLUE" "📖 Opening $OUTPUT_FILE..."
        
        # Try different methods to open the file
        if command -v open &> /dev/null; then
            # macOS
            open "$OUTPUT_FILE"
        elif command -v xdg-open &> /dev/null; then
            # Linux
            xdg-open "$OUTPUT_FILE"
        elif command -v code &> /dev/null; then
            # VS Code
            code "$OUTPUT_FILE"
        else
            print_status "$YELLOW" "⚠️  Could not find a suitable program to open the file"
            print_status "$YELLOW" "   You can manually open: $OUTPUT_FILE"
        fi
    else
        print_status "$RED" "❌ Output file $OUTPUT_FILE not found"
    fi
}

# Main execution
main() {
    print_status "$BLUE" "🔍 zkEVM Benchmark Results Generator"
    
    # Pre-flight checks
    check_workspace
    validate_name_format
    
    # Find available categories
    print_status "$BLUE" "🔍 Scanning for available metrics folders..."
    available_categories=($(find_available_categories))
    
    if [ ${#available_categories[@]} -eq 0 ]; then
        print_status "$YELLOW" "⚠️  No metrics folders found with JSON files"
        print_status "$YELLOW" "   Expected folders: ${BASE_METRICS_DIR}-{zkvm}-{1M,10M,30M,45M,60M,100M,500M}"
        print_status "$YELLOW" "   Run './scripts/run-gas-categorized-benchmarks.sh' first to generate metrics"
        exit 1
    fi
    
    print_status "$GREEN" "✅ Found ${#available_categories[@]} metrics folders:"
    for category in "${available_categories[@]}"; do
        local dir_name=$(basename "$category")
        local zkvm_and_gas=$(echo "$dir_name" | sed 's/zkevm-metrics-//')
        # Extract zkvm and gas value from pattern: zkvm-gas_value
        local zkvm=$(echo "$zkvm_and_gas" | sed 's/-[^-]*$//')
        local gas_value=$(echo "$zkvm_and_gas" | sed 's/^[^-]*-//')
        print_status "$GREEN" "  - $category (zkVM: $zkvm, Gas: $gas_value)"
    done
    
    print_status "$BLUE" "📝 Using name format: $NAME_FORMAT"
    
    # Determine what to do
    if [ "$SHOW_ALL" = true ] || [ "$COMPARE" = true ]; then
        # Use all available categories
        if run_generator "${available_categories[@]}"; then
            if [ "$OPEN_FILE" = true ]; then
                open_generated_file
            fi
        else
            exit 1
        fi
    else
        print_status "$YELLOW" "⚠️  No action specified. Use --all or --compare to generate tables"
        print_status "$YELLOW" "   Or specify individual metrics folders as arguments"
        show_help
    fi
}

# Run main function
main "$@"
