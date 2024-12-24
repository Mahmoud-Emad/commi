import subprocess

def install_clipboard_tool():
    """Installs necessary clipboard tools on Linux if they are not available."""
    from commi.logs import LOGGER
    try:
        LOGGER.info("Installing clipboard tool...")
        subprocess.check_call(["sudo", "apt-get", "install", "-y", "xclip"])
        LOGGER.info("xclip installed successfully.")
    except subprocess.CalledProcessError:
        LOGGER.error("Failed to install clipboard tool. Please install it manually.")
