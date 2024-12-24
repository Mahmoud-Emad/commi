import argparse
from commi.logs import LOGGER

class CommiCommands:
    def __init__(self):
        # Initialize argument parser
        self.parser = argparse.ArgumentParser(
            description="A CLI tool to generate Git commit messages using Gemini AI."
        )

        # Subparsers for subcommands
        self.subparsers = self.parser.add_subparsers(dest='command')

        # Define 'commit' subcommand
        # self.commit_parser = self.subparsers.add_parser(
        #     'commit', 
        #     help="Generate or manage commit messages"
        # )
        
        # Add arguments under 'commit' subcommand
        self.parser.add_argument(
            "--repo", 
            help="The repository path to process (optional, defaults to current directory)."
        )
        self.parser.add_argument(
            "--api-key", 
            required=False, 
            help="The API key for accessing Gemini AI (optional)."
        )
        self.parser.add_argument(
            "--cached",
            action="store_true",
            help="Generate commit message from staged changes (using git diff --cached)."
        )
        self.parser.add_argument(
            "--copy",
            action="store_true",
            help="Copy the generated commit message to the clipboard."
        )
        self.parser.add_argument(
            "--generate", 
            action="store_true", 
            help="Generate a commit message based on the repo diff."
        )
        self.parser.add_argument(
            "--regenerate", 
            action="store_true",
            help="Regenerate the commit message based on the repo diff."
        )
        self.parser.add_argument(
            "--commit",
            action="store_true",
            help="Commit the generated commit message."
        )
        # self.parser.add_argument(
        #     "--suggest",
        #     action="store_true",
        #     help="Suggest a commit message based on the repository changes."
        # )

        # Parse the arguments
        self.args = self.parser.parse_args()

    def _validate_arguments(self):
        """Validate mutually exclusive arguments for commit generation."""
        # Check if at least one of --generate, or --regenerate is provided
        # if not (self.args.generate or self.args.regenerate):
        #     LOGGER.critical("You must specify either --generate, or --regenerate.")
        #     self.print_usage()
        #     exit(1)

        # Ensure that --generate and --regenerate are not used together
        if self.args.generate and self.args.regenerate:
            LOGGER.critical("You cannot use both --generate and --regenerate at the same time.")
            self.print_usage()
            exit(1)

    def get_args(self):
        """Returns the parsed arguments."""
        # Validate mutually exclusive arguments under the 'commit' subcommand
        if self.args:
            self._validate_arguments()

        return self.args

    def print_usage(self):
        """Prints out the usage message."""
        self.parser.print_help()
