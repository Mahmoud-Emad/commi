#!/bin/bash

# Exit on any error
set -e

# Define color variables for better readability
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
RESET='\033[0m'

export TERM=xterm-color

# Function to print Commi ASCII art with optional coloring
print_commi_ascii() {
    echo -e "${CYAN}"
    poetry run python3 -c "import pyfiglet; print(pyfiglet.figlet_format('Commi', font='slant'))"
    echo -e "${RESET}"
}

# Function to print the description of the tool
print_commi_description() {
    echo -e "${GREEN}"
    echo "Welcome to Commi, an AI-powered Git commit message generator tool!"
    echo "This tool uses Google's Gemini AI to suggest meaningful commit messages based on your git diffs."
    echo "For more details, visit: https://github.com/Mahmoud-Emad/commi"
    echo ""
    echo -e "${RESET}"
}

# Function to ensure grpcio is properly installed
install_grpcio() {
    echo -e "${YELLOW}Ensuring grpcio is correctly installed...${RESET}"

    # Set macOS build flags for gRPC
    if [[ "$OSTYPE" == "darwin"* ]]; then
        export GRPC_PYTHON_BUILD_SYSTEM_OPENSSL=1
        export GRPC_PYTHON_BUILD_SYSTEM_ZLIB=1
    fi

    # Force reinstall grpcio
    poetry run pip uninstall -y grpcio grpcio-tools
    poetry run pip install --no-cache-dir --force-reinstall grpcio grpcio-tools

    # Verify installation
    if ! poetry run python -c "import grpc" &>/dev/null; then
        echo -e "${RED}Error: grpcio installation failed.${RESET}"
        exit 1
    fi

    echo -e "${GREEN}grpcio installed successfully.${RESET}"
}

# Function to check Python version and prompt upgrade if needed
check_python_version() {
    REQUIRED_PYTHON_VERSION=3.10
    INSTALLED_PYTHON_VERSION=$(python3 -c 'import sys; print(".".join(map(str, sys.version_info[:2])))')
    
    if [[ $(echo -e "$INSTALLED_PYTHON_VERSION\n$REQUIRED_PYTHON_VERSION" | sort -V | head -n1) != "$REQUIRED_PYTHON_VERSION" ]]; then
        echo -e "${RED}Warning: Your Python version ($INSTALLED_PYTHON_VERSION) is lower than the recommended version ($REQUIRED_PYTHON_VERSION).${RESET}"
        echo -e "${YELLOW}Please upgrade Python for better compatibility.${RESET}"
        echo -e "${CYAN}On macOS, run:${RESET}"
        echo -e "  brew update && brew upgrade python"
        echo -e "${CYAN}On Linux (Debian-based), run:${RESET}"
        echo -e "  sudo apt update && sudo apt install python3"
        echo ""
    fi
}

# Function to check if a package is installed in the current Poetry environment
check_package_installed() {
    package_name=$1
    poetry show "$package_name" &>/dev/null
}

# Function to prompt user for confirmation
ask_confirmation() {
    while true; do
        read -p "$1 (y/n): " yn
        case $yn in
            [Yy]* ) return 0;;  # Yes
            [Nn]* ) return 1;;  # No
            * ) echo "Please answer yes or no.";;
        esac
    done
}

# Function to detect the OS and install Poetry accordingly
install_poetry() {
    # Detect the operating system
    os=$(uname -s)
    case "$os" in
        Linux)
            # For Linux, install Poetry using the recommended method
            if ! command -v poetry &>/dev/null; then
                echo -e "${CYAN}Installing Poetry for Linux...${RESET}"
                curl -sSL https://install.python-poetry.org | python3 -
            fi
            ;;
        Darwin)
            # For macOS, use Homebrew or the recommended install method
            if ! command -v poetry &>/dev/null; then
                echo -e "${CYAN}Installing Poetry for macOS...${RESET}"
                brew install poetry
            fi
            ;;
        *)
            echo -e "${RED}Unsupported OS: $os${RESET}"
            exit 1
            ;;
    esac
}

# Function to check if xclip is installed and install if missing
install_xclip() {
    if ! command -v xclip &>/dev/null; then
        echo -e "${YELLOW}xclip is not installed. Installing xclip...${RESET}"
        
        # Check the platform and install xclip
        os=$(uname -s)
        case "$os" in
            Linux)
                sudo apt-get update
                sudo apt-get install -y xclip
                ;;
            Darwin)
                brew install xclip
                ;;
            *)
                echo -e "${RED}Unsupported OS: $os${RESET}"
                exit 1
                ;;
        esac

        echo -e "${CYAN}xclip installed successfully!${RESET}"
    else
        echo -e "${CYAN}xclip is already installed.${RESET}"
    fi
}

# Ensure Poetry is installed
install_poetry

# Ensure grpcio is properly installed
install_grpcio

# Check Python version
check_python_version

# Ensure xclip is installed
install_xclip

# Check if Poetry environment exists or create a new one if missing
ENV_PATH="$HOME/.cache/pypoetry/virtualenvs/poetry_commi_env"
if [ ! -d "$ENV_PATH" ]; then
    echo -e "${YELLOW}No Poetry environment found at '$ENV_PATH'. Creating a new one...${RESET}"
    poetry config virtualenvs.path "$HOME/.cache/pypoetry/virtualenvs"
    poetry env use python3
fi

# Activate the Poetry environment and install dependencies
poetry install

# Define the paths and filenames
PYTHON_ENV=$(poetry env info -p)
BIN_DIR="/usr/local/bin"
OUTPUT_BINARY="commi_$(date +'%Y%m%d%H%M%S')"

# Ensure required directories exist
mkdir -p "$BIN_DIR"

# Display initial message
echo -e "${CYAN}Preparing to install Commi...${RESET}"

# List of required dependencies
required_packages=("colorlog" "gitpython" "google-generativeai" "pyfiglet" "pyperclip" "pytest" "python-decouple")

# Install packages using Poetry if not installed already
for package in "${required_packages[@]}"; do
    if ! check_package_installed "$package"; then
        echo -e "${YELLOW}Installing missing package: $package...${RESET}"
        poetry add "$package"
    else
        echo -e "${CYAN}|+| Package $package is already installed in the Poetry environment.${RESET}"
    fi
done

# Build the binary
echo -e "${YELLOW}Building the Commi binary...${RESET}"
# On macOS, we need to ensure grpc is built from source

poetry run shiv . \
    --output-file "$OUTPUT_BINARY" \
    --python '/usr/bin/env python3' \
    --entry-point 'commi.run:main' \
    --compressed

# Check if the binary was successfully created
if [ ! -f "$OUTPUT_BINARY" ]; then
    echo -e "${RED}Error: Binary build failed. $OUTPUT_BINARY does not exist.${RESET}"
    exit 1
fi

# Check if the binary already exists in the bin directory
if [ -f "$BIN_DIR/commi" ]; then
    echo -e "${CYAN}A previous version of Commi already exists in $BIN_DIR. Overwrite?${RESET}"
    if ! ask_confirmation "Do you want to overwrite it?"; then
        echo -e "${GREEN}Installation canceled. No changes were made.${RESET}"
        exit 0
    fi
fi

# Move the binary to the bin directory
echo -e "${CYAN}Moving $OUTPUT_BINARY to $BIN_DIR...${RESET}"
sudo cp "$OUTPUT_BINARY" "$BIN_DIR/commi"

# Make the binary executable
echo -e "${CYAN}Making the binary executable...${RESET}"
sudo chmod +x "$BIN_DIR/commi"

# Verify the binary installation
if ! command -v commi &> /dev/null; then
    echo -e "${RED}Error: Installation failed. 'commi' command not found.${RESET}"
    exit 1
fi

# Clean up the temporary binary file
rm -f "$OUTPUT_BINARY"

# Clear the terminal
clear

# Print the Commi ASCII header
print_commi_ascii

# Print the description of the tool
print_commi_description

# Display usage instructions
echo -e "${GREEN}Commi installed successfully!${RESET}"
echo -e "\nUsage:\n"
echo -e "${CYAN}1. Pass the required repository link:${RESET}"
echo -e "   $ commi --repo '${HOME}/example/repo'"
echo -e "${CYAN}2. Optionally provide an API key:${RESET}"
echo -e "   $ commi --repo '${HOME}/example/repo' --api-key 'your_api_key'"
echo -e "${CYAN}3. Or set the API key as an environment variable:${RESET}"
echo -e "   $ export COMMI_API_KEY='your_api_key' && commi --repo '${HOME}/example/repo'"
echo -e "${CYAN}If you don't have an API key, you can use one of the pre-built binaries: https://github.com/Mahmoud-Emad/commi/releases.${RESET}"
