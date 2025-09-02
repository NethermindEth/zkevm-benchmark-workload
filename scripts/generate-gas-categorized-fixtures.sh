#!/usr/bin/env bash
#
# generate-gas-categorized-fixtures.sh
#
# Generates zkevm-fixtures-input categorized by gas parameters into different folders.
# Each gas category gets its own output folder with the gas parameter appended to the name.
#
# Usage:
#   ./scripts/generate-gas-categorized-fixtures.sh [EEST_TAG] [BASE_OUTPUT_DIR]
#
# Examples:
#   # Generate fixtures for all gas categories using latest EEST release
#   ./scripts/generate-gas-categorized-fixtures.sh
#   
#   # Generate fixtures for all gas categories using specific EEST release
#   ./scripts/generate-gas-categorized-fixtures.sh v0.1.0
#   
#   # Generate fixtures for all gas categories in custom base directory
#   ./scripts/generate-gas-categorized-fixtures.sh v0.1.0 /tmp/fixtures
#
# Gas Categories:
#   - 1M: 1 million gas limit
#   - 10M: 10 million gas limit  
#   - 30M: 30 million gas limit
#   - 45M: 45 million gas limit
#   - 60M: 60 million gas limit
#   - 100M: 100 million gas limit
#   - 500M: 500 million gas limit
#

set -euo pipefail

# Default values
EEST_TAG="${1:-}"
BASE_OUTPUT_DIR="${2:-./zkevm-fixtures-input}"

# Check for help flag
if [ "${1:-}" = "--help" ] || [ "${1:-}" = "-h" ]; then
    echo "Usage: $0 [EEST_TAG] [BASE_OUTPUT_DIR] [OPTIONS]"
    echo ""
    echo "Generates zkevm-fixtures-input categorized by gas parameters into different folders."
    echo ""
    echo "Arguments:"
    echo "  EEST_TAG        EEST release tag to use (e.g., v0.1.0). If empty, latest will be used."
    echo "  BASE_OUTPUT_DIR Base output directory (default: ./zkevm-fixtures-input)"
    echo ""
    echo "Options:"
    echo "  --dry-run       Show what would be executed without actually running"
    echo "  --help, -h      Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0                                    # Use latest EEST release, default output dir"
    echo "  $0 v0.1.0                            # Use specific EEST release, default output dir"
    echo "  $0 v0.1.0 /tmp/fixtures              # Use specific EEST release, custom output dir"
    echo "  $0 --dry-run                          # Show what would be executed"
    echo ""
    echo "Gas Categories:"
echo "  - benchmark-gas-value_1M: 1 million gas limit"
echo "  - benchmark-gas-value_10M: 10 million gas limit"
echo "  - benchmark-gas-value_30M: 30 million gas limit"
echo "  - benchmark-gas-value_45M: 45 million gas limit"
echo "  - benchmark-gas-value_60M: 60 million gas limit"
echo "  - benchmark-gas-value_100M: 100 million gas limit"
echo "  - benchmark-gas-value_500M: 500 million gas limit"
    exit 0
fi

# Check for dry-run flag
DRY_RUN=false
if [ "${1:-}" = "--dry-run" ]; then
    DRY_RUN=true
    shift
fi

# Now get the actual arguments after processing flags
EEST_TAG="${1:-}"
BASE_OUTPUT_DIR="${2:-./zkevm-fixtures-input}"

# Gas parameter categories
declare -a GAS_CATEGORIES=(
    "benchmark-gas-value_1M"
    "benchmark-gas-value_10M"
    "benchmark-gas-value_30M"
    "benchmark-gas-value_45M"
    "benchmark-gas-value_60M"
    "benchmark-gas-value_100M"
    "benchmark-gas-value_500M"
)

# Function to validate gas categories
validate_gas_categories() {
    print_status "$BLUE" "🔍 Validating gas parameter categories..."
    
    for category in "${GAS_CATEGORIES[@]}"; do
        if [[ "$category" =~ ^benchmark-gas-value_[0-9]+[KMG]$ ]]; then
            print_status "$GREEN" "  ✅ $category - Valid format"
        else
            print_status "$RED" "  ❌ $category - Invalid format"
            return 1
        fi
    done
    
    print_status "$GREEN" "✅ All gas categories validated"
    return 0
}

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
    
    if [ ! -d "crates/witness-generator-cli" ]; then
        print_status "$RED" "❌ Error: witness-generator-cli crate not found. This script must be run from the project root directory"
        exit 1
    fi
    
    print_status "$GREEN" "✅ Project structure verified"
}

# Function to build the project if needed
build_project() {
    print_status "$BLUE" "🔨 Building witness-generator-cli..."
    cargo build --release --bin witness-generator-cli
    if [ $? -eq 0 ]; then
        print_status "$GREEN" "✅ Build successful"
    else
        print_status "$RED" "❌ Build failed"
        exit 1
    fi
}

# Function to generate fixtures for a specific gas category
generate_fixtures() {
    local gas_category="$1"
    local output_dir="$2"
    local eest_tag_arg=""
    
    if [ -n "$EEST_TAG" ]; then
        eest_tag_arg="--tag $EEST_TAG"
    fi
    
    print_status "$BLUE" "🚀 Generating fixtures for gas category: $gas_category"
    print_status "$BLUE" "📁 Output directory: $output_dir"
    
    # Create output directory if it doesn't exist
    mkdir -p "$output_dir"
    
    # Run the witness-generator-cli
    if cargo run --release --bin witness-generator-cli -- --output-folder "$output_dir" tests $eest_tag_arg --include "$gas_category" --include "Prague"; then
        print_status "$GREEN" "✅ Successfully generated fixtures for $gas_category"
        
        # Count the generated files
        local file_count=$(find "$output_dir" -type f 2>/dev/null | wc -l)
        print_status "$GREEN" "📊 Generated $file_count files in $output_dir"
    else
        print_status "$RED" "❌ Failed to generate fixtures for $gas_category"
        return 1
    fi
}

# Function to show summary
show_summary() {
    print_status "$GREEN" "\n🎉 Fixture generation completed!"
    print_status "$BLUE" "\n📊 Summary of generated fixtures:"
    
    for category in "${GAS_CATEGORIES[@]}"; do
        # Extract the gas value part (e.g., "1M" from "benchmark-gas-value_1M")
        local gas_value=$(echo "$category" | sed 's/benchmark-gas-value_//')
        local output_dir="${BASE_OUTPUT_DIR}-${gas_value}"
        if [ -d "$output_dir" ]; then
            local file_count=$(find "$output_dir" -type f 2>/dev/null | wc -l)
            print_status "$GREEN" "  ✅ $category: $file_count files in $output_dir"
        else
            print_status "$RED" "  ❌ $category: Failed or no output directory"
        fi
    done
    
    print_status "$BLUE" "\n📁 All fixtures are located in: $BASE_OUTPUT_DIR-*"
}

# Main execution
main() {
    if [ "$DRY_RUN" = true ]; then
        print_status "$YELLOW" "🔍 DRY RUN MODE - No actual execution will occur"
        print_status "$BLUE" "🚀 Would start zkEVM fixture generation by gas categories..."
        print_status "$BLUE" "📅 EEST Tag: ${EEST_TAG:-latest}"
        print_status "$BLUE" "📁 Base Output Directory: $BASE_OUTPUT_DIR"
        print_status "$BLUE" "\n📋 Would execute the following commands:"
        
            for category in "${GAS_CATEGORIES[@]}"; do
            # Extract the gas value part (e.g., "1M" from "benchmark-gas-value_1M")
            local gas_value=$(echo "$category" | sed 's/benchmark-gas-value_//')
            local output_dir="${BASE_OUTPUT_DIR}-${gas_value}"
            local eest_tag_arg=""
            if [ -n "$EEST_TAG" ]; then
                eest_tag_arg="--tag $EEST_TAG"
            fi
            print_status "$BLUE" "  cargo run --release --bin witness-generator-cli -- --output-folder \"$output_dir\" tests $eest_tag_arg --include \"$category\" --include \"Prague\""
        done
        
        print_status "$GREEN" "\n✅ Dry run completed. Use without --dry-run to execute."
        exit 0
    fi
    
    print_status "$BLUE" "🚀 Starting zkEVM fixture generation by gas categories..."
    print_status "$BLUE" "📅 EEST Tag: ${EEST_TAG:-latest}"
    print_status "$BLUE" "📁 Base Output Directory: $BASE_OUTPUT_DIR"
    
    # Pre-flight checks
    check_cargo
    check_workspace
    
    # Validate gas categories
    if ! validate_gas_categories; then
        print_status "$RED" "❌ Gas category validation failed"
        exit 1
    fi
    
    # Build the project
    build_project
    
    # Generate fixtures for each gas category
    local failed_categories=()
    
    for category in "${GAS_CATEGORIES[@]}"; do
        # Extract the gas value part (e.g., "1M" from "benchmark-gas-value_1M")
        local gas_value=$(echo "$category" | sed 's/benchmark-gas-value_//')
        local output_dir="${BASE_OUTPUT_DIR}-${gas_value}"
        
        if generate_fixtures "$category" "$output_dir"; then
            print_status "$GREEN" "✅ Completed: $category"
        else
            print_status "$RED" "❌ Failed: $category"
            failed_categories+=("$category")
        fi
        
        echo "" # Add spacing between categories
    done
    
    # Show summary
    show_summary
    
    # Exit with error if any categories failed
    if [ ${#failed_categories[@]} -gt 0 ]; then
        print_status "$YELLOW" "\n⚠️  Some gas categories failed to generate:"
        for category in "${failed_categories[@]}"; do
            print_status "$YELLOW" "  - $category"
        done
        exit 1
    fi
    
    print_status "$GREEN" "\n🎯 All gas categories completed successfully!"
}

# Run main function
main "$@"
