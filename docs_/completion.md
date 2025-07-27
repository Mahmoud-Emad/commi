# Shell Completion for Commi

Commi provides tab completion for bash, zsh, and fish shells. This makes it easier to use the CLI by providing suggestions for commands, options, and arguments.

## Quick Setup

### Automatic Installation (Recommended)

The easiest way to set up completion is using the installation script:

```bash
./scripts/install-completions.sh
```

This script will:

- Detect your shell automatically
- Install the completion script in the right location
- Update your shell configuration
- Show you how to test it

### Manual Installation

If you prefer to install manually, choose your shell below:

## Bash

1. **Generate the completion script:**

   ```bash
   commi completion bash > ~/.local/share/bash-completion/completions/commi
   ```

2. **Add to your `~/.bashrc`:**

   ```bash
   echo 'for f in ~/.local/share/bash-completion/completions/*; do [ -r "$f" ] && source "$f"; done' >> ~/.bashrc
   ```

3. **Reload your shell:**

   ```bash
   source ~/.bashrc
   ```

## Zsh

1. **Create completion directory:**

   ```bash
   mkdir -p ~/.local/share/zsh/site-functions
   ```

2. **Generate the completion script:**

   ```bash
   commi completion zsh > ~/.local/share/zsh/site-functions/_commi
   ```

3. **Add to your `~/.zshrc`:**

   ```bash
   echo 'fpath=(~/.local/share/zsh/site-functions $fpath)' >> ~/.zshrc
   echo 'autoload -U compinit && compinit' >> ~/.zshrc
   ```

4. **Reload your shell:**

   ```bash
   exec zsh
   ```

## Fish

1. **Create completion directory:**

   ```bash
   mkdir -p ~/.config/fish/completions
   ```

2. **Generate the completion script:**

   ```bash
   commi completion fish > ~/.config/fish/completions/commi.fish
   ```

3. **Fish will automatically load the completion** (no reload needed)

## Testing Completion

After installation, test that completion works:

```bash
commi <TAB><TAB>
```

You should see available commands:

- `generate` (or `gen`) - Generate commit messages
- `config` - Manage configuration
- `completion` - Generate completion scripts
- `update` - Check for updates
- `status` - Show repository status

Try more specific completions:

```bash
commi config <TAB><TAB>     # Shows: get, list, reset, set
commi completion <TAB><TAB>  # Shows: bash, fish, zsh
commi update <TAB><TAB>      # Shows: check, install
```

## System-wide Installation

If you want to install completion for all users (requires admin rights):

### Bash (system-wide)

```bash
sudo commi completion bash > /etc/bash_completion.d/commi
```

### Zsh (system-wide)

```bash
sudo commi completion zsh > /usr/local/share/zsh/site-functions/_commi
```

### Fish (system-wide)

```bash
sudo commi completion fish > /usr/share/fish/vendor_completions.d/commi.fish
```

## Troubleshooting

### Completion not working?

1. **Make sure commi is in your PATH:**

   ```bash
   which commi
   ```

2. **Check if completion file exists:**

   ```bash
   # For zsh
   ls -la ~/.local/share/zsh/site-functions/_commi
   
   # For bash
   ls -la ~/.local/share/bash-completion/completions/commi
   
   # For fish
   ls -la ~/.config/fish/completions/commi.fish
   ```

3. **Verify shell configuration:**

   ```bash
   # For zsh, check if these lines are in ~/.zshrc
   grep -n "fpath.*zsh/site-functions" ~/.zshrc
   grep -n "compinit" ~/.zshrc
   
   # For bash, check if completion loading is in ~/.bashrc
   grep -n "bash-completion" ~/.bashrc
   ```

4. **Try regenerating the completion:**

   ```bash
   # Re-run the installation script
   ./scripts/install-completions.sh
   ```

### Still having issues?

- Make sure you've reloaded your shell after installation
- Some terminals may need to be completely restarted
- Check that your shell supports completion (most modern shells do)

## What Gets Completed

Commi's completion provides suggestions for:

- **Commands**: `generate`, `config`, `completion`, `update`, `status`
- **Subcommands**: `config set`, `config get`, `update check`, etc.
- **Options**: `--help`, `--verbose`, `--no-color`, `--cached`, etc.
- **Configuration keys**: When using `commi config set <TAB>`
- **Shell types**: When using `commi completion <TAB>`

The completion is context-aware, so it only shows relevant options for each command.
