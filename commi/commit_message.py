import git
import google.generativeai as genai
from commi.logs import LOGGER

class CommitMessageGenerator:
    def __init__(self, repo_path, api_key, model_name, max_retries=3):
        """Initializes the commit message generator with repo path, API key, and model name."""
        self.repo = None
        self.model = None
        self.max_retries = max_retries  # Set maximum retries limit
        self.retry_count = 0  # Track retry attempts
        
        try:
            self.repo = git.Repo(repo_path)
            genai.configure(api_key=api_key)
            self.model = genai.GenerativeModel(model_name)
            LOGGER.info("CommitMessageGenerator initialized successfully.")
        except Exception as e:
            self._handle_error("initialization", e)

    def _handle_error(self, context, exception):
        """Handles errors by logging and raising the exception."""
        error_msg = f"Error during {context}: {str(exception)}"
        LOGGER.error(error_msg)
        if isinstance(exception, Exception):
            raise type(exception)(error_msg)
        raise Exception(error_msg)

    def get_diff(self, cached=False):
        """Fetches the git diff based on staged changes or the latest commit."""
        try:
            diff = self.repo.git.diff('--cached' if cached else 'HEAD')
            LOGGER.info("Successfully fetched git diff.")
            return diff
        except Exception as e:
            self._handle_error("fetching git diff", e)

    def generate_commit_message(self, diff_text):
        """Generates a commit message based on the provided diff."""
        try:
            # Build prompt with additional guidance if retrying
            if self.retry_count > 0:
                diff_text = diff_text + "\nPlease strictly follow the commit message format guidelines."

            # Generate commit message
            prompt = self._build_commit_message_prompt(diff_text)
            response = self.model.generate_content(prompt)
            commit_message = str(response.text.strip())

            # Validate if commit message follows the format
            if not self._is_valid_commit_message(commit_message):
                LOGGER.warning("Commit message does not follow the expected format. Regenerating...")
                self.retry_count += 1
                
                if self.retry_count > self.max_retries:
                    LOGGER.warning("Maximum retries exceeded. Using the last generated message.")
                    return commit_message
                
                return self.generate_commit_message(diff_text)

            LOGGER.info("Commit message generated successfully.")
            return commit_message
        except Exception as e:
            self._handle_error("generating commit message", e)

    def _build_commit_message_prompt(self, diff_text):
        """Builds the prompt used to generate the commit message."""
        commit_types = {
            'feat': 'New feature',
            'fix': 'Bug fix',
            'docs': 'Documentation changes',
            'style': 'Code style changes (formatting, etc)',
            'refactor': 'Code refactoring',
            'perf': 'Performance improvements',
            'test': 'Adding or updating tests',
            'build': 'Build system changes',
            'ci': 'CI/CD changes',
            'chore': 'General maintenance',
            'revert': 'Reverting changes',
            'merge': 'Merge commits'
        }

        prompt = (
            "Given the following code changes, generate a commit message following these guidelines:\n\n"
            "1. Start with a type prefix from the following list:\n"
            "\n".join([f"   {type_}: {desc}" for type_, desc in commit_types.items()]) + "\n\n"
            "2. After the type, add a colon and space, then a short (72 chars or less) summary\n"
            "3. Leave one blank line after the summary\n"
            "4. Use bullet points (with - ) for listing multiple changes\n"
            "5. Each bullet point should explain WHAT and WHY, not HOW\n\n"
            "The changes are:\n"
            f"{diff_text}\n\n"
            "Reference format:\n"
            "```\n"
            "feat: add CPU arch filter scheduler support\n\n"
            "- Implement new filtering mechanism for CPU architectures\n"
            "- Add configuration options for arch-based scheduling\n"
            "- Update documentation with new filter details\n"
            "```\n"
            "Generate a commit message for these changes following the above format."
        )

        return prompt

    def _is_valid_commit_message(self, message):
        """Validates if the commit message fits the expected format."""
        try:
            lines = str(message).splitlines()

            # Need at least a summary line
            if not lines:
                return False

            # Summary line validation
            summary = lines[0].strip()
            if len(summary) > 72:
                return False
            
            # Check for conventional commit format
            first_word = summary.split(':')[0].lower() if ':' in summary else ""
            commit_types = {
                'feat', 'fix', 'docs', 'style', 'refactor', 'perf', 
                'test', 'build', 'ci', 'chore', 'revert', 'merge'
            }
            if first_word not in commit_types:
                # Special case for merge commits
                if summary.lower().startswith('merge'):
                    return True
                return False

            # If there's a body, it should be separated by a blank line
            if len(lines) > 1 and lines[1].strip():
                return False

            # Check line lengths and bullet points in body
            for line in lines[2:]:
                line = line.strip()
                if not line:
                    continue
                if len(line) > 72:
                    return False
                # Bullet points should be properly formatted
                if line.startswith('-') and not line.startswith('- '):
                    return False

            return True
        except Exception as e:
            LOGGER.error(f"Error validating commit message: {str(e)}")
            return False
