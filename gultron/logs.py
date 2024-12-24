import pyfiglet
import logging
import colorlog
import pyperclip

from gultron.utils import install_clipboard_tool

def print_ultron_header():
    """Prints the banner for the application."""
    ultron_art = pyfiglet.figlet_format("Gultron", font="slant")
    print(ultron_art)
    print("-" * 50)
    print("Welcome to Gultron, an AI-powered Git commit message generator tool!")
    print("This tool uses Google's Gemini AI to suggest meaningful commit messages based on your git diffs.")
    # print("\nFeatures to be added in the future:")
    # print("  - Integration with CI/CD pipelines for automated commit message generation")
    # print("  - Support for different code formats and languages")
    # print("  - Custom commit message templates and configurations\n")
    print("-" * 50)

def setup_logger():
    """Sets up the logger for colored logging output."""
    handler = colorlog.StreamHandler()
    handler.setFormatter(
        colorlog.ColoredFormatter(
            "%(log_color)s%(asctime)s - %(levelname)s - %(message)s",
            log_colors={
                "DEBUG": "cyan",
                "INFO": "green",
                "WARNING": "yellow",
                "ERROR": "red",
                "CRITICAL": "bold_red",
            },
        )
    )
    logger = logging.getLogger(__name__)
    logger.addHandler(handler)
    logger.setLevel(logging.INFO)
    return logger

LOGGER = setup_logger()

# def print_commit_message_in_box(commit_message, args):
#     """Prints the commit message inside an ASCII box."""
#     lines = commit_message.split("\n")
#     max_line_length = max(len(line) for line in lines)
#     box_width = max_line_length + 4

#     print("+" + "-" * (box_width - 2) + "+")
#     for line in lines:
#         print(f"| {line.ljust(max_line_length)} |")
#     print("+" + "-" * (box_width - 2) + "+")

#     if args.copy:
#         try:
#             pyperclip.copy(commit_message)
#             print("\nThe commit message has been copied to your clipboard!")
#         except pyperclip.PyperclipException:
#             LOGGER.warning("Failed to copy commit message to clipboard. Trying to install required tools...")
#             install_clipboard_tool()
