#!/bin/bash
#
# run_benchmark_and_gen_report.sh
#
# Runs zkEVM benchmarks on pre-generated fixtures and generates a report.
#
# Usage:
#   ./run_benchmark_and_gen_report.sh --fixtures ./zkevm-fixtures-worstcase
#   ./run_benchmark_and_gen_report.sh --fixtures ./dir1 ./dir2 --action prove --prover risc0
#

set -e

# Default values
ACTION="execute"
PROVER="sp1"
SAMPLES=3
OUTPUT_DIR=""
RESOURCE="gpu"
FIXTURE_DIRS=()
FORCE_RERUN=false

# SP1-cluster specific options
CLUSTER_ENDPOINT=""
CLUSTER_REDIS_URL=""
CLUSTER_NUM_GPUS=4

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_usage() {
    echo "Usage: $0 --fixtures <dir> [<dir2> ...] [options]"
    echo ""
    echo "Required:"
    echo "  --fixtures <dir> [<dir2> ...]  Path(s) to zkEVM fixture directories"
    echo ""
    echo "Options:"
    echo "  --action <execute|prove>       Benchmark action (default: execute)"
    echo "  --prover <sp1|risc0|sp1-cluster>  Prover to use (default: sp1)"
    echo "  --samples <n>                  Number of benchmark runs (default: 3)"
    echo "  --output <dir>                 Output directory (default: results-<timestamp>)"
    echo "  --resource <cpu|gpu|cluster>   Resource type (default: gpu, forced to 'cluster' for sp1-cluster)"
    echo "  --force-rerun                  Re-run all benchmarks (default: skip existing)"
    echo "  -h, --help                     Show this help message"
    echo ""
    echo "SP1-Cluster Options (required when --prover sp1-cluster):"
    echo "  --cluster-endpoint <url>       SP1 cluster endpoint (or set SP1_CLUSTER_ENDPOINT env var)"
    echo "  --cluster-redis-url <url>      SP1 cluster Redis URL (or set SP1_CLUSTER_REDIS_URL env var)"
    echo "  --cluster-num-gpus <n>         Number of GPUs to use (default: 4)"
    echo ""
    echo "Examples:"
    echo "  $0 --fixtures ./zkevm-fixtures-worstcase"
    echo "  $0 --fixtures ./dir1 ./dir2 --action prove --prover risc0 --samples 5"
    echo "  $0 --fixtures ./dir1 --force-rerun  # Re-run even if results exist"
    echo "  $0 --fixtures ./dir1 --action prove --prover sp1-cluster \\"
    echo "      --cluster-endpoint \$SP1_CLUSTER_ENDPOINT --cluster-redis-url \$SP1_CLUSTER_REDIS_URL"
}

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[DONE]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --fixtures)
            shift
            while [[ $# -gt 0 && ! "$1" =~ ^-- ]]; do
                FIXTURE_DIRS+=("$1")
                shift
            done
            ;;
        --action)
            ACTION="$2"
            shift 2
            ;;
        --prover)
            PROVER="$2"
            shift 2
            ;;
        --samples)
            SAMPLES="$2"
            shift 2
            ;;
        --output)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        --resource)
            RESOURCE="$2"
            shift 2
            ;;
        --force-rerun)
            FORCE_RERUN=true
            shift
            ;;
        --cluster-endpoint)
            CLUSTER_ENDPOINT="$2"
            shift 2
            ;;
        --cluster-redis-url)
            CLUSTER_REDIS_URL="$2"
            shift 2
            ;;
        --cluster-num-gpus)
            CLUSTER_NUM_GPUS="$2"
            shift 2
            ;;
        -h|--help)
            print_usage
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            print_usage
            exit 1
            ;;
    esac
done

# Validate required arguments
if [ ${#FIXTURE_DIRS[@]} -eq 0 ]; then
    log_error "At least one fixture directory is required"
    print_usage
    exit 1
fi

# Validate fixture directories exist
for dir in "${FIXTURE_DIRS[@]}"; do
    if [ ! -d "$dir" ]; then
        log_error "Fixture directory not found: $dir"
        exit 1
    fi
done

# Validate action
if [[ "$ACTION" != "execute" && "$ACTION" != "prove" ]]; then
    log_error "Invalid action: $ACTION (must be 'execute' or 'prove')"
    exit 1
fi

# Validate prover
if [[ "$PROVER" != "sp1" && "$PROVER" != "risc0" && "$PROVER" != "sp1-cluster" ]]; then
    log_error "Invalid prover: $PROVER (must be 'sp1', 'risc0', or 'sp1-cluster')"
    exit 1
fi

# Validate and configure sp1-cluster specific options
if [[ "$PROVER" == "sp1-cluster" ]]; then
    # Force resource to cluster for sp1-cluster
    RESOURCE="cluster"
    
    # Use environment variables as fallback for cluster options
    if [ -z "$CLUSTER_ENDPOINT" ]; then
        CLUSTER_ENDPOINT="${SP1_CLUSTER_ENDPOINT:-}"
    fi
    if [ -z "$CLUSTER_REDIS_URL" ]; then
        CLUSTER_REDIS_URL="${SP1_CLUSTER_REDIS_URL:-}"
    fi
    
    # Validate required cluster options
    if [ -z "$CLUSTER_ENDPOINT" ]; then
        log_error "SP1 cluster endpoint is required (use --cluster-endpoint or set SP1_CLUSTER_ENDPOINT)"
        exit 1
    fi
    if [ -z "$CLUSTER_REDIS_URL" ]; then
        log_error "SP1 cluster Redis URL is required (use --cluster-redis-url or set SP1_CLUSTER_REDIS_URL)"
        exit 1
    fi
fi

# Set default output directory if not specified
if [ -z "$OUTPUT_DIR" ]; then
    TIMESTAMP=$(date +%Y%m%d-%H%M%S)
    OUTPUT_DIR="results-${PROVER}-${ACTION}-${TIMESTAMP}"
fi

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Count fixtures in each directory
echo ""
echo -e "${BLUE}=== ZK Benchmark Pipeline ===${NC}"
echo ""
echo "Prover:     $PROVER"
echo "Action:     $ACTION"
echo "Fixtures:   ${#FIXTURE_DIRS[@]} directory(ies)"
TOTAL_FIXTURES=0
for dir in "${FIXTURE_DIRS[@]}"; do
    COUNT=$(find "$dir" -name "*.json" -type f 2>/dev/null | wc -l)
    TOTAL_FIXTURES=$((TOTAL_FIXTURES + COUNT))
    echo "            - $(basename "$dir"): $COUNT fixtures"
done
echo "Samples:    $SAMPLES"
echo "Resource:   $RESOURCE"
if [[ "$PROVER" == "sp1-cluster" ]]; then
    echo "Cluster:    endpoint=${CLUSTER_ENDPOINT}"
    echo "            redis=${CLUSTER_REDIS_URL}"
    echo "            gpus=${CLUSTER_NUM_GPUS}"
fi
if [ "$FORCE_RERUN" = true ]; then
    echo "Force Run:  yes (re-run all benchmarks)"
else
    echo "Force Run:  no (skip existing results)"
fi
echo "Output:     $OUTPUT_DIR"
echo ""
echo "Total: $TOTAL_FIXTURES fixtures × $SAMPLES samples"
echo ""

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Build ere-hosts command base
ERE_HOSTS_CMD="cargo run --release -p ere-hosts --"
ERE_HOSTS_CMD="$ERE_HOSTS_CMD --zkvms $PROVER"
ERE_HOSTS_CMD="$ERE_HOSTS_CMD --action $ACTION"
ERE_HOSTS_CMD="$ERE_HOSTS_CMD --resource $RESOURCE"

# Add cluster-specific options for sp1-cluster
if [[ "$PROVER" == "sp1-cluster" ]]; then
    ERE_HOSTS_CMD="$ERE_HOSTS_CMD --cluster-endpoint \"$CLUSTER_ENDPOINT\""
    ERE_HOSTS_CMD="$ERE_HOSTS_CMD --cluster-redis-url \"$CLUSTER_REDIS_URL\""
    ERE_HOSTS_CMD="$ERE_HOSTS_CMD --cluster-num-gpus $CLUSTER_NUM_GPUS"
fi

if [ "$FORCE_RERUN" = true ]; then
    ERE_HOSTS_CMD="$ERE_HOSTS_CMD --force-rerun"
fi

# Track total time
TOTAL_START=$(date +%s)

# Clean up any stale docker container from previous runs
# SP1 uses port 4181, RISC0 uses port 4180
docker rm -f "ere-server-sp1-4181" 2>/dev/null || true
docker rm -f "ere-server-risc0-4180" 2>/dev/null || true

# Run benchmarks
for SAMPLE in $(seq 1 $SAMPLES); do
    SAMPLE_OUTPUT="$OUTPUT_DIR/run_${SAMPLE}"
    mkdir -p "$SAMPLE_OUTPUT"
    
    echo -e "${BLUE}=== Sample $SAMPLE/$SAMPLES ===${NC}"
    
    for FIXTURE_DIR in "${FIXTURE_DIRS[@]}"; do
        FIXTURE_NAME=$(basename "$FIXTURE_DIR")
        FIXTURE_COUNT=$(find "$FIXTURE_DIR" -name "*.json" -type f 2>/dev/null | wc -l)
        
        log_info "Running $FIXTURE_NAME ($FIXTURE_COUNT fixtures)..."
        
        RUN_START=$(date +%s)
        
        # Run ere-hosts with --input-folder for batch processing
        FULL_CMD="$ERE_HOSTS_CMD --output-folder \"$SAMPLE_OUTPUT\" stateless-executor --execution-client reth --input-folder \"$FIXTURE_DIR\""
        if eval "$FULL_CMD" 2>&1 | tee -a "$SAMPLE_OUTPUT/ere-hosts-${FIXTURE_NAME}.log"; then
            
            RUN_END=$(date +%s)
            RUN_DURATION=$((RUN_END - RUN_START))
            log_success "$FIXTURE_NAME completed in ${RUN_DURATION}s"
        else
            RUN_END=$(date +%s)
            RUN_DURATION=$((RUN_END - RUN_START))
            log_error "$FIXTURE_NAME failed after ${RUN_DURATION}s"
        fi
    done
    echo ""
done

# Process results
log_info "Processing JSON results..."

# Run the JSON to CSV converter
python3 "$SCRIPT_DIR/scripts/json_to_csv.py" \
    --input "$OUTPUT_DIR" \
    --output "$OUTPUT_DIR/results.csv"

RECORD_COUNT=$(wc -l < "$OUTPUT_DIR/results.csv")
RECORD_COUNT=$((RECORD_COUNT - 1))  # Subtract header
log_success "Processed $RECORD_COUNT records → $OUTPUT_DIR/results.csv"

# Generate report
log_info "Generating report..."

python3 "$SCRIPT_DIR/scripts/generate_report.py" \
    --input "$OUTPUT_DIR/results.csv" \
    --output "$OUTPUT_DIR/report.md" \
    --title "ZK Benchmark Report - ${PROVER^^} ${ACTION^}"

log_success "Report generated → $OUTPUT_DIR/report.md"

# Summary
TOTAL_END=$(date +%s)
TOTAL_DURATION=$((TOTAL_END - TOTAL_START))
TOTAL_MINUTES=$((TOTAL_DURATION / 60))
TOTAL_SECONDS=$((TOTAL_DURATION % 60))

echo ""
echo -e "${GREEN}=== Complete ===${NC}"
echo ""
echo "Total time:  ${TOTAL_MINUTES}m ${TOTAL_SECONDS}s"
echo "Results CSV: $OUTPUT_DIR/results.csv"
echo "Report:      $OUTPUT_DIR/report.md"
echo ""
