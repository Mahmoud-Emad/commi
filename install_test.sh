#!/bin/sh

# Commi Installation Script
# POSIX-compliant shell script to install commi from GitHub releases

set -e  # Exit on any error

# Configuration
REPO="Mahmoud-Emad/commi"
BINARY_NAME="commi"
INSTALL_DIR="/usr/local/bin"
MAN_DIR="/usr/local/share/man/man1"
GITHUB_API="https://api.github.com/repos/$REPO/releases/latest"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    printf "${BLUE}[INFO]${NC} %s\n" "$1"
}

log_success() {
    printf "${GREEN}[SUCCESS]${NC} %s\n" "$1"
}

log_warning() {
    printf "${YELLOW}[WARNING]${NC} %s\n" "$1"
}

log_error() {
    printf "${RED}[ERROR]${NC} %s\n" "$1" >&2
}

# Error handler
error_exit() {
    log_error "$1"
    exit 1
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Detect platform and architecture
detect_platform() {
    local os arch
    
    # Detect OS
    case "$(uname -s)" in
        Darwin*) os="apple-darwin" ;;
        Linux*)  os="unknown-linux-gnu" ;;
        *)       error_exit "Unsupported operating system: $(uname -s)" ;;
    esac
    
    # Detect architecture
    case "$(uname -m)" in
        x86_64|amd64) arch="x86_64" ;;
        arm64|aarch64) arch="aarch64" ;;
        *)            error_exit "Unsupported architecture: $(uname -m)" ;;
    esac
    
    echo "${arch}-${os}"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    if ! command_exists curl && ! command_exists wget; then
        error_exit "Either curl or wget is required for downloading"
    fi
    
    if ! command_exists jq; then
        log_warning "jq not found. Will use basic JSON parsing (less reliable)"
    fi
    
    log_success "Prerequisites check completed"
}

# Get latest release info from GitHub API
get_latest_release() {
    local api_response temp_file
    temp_file=$(mktemp)
    
    log_info "Fetching latest release information..."
    
    if command_exists curl; then
        curl -s "$GITHUB_API" > "$temp_file" || error_exit "Failed to fetch release information"
    elif command_exists wget; then
        wget -q -O "$temp_file" "$GITHUB_API" || error_exit "Failed to fetch release information"
    fi
    
    # Extract download URL for the detected platform
    local platform asset_name download_url
    platform=$(detect_platform)
    asset_name="${BINARY_NAME}-${platform}"
    
    if command_exists jq; then
        download_url=$(jq -r ".assets[] | select(.name == \"$asset_name\") | .browser_download_url" "$temp_file")
    else
        # Fallback: basic grep/sed parsing
        download_url=$(grep "\"name\":\"$asset_name\"" "$temp_file" -A 10 | grep "browser_download_url" | head -1 | sed 's/.*"browser_download_url": *"\([^"]*\)".*/\1/')
    fi
    
    rm -f "$temp_file"
    
    if [ -z "$download_url" ] || [ "$download_url" = "null" ]; then
        error_exit "No binary found for platform: $platform"
    fi

    # Validate URL format
    case "$download_url" in
        https://github.com/*)
            log_success "Found binary for platform: $platform"
            ;;
        *)
            error_exit "Invalid download URL format: $download_url"
            ;;
    esac

    echo "$download_url"
}

# Download and install binary
install_binary() {
    local download_url="$1"
    local temp_binary
    temp_binary=$(mktemp)
    
    log_info "Downloading commi binary..."
    
    if command_exists curl; then
        curl -L -o "$temp_binary" "$download_url" || error_exit "Failed to download binary"
    elif command_exists wget; then
        wget -O "$temp_binary" "$download_url" || error_exit "Failed to download binary"
    fi
    
    # Verify download
    if [ ! -s "$temp_binary" ]; then
        rm -f "$temp_binary"
        error_exit "Downloaded binary is empty"
    fi
    
    log_info "Installing binary to $INSTALL_DIR/$BINARY_NAME..."

    # Create install directory if it doesn't exist
    if [ ! -d "$INSTALL_DIR" ]; then
        if [ -w "$(dirname "$INSTALL_DIR")" ] || [ -n "$NO_SUDO" ]; then
            mkdir -p "$INSTALL_DIR" || error_exit "Failed to create install directory"
        else
            sudo mkdir -p "$INSTALL_DIR" || error_exit "Failed to create install directory"
        fi
    fi

    # Install binary
    if [ -w "$INSTALL_DIR" ] || [ -n "$NO_SUDO" ]; then
        cp "$temp_binary" "$INSTALL_DIR/$BINARY_NAME" || error_exit "Failed to install binary"
        chmod +x "$INSTALL_DIR/$BINARY_NAME" || error_exit "Failed to make binary executable"
    else
        sudo cp "$temp_binary" "$INSTALL_DIR/$BINARY_NAME" || error_exit "Failed to install binary"
        sudo chmod +x "$INSTALL_DIR/$BINARY_NAME" || error_exit "Failed to make binary executable"
    fi
    
    rm -f "$temp_binary"
    log_success "Binary installed successfully"
}

# Install manual page
install_manual() {
    log_info "Installing manual page..."
    
    # Create man directory if it doesn't exist
    if [ ! -d "$MAN_DIR" ]; then
        sudo mkdir -p "$MAN_DIR" || {
            log_warning "Failed to create man directory, skipping manual installation"
            return
        }
    fi
    
    # Check if commi has a built-in manual
    if command_exists "$INSTALL_DIR/$BINARY_NAME"; then
        # Try to generate man page (if commi supports it)
        if "$INSTALL_DIR/$BINARY_NAME" --help >/dev/null 2>&1; then
            # Create a basic man page from help output
            {
                echo ".TH COMMI 1 \"$(date +'%B %Y')\" \"commi\" \"User Commands\""
                echo ".SH NAME"
                echo "commi \\- AI-powered Git commit message generator"
                echo ".SH SYNOPSIS"
                echo ".B commi"
                echo "[\\fIOPTIONS\\fR] [\\fICOMMAND\\fR]"
                echo ".SH DESCRIPTION"
                echo "Commi is an AI-powered Git commit message generator that uses Google's Gemini AI"
                echo "to suggest meaningful commit messages based on your git diffs."
                echo ".SH OPTIONS"
                "$INSTALL_DIR/$BINARY_NAME" --help 2>/dev/null | sed 's/^/.TP\n/' || true
                echo ".SH SEE ALSO"
                echo "git(1)"
                echo ".SH AUTHOR"
                echo "Visit https://github.com/$REPO for more information."
            } | sudo tee "$MAN_DIR/$BINARY_NAME.1" >/dev/null
            
            log_success "Manual page installed"
        else
            log_warning "Could not generate manual page"
        fi
    fi
}

# Setup shell completions
setup_completions() {
    log_info "Setting up shell completions..."
    
    if ! command_exists "$INSTALL_DIR/$BINARY_NAME"; then
        log_warning "Binary not found, skipping completion setup"
        return
    fi
    
    # Check if commi supports completion generation
    if ! "$INSTALL_DIR/$BINARY_NAME" completion --help >/dev/null 2>&1; then
        log_warning "Commi does not support completion generation"
        return
    fi
    
    # Setup bash completion
    if [ -d "/etc/bash_completion.d" ] || [ -d "/usr/local/etc/bash_completion.d" ]; then
        local bash_dir="/usr/local/etc/bash_completion.d"
        [ -d "/etc/bash_completion.d" ] && bash_dir="/etc/bash_completion.d"
        
        if "$INSTALL_DIR/$BINARY_NAME" completion bash >/dev/null 2>&1; then
            sudo "$INSTALL_DIR/$BINARY_NAME" completion bash > "$bash_dir/$BINARY_NAME" 2>/dev/null && \
                log_success "Bash completion installed" || \
                log_warning "Failed to install bash completion"
        fi
    fi
    
    # Setup zsh completion
    if [ -d "/usr/local/share/zsh/site-functions" ] || [ -d "/usr/share/zsh/site-functions" ]; then
        local zsh_dir="/usr/local/share/zsh/site-functions"
        [ -d "/usr/share/zsh/site-functions" ] && zsh_dir="/usr/share/zsh/site-functions"
        
        if "$INSTALL_DIR/$BINARY_NAME" completion zsh >/dev/null 2>&1; then
            sudo "$INSTALL_DIR/$BINARY_NAME" completion zsh > "$zsh_dir/_$BINARY_NAME" 2>/dev/null && \
                log_success "Zsh completion installed" || \
                log_warning "Failed to install zsh completion"
        fi
    fi
    
    # Setup fish completion
    if [ -d "/usr/local/share/fish/vendor_completions.d" ] || [ -d "/usr/share/fish/vendor_completions.d" ]; then
        local fish_dir="/usr/local/share/fish/vendor_completions.d"
        [ -d "/usr/share/fish/vendor_completions.d" ] && fish_dir="/usr/share/fish/vendor_completions.d"
        
        if "$INSTALL_DIR/$BINARY_NAME" completion fish >/dev/null 2>&1; then
            sudo "$INSTALL_DIR/$BINARY_NAME" completion fish > "$fish_dir/$BINARY_NAME.fish" 2>/dev/null && \
                log_success "Fish completion installed" || \
                log_warning "Failed to install fish completion"
        fi
    fi
}

# Verify installation
verify_installation() {
    log_info "Verifying installation..."
    
    if ! command_exists "$BINARY_NAME"; then
        error_exit "Installation failed: $BINARY_NAME not found in PATH"
    fi
    
    # Test basic functionality
    if ! "$BINARY_NAME" --version >/dev/null 2>&1 && ! "$BINARY_NAME" --help >/dev/null 2>&1; then
        error_exit "Installation failed: $BINARY_NAME is not working properly"
    fi
    
    log_success "Installation verified successfully"
    
    # Show version
    log_info "Running commi to verify installation:"
    "$BINARY_NAME" --version 2>/dev/null || "$BINARY_NAME" --help | head -3
}

# Show usage information
show_usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Install commi from the latest GitHub release.

OPTIONS:
    -h, --help     Show this help message
    -v, --verbose  Enable verbose output
    --no-sudo      Skip operations that require sudo (manual and completions)
    --install-dir DIR  Install binary to DIR (default: /usr/local/bin)

EXAMPLES:
    $0                    # Install with default settings
    $0 --no-sudo          # Install without sudo operations
    $0 --install-dir ~/.local/bin  # Install to user directory

EOF
}

# Parse command line arguments
parse_args() {
    while [ $# -gt 0 ]; do
        case $1 in
            -h|--help)
                show_usage
                exit 0
                ;;
            -v|--verbose)
                set -x
                shift
                ;;
            --no-sudo)
                NO_SUDO=1
                shift
                ;;
            --install-dir)
                if [ -n "$2" ]; then
                    INSTALL_DIR="$2"
                    shift 2
                else
                    error_exit "--install-dir requires a directory argument"
                fi
                ;;
            *)
                error_exit "Unknown option: $1. Use --help for usage information."
                ;;
        esac
    done
}

# Main installation function
main() {
    parse_args "$@"

    log_info "Starting commi installation..."
    log_info "Platform: $(detect_platform)"
    log_info "Install directory: $INSTALL_DIR"

    check_prerequisites

    local download_url
    download_url=$(get_latest_release)

    install_binary "$download_url"

    if [ -z "$NO_SUDO" ]; then
        install_manual
        setup_completions
    else
        log_info "Skipping manual and completion installation (--no-sudo specified)"
    fi

    verify_installation

    log_success "Commi installation completed successfully!"
    log_info "You can now use 'commi' from anywhere in your terminal"
    log_info "Run 'commi --help' to get started"

    if [ -n "$NO_SUDO" ]; then
        log_info "Note: Manual page and shell completions were not installed due to --no-sudo"
        log_info "You can install completions manually using: commi completion <shell>"
    fi
}

# Run main function
