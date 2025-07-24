#!/bin/bash

# Build script for Commi Rust version
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
RESET='\033[0m'

# Print header
echo -e "${CYAN}"
echo "   ______                          _   "
echo "  / ____/___  ____ ___  ____ ___  (_)  "
echo " / /   / __ \/ __ \`__ \/ __ \`__ \/ / "
echo "/ /___/ /_/ / / / / / / / / / / / /    "
echo "\____/\____/_/ /_/ /_/_/ /_/ /_/_/     "
echo -e "${RESET}"
echo -e "${GREEN}Building Commi Rust version...${RESET}"
echo

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust/Cargo not found. Please install Rust first.${RESET}"
    echo -e "${YELLOW}Visit: https://rustup.rs/${RESET}"
    exit 1
fi

# Check Rust version
RUST_VERSION=$(rustc --version | cut -d' ' -f2)
echo -e "${BLUE}Using Rust version: ${RUST_VERSION}${RESET}"

# Build type
BUILD_TYPE=${1:-release}
if [ "$BUILD_TYPE" = "debug" ]; then
    echo -e "${YELLOW}Building in debug mode...${RESET}"
    cargo build
    BINARY_PATH="target/debug/commi"
else
    echo -e "${YELLOW}Building in release mode...${RESET}"
    cargo build --release
    BINARY_PATH="target/release/commi"
fi

# Check if build was successful
if [ ! -f "$BINARY_PATH" ]; then
    echo -e "${RED}Error: Build failed. Binary not found at $BINARY_PATH${RESET}"
    exit 1
fi

# Get binary size
BINARY_SIZE=$(du -h "$BINARY_PATH" | cut -f1)
echo -e "${GREEN}Build successful! Binary size: ${BINARY_SIZE}${RESET}"

# Ask if user wants to install
echo
read -p "Do you want to install commi to /usr/local/bin? (y/n): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}Installing commi...${RESET}"
    sudo cp "$BINARY_PATH" /usr/local/bin/commi
    sudo chmod +x /usr/local/bin/commi
    echo -e "${GREEN}Commi installed successfully!${RESET}"
    
    # Verify installation
    if command -v commi &> /dev/null; then
        echo -e "${GREEN}Installation verified. You can now use 'commi' from anywhere.${RESET}"
    else
        echo -e "${RED}Warning: 'commi' command not found in PATH.${RESET}"
    fi
else
    echo -e "${BLUE}Binary available at: ${BINARY_PATH}${RESET}"
    echo -e "${BLUE}You can run it with: ./${BINARY_PATH}${RESET}"
fi

echo
echo -e "${CYAN}Usage examples:${RESET}"
echo -e "  commi                           # Generate commit message"
echo -e "  commi --copy                    # Copy to clipboard"
echo -e "  commi --commit                  # Auto-commit"
echo -e "  commi --help                    # Show help"
echo
echo -e "${GREEN}Happy committing! 🦀${RESET}"
