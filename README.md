# Commi: AI-powered Git Commit Message Generator

Commi is an AI-powered tool that automatically generates Git commit messages based on your code changes. It uses Google's **Gemini AI** to analyze your Git diffs and suggest meaningful, descriptive commit messages. Commi helps save time and ensures your commit history is consistent and descriptive.

## Features

- **AI-powered commit message generation**: Suggest commit messages based on the code changes using Google's Gemini AI.
- **Staged changes support**: Generate commit messages based on the current staged changes.
- **Direct commit**: Generate and commit the suggested commit message directly to your repository.
- **Clipboard integration**: Optionally copy the generated commit message to your clipboard for easy pasting.
<!-- - **Future updates**: Integration with CI/CD pipelines, support for different languages, and customizable commit message templates. -->

## Installation

### Build from source

To install **Commi** manually, follow these steps:
1. Clone the repository:

   ```bash
   git clone https://github.com/Mahmoud-Emad/commi.git
   cd commi
   ```

2. Install the dependencies:

   ```bash
   poetry install
   ```

3. Build the executable:

   ```bash
   ./build.sh
   # This aslo will move the excutable to `/usr/local/bin`
   ```

4. Run the executable:

   ```bash
   # Inside a repo you can do
   commi --copy
   # or run it from everywhere and pass the repo path as an argument
   commi --repo "/path/to/your/repo" --commit
   ```

### Using pip

You can install **Commi** using pip:

```bash
    pip install commi
```

Run the executable:

```bash
# Inside a repo you can do
commi --generate
# or run it from everywhere and pass the repo path as an argument
commi --repo "/path/to/your/repo" --generate
```

### Install the binary

You can also download the pre-built binary for your system from the [releases page](https://github.com/Mahmoud-Emad/commi/releases).

Now you're ready to use **Commi**!

## Usage

After installation, you can use the command-line interface (CLI) to generate commit messages.

### Basic Usage

To generate a commit message based on the latest changes in your repository:

```bash
commi --repo "/path/to/your/repo"
```

### Use Staged Changes

If you want to generate a commit message based on the staged changes:

```bash
commi --repo "/path/to/your/repo" --cached
```

### Set an API Key

You can provide your API key directly via the `--api-key` option:

```bash
commi --repo "/path/to/your/repo" --api-key "your_api_key"
```

Alternatively, you can set the API key as an environment variable:

```bash
export API_KEY="your_api_key"
commi --repo "/path/to/your/repo"
```

If no API key is provided, a default API key will be used.

### Copy the Commit Message to Clipboard

To copy the generated commit message to your clipboard, use the `--copy` flag:

```bash
commi --repo "/path/to/your/repo" --copy
```

This requires the installation of `xclip` (on Linux systems). If it's not installed, Commi will attempt to install it automatically.

### Regenerate a Commit Message

To regenerate the commit message, you can use the `--regenerate` flag, which will generate a new commit message based on the latest changes.

```bash
commi --repo "/path/to/your/repo" --regenerate
```

### Commit the suggested Commit Message

To commit the generated commit message, use the `--commit` flag:

```bash
commi --repo "/path/to/your/repo" --commit
```

## Configuration

Commi uses a `.env` file to configure certain settings. You can modify the following settings:

- `API_KEY`: Your Gemini AI API key.
- `MODEL_NAME`: The AI model to use for commit message generation (e.g., `gemini-1.5-flash`).

## Contributing

We welcome contributions to Commi! If you find a bug or want to suggest a new feature, please open an issue or submit a pull request.

1. Fork the repository.
2. Create a new branch for your feature (`git checkout -b feature-name`).
3. Commit your changes (`git commit -am 'Add new feature'`).
4. Push to the branch (`git push origin feature-name`).
5. Create a new pull request.

## License

Commi is open-source software licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
