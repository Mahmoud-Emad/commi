import os
import requests
import argparse
import sys
import subprocess
from datetime import timedelta
from commi.logs import LOGGER


class CommiCommands:
    VERSION_CACHE_FILE = os.path.expanduser("~/.commi_version")
    VERSION_CACHE_EXPIRY = timedelta(days=1)

    def __init__(self):
        self.installed_version = self.get_installed_version()
        self.latest_version = self.get_latest_version()

        self.parser = argparse.ArgumentParser(
            description=(
                "AI-powered Git commit message generator using Gemini AI.\n\n"
                "Generates commit messages following standard Git commit message format:\n"
                "- Short (72 chars or less) summary line in imperative mood\n"
                "- Blank line separating summary from body\n"
                "- Detailed explanatory text wrapped at 72 characters\n"
                "- Use bullet points for multiple changes"
                f"\nVersion: {self.installed_version}"
            ),
            formatter_class=argparse.RawDescriptionHelpFormatter,
        )
        self.parser.add_argument(
            "--version",
            "-v",
            action="version",
            version=self.installed_version,
            help="Show version number and exit",
        )
        self.parser.add_argument(
            "--repo",
            "-r",
            help="Path to Git repository (defaults to current directory)",
        )
        self.parser.add_argument(
            "--api-key", "-k", help="Gemini AI API key (or set GEMINI_API_KEY env var)"
        )
        self.parser.add_argument(
            "--cached", "-c", action="store_true", help="Use staged changes only"
        )
        self.parser.add_argument(
            "-t", "--copy", action="store_true", help="Copy message to clipboard"
        )
        self.parser.add_argument(
            "-m",
            "--commit",
            action="store_true",
            help="Auto commit with generated message",
        )
        self.parser.add_argument(
            "--co-author", "-a", metavar="EMAIL", help="Add a co-author to the commit"
        )
        self.parser.add_argument(
            "--update",
            "-u",
            action="store_true",
            help="Update Commi to the latest version",
        )

        self.args = self.parser.parse_args()

    def get_args(self):
        if len(sys.argv) == 1:
            self.parser.print_help()
            sys.exit(0)
        return self.args

    def get_installed_version(self):
        """Get the installed version from pyproject.toml or package metadata"""
        try:
            import toml
            import os
            import importlib.metadata

            # First try to get version from package metadata (works for installed packages)
            try:
                return importlib.metadata.version("commi")
            except importlib.metadata.PackageNotFoundError:
                pass

            # If that fails, try to find pyproject.toml relative to the module
            module_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
            pyproject_path = os.path.join(module_dir, "pyproject.toml")

            if os.path.exists(pyproject_path):
                with open(pyproject_path, "r") as f:
                    pyproject = toml.load(f)
                    return pyproject["tool"]["poetry"]["version"]

            return None
        except Exception as e:
            import sys

            LOGGER.warning(f"Failed to get installed version: {e}", file=sys.stderr)
            return None

    def get_latest_version(self):
        """Get the latest version from GitHub releases"""
        # Fetch from GitHub Releases API
        try:
            resp = requests.get(
                "https://api.github.com/repos/Mahmoud-Emad/commi/releases/latest",
                timeout=5,
            )
            if resp.status_code == 200:
                latest_version = resp.json()["tag_name"]
                return latest_version
        except Exception as e:
            raise Exception(f"Failed to fetch latest version: {e}")

    def is_update_available(self):
        """Check if an update is available"""
        try:
            from packaging import version

            return version.parse(self.latest_version) > version.parse(
                self.installed_version
            )
        except Exception:
            return False

    def update_binary(self):
        """Update the binary to the latest version"""
        try:
            LOGGER.info(
                f"Updating Commi from {self.installed_version} to {self.latest_version}"
            )

            # We need to check the user platform first
            import platform

            publishing_name = "commi-"
            # Check if the platform isn't linux, check if macos
            if platform.system().lower() == "darwin":
                publishing_name = "commi-macos"
            elif platform.system().lower() == "linux":
                publishing_name = "commi-linux"
            else:
                LOGGER.error("Unsupported platform")
                return False

            # Download the latest release binary
            releases_page = "https://github.com/Mahmoud-Emad/commi/releases"
            release_url = (
                f"{releases_page}/download/{self.latest_version}/{publishing_name}"
            )

            # Now we need to make sure first that the release is a valid binary

            LOGGER.info("Validating the latest release binary")
            response = requests.head(release_url, timeout=5)
            if response.status_code == 404:
                LOGGER.error("Failed to download the latest release binary.")
                LOGGER.error(f"The binary not found at {release_url}.")
                LOGGER.error(f"You could check the release page at {releases_page}.")
                return False

            # Create a temporary binary
            temp_binary = "/tmp/commi"

            # Check if curl is installed
            LOGGER.info("Checking if curl is installed")
            if (
                subprocess.run(
                    ["which", "curl"], check=False, stdout=subprocess.DEVNULL
                ).returncode
                != 0
            ):
                LOGGER.error("curl is not installed. Please install it and try again.")
                return False

            # Download the binary, and rename it to binname
            LOGGER.info("Downloading the latest release binary please be patient...")
            subprocess.run(
                ["curl", "-L", release_url, "-o", temp_binary],
                check=True,
                stdout=subprocess.PIPE,
            )

            # Make it executable
            LOGGER.info("Making the binary executable")
            subprocess.run(
                ["chmod", "+x", temp_binary],
                check=True,
                stdout=subprocess.DEVNULL,
            )

            # Get the current binary path
            current_binary = (
                subprocess.check_output(["which", "commi"]).decode().strip()
            )

            # Replace the current binary with the new one
            LOGGER.info("Replacing the current binary with the new one")
            subprocess.run(["sudo", "mv", temp_binary, current_binary], check=True)

            LOGGER.info(f"Successfully updated Commi to version {self.latest_version}!")
            return True
        except Exception as e:
            print(f"Error updating Commi: {e}")
            return False
