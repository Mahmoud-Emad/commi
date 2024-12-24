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
        LOGGER.error(f"Error during {context}: {str(exception)}")
        raise exception

    def get_diff(self, cached=False):
        """Fetches the git diff based on staged changes or the latest commit."""
        try:
            diff = self.repo.git.diff('--cached' if cached else 'HEAD')
            LOGGER.info("Successfully fetched git diff.")
            return diff
        except git.exc.GitCommandError as e:
            self._handle_error("fetching git diff", e)

    def generate_commit_message(self, diff_text):
        """Generates a commit message based on the provided diff."""
        try:
            if self.retry_count > 0:
                # Additional prompt, with guidelines
                diff_text_ = diff_text + "Please follow the exact format of the commit message as i requested."
                prompt_with_guidelines = self._build_commit_message_prompt(diff_text_)
            else:
                # Initial prompt
                prompt_with_guidelines = self._build_commit_message_prompt(diff_text)

            # Generate commit message
            prompt_with_guidelines = self._build_commit_message_prompt(diff_text)
            response = self.model.generate_content(prompt_with_guidelines)
            commit_message = response.text.strip()

            # Validate if commit message follows the format
            if not self._is_valid_commit_message(commit_message):
                LOGGER.warning("Commit message does not follow the expected format. Regenerating...")
                self.retry_count += 1
                
                # If the retry count exceeds the maximum allowed, raise an error
                if self.retry_count > self.max_retries:
                    LOGGER.warning("Maximum retries exceeded. The commit message does not follow the expected format.")
                    return commit_message
                
                # Regenerate commit message with additional guidance
                return self.generate_commit_message(diff_text)

            LOGGER.info("Commit message generated successfully.")
            return commit_message
        except Exception as e:
            self._handle_error("generating commit message", e)

    def _build_commit_message_prompt(self, diff_text):
        """Builds the prompt used to generate the commit message."""
        prompt = (
            f"Given the following changes in the code, suggest an appropriate commit message:\n\n"
            f"{diff_text}\n\n"
            "Commit message:\n"
        )
        guidelines = (
            "Please follow the exact format below for the commit message. "
            "Only return the commit message, no additional text or commentary.\n"
            "Commit message format:\n"
        )
        commit_format = """
        Commit message format:
        The message must start with one of these types: [feat, fix, docs, style, refactor, perf, test, chore]
        The summary of the change should be 50 characters or less.
        
        Follow this summary with a detailed description, wrapped at 72 characters. The detailed description should start with '- '.
        
        Example format:
        feat: add new feature

        - Add a new feature to the project
        - This feature does the following
        - It improves user experience by...

        **Strictly follow this format. Do not generate any message that does not follow this pattern.**
        """
        return f"{prompt}{guidelines}{commit_format}"

    def _is_valid_commit_message(self, message):
        """Validates if the commit message fits the expected format."""
        lines = message.splitlines()

        # Basic checks: we expect at least two lines
        if len(lines) < 2:
            return False

        # Check if the first line starts with a valid commit type (e.g., feat, fix, refactor)
        valid_types = ['feat', 'fix', 'docs', 'style', 'refactor', 'perf', 'test', 'chore']
        first_line = lines[0].strip()
        is_valid_type = False

        for valid_type in valid_types:
            if first_line.lower().startswith(valid_type):
                is_valid_type = True
                break

        if not is_valid_type:
            return False

        lines = lines[1:]

        # Check if each line starts with '- '
        for line in lines:
            if len(line) > 0 and not line.strip().startswith('-'):
                return False

        return True
