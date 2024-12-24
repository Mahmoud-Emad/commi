import sys
import subprocess
import platform

def install_poetry():
    """Installs poetry if not already installed."""
    try:
        # Check if poetry is already installed
        subprocess.run(["poetry", "--version"], check=True)
        print("Poetry is already installed.")
    except subprocess.CalledProcessError:
        print("Poetry is not installed. Installing Poetry...")

        # Determine OS
        os_type = platform.system().lower()
        install_command = []

        if os_type == "linux":
            # For Linux
            install_command = ["curl", "-sSL", "https://install.python-poetry.org | python3 -"]
        elif os_type == "darwin":
            # For macOS
            install_command = ["curl", "-sSL", "https://install.python-poetry.org | python3 -"]
        elif os_type == "windows":
            # For Windows, Poetry installation requires different approach (Windows batch file)
            install_command = ["(Invoke-WebRequest -Uri https://install.python-poetry.org -UseBasicP) | python"]

        # Run the installation command
        subprocess.run(install_command, shell=True, check=True)
        print("Poetry installed successfully.")


def install_dependencies_and_build():
    """Installs dependencies using Poetry and runs the build script."""
    try:
        print("Installing dependencies with Poetry...")
        # Run `poetry install` to set up the environment
        subprocess.run(["poetry", "install"], check=True)

        print("Running build script...")
        # Now run the build script
        subprocess.run(["./build.sh"], check=True)
        print("Build script executed successfully.")

    except subprocess.CalledProcessError as e:
        print(f"Error during installation or build: {str(e)}")
        sys.exit(1)

def main():
    try:
        print("Starting the installation process...")

        # Step 1: Install Poetry if it's not already installed
        install_poetry()

        # Step 2: Install dependencies and run the build script
        install_dependencies_and_build()

        print("Installation and build process completed successfully.")

    except Exception as e:
        print(f"An error occurred: {str(e)}")
        sys.exit(1)
