import subprocess
import sys

def run():
    try:
        print("Running build.sh script...")
        subprocess.check_call(["bash", "build.sh"])
        print("build.sh script completed successfully.")
    except subprocess.CalledProcessError as e:
        print(f"Error during execution of build.sh: {e}", file=sys.stderr)
        sys.exit(1)
