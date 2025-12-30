#!/usr/bin/env bash
#
# SP1 Cluster Stop Script
#
# Usage:
#   ./stop-sp1-cluster.sh [OPTIONS]
#
# Options:
#   --remove-volumes    Remove persistent volumes (database, redis data)
#   --remove-images     Remove Docker images
#   --all               Remove everything (volumes + images)
#   --help, -h          Show this help message

set -euo pipefail

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Default values
REMOVE_VOLUMES=false
REMOVE_IMAGES=false

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
SP1 Cluster Stop Script

Usage:
  $0 [OPTIONS]

Options:
  --remove-volumes    Remove persistent volumes (database, redis data)
  --remove-images     Remove Docker images
  --all               Remove everything (volumes + images)
  --help, -h          Show this help message

Examples:
  $0                      # Stop services, keep data
  $0 --remove-volumes     # Stop and remove all data
  $0 --all                # Complete cleanup

EOF
    exit 0
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --remove-volumes)
            REMOVE_VOLUMES=true
            shift
            ;;
        --remove-images)
            REMOVE_IMAGES=true
            shift
            ;;
        --all)
            REMOVE_VOLUMES=true
            REMOVE_IMAGES=true
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

# Stop all services
stop_services() {
    log_info "Stopping SP1 Cluster services..."
    
    # Stop with all possible compose files to ensure everything is stopped
    docker compose \
        -f docker-compose.yml \
        -f docker-compose.gpu.yml \
        -f docker-compose.cpu.yml \
        down 2>/dev/null || true
    
    log_success "Services stopped"
}

# Remove volumes
remove_volumes() {
    if [[ "$REMOVE_VOLUMES" == true ]]; then
        log_info "Removing persistent volumes..."
        
        docker compose \
            -f docker-compose.yml \
            -f docker-compose.gpu.yml \
            -f docker-compose.cpu.yml \
            down -v 2>/dev/null || true
        
        log_success "Volumes removed"
    fi
}

# Remove images
remove_images() {
    if [[ "$REMOVE_IMAGES" == true ]]; then
        log_info "Removing Docker images..."
        
        # Get image names from compose files and remove them
        docker compose \
            -f docker-compose.yml \
            -f docker-compose.gpu.yml \
            -f docker-compose.cpu.yml \
            down --rmi local 2>/dev/null || true
        
        log_success "Images removed"
    fi
}

# Main execution
main() {
    echo ""
    echo "========================================"
    echo "        SP1 Cluster Stop Script         "
    echo "========================================"
    echo ""
    
    stop_services
    remove_volumes
    remove_images
    
    echo ""
    log_success "SP1 Cluster has been stopped"
    
    if [[ "$REMOVE_VOLUMES" == true ]]; then
        log_info "Persistent data has been removed"
    else
        log_info "Persistent data preserved. Use --remove-volumes to delete."
    fi
    echo ""
}

main

