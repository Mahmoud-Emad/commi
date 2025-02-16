import argparse
import sys

class CommiCommands:
    def __init__(self):
        self.parser = argparse.ArgumentParser(
            description=(
                "AI-powered Git commit message generator using Gemini AI.\n\n"
                "Generates commit messages following standard Git commit message format:\n"
                "- Short (72 chars or less) summary line in imperative mood\n"
                "- Blank line separating summary from body\n"
                "- Detailed explanatory text wrapped at 72 characters\n"
                "- Use bullet points for multiple changes"
            ),
            formatter_class=argparse.RawDescriptionHelpFormatter
        )
        
        self.parser.add_argument(
            "--repo", 
            help="Path to Git repository (defaults to current directory)"
        )
        self.parser.add_argument(
            "--api-key", 
            help="Gemini AI API key (can also be set via GEMINI_API_KEY environment variable)"
        )
        self.parser.add_argument(
            "--cached",
            action="store_true",
            help="Generate message from staged changes only (git diff --cached)"
        )
        self.parser.add_argument(
            "--copy",
            action="store_true",
            help="Copy the generated message to clipboard"
        )
        self.parser.add_argument(
            "--commit",
            action="store_true",
            help="Automatically commit the changes with the generated message"
        )
        self.parser.add_argument(
            "--co-author",
            type=str,
            metavar="EMAIL",
            help="Add a co-author to the commit (format: email@example.com)"
        )

        # Parse the arguments
        self.args = self.parser.parse_args()

    def get_args(self):
        """Parse and return command line arguments."""
        args = self.parser.parse_args()
        
        # Print help if no arguments provided
        if len(sys.argv) == 1:
            self.parser.print_help()
            sys.exit(0)
            
        return args
