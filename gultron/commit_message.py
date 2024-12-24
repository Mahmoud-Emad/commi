import git
import google.generativeai as genai

from gultron.logs import LOGGER

class CommitMessageGenerator:
    def __init__(self, repo_path, api_key, model_name):
        """Initializes the commit message generator with repo path, API key, and model name."""
        try:
            self.repo = git.Repo(repo_path)
            genai.configure(api_key=api_key)
            self.model = genai.GenerativeModel(model_name)
            LOGGER.info("CommitMessageGenerator initialized successfully.")
        except Exception as e:
            LOGGER.error(f"Error during initialization: {str(e)}")
            raise

    def get_diff(self, cached=False):
        """Fetches the git diff based on staged changes or the latest commit."""
        try:
            diff = self.repo.git.diff('--cached' if cached else 'HEAD')
            LOGGER.info("Successfully fetched git diff.")
            return diff
        except git.exc.GitCommandError as e:
            LOGGER.error(f"Git command error: {str(e)}")
            raise

    def generate_commit_message(self, diff_text):
        """Generates a commit message based on the provided diff."""
        try:
            prompt = (
                f"Given the following changes in the code, suggest an appropriate commit message:\n\n"
                f"{diff_text}\n\n"
                "Commit message:\n"
            )
            guidelines = (
                "Please don't send messages unrelated to the commit message; "
                "I only want to see the commit message in the response. "
                "Follow the format below:\n"
            )
            commit_format = """
            starting with one of the following based on the changes [feat, fix, docs, style, refactor, perf, test, chore]

            <Summarize change(s) in around 50 characters or less>

            <More detailed explanatory description of the change wrapped into about 72 characters should start with - >

            Example:
            feat: add new feature

            - Add a new feature
            - This feature does this
            - This feature does that
            """
            prompt_with_guidelines = f"{prompt}{guidelines}{commit_format}"
            response = self.model.generate_content(prompt_with_guidelines)
            LOGGER.info("Commit message generated successfully.")
            return response.text
        except Exception as e:
            LOGGER.error(f"Failed to generate commit message: {str(e)}")
            raise
