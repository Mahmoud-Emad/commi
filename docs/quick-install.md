# Commi Installation Guide

> AI-powered Git commit message generator

---

## Quick Install (Recommended)

```bash
curl -sSf https://raw.githubusercontent.com/Mahmoud-Emad/commi/development_install/scripts/quick-install.sh | sh
```

> Or with wget:

```bash
wget -qO- https://raw.githubusercontent.com/Mahmoud-Emad/commi/development_install/scripts/quick-install.sh | sh
```

---

## Manual Install

1. **Download the binary** for your platform from [Releases](https://github.com/Mahmoud-Emad/commi/releases/latest).
2. **Install it**:

```bash
mv commi-* commi && chmod +x commi
sudo mv commi /usr/local/bin/
```

3. **(Optional)**: Shell completions:

```bash
sudo commi completion bash > /etc/bash_completion.d/commi
sudo commi completion zsh > /usr/local/share/zsh/site-functions/_commi
sudo commi completion fish > /usr/local/share/fish/vendor_completions.d/commi.fish
```

---

## What the Script Does

* Detects your platform & architecture
* Downloads the latest release
* Installs the binary to `/usr/local/bin/commi`
* Installs a manual page
* Sets up shell completions (Bash, Zsh, Fish)
* Verifies installation

---

## Requirements

* `curl` or `wget`
* POSIX-compliant shell
* `sudo` (optional; can be bypassed)

> Optional:
> `jq` – for better GitHub API parsing

---

## Troubleshooting

### “Permission denied”

Use a local install:

```bash
./install.sh --no-sudo --install-dir ~/.local/bin
```

Then add to your PATH:

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc && source ~/.bashrc
```

### “commi: command not found”

Make sure `/usr/local/bin` or your install dir is in your `$PATH`.

---

## Uninstall

```bash
sudo rm -f /usr/local/bin/commi \
    /usr/local/share/man/man1/commi.1 \
    /etc/bash_completion.d/commi \
    /usr/local/share/zsh/site-functions/_commi \
    /usr/local/share/fish/vendor_completions.d/commi.fish
```

---

## Help & Docs

* `commi --help`
* [GitHub Repo](https://github.com/Mahmoud-Emad/commi)
* [Report Issues](https://github.com/Mahmoud-Emad/commi/issues)
* `man commi` (if manpage was installed)
