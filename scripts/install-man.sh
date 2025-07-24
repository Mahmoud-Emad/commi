#!/bin/bash

# Install commi manual page
# This script installs the commi.1 manual page so users can access it with 'man commi'

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RESET='\033[0m'

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${RESET} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${RESET} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${RESET} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${RESET} $1"
}

# Check if running as root for system-wide installation
if [[ $EUID -eq 0 ]]; then
    MAN_DIR="/usr/share/man/man1"
    INSTALL_TYPE="system-wide"
else
    # User-specific installation
    MAN_DIR="$HOME/.local/share/man/man1"
    INSTALL_TYPE="user-specific"
fi

print_status "Installing commi manual page ($INSTALL_TYPE installation)"

# Check if commi.1 exists
if [[ ! -f "docs/commi.1" ]]; then
    print_error "docs/commi.1 manual page not found"
    print_status "Make sure you're running this script from the commi repository root"
    exit 1
fi

# Create man directory if it doesn't exist
if [[ ! -d "$MAN_DIR" ]]; then
    print_status "Creating manual page directory: $MAN_DIR"
    mkdir -p "$MAN_DIR"
fi

# Copy the manual page
print_status "Installing manual page to $MAN_DIR/commi.1"
cp docs/commi.1 "$MAN_DIR/commi.1"

# Set appropriate permissions
chmod 644 "$MAN_DIR/commi.1"

# Update man database if possible
if command -v mandb >/dev/null 2>&1; then
    if [[ $EUID -eq 0 ]]; then
        print_status "Updating system manual database..."
        mandb -q
    else
        print_status "Updating user manual database..."
        mandb -u -q "$HOME/.local/share/man" 2>/dev/null || true
    fi
elif command -v makewhatis >/dev/null 2>&1; then
    print_status "Updating manual database with makewhatis..."
    if [[ $EUID -eq 0 ]]; then
        makewhatis /usr/share/man
    else
        makewhatis "$HOME/.local/share/man" 2>/dev/null || true
    fi
fi

print_success "Manual page installed successfully!"
print_status "You can now use: ${GREEN}man commi${RESET}"

# Test if man page is accessible
if man commi >/dev/null 2>&1; then
    print_success "Manual page is accessible via 'man commi'"
else
    print_warning "Manual page installed but may not be immediately accessible"
    if [[ $EUID -ne 0 ]]; then
        print_status "For user installation, you may need to add the following to your shell profile:"
        echo "export MANPATH=\"\$HOME/.local/share/man:\$MANPATH\""
        print_status "Then reload your shell or run: source ~/.bashrc (or ~/.zshrc)"
    fi
fi

print_status "Installation complete!"
