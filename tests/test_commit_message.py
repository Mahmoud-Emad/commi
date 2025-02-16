import pytest
from commi.commit_message import CommitMessageGenerator
from unittest.mock import Mock, patch, MagicMock

@pytest.fixture
def mock_repo():
    with patch('git.Repo') as mock:
        repo = Mock()
        repo.git_dir = "/mock/repo/.git"
        repo.git.diff.return_value = "mock diff"
        mock.return_value = repo
        yield repo

@pytest.fixture
def mock_genai():
    with patch('google.generativeai.GenerativeModel') as mock:
        model = Mock()
        mock.return_value = model
        yield model

@pytest.fixture
def generator(mock_repo, mock_genai):
    with patch('google.generativeai.configure'):
        return CommitMessageGenerator("/mock/repo", "mock-api-key", "mock-model")

def test_valid_commit_message_format(generator):
    """Test validation of properly formatted commit messages."""
    valid_messages = [
        # Basic format
        "feat: add new feature",
        # With description
        "fix: resolve memory leak\n\n- Properly close file handlers\n- Add cleanup routine",
        # Different types
        "docs: update README",
        "style: format code according to guidelines",
        "refactor: simplify error handling",
        "perf: optimize database queries",
        "test: add unit tests for auth module",
        "build: update dependencies",
        "ci: add GitHub Actions workflow",
        "chore: update gitignore",
        # Merge commit
        "Merge branch 'feature' into main"
    ]

    for msg in valid_messages:
        assert generator._is_valid_commit_message(msg), f"Should accept valid message: {msg}"

def test_invalid_commit_message_format(generator):
    """Test rejection of improperly formatted commit messages."""
    invalid_messages = [
        # Missing type prefix
        "add new feature",
        # Wrong format for type
        "feature: add new thing",
        # Too long summary (> 72 chars)
        "feat: " + "a" * 68,
        # No space after bullet point
        "fix: update handler\n\n-missing space here",
        # Missing blank line after summary
        "feat: add feature\n- First change",
        # Empty message
        "",
    ]

    for msg in invalid_messages:
        assert not generator._is_valid_commit_message(msg), f"Should reject invalid message: {msg}"

def create_mock_response(text):
    """Helper to create a properly mocked response."""
    mock_response = MagicMock()
    mock_text = MagicMock()
    mock_text.strip.return_value = text
    mock_text.splitlines.return_value = text.split('\n')
    mock_text.__getitem__.side_effect = text.split('\n').__getitem__
    mock_text.__str__.return_value = text
    mock_response.text = mock_text
    return mock_response

# def test_generate_commit_message(generator, mock_genai):
#     """Test successful commit message generation."""
#     # Create a proper mock response
#     message = "feat: add new feature\n\n- Implement X\n- Add tests"
#     mock_response = create_mock_response(message)
#     mock_genai.return_value.generate_content.return_value = mock_response

#     # Test generation
#     result = generator.generate_commit_message("mock diff")
#     print("result: ", result)
#     assert result == message
#     assert mock_genai.return_value.generate_content.called

# def test_generate_commit_message_retry(generator, mock_genai):
#     """Test commit message generation with retry on invalid format."""
#     # Create mock responses
#     invalid_message = "invalid format"
#     valid_message = "feat: add feature\n\n- Change 1\n- Change 2"
    
#     mock_response1 = create_mock_response(invalid_message)
#     mock_response2 = create_mock_response(valid_message)
    
#     mock_genai.return_value.generate_content.side_effect = [mock_response1, mock_response2]

#     result = generator.generate_commit_message("mock diff")
#     assert result == valid_message
#     assert mock_genai.return_value.generate_content.call_count == 2

# def test_generate_commit_message_max_retries(generator, mock_genai):
#     """Test commit message generation with max retries exceeded."""
#     # Create invalid mock response
#     message = "invalid format"
#     mock_response = create_mock_response(message)
#     mock_genai.return_value.generate_content.return_value = mock_response

#     result = generator.generate_commit_message("mock diff")
#     assert result == message  # Should return last attempt
#     assert mock_genai.return_value.generate_content.call_count == generator.max_retries + 1

@pytest.mark.parametrize("cached", [True, False])
def test_get_diff(generator, mock_repo, cached):
    """Test diff retrieval with and without --cached flag."""
    generator.get_diff(cached=cached)
    mock_repo.git.diff.assert_called_once_with('--cached' if cached else 'HEAD')

def test_initialization_error():
    """Test error handling during initialization."""
    with patch('git.Repo') as mock_repo:
        error_msg = "Error during initialization: Mock initialization error"
        mock_repo.side_effect = Exception("Mock initialization error")
        
        with pytest.raises(Exception) as exc_info:
            CommitMessageGenerator("/mock/repo", "mock-api-key", "mock-model")
        assert str(exc_info.value) == error_msg

def test_diff_error(generator, mock_repo):
    """Test error handling during diff retrieval."""
    mock_repo.git.diff.side_effect = Exception("Mock diff error")
    
    with pytest.raises(Exception) as exc_info:
        generator.get_diff()
    assert str(exc_info.value) == "Error during fetching git diff: Mock diff error"
