import os
import requests
import json
import argparse
import sys
import subprocess
from datetime import datetime, timedelta


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
            "-v", "--version", action="version", version=self.installed_version,
            help="Show version number and exit"
        )
        self.parser.add_argument(
            "-r", "--repo", help="Path to Git repository (defaults to current directory)"
        )
        self.parser.add_argument(
            "-k", "--api-key", help="Gemini AI API key (or set GEMINI_API_KEY env var)"
        )
        self.parser.add_argument(
            "-c","--cached", action="store_true", help="Use staged changes only"
        )
        self.parser.add_argument(
            "-t","--copy", action="store_true", help="Copy message to clipboard"
        )
        self.parser.add_argument(
            "-m", "--commit", action="store_true", help="Auto commit with generated message"
        )
        self.parser.add_argument(
            "-a", "--co-author", metavar="EMAIL", help="Add a co-author to the commit"
        )
        self.parser.add_argument(
            "-u", "--update", action="store_true", help="Update Commi to the latest version"
        )

        self.args = self.parser.parse_args()

    def get_args(self):
        if len(sys.argv) == 1:
            self.parser.print_help()
            sys.exit(0)
        return self.args

    def get_installed_version(self):
        """Get the installed version from pyproject.toml"""
        try:
            import toml
            with open("pyproject.toml", "r") as f:
                pyproject = toml.load(f)
                return pyproject["tool"]["poetry"]["version"]
        except Exception as e:
            raise Exception(f"Failed to get installed version: {e}")

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
            print(
                f"Updating Commi from {self.installed_version} to {self.latest_version}..."
            )

            # Download the latest release binary
            release_url = f"https://github.com/Mahmoud-Emad/commi/releases/download/{self.latest_version}/commi"
            temp_binary = "/tmp/commi_new"

            # Download the binary
            subprocess.run(["curl", "-L", release_url, "-o", temp_binary], check=True)

            # Make it executable
            subprocess.run(["chmod", "+x", temp_binary], check=True)

            # Get the current binary path
            current_binary = (
                subprocess.check_output(["which", "commi"]).decode().strip()
            )

            # Replace the current binary with the new one
            subprocess.run(["sudo", "mv", temp_binary, current_binary], check=True)

            print(f"Successfully updated Commi to version {self.latest_version}!")
            return True
        except Exception as e:
            print(f"Error updating Commi: {e}")
            return False
