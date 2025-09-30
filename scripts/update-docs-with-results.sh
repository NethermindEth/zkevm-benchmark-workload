#!/usr/bin/env bash
#
# update-docs-with-results.sh
#
# Updates the documentation with the latest benchmark results by copying
# generated markdown reports to the documentation website.
#
# Usage:
#   ./scripts/update-docs-with-results.sh [OPTIONS]
#
# Options:
#   --help, -h              Show this help message
#   --source <DIR>          Source directory for results (default: benchmark-results/markdown-reports/latest)
#   --target <DIR>          Target directory in docs (default: www/docs/pages/benchmark-results)
#   --dry-run               Show what would be copied without actually copying
#
# Examples:
#   # Update docs with latest results
#   ./scripts/update-docs-with-results.sh
#
#   # Update docs with results from specific directory
#   ./scripts/update-docs-with-results.sh --source benchmark-results/markdown-reports/comparisons
#
#   # Preview what would be updated
#   ./scripts/update-docs-with-results.sh --dry-run
#

set -euo pipefail

# Default values
SOURCE_DIR="benchmark-results/markdown-reports/latest"
TARGET_DIR="www/docs/pages/benchmark-results"
DRY_RUN=false

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
    echo "Updates the documentation with the latest benchmark results by copying"
    echo "generated markdown reports to the documentation website."
    echo ""
    echo "Options:"
    echo "  --help, -h              Show this help message"
    echo "  --source <DIR>          Source directory for results (default: benchmark-results/markdown-reports/latest)"
    echo "  --target <DIR>          Target directory in docs (default: www/docs/pages/benchmark-results)"
    echo "  --dry-run               Show what would be copied without actually copying"
    echo ""
    echo "Examples:"
    echo "  $0                                    # Update docs with latest results"
    echo "  $0 --source benchmark-results/markdown-reports/comparisons  # Update with comparison results"
    echo "  $0 --dry-run                          # Preview what would be updated"
    exit 0
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --help|-h)
            show_help
            ;;
        --source)
            SOURCE_DIR="$2"
            shift 2
            ;;
        --target)
            TARGET_DIR="$2"
            shift 2
            ;;
        --dry-run)
            DRY_RUN=true
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
    
    if [ ! -d "www/docs/pages" ]; then
        print_status "$RED" "❌ Error: www/docs/pages directory not found. This script must be run from the project root directory"
        exit 1
    fi
    
    print_status "$GREEN" "✅ Project structure verified"
}

# Function to check if source directory exists
check_source() {
    if [ ! -d "$SOURCE_DIR" ]; then
        print_status "$YELLOW" "⚠️  Warning: Source directory $SOURCE_DIR does not exist"
        print_status "$YELLOW" "   Run profiling scripts first to generate results"
        return 1
    fi
    
    # Check if there are any markdown files
    if ! find "$SOURCE_DIR" -name "*.md" -type f | grep -q .; then
        print_status "$YELLOW" "⚠️  Warning: No markdown files found in $SOURCE_DIR"
        print_status "$YELLOW" "   Generate markdown reports first"
        return 1
    fi
    
    print_status "$GREEN" "✅ Source directory verified: $SOURCE_DIR"
    return 0
}

# Function to create target directory structure
create_target_structure() {
    if [ "$DRY_RUN" = true ]; then
        print_status "$BLUE" "🔍 Would create target directory: $TARGET_DIR"
        return 0
    fi
    
    mkdir -p "$TARGET_DIR"
    print_status "$GREEN" "✅ Created target directory: $TARGET_DIR"
}

# Function to copy results to documentation
copy_results() {
    local source="$1"
    local target="$2"
    
    if [ "$DRY_RUN" = true ]; then
        print_status "$BLUE" "🔍 Would copy files from $source to $target"
        find "$source" -name "*.md" -type f | while read -r file; do
            local rel_path="${file#$source/}"
            print_status "$BLUE" "  - $file -> $target/$rel_path"
        done
        return 0
    fi
    
    # Copy all markdown files
    find "$source" -name "*.md" -type f | while read -r file; do
        local rel_path="${file#$source/}"
        local target_file="$target/$rel_path"
        local target_dir=$(dirname "$target_file")
        
        # Create target directory if it doesn't exist
        mkdir -p "$target_dir"
        
        # Copy the file
        cp "$file" "$target_file"
        print_status "$GREEN" "✅ Copied: $rel_path"
    done
}

# Function to update the main benchmark-results.mdx file
update_main_docs() {
    local results_dir="$1"
    
    if [ "$DRY_RUN" = true ]; then
        print_status "$BLUE" "🔍 Would update main benchmark-results.mdx with results from $results_dir"
        return 0
    fi
    
    # Find the main results file
    local main_results_file=""
    if [ -f "$results_dir/profiling-results.md" ]; then
        main_results_file="$results_dir/profiling-results.md"
    elif [ -f "$results_dir/comprehensive-analysis.md" ]; then
        main_results_file="$results_dir/comprehensive-analysis.md"
    else
        # Find any markdown file in the directory
        main_results_file=$(find "$results_dir" -name "*.md" -type f | head -1)
    fi
    
    if [ -n "$main_results_file" ] && [ -f "$main_results_file" ]; then
        # Create a backup of the original file
        cp "www/docs/pages/benchmark-results.mdx" "www/docs/pages/benchmark-results.mdx.backup"
        
        # Create new content
        cat > "www/docs/pages/benchmark-results.mdx" << EOF
# Benchmark Results

This page displays the generated benchmark results from zkGas profiling. Here you can view, analyze, and compare OPCODE resource requirements across different gas categories and zkVM implementations.

## Latest Results

*Results generated on: $(date)*

EOF
        
        # Append the results content
        cat "$main_results_file" >> "www/docs/pages/benchmark-results.mdx"
        
        print_status "$GREEN" "✅ Updated main benchmark-results.mdx with latest results"
    else
        print_status "$YELLOW" "⚠️  Warning: No main results file found to update documentation"
    fi
}

# Function to show summary
show_summary() {
    print_status "$GREEN" "\n🎉 Documentation update completed!"
    
    if [ "$DRY_RUN" = true ]; then
        print_status "$BLUE" "\n📋 Summary of what would be updated:"
        print_status "$BLUE" "  - Source: $SOURCE_DIR"
        print_status "$BLUE" "  - Target: $TARGET_DIR"
        print_status "$BLUE" "  - Main docs: www/docs/pages/benchmark-results.mdx"
        print_status "$GREEN" "\n✅ Dry run completed. Use without --dry-run to execute."
    else
        print_status "$BLUE" "\n📊 Summary of updated documentation:"
        print_status "$BLUE" "  - Source: $SOURCE_DIR"
        print_status "$BLUE" "  - Target: $TARGET_DIR"
        print_status "$BLUE" "  - Main docs: www/docs/pages/benchmark-results.mdx"
        
        if [ -d "$TARGET_DIR" ]; then
            local file_count=$(find "$TARGET_DIR" -name "*.md" -type f | wc -l)
            print_status "$GREEN" "  - Files copied: $file_count"
        fi
        
        print_status "$GREEN" "\n📖 View updated documentation at: www/docs/pages/benchmark-results.mdx"
    fi
}

# Main execution
main() {
    print_status "$BLUE" "🔍 zkGas Profiling Documentation Updater"
    
    # Pre-flight checks
    check_workspace
    
    # Check source directory
    if ! check_source; then
        print_status "$YELLOW" "⚠️  No results to update documentation with"
        print_status "$YELLOW" "   Run profiling scripts first to generate results"
        exit 0
    fi
    
    # Create target structure
    create_target_structure
    
    # Copy results to documentation
    copy_results "$SOURCE_DIR" "$TARGET_DIR"
    
    # Update main documentation file
    update_main_docs "$SOURCE_DIR"
    
    # Show summary
    show_summary
}

# Run main function
main "$@"
