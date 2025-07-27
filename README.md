# Commi: AI-powered Git Commit Message Generator

[![CI](https://github.com/Mahmoud-Emad/commi/workflows/CI/badge.svg)](https://github.com/Mahmoud-Emad/commi/actions)
[![Security](https://github.com/Mahmoud-Emad/commi/workflows/Security/badge.svg)](https://github.com/Mahmoud-Emad/commi/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)

**Fast, reliable, and dependency-free** - Commi is an AI-powered tool written in Rust that automatically generates Git commit messages based on your code changes using Google's Gemini AI.

## Key Features

- **AI-powered commit messages** - Uses Google's Gemini AI to analyze git diffs
- **100x faster startup** - Near-instantaneous execution (~5ms)
- **Single binary** - No runtime dependencies (3.8MB)
- **Modern CLI** - Hierarchical subcommands with shell completion
- **Cross-platform** - Works on Linux, macOS, and Windows
- **Safety confirmations** - Interactive prompts for dangerous operations

## Quick Start

1. **Get your API key** from [Google AI Studio](https://makersuite.google.com/app/apikey)
2. **Set your API key** (choose one method):

   ```bash
   # Method 1: Save to config file (recommended)
   commi config set api-key "your_api_key"

   # Method 2: Environment variable
   export COMMI_API_KEY="your_api_key"

   # Method 3: Command line flag
   commi generate --api-key "your_api_key"
   ```

3. **Generate commit messages**: `commi generate`

## Installation

```bash
# Download and install
wget https://github.com/Mahmoud-Emad/commi/releases/latest/download/commi-$(uname -s)-$(uname -m)
chmod +x commi-*
sudo mv commi-* /usr/local/bin/commi

# Or build from source
git clone https://github.com/Mahmoud-Emad/commi.git && cd commi && ./scripts/build.sh

# Install shell completion
./scripts/install-completions.sh
```

## Usage

```bash
# Basic usage
commi generate                          # Generate commit message
commi generate --copy                   # Copy to clipboard
commi generate --commit                 # Auto-commit with message

# Configuration
commi config set api-key "your_key"     # Set API key
commi config get                       # List all settings

# AI Models
commi model list                        # List supported models
commi config set model gemini-2.5-pro   # Change AI model

# Other commands
commi status                            # Show repository status
commi update check                      # Check for updates
commi completion bash                   # Generate shell completion
```

## Examples

```bash
# Generate commit message for staged changes
git add .
commi generate --cached --copy

# Generate and commit in one step
git add .
commi generate --commit

# Check what's changed
commi status --verbose
```

## Configuration

Commi stores configuration in `~/.config/commi.toml`:

```bash
# View all configuration
commi config get

# Set API key (required)
commi config set api-key "your_api_key"

# Set model (optional)
commi config set model "gemini-1.5-pro"

# View specific setting
commi config get api-key

# View cached version info
commi config get version
commi config get last-version-check
```

**Configuration Options:**

- `api-key` - Your Google AI Studio API key (required)
- `model` - AI model to use (default: "gemini-1.5-flash")
- `max-tokens` - Maximum tokens per request (default: 800000)
- `chunk-overlap` - Lines to overlap between chunks (default: 200)
- `enable-chunking` - Enable large diff chunking (default: true)
- `validate-format` - Enable AI-powered commit message validation (default: true)

## AI Models

Commi supports multiple Gemini AI models with different capabilities:

```bash
# List all supported models
commi model list

# Change to a different model
commi config set model gemini-2.5-pro
```

**Available Models:**

- `gemini-1.5-flash` - Fast and efficient (default)
- `gemini-2.5-flash` - Latest fast model
- `gemini-2.5-flash-lite` - Lightweight version
- `gemini-2.5-pro` - Most capable model

**Rate Limiting:**
If you encounter rate limit errors, try switching to a different model:

```bash
commi config set model gemini-2.5-pro
```

Different models may have different rate limits and quotas.

## Commit Message Validation

Commi includes AI-powered commit message validation to ensure your messages follow best practices:

```bash
# Enable validation (default)
commi config set validate-format true

# Disable validation
commi config set validate-format false
```

**Validation Rules:**

- **Subject line**: ≤ 50 characters, imperative mood, conventional commit format
- **Body**: Lines wrapped at 72 characters, explains why and what
- **Footer**: Issue references and breaking change descriptions

When enabled, Commi automatically validates and formats commit messages to follow:

- [Conventional Commits](https://www.conventionalcommits.org/) specification
- Git best practices for line length and formatting
- Proper imperative mood and clear descriptions

## Documentation

- **[Migration Guide](docs/MIGRATION.md)** - Upgrading from v3.x to v4.0
- **[Shell Completion Guide](docs/SHELL_COMPLETION.md)** - Setting up tab completion
- **[Testing Guide](docs/TESTING.md)** - Running and writing tests
- **[Deployment Guide](docs/DEPLOYMENT.md)** - Production deployment
- **[Changelog](docs/CHANGELOG.md)** - Version history and changes

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

MIT License - see [LICENSE](LICENSE) file for details.
