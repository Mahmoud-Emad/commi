import os
import sys
import git
from decouple import config
from commi.cmd import CommiCommands
from commi.commit_message import CommitMessageGenerator
from commi.logs import print_ultron_header, LOGGER
import pyperclip

# Validate if the given path is a valid Git repository
def validate_repo_path(path):
    """Validates if the given path is a valid Git repository."""
    try:
        repo = git.Repo(path)
        return repo.git_dir is not None
    except git.exc.InvalidGitRepositoryError:
        return False

# Commit changes to the repository
def commit_changes(repo, commit_message):
    """Commits the generated commit message to the repository."""
    try:
        if not repo.is_dirty(untracked_files=True):
            LOGGER.warning("No changes to commit.")
            return

        # Stage all changes and commit
        repo.git.add(A=True)
        repo.git.commit('-m', commit_message)
        LOGGER.info(f"Changes committed successfully.")
    except git.exc.GitCommandError as e:
        LOGGER.error(f"Failed to commit changes: {e}")
        sys.exit(1)

# Load configuration from environment or command-line arguments
def load_configuration(args):
    """Load configuration values."""
    API_KEY = config("API_KEY")
    if args.api_key:
        API_KEY = args.api_key

    if not API_KEY:
        LOGGER.error("API_KEY is not set. Please set it in the environment or provide it as an argument.")
        sys.exit(1)

    MODEL_NAME = config("MODEL_NAME", default="gemini-1.5-flash")
    return API_KEY, MODEL_NAME

# Set up repository path and validate it
def setup_repo_path(args):
    """Determine and validate the repository path."""
    repo_path = args.repo if args.repo else os.getcwd()
    if not args.repo:
        LOGGER.warning("No repository path provided. Using current directory.")
    
    if not validate_repo_path(repo_path):
        LOGGER.error(f"The directory '{repo_path}' is not a valid Git repository.")
        LOGGER.error("You can either run it from a valid repository path or use the --repo option.")
        sys.exit(1)

    return repo_path

# Generate and process the commit message
def generate_commit_message(generator, args):
    """Generate commit message based on the git diff."""
    try:
        LOGGER.info("Fetching git diff...")
        diff_text = generator.get_diff(cached=args.cached)

        if not diff_text:
            LOGGER.error("Cannot generate commit message. No changes found in the git diff.")
            sys.exit(1)

        LOGGER.info("Generating commit message...")
        commit_message = generator.generate_commit_message(diff_text)
        LOGGER.info(f"Generated Commit Message: \n{commit_message}")

        return commit_message
    except Exception as e:
        LOGGER.critical(f"An error occurred while generating commit message: {str(e)}")
        sys.exit(1)

# Handle commit process based on flags
def handle_commit_process(args, repo_path, commit_message):
    """Handle the commit process based on the --commit flag."""
    if args.commit:
        LOGGER.info("Committing changes to the repository...")
        repo = git.Repo(repo_path)
        commit_changes(repo, commit_message)

# Handle clipboard copy process
def handle_copy_process(args, commit_message):
    """Handle the clipboard copy process based on the --copy flag."""
    if args.copy:
        LOGGER.info("Copying commit message to clipboard...")
        pyperclip.copy(commit_message)
        LOGGER.info("Commit message copied to clipboard.")

# Main entry point
def main():
    """Main entry point for the program."""
    print_ultron_header()
    
    commi_commands = CommiCommands()
    args = commi_commands.get_args()

    # Load configuration settings
    API_KEY, MODEL_NAME = load_configuration(args)

    # Setup and validate repo path
    repo_path = setup_repo_path(args)

    # Initialize the CommitMessageGenerator
    generator = CommitMessageGenerator(repo_path, API_KEY, MODEL_NAME)

    try:
        # Generate the commit message
        commit_message = generate_commit_message(generator, args)

        # Handle commit operation if --commit flag is provided
        handle_commit_process(args, repo_path, commit_message)

        # Handle copy operation if --copy flag is provided
        handle_copy_process(args, commit_message)

        # Provide feedback if neither --copy nor --commit is provided
        if not args.copy and not args.commit:
            LOGGER.info("Commit message can be copied to clipboard by using --copy flag.")

    except Exception as e:
        LOGGER.critical(f"An error occurred: {str(e)}")
        sys.exit(1)

if __name__ == "__main__":
    main()
