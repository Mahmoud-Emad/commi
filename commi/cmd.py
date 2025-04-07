import os
import requests
import json
import argparse
import sys
from datetime import datetime, timedelta

class CommiCommands:
    VERSION_CACHE_FILE = os.path.expanduser("~/.commi_version")
    VERSION_CACHE_EXPIRY = timedelta(days=1)

    def __init__(self):
        version = self.get_version()
        self.parser = argparse.ArgumentParser(
            description=(
                "AI-powered Git commit message generator using Gemini AI.\n\n"
                "Generates commit messages following standard Git commit message format:\n"
                "- Short (72 chars or less) summary line in imperative mood\n"
                "- Blank line separating summary from body\n"
                "- Detailed explanatory text wrapped at 72 characters\n"
                "- Use bullet points for multiple changes"
                f"\nVersion: {version}"
            ),
            formatter_class=argparse.RawDescriptionHelpFormatter,
        )
        self.parser.add_argument("--version", action="version", version=version)
        self.parser.add_argument("--repo", help="Path to Git repository (defaults to current directory)")
        self.parser.add_argument("--api-key", help="Gemini AI API key (or set GEMINI_API_KEY env var)")
        self.parser.add_argument("--cached", action="store_true", help="Use staged changes only")
        self.parser.add_argument("--copy", action="store_true", help="Copy message to clipboard")
        self.parser.add_argument("--commit", action="store_true", help="Auto commit with generated message")
        self.parser.add_argument("--co-author", metavar="EMAIL", help="Add a co-author to the commit")

        self.args = self.parser.parse_args()

    def get_args(self):
        if len(sys.argv) == 1:
            self.parser.print_help()
            sys.exit(0)
        return self.args

    def get_version(self):
        # Check if version is cached and not expired
        if os.path.exists(self.VERSION_CACHE_FILE):
            with open(self.VERSION_CACHE_FILE, "r") as f:
                try:
                    cache = json.load(f)
                    cached_time = datetime.fromisoformat(cache.get("fetched_at"))
                    if datetime.now() - cached_time < self.VERSION_CACHE_EXPIRY:
                        return cache.get("version")
                except Exception:
                    pass  # In case of JSON error, fall back to fetching again

        # Fetch from GitHub Releases API
        try:
            resp = requests.get("https://api.github.com/repos/Mahmoud-Emad/commi/releases/latest", timeout=5)
            if resp.status_code == 200:
                version = resp.json()["tag_name"]
                # Cache it
                with open(self.VERSION_CACHE_FILE, "w") as f:
                    json.dump({"version": version, "fetched_at": datetime.now().isoformat()}, f)
                return version
        except Exception as e:
            print("Warning: Failed to fetch latest version:", e)

        return "0.0.0"  # Default if fetch fails
