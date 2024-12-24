import subprocess
import shutil
from commi.logs import LOGGER

def install_clipboard_tool():
    """Installs necessary clipboard tools on Linux if they are not available."""
    try:
        # Check if 'xclip' is already installed by using 'which'
        if shutil.which("xclip"):
            LOGGER.info("xclip is already installed.")
            return  # No need to install if it's already available

        LOGGER.info("xclip not found. Installing clipboard tool...")

        # Attempt to install 'xclip' via apt-get
        subprocess.check_call(["sudo", "apt-get", "install", "-y", "xclip"])
        LOGGER.info("xclip installed successfully.")
    
    except subprocess.CalledProcessError:
        LOGGER.error("Failed to install clipboard tool. Please install it manually.")
    except Exception as e:
        LOGGER.error(f"An error occurred while checking or installing xclip: {str(e)}")
