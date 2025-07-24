#!/bin/sh

# Commi Installation Script
# POSIX-compliant shell script to install commi from GitHub releases
#
# Usage:
#   curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/Mahmoud-Emad/commi/main/scripts/install.sh | sh

set -e  # Exit on any error

# Configuration
REPO="Mahmoud-Emad/commi"
BINARY_NAME="commi"
INSTALL_DIR="/usr/local/bin"
MAN_DIR="/usr/local/share/man/man1"
GITHUB_API="https://api.github.com/repos/$REPO/releases/latest"

# Logging functions
log_info() { printf "[INFO] %s\n" "$1"; }
log_success() { printf "[SUCCESS] %s\n" "$1"; }
log_warning() { printf "[WARNING] %s\n" "$1"; }
log_error() { printf "[ERROR] %s\n" "$1" >&2; }
error_exit() { log_error "$1"; exit 1; }
command_exists() { command -v "$1" >/dev/null 2>&1; }
is_piped() { [ ! -t 0 ]; }

# Detect platform and architecture
detect_platform() {
    case "$(uname -s)" in
        Darwin*) os="apple-darwin" ;;
        Linux*) os="unknown-linux-gnu" ;;
        *) error_exit "Unsupported OS: $(uname -s)" ;;
    esac

    case "$(uname -m)" in
        x86_64|amd64) arch="x86_64" ;;
        arm64|aarch64) arch="aarch64" ;;
        *) error_exit "Unsupported arch: $(uname -m)" ;;
    esac

    echo "$arch-$os"
}

# Get latest release download URL
get_latest_release_url() {
    platform=$(detect_platform)
    tmpfile=$(mktemp)

    if command_exists curl; then
        curl -sSf "$GITHUB_API" > "$tmpfile" || error_exit "Failed to fetch release info"
    elif command_exists wget; then
        wget -q -O "$tmpfile" "$GITHUB_API" || error_exit "Failed to fetch release info"
    else
        error_exit "curl or wget required"
    fi

    if command_exists jq; then
        url=$(jq -r ".assets[] | select(.name == \"$BINARY_NAME-$platform\") | .browser_download_url" "$tmpfile")
    else
        url=$(grep "name.*$platform" "$tmpfile" -A 10 | grep browser_download_url | head -1 | sed 's/.*: \"\(.*\)\".*/\1/')
    fi
    rm -f "$tmpfile"
    [ -z "$url" ] && error_exit "No binary for $platform"
    echo "$url"
}

# Install binary
install_binary() {
    url="$1"
    bin_tmp=$(mktemp)
    curl -sSfL "$url" -o "$bin_tmp" || error_exit "Download failed"
    [ ! -s "$bin_tmp" ] && error_exit "Binary is empty"

    sudo mkdir -p "$INSTALL_DIR"
    sudo cp "$bin_tmp" "$INSTALL_DIR/$BINARY_NAME"
    sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"
    rm -f "$bin_tmp"
    log_success "Installed commi to $INSTALL_DIR"
}

# Install man page (basic from --help)
install_manual() {
    [ ! -x "$INSTALL_DIR/$BINARY_NAME" ] && return
    mkdir -p "$MAN_DIR"
    {
        echo ".TH COMMI 1 \"$(date +'%B %Y')\" \"commi\" \"User Commands\""
        echo ".SH NAME"
        echo "commi - AI-powered Git commit message generator"
        echo ".SH SYNOPSIS"
        echo "commi [OPTIONS]"
        echo ".SH DESCRIPTION"
        "$INSTALL_DIR/$BINARY_NAME" --help | sed 's/^/.TP\n/'
    } | sudo tee "$MAN_DIR/$BINARY_NAME.1" >/dev/null || true
    log_success "Manual page installed"
}

# Install shell completions
install_completions() {
    for sh in bash zsh fish; do
        dir=""
        case "$sh" in
            bash) dir="/etc/bash_completion.d" ;;
            zsh) dir="/usr/share/zsh/site-functions" ;;
            fish) dir="/usr/share/fish/vendor_completions.d" ;;
        esac
        if [ -d "$dir" ]; then
            sudo "$INSTALL_DIR/$BINARY_NAME" completion "$sh" > "$dir/${BINARY_NAME}${sh:+.}${sh}" 2>/dev/null || true
            log_success "$sh completion installed"
        fi
    done
}

# Verify installation
verify() {
    "$BINARY_NAME" --version >/dev/null 2>&1 || "$BINARY_NAME" --help >/dev/null 2>&1 || error_exit "Verification failed"
    log_success "Commi is working!"
}

# Run installer
log_info "Installing commi..."
check_prerequisites() { command_exists curl || command_exists wget || error_exit "curl or wget is required"; }
check_prerequisites
url=$(get_latest_release_url)
install_binary "$url"
install_manual
install_completions
verify
log_info "Done! Run 'commi --help' to get started."
