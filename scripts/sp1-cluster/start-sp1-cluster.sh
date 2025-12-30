#!/usr/bin/env bash
#
# SP1 Cluster Startup Script
# 
# Usage:
#   ./start-sp1-cluster.sh [OPTIONS]
#
# Options:
#   --gpu-nodes N    Number of GPU worker nodes (default: 1)
#                    Use 0 for CPU-only mode
#   --build          Force rebuild of Docker images
#   --detach, -d     Run in detached mode
#   --help, -h       Show this help message
#
# Examples:
#   ./start-sp1-cluster.sh                  # 1 GPU worker (default)
#   ./start-sp1-cluster.sh --gpu-nodes 2    # 2 GPU workers
#   ./start-sp1-cluster.sh --gpu-nodes 0    # CPU-only mode
#   ./start-sp1-cluster.sh --gpu-nodes 4 -d # 4 GPU workers, detached

set -euo pipefail

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Default values
GPU_NODES=1
FORCE_BUILD=false
DETACH=false

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Show help
show_help() {
    cat << EOF
SP1 Cluster Startup Script

Usage:
  $0 [OPTIONS]

Options:
  --gpu-nodes N    Number of GPU worker nodes (default: 1)
                   Use 0 for CPU-only mode
  --build          Force rebuild of Docker images
  --detach, -d     Run in detached mode
  --help, -h       Show this help message

Examples:
  $0                        # 1 GPU worker (default)
  $0 --gpu-nodes 2          # 2 GPU workers
  $0 --gpu-nodes 0          # CPU-only mode
  $0 --gpu-nodes 4 -d       # 4 GPU workers, detached

API Endpoint:
  Once running, the proving API will be available at:
  http://localhost:8080/api/v1/prove

EOF
    exit 0
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --gpu-nodes)
            GPU_NODES="$2"
            shift 2
            ;;
        --build)
            FORCE_BUILD=true
            shift
            ;;
        --detach|-d)
            DETACH=true
            shift
            ;;
        --help|-h)
            show_help
            ;;
        *)
            log_error "Unknown option: $1"
            show_help
            ;;
    esac
done

# Validate GPU_NODES is a non-negative integer
if ! [[ "$GPU_NODES" =~ ^[0-9]+$ ]]; then
    log_error "--gpu-nodes must be a non-negative integer"
    exit 1
fi

# Load environment variables if .env exists
if [[ -f ".env" ]]; then
    log_info "Loading environment from .env"
    set -a
    source .env
    set +a
else
    log_warn "No .env file found. Using defaults. Copy env.example to .env to customize."
fi

# Configuration
SP1_CLUSTER_REPO="${SP1_CLUSTER_REPO:-https://github.com/succinctlabs/sp1-cluster.git}"
SP1_CLUSTER_BRANCH="${SP1_CLUSTER_BRANCH:-main}"
SP1_CLUSTER_PATH="${SP1_CLUSTER_PATH:-./sp1-cluster}"
API_PORT="${API_PORT:-8080}"

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check Docker
    if ! command -v docker &> /dev/null; then
        log_error "Docker is not installed. Please install Docker first."
        exit 1
    fi
    
    # Check Docker Compose
    if ! docker compose version &> /dev/null; then
        log_error "Docker Compose is not available. Please install Docker Compose v2."
        exit 1
    fi
    
    # Check NVIDIA runtime if GPU nodes requested
    if [[ "$GPU_NODES" -gt 0 ]]; then
        if ! docker info 2>/dev/null | grep -q "nvidia"; then
            log_warn "NVIDIA Docker runtime not detected. GPU workers may not function correctly."
            log_warn "Install nvidia-docker2 or nvidia-container-toolkit for GPU support."
        fi
    fi
    
    log_success "Prerequisites check passed"
}

# Clone or update sp1-cluster repository
setup_sp1_cluster_repo() {
    log_info "Setting up sp1-cluster repository..."
    
    if [[ -d "$SP1_CLUSTER_PATH" ]]; then
        log_info "sp1-cluster repository already exists at $SP1_CLUSTER_PATH"
        
        # Update to latest
        pushd "$SP1_CLUSTER_PATH" > /dev/null
        git fetch origin
        git checkout "$SP1_CLUSTER_BRANCH"
        git pull origin "$SP1_CLUSTER_BRANCH" || log_warn "Could not pull latest changes"
        popd > /dev/null
    else
        log_info "Cloning sp1-cluster from $SP1_CLUSTER_REPO"
        git clone --branch "$SP1_CLUSTER_BRANCH" "$SP1_CLUSTER_REPO" "$SP1_CLUSTER_PATH"
    fi
    
    log_success "sp1-cluster repository ready"
}

# Build Docker images
build_images() {
    log_info "Building Docker images..."
    
    local build_args=""
    if [[ "$FORCE_BUILD" == true ]]; then
        build_args="--build"
    fi
    
    # Build base services
    docker compose -f docker-compose.yml build $build_args
    
    # Build worker images based on mode
    if [[ "$GPU_NODES" -gt 0 ]]; then
        docker compose -f docker-compose.yml -f docker-compose.gpu.yml build $build_args
    else
        docker compose -f docker-compose.yml -f docker-compose.cpu.yml build $build_args
    fi
    
    log_success "Docker images built"
}

# Start the cluster
start_cluster() {
    local compose_files="-f docker-compose.yml"
    local scale_args=""
    local detach_flag=""
    
    if [[ "$DETACH" == true ]]; then
        detach_flag="-d"
    fi
    
    # Determine worker configuration
    if [[ "$GPU_NODES" -gt 0 ]]; then
        compose_files="$compose_files -f docker-compose.gpu.yml"
        scale_args="--scale sp1-gpu-worker=$GPU_NODES"
        log_info "Starting SP1 Cluster with $GPU_NODES GPU worker(s)..."
    else
        compose_files="$compose_files -f docker-compose.cpu.yml"
        scale_args="--scale sp1-cpu-worker=1"
        log_info "Starting SP1 Cluster in CPU-only mode..."
    fi
    
    # Start services
    docker compose $compose_files up $detach_flag $scale_args
}

# Wait for services to be healthy
wait_for_health() {
    log_info "Waiting for services to be healthy..."
    
    local max_attempts=30
    local attempt=0
    
    while [[ $attempt -lt $max_attempts ]]; do
        if curl -s "http://localhost:${API_PORT}/health" > /dev/null 2>&1; then
            log_success "SP1 Cluster API is healthy"
            return 0
        fi
        
        attempt=$((attempt + 1))
        echo -n "."
        sleep 2
    done
    
    log_warn "API health check timed out. Services may still be starting..."
    return 1
}

# Print cluster information
print_info() {
    echo ""
    echo "========================================"
    echo -e "${GREEN}SP1 Cluster is running!${NC}"
    echo "========================================"
    echo ""
    echo "API Endpoint:    http://localhost:${API_PORT}"
    echo "Prove Endpoint:  http://localhost:${API_PORT}/api/v1/prove"
    echo ""
    if [[ "$GPU_NODES" -gt 0 ]]; then
        echo "Worker Mode:     GPU ($GPU_NODES worker(s))"
    else
        echo "Worker Mode:     CPU"
    fi
    echo ""
    echo "Useful commands:"
    echo "  View logs:     docker compose logs -f"
    echo "  Stop cluster:  ./stop-sp1-cluster.sh"
    echo ""
}

# Main execution
main() {
    echo ""
    echo "========================================"
    echo "       SP1 Cluster Startup Script       "
    echo "========================================"
    echo ""
    
    check_prerequisites
    setup_sp1_cluster_repo
    
    if [[ "$FORCE_BUILD" == true ]] || ! docker images | grep -q "sp1-cluster"; then
        build_images
    fi
    
    start_cluster
    
    if [[ "$DETACH" == true ]]; then
        wait_for_health || true
        print_info
    fi
}

main

