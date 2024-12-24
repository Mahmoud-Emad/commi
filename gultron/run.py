from gultron.cmd import GultronCommands
from gultron.commit_message import CommitMessageGenerator
from gultron.logs import print_commit_message_in_box, print_ultron_header, LOGGER
import os
import sys
import git
from decouple import config

def validate_repo_path(path):
    """Validates if the given path is a valid Git repository."""
    try:
        repo = git.Repo(path)
        return repo.git_dir is not None
    except git.exc.InvalidGitRepositoryError:
        return False

def main():
    """Main entry point for the program."""
    print_ultron_header()
    try:
        gultron_commands = GultronCommands()
        args = gultron_commands.get_args()

    except ValueError as e:
        print(f"Error: {e}")
        gultron_commands.print_usage()

    # Load configuration from .env or default values
    API_KEY = config("API_KEY", default="AIzaSyA6IkFVB8FRwhwy5ZZbVjgLuS-Ye8JMF_I")
    if args.api_key:
        API_KEY = args.api_key

    MODEL_NAME = config("MODEL_NAME", default="gemini-1.5-flash")

    # Determine repo path
    repo_path = args.repo if args.repo else os.getcwd()

    if not args.repo:
        LOGGER.warning("No repository path provided. Using current directory.")

    # Validate if the path is a valid Git repository
    if not validate_repo_path(repo_path):
        LOGGER.error(f"The directory '{repo_path}' is not a valid Git repository.")
        LOGGER.error("You can either run it from a valid repository path or use the --repo option.")
        sys.exit(1)

    # Initialize generator
    generator = CommitMessageGenerator(repo_path, API_KEY, MODEL_NAME)

    try:
        LOGGER.info("Fetching git diff...")
        diff_text = generator.get_diff(cached=args.cached)

        if not diff_text:
            LOGGER.error("Cannot generate commit message. No changes found in the git diff.")
            sys.exit(1)

        LOGGER.info("Generating commit message...")
        commit_message = generator.generate_commit_message(diff_text)

        print("\nGenerated Commit Message:")
        print_commit_message_in_box(commit_message, args)

    except Exception as e:
        LOGGER.critical(f"An error occurred: {str(e)}")
        sys.exit(1)

if __name__ == "__main__":
    main()
