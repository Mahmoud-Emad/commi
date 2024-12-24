#!/bin/bash

# Exit on any error
set -e

# Define color variables for better readability
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
RESET='\033[0m'

# Function to print Gultron ASCII art with optional coloring
print_gultron_ascii() {
    echo -e "${CYAN}"
    poetry run python3 -c "import pyfiglet; print(pyfiglet.figlet_format('Gultron', font='slant'))"
    echo -e "${RESET}"
}

# Function to print the description of the tool
print_gultron_description() {
    echo -e "${GREEN}"
    echo "Welcome to Gultron, an AI-powered Git commit message generator tool!"
    echo "This tool uses Google's Gemini AI to suggest meaningful commit messages based on your git diffs."
    echo "For more details, visit: https://github.com/example/repo"
    echo ""
    echo -e "${RESET}"
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

# Ensure Poetry is installed
install_poetry

# Define the paths and filenames
PYTHON_ENV=$(poetry env info -p)
BIN_DIR="/usr/local/bin"
OUTPUT_BINARY="gultron_$(date +'%Y%m%d%H%M%S')"

# Ensure required directories exist
mkdir -p "$BIN_DIR"

# Display initial message
echo -e "${CYAN}Preparing to install Gultron...${RESET}"

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
echo -e "${YELLOW}Building the Gultron binary...${RESET}"
poetry run shiv --site-packages "$PYTHON_ENV/lib/python3.12/site-packages" \
    --output-file "$OUTPUT_BINARY" \
    --python '/usr/bin/env python3' \
    --entry-point 'gultron.run:main'

# Check if the binary was successfully created
if [ ! -f "$OUTPUT_BINARY" ]; then
    echo -e "${RED}Error: Binary build failed. $OUTPUT_BINARY does not exist.${RESET}"
    exit 1
fi

# Check if the binary already exists in the bin directory
if [ -f "$BIN_DIR/gultron" ]; then
    echo -e "${CYAN}A previous version of Gultron already exists in $BIN_DIR. Overwrite?${RESET}"
    if ! ask_confirmation "Do you want to overwrite it?"; then
        echo -e "${GREEN}Installation canceled. No changes were made.${RESET}"
        exit 0
    fi
fi

# Move the binary to the bin directory
echo -e "${CYAN}Moving $OUTPUT_BINARY to $BIN_DIR...${RESET}"
sudo cp "$OUTPUT_BINARY" "$BIN_DIR/gultron"

# Make the binary executable
echo -e "${CYAN}Making the binary executable...${RESET}"
sudo chmod +x "$BIN_DIR/gultron"

# Verify the binary installation
if ! command -v gultron &> /dev/null; then
    echo -e "${RED}Error: Installation failed. 'gultron' command not found.${RESET}"
    exit 1
fi

# Clean up the temporary binary file
rm -f "$OUTPUT_BINARY"

# Clear the terminal
clear

# Print the Gultron ASCII header
print_gultron_ascii

# Print the description of the tool
print_gultron_description

# Display usage instructions
echo -e "${GREEN}Gultron installed successfully!${RESET}"
echo -e "\nUsage:\n"
echo -e "${CYAN}1. Pass the required repository link:${RESET}"
echo -e "   $ gultron --repo '${HOME}/example/repo'"
echo -e "${CYAN}2. Optionally provide an API key:${RESET}"
echo -e "   $ gultron --repo '${HOME}/example/repo' --api-key 'your_api_key'"
echo -e "${CYAN}3. Or set the API key as an environment variable:${RESET}"
echo -e "   $ export API_KEY='your_api_key' && gultron --repo '${HOME}/example/repo'"
echo -e "${CYAN}If no API key is provided, a default value will be used.${RESET}"
