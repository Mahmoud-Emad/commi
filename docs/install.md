# Commi Installation Guide

This guide provides multiple ways to install commi on your system.

## Quick Install (Recommended)

### One-line Installation

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/Mahmoud-Emad/commi/main/scripts/install.sh | sh
```

### Alternative Methods

```bash
# With wget (less secure)
wget -qO- https://raw.githubusercontent.com/Mahmoud-Emad/commi/main/scripts/install.sh | sh
```

### Download and Run

```bash
# Download the script
curl -fsSL https://raw.githubusercontent.com/Mahmoud-Emad/commi/main/scripts/install.sh -o install.sh

# Make it executable
chmod +x install.sh

# Run the installer
./install.sh
```

## Installation Options

The install script provides a simple, one-command installation.

## What the Script Does

The installation script automatically:

1. **Detects your platform** (macOS/Linux) and architecture (x86_64/ARM64)
2. **Downloads the latest release** from GitHub for your platform
3. **Installs the binary** to `/usr/local/bin/commi` (or custom directory)
4. **Creates a manual page** at `/usr/local/share/man/man1/commi.1`
5. **Sets up shell completions** for Bash, Zsh, and Fish
6. **Verifies the installation** by running commi

## Requirements

### Required

- `curl` or `wget` for downloading
- POSIX-compliant shell (`sh`, `bash`, `zsh`, etc.)

### Optional

- `jq` for better JSON parsing (fallback available)
- `sudo` for system-wide installation (can be skipped with `--no-sudo`)

## Platform Support

The script supports:

- **macOS**: x86_64 (Intel) and ARM64 (Apple Silicon)
- **Linux**: x86_64 (AMD64) and ARM64 (AArch64)

## Manual Installation

If you prefer to install manually:

### 1. Download Binary

Visit the [releases page](https://github.com/Mahmoud-Emad/commi/releases/latest) and download the appropriate binary for your platform:

- `commi-x86_64-apple-darwin` (macOS Intel)
- `commi-aarch64-apple-darwin` (macOS Apple Silicon)
- `commi-x86_64-unknown-linux-gnu` (Linux x86_64)
- `commi-aarch64-unknown-linux-gnu` (Linux ARM64)

### 2. Install Binary

```bash
# Rename and move to PATH
mv commi-* commi
chmod +x commi
sudo mv commi /usr/local/bin/
```

### 3. Set Up Completions (Optional)

```bash
# Bash
sudo commi completion bash > /etc/bash_completion.d/commi

# Zsh
sudo commi completion zsh > /usr/local/share/zsh/site-functions/_commi

# Fish
sudo commi completion fish > /usr/local/share/fish/vendor_completions.d/commi.fish
```

## Troubleshooting

### Permission Denied

If you get permission errors:

```bash
# Use --no-sudo and install to user directory
./install.sh --no-sudo --install-dir ~/.local/bin

# Make sure ~/.local/bin is in your PATH
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Binary Not Found

If commi is not found after installation:

```bash
# Check if it's installed
ls -la /usr/local/bin/commi

# Check your PATH
echo $PATH

# Add /usr/local/bin to PATH if missing
echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Platform Not Supported

If you get "Unsupported platform" error:

1. Check the [releases page](https://github.com/Mahmoud-Emad/commi/releases/latest) for available binaries
2. Your platform might not be supported yet
3. Consider building from source or opening an issue

## Uninstallation

To remove commi:

```bash
# Remove binary
sudo rm -f /usr/local/bin/commi

# Remove manual page
sudo rm -f /usr/local/share/man/man1/commi.1

# Remove completions
sudo rm -f /etc/bash_completion.d/commi
sudo rm -f /usr/local/share/zsh/site-functions/_commi
sudo rm -f /usr/local/share/fish/vendor_completions.d/commi.fish
```

## Getting Help

- Run `commi --help` for usage information
- Visit the [GitHub repository](https://github.com/Mahmoud-Emad/commi) for documentation
- Check the [issues page](https://github.com/Mahmoud-Emad/commi/issues) for known problems
- Read the manual page: `man commi`
