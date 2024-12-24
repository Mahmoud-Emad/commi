# Gultron: AI-powered Git Commit Message Generator

Gultron is an AI-powered tool that automatically generates Git commit messages based on your code changes. It uses Google's **Gemini AI** to analyze your Git diffs and suggest meaningful, descriptive commit messages. Gultron helps save time and ensures your commit history is consistent and descriptive.

## Features

- **AI-powered commit message generation**: Suggest commit messages based on the code changes using Google's Gemini AI.
- **Staged changes support**: Generate commit messages based on the current staged changes.
- **Clipboard integration**: Optionally copy the generated commit message to your clipboard for easy pasting.
<!-- - **Future updates**: Integration with CI/CD pipelines, support for different languages, and customizable commit message templates. -->

## Installation

### Manual Installation

To install **Gultron** manually, follow these steps:
1. Clone the repository:

   ```bash
   git clone https://github.com/Mahmoud-Emad/gultron.git
   cd gultron
   ```

2. Install the dependencies:

   ```bash
   poetry install
   ```

3. Build the executable:

   ```bash
   ./build
   ```

4. Run the executable:

   ```bash
   # Inside a repo you can do
   gultron --generate
   # or run it from everywhere and pass the repo path as an argument
   gultron --repo "/path/to/your/repo" --generate
   ```

### Using pip

You can install **Gultron** using pip:

```bash
    pip install gultron
```

Run the executable:

```bash
# Inside a repo you can do
gultron --generate
# or run it from everywhere and pass the repo path as an argument
gultron --repo "/path/to/your/repo" --generate
```

Now you're ready to use **Gultron**!

## Usage

After installation, you can use the command-line interface (CLI) to generate commit messages.

### Basic Usage

To generate a commit message based on the latest changes in your repository:

```bash
gultron --repo "/path/to/your/repo" --generate
```

### Use Staged Changes

If you want to generate a commit message based on the staged changes:

```bash
gultron --repo "/path/to/your/repo" --cached
```

### Set an API Key

You can provide your API key directly via the `--api-key` option:

```bash
gultron --repo "/path/to/your/repo" --api-key "your_api_key"
```

Alternatively, you can set the API key as an environment variable:

```bash
export API_KEY="your_api_key"
gultron --repo "/path/to/your/repo"
```

If no API key is provided, a default API key will be used.

### Copy the Commit Message to Clipboard

To copy the generated commit message to your clipboard, use the `--copy` flag:

```bash
gultron --repo "/path/to/your/repo" --copy
```

This requires the installation of `xclip` (on Linux systems). If it's not installed, Gultron will attempt to install it automatically.

### Regenerate a Commit Message

To regenerate the commit message, you can use the `--regenerate` flag, which will generate a new commit message based on the latest changes.

```bash
gultron --repo "/path/to/your/repo" --regenerate
```

## Configuration

Gultron uses a `.env` file to configure certain settings. You can modify the following settings:

- `API_KEY`: Your Gemini AI API key.
- `MODEL_NAME`: The AI model to use for commit message generation (e.g., `gemini-1.5-flash`).

## Contributing

We welcome contributions to Gultron! If you find a bug or want to suggest a new feature, please open an issue or submit a pull request.

1. Fork the repository.
2. Create a new branch for your feature (`git checkout -b feature-name`).
3. Commit your changes (`git commit -am 'Add new feature'`).
4. Push to the branch (`git push origin feature-name`).
5. Create a new pull request.

## License

Gultron is open-source software licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
