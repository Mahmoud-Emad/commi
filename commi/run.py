import os
import sys
import git
from decouple import config
from commi.cmd import CommiCommands
from commi.commit_message import CommitMessageGenerator
from commi.logs import print_ultron_header, LOGGER
import pyperclip

# Validate if the given path is a valid Git repository
class CommiError(Exception):
    """Base exception class for Commi errors."""
    pass

def validate_repo_path(path):
    """Validates if the given path is a valid Git repository."""
    try:
        repo = git.Repo(path)
        return repo.git_dir is not None
    except git.exc.InvalidGitRepositoryError:
        return False

def has_changes(repo, cached=False):
    """Check if there are changes to commit."""
    if cached:
        # Check for staged changes
        diff = repo.git.diff('--cached')
        return bool(diff.strip())
    else:
        # Check for any changes (staged or unstaged)
        return repo.is_dirty(untracked_files=True)

def commit_changes(repo, commit_message):
    """Commits the generated commit message to the repository."""
    try:
        if not has_changes(repo, cached=True):
            raise CommiError("No staged changes to commit. Use 'git add' to stage changes.")

        repo.git.commit('-m', commit_message)
        LOGGER.info("Changes committed successfully.")
    except git.exc.GitCommandError as e:
        raise CommiError(f"Failed to commit changes: {e}")

def validate_model_name(model_name):
    """Validate the model name."""
    valid_models = {'gemini-1.0-pro', 'gemini-1.5-pro', 'gemini-1.5-flash'}
    if model_name not in valid_models:
        LOGGER.warning(f"Unrecognized model: {model_name}. Using default model gemini-1.5-flash")
        return "gemini-1.5-flash"
    return model_name

def load_configuration(args):
    """Load and validate configuration values."""
    # Handle API key
    COMMI_API_KEY = args.api_key or config("COMMI_API_KEY", default=None)
    if not COMMI_API_KEY:
        raise CommiError("COMMI_API_KEY is not set. Please set it in the environment or provide it as an argument.")

    # Handle and validate model name
    MODEL_NAME = validate_model_name(config("MODEL_NAME", default="gemini-1.5-flash"))
    
    return COMMI_API_KEY, MODEL_NAME

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
    LOGGER.info("Fetching git diff...")
    diff_text = generator.get_diff(cached=args.cached)

    if not diff_text:
        raise CommiError("No changes found in the git diff. Make sure you have changes to commit.")

    LOGGER.info("Generating commit message...")
    commit_message = generator.generate_commit_message(diff_text)

    if args.co_author:
        if '@' not in args.co_author:
            raise CommiError("Invalid co-author email. Please provide a valid email address.")

        author_name = args.co_author.split('@')[0]
        commit_message += f"\n\nCo-authored-by: {author_name} <{args.co_author}>"

    LOGGER.info(f"Generated Commit Message:\n{commit_message}")
    return commit_message

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
    
    try:
        commi_commands = CommiCommands()
        args = commi_commands.get_args()

        # Load and validate configuration
        COMMI_API_KEY, MODEL_NAME = load_configuration(args)
        repo_path = setup_repo_path(args)

        # Initialize generator
        generator = CommitMessageGenerator(repo_path, COMMI_API_KEY, MODEL_NAME)

        # Generate commit message
        commit_message = generate_commit_message(generator, args)

        # Handle commit operation
        if args.commit:
            repo = git.Repo(repo_path)
            if not has_changes(repo, args.cached):
                LOGGER.warning("No changes to commit. Stage changes with 'git add' first.")
            else:
                commit_changes(repo, commit_message)

        # Handle copy operation
        if args.copy:
            pyperclip.copy(commit_message)
            LOGGER.info("Commit message copied to clipboard.")
        elif not args.commit:
            LOGGER.info("Use --copy to copy the message or --commit to commit changes.")

    except CommiError as e:
        LOGGER.error(str(e))
        sys.exit(1)
    except Exception as e:
        LOGGER.critical(f"An unexpected error occurred: {str(e)}")
        sys.exit(1)

if __name__ == "__main__":
    main()
