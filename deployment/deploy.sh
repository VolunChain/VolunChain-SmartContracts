#!/bin/bash

# Exit script on any error
set -e

# Default environment
ENVIRONMENT="testnet"
CONFIG_DIR="$(dirname "$0")/config"
CONTRACTS_DIR="$(dirname "$0")/contracts"

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to display usage
usage() {
    echo "Usage: $0 [-e <environment>] [-c <contract_name>]"
    echo "  -e: Environment (testnet/mainnet). Default: testnet"
    echo "  -c: Specific contract to deploy (optional)"
    exit 1
}

# Function to log messages
log() {
    echo -e "${GREEN}[$(date '+%Y-%m-%d %H:%M:%S')] $1${NC}"
}

# Function to log errors
error() {
    echo -e "${RED}[ERROR] $1${NC}" >&2
}

# Function to log warnings
warn() {
    echo -e "${YELLOW}[WARNING] $1${NC}"
}

# Parse command line arguments
while getopts "e:c:h" opt; do
    case $opt in
        e)
            ENVIRONMENT=$OPTARG
            if [[ "$ENVIRONMENT" != "testnet" && "$ENVIRONMENT" != "mainnet" ]]; then
                error "Invalid environment. Must be either 'testnet' or 'mainnet'"
                exit 1
            fi
            ;;
        c)
            CONTRACT_NAME=$OPTARG
            ;;
        h)
            usage
            ;;
        \?)
            usage
            ;;
    esac
done

# Check if config file exists
CONFIG_FILE="$CONFIG_DIR/${ENVIRONMENT}.json"
if [[ ! -f "$CONFIG_FILE" ]]; then
    error "Configuration file not found: $CONFIG_FILE"
    exit 1
fi

# Main deployment function
deploy() {
    log "Starting deployment for environment: $ENVIRONMENT"
    
    # Load configuration
    log "Loading configuration from $CONFIG_FILE"
    
    # If specific contract is specified
    if [[ -n "$CONTRACT_NAME" ]]; then
        CONTRACT_SCRIPT="$CONTRACTS_DIR/${CONTRACT_NAME}.sh"
        if [[ ! -f "$CONTRACT_SCRIPT" ]]; then
            error "Contract deployment script not found: $CONTRACT_SCRIPT"
            exit 1
        fi
        log "Deploying specific contract: $CONTRACT_NAME"
        bash "$CONTRACT_SCRIPT" -e "$ENVIRONMENT"
    else
        # Deploy all contracts in order
        log "Deploying all contracts"
        for script in "$CONTRACTS_DIR"/*.sh; do
            if [[ -f "$script" ]]; then
                log "Executing deployment script: $(basename "$script")"
                bash "$script" -e "$ENVIRONMENT"
            fi
        done
    fi
    
    log "Deployment completed successfully!"
}

# Trap errors
trap 'error "An error occurred during deployment. Check the logs for details."' ERR

# Execute deployment
deploy 