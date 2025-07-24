#!/bin/bash

# Shell completion installation script for Commi
# This script automatically detects your shell and installs the appropriate completion

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
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

print_header() {
    echo -e "${CYAN}"
    echo "   ______                          _   "
    echo "  / ____/___  ____ ___  ____ ___  (_)  "
    echo " / /   / __ \/ __ \`__ \/ __ \`__ \/ / "
    echo "/ /___/ /_/ / / / / / / / / / / / /    "
    echo "\____/\____/_/ /_/ /_/_/ /_/ /_/_/     "
    echo -e "${RESET}"
    echo -e "${GREEN}Shell Completion Installation${RESET}"
    echo
}

# Check if commi is available
check_commi() {
    if ! command -v commi >/dev/null 2>&1; then
        print_error "commi command not found in PATH"
        print_status "Please install commi first or ensure it's in your PATH"
        exit 1
    fi
    
    print_status "Found commi: $(which commi)"
}

# Detect shell
detect_shell() {
    if [ -n "$ZSH_VERSION" ]; then
        echo "zsh"
    elif [ -n "$BASH_VERSION" ]; then
        echo "bash"
    elif [ -n "$FISH_VERSION" ]; then
        echo "fish"
    else
        # Fallback to checking SHELL environment variable
        case "$(basename "$SHELL")" in
            zsh) echo "zsh" ;;
            bash) echo "bash" ;;
            fish) echo "fish" ;;
            *) echo "unknown" ;;
        esac
    fi
}

# Install bash completion
install_bash_completion() {
    print_status "Installing bash completion..."
    
    # Create completion directory
    local completion_dir="$HOME/.local/share/bash-completion/completions"
    mkdir -p "$completion_dir"
    
    # Generate completion script
    commi completion bash > "$completion_dir/commi"
    
    # Check if sourcing is set up in .bashrc
    local bashrc="$HOME/.bashrc"
    local source_line='for f in ~/.local/share/bash-completion/completions/*; do [ -r "$f" ] && source "$f"; done'
    
    if [ -f "$bashrc" ] && ! grep -q "bash-completion/completions" "$bashrc"; then
        print_status "Adding completion sourcing to .bashrc"
        echo "" >> "$bashrc"
        echo "# Load bash completions" >> "$bashrc"
        echo "$source_line" >> "$bashrc"
    fi
    
    print_success "Bash completion installed to $completion_dir/commi"
    print_status "Reload your shell with: source ~/.bashrc"
}

# Install zsh completion
install_zsh_completion() {
    print_status "Installing zsh completion..."
    
    # Create completion directory
    local completion_dir="$HOME/.local/share/zsh/site-functions"
    mkdir -p "$completion_dir"
    
    # Generate completion script
    commi completion zsh > "$completion_dir/_commi"
    
    # Check if fpath is set up in .zshrc
    local zshrc="$HOME/.zshrc"
    local fpath_line='fpath=(~/.local/share/zsh/site-functions $fpath)'
    local compinit_line='autoload -U compinit && compinit'
    
    if [ -f "$zshrc" ]; then
        if ! grep -q "~/.local/share/zsh/site-functions" "$zshrc"; then
            print_status "Adding completion directory to fpath in .zshrc"
            echo "" >> "$zshrc"
            echo "# Add custom completion directory to fpath" >> "$zshrc"
            echo "$fpath_line" >> "$zshrc"
        fi
        
        if ! grep -q "autoload -U compinit" "$zshrc"; then
            print_status "Adding compinit to .zshrc"
            echo "# Initialize completion system" >> "$zshrc"
            echo "$compinit_line" >> "$zshrc"
        fi
    else
        print_status "Creating .zshrc with completion setup"
        echo "$fpath_line" > "$zshrc"
        echo "$compinit_line" >> "$zshrc"
    fi
    
    print_success "Zsh completion installed to $completion_dir/_commi"
    print_status "Reload your shell with: exec zsh"
}

# Install fish completion
install_fish_completion() {
    print_status "Installing fish completion..."
    
    # Create completion directory
    local completion_dir="$HOME/.config/fish/completions"
    mkdir -p "$completion_dir"
    
    # Generate completion script
    commi completion fish > "$completion_dir/commi.fish"
    
    print_success "Fish completion installed to $completion_dir/commi.fish"
    print_status "Fish will automatically load the completion"
}

# Main installation function
install_completion() {
    local shell_type="$1"
    
    case "$shell_type" in
        bash)
            install_bash_completion
            ;;
        zsh)
            install_zsh_completion
            ;;
        fish)
            install_fish_completion
            ;;
        *)
            print_error "Unsupported shell: $shell_type"
            print_status "Supported shells: bash, zsh, fish"
            print_status "You can manually install completion using:"
            print_status "  commi completion bash > /path/to/completion/file"
            print_status "  commi completion zsh > /path/to/completion/file"
            print_status "  commi completion fish > /path/to/completion/file"
            exit 1
            ;;
    esac
}

# Test completion
test_completion() {
    print_status "Testing completion installation..."
    print_status "Try typing: commi <TAB><TAB>"
    print_status "You should see available subcommands: generate, config, completion, update, status"
}

# Main script
main() {
    print_header
    
    # Check if commi is available
    check_commi
    
    # Detect shell or use provided argument
    local shell_type="${1:-$(detect_shell)}"
    
    print_status "Detected shell: $shell_type"
    
    # Install completion for detected shell
    install_completion "$shell_type"
    
    echo
    print_success "Shell completion installation completed!"
    test_completion
    
    echo
    print_status "For more information, see: SHELL_COMPLETION.md"
}

# Run main function with all arguments
main "$@"
