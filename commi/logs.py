import pyfiglet
import logging
import colorlog

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

def print_ultron_header():
    """Prints the banner for the application in color."""
    CYAN = "\033[0;36m"
    RESET = "\033[0m"
    # GREEN = "\033[0;32m"
    
    ultron_art = pyfiglet.figlet_format("Commi", font="slant")
    print(f"{CYAN}{ultron_art}{RESET}")
    print("-" * 100)
    print(f"Welcome to Commi, an AI-powered Git commit message generator tool!")
    print("This tool uses Google's Gemini AI to suggest meaningful commit messages based on your git diffs.")
    print("-" * 100)
