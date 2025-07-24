#!/bin/sh

# Commi Installation Script - Fixed Version
# POSIX-compliant shell script to install commi from GitHub releases

set -e  # Exit on any error

# Configuration
REPO="Mahmoud-Emad/commi"
BINARY_NAME="commi"
INSTALL_DIR="/usr/local/bin"
GITHUB_API="https://api.github.com/repos/$REPO/releases/latest"

# Logging functions
log_info() { printf "[INFO] %s\n" "$1"; }
log_error() { printf "[ERROR] %s\n" "$1" >&2; }
error_exit() { log_error "$1"; exit 1; }

# Check if command exists
command_exists() { command -v "$1" >/dev/null 2>&1; }

# Detect platform and architecture
detect_platform() {
    case "$(uname -s)" in
        Darwin*) os="apple-darwin" ;;
        Linux*)  os="unknown-linux-gnu" ;;
        *)       error_exit "Unsupported OS: $(uname -s)" ;;
    esac
    
    case "$(uname -m)" in
        x86_64|amd64) arch="x86_64" ;;
        arm64|aarch64) arch="aarch64" ;;
        *)            error_exit "Unsupported architecture: $(uname -m)" ;;
    esac
    
    echo "$arch-$os"
}

# Get download URL for the platform
get_download_url() {
    platform=$(detect_platform)
    asset_name="$BINARY_NAME-$platform"
    tmpfile=$(mktemp)
    
    log_info "Looking for asset: $asset_name"
    
    # Fetch release info
    if command_exists curl; then
        curl -sSf "$GITHUB_API" > "$tmpfile" || error_exit "Failed to fetch release info"
    elif command_exists wget; then
        wget -qO- "$GITHUB_API" > "$tmpfile" || error_exit "Failed to fetch release info"
    else
        error_exit "curl or wget required"
    fi
    
    # Parse JSON to find download URL
    if command_exists jq; then
        url=$(jq -r ".assets[] | select(.name == \"$asset_name\") | .browser_download_url" "$tmpfile" 2>/dev/null)
    else
        # Simple fallback without complex awk
        url=$(grep -A 20 "\"name\":\"$asset_name\"" "$tmpfile" | grep "browser_download_url" | head -1 | sed 's/.*"browser_download_url": *"\([^"]*\)".*/\1/')
    fi
    
    if [ -z "$url" ]; then
        log_error "No binary found for: $asset_name"
        log_error "This usually means:"
        log_error "  1. The platform is not supported"
        log_error "  2. The release doesn't have binaries yet"
        log_error "  3. There's a network issue"
        log_error ""
        log_error "Supported platforms:"
        log_error "  - x86_64-unknown-linux-gnu (Linux 64-bit)"
        log_error "  - aarch64-unknown-linux-gnu (Linux ARM64)"
        log_error "  - x86_64-apple-darwin (macOS Intel)"
        log_error "  - aarch64-apple-darwin (macOS Apple Silicon)"
        log_error "  - x86_64-pc-windows-msvc.exe (Windows 64-bit)"
        rm -f "$tmpfile"
        error_exit "Binary not available"
    fi
    
    rm -f "$tmpfile"
    echo "$url"
}

# Download and install binary
install_binary() {
    url="$1"
    tmpbin=$(mktemp)
    
    log_info "Downloading from: $url"
    
    if command_exists curl; then
        curl -sSfL "$url" -o "$tmpbin" || error_exit "Download failed"
    elif command_exists wget; then
        wget -qO "$tmpbin" "$url" || error_exit "Download failed"
    fi
    
    # Verify download
    if [ ! -s "$tmpbin" ]; then
        rm -f "$tmpbin"
        error_exit "Downloaded file is empty"
    fi
    
    log_info "Installing to $INSTALL_DIR/$BINARY_NAME"
    
    # Install with sudo if needed
    if [ -w "$INSTALL_DIR" ]; then
        cp "$tmpbin" "$INSTALL_DIR/$BINARY_NAME" || error_exit "Install failed"
        chmod +x "$INSTALL_DIR/$BINARY_NAME" || error_exit "chmod failed"
    else
        sudo cp "$tmpbin" "$INSTALL_DIR/$BINARY_NAME" || error_exit "Install failed (try with sudo)"
        sudo chmod +x "$INSTALL_DIR/$BINARY_NAME" || error_exit "chmod failed"
    fi
    
    rm -f "$tmpbin"
}

# Verify installation
verify_installation() {
    if ! command_exists "$BINARY_NAME"; then
        error_exit "Installation failed: $BINARY_NAME not found in PATH"
    fi
    
    log_info "Verifying installation..."
    if "$BINARY_NAME" --version >/dev/null 2>&1 || "$BINARY_NAME" --help >/dev/null 2>&1; then
        log_info "✅ Installation successful!"
        log_info "Run '$BINARY_NAME --help' to get started"
    else
        error_exit "Installation verification failed"
    fi
}

# Main installation
main() {
    log_info "Installing commi for $(detect_platform)..."
    
    url=$(get_download_url)
    install_binary "$url"
    verify_installation
    
    log_info "Done! Commi is ready to use."
}

# Run installation
main
