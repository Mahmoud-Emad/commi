import pytest
from commi.run import (
    validate_repo_path, has_changes, commit_changes, 
    validate_model_name, load_configuration, CommiError
)
from unittest.mock import Mock, patch, MagicMock
import git

def test_validate_repo_path():
    """Test Git repository path validation."""
    with patch('git.Repo') as mock_repo:
        # Valid repo
        mock_repo.return_value.git_dir = "/mock/.git"
        assert validate_repo_path("/mock")
        
        # Invalid repo
        mock_repo.side_effect = git.exc.InvalidGitRepositoryError("Not a git repo")
        assert not validate_repo_path("/not-a-repo")

def test_has_changes():
    """Test detection of repository changes."""
    mock_repo = Mock()
    
    # Test staged changes
    mock_repo.git.diff.return_value = "staged changes"
    assert has_changes(mock_repo, cached=True)
    mock_repo.git.diff.assert_called_with('--cached')
    
    # Test unstaged changes
    mock_repo.is_dirty.return_value = True
    assert has_changes(mock_repo, cached=False)
    mock_repo.is_dirty.assert_called_with(untracked_files=True)
    
    # Test no changes
    mock_repo.git.diff.return_value = ""
    mock_repo.is_dirty.return_value = False
    assert not has_changes(mock_repo, cached=True)
    assert not has_changes(mock_repo, cached=False)

def test_commit_changes():
    """Test commit operation."""
    mock_repo = Mock()
    
    # Test successful commit
    mock_repo.git.diff.return_value = "changes"
    commit_changes(mock_repo, "test commit")
    mock_repo.git.commit.assert_called_with('-m', "test commit")
    
    # Test commit with no changes
    mock_repo.git.diff.return_value = ""
    with pytest.raises(CommiError) as exc_info:
        commit_changes(mock_repo, "test commit")
    assert "No staged changes" in str(exc_info.value)
    
    # Test commit error
    mock_repo.git.diff.return_value = "changes"
    mock_repo.git.commit.side_effect = git.exc.GitCommandError("commit", "Commit failed")
    with pytest.raises(CommiError) as exc_info:
        commit_changes(mock_repo, "test commit")
    assert "Failed to commit changes" in str(exc_info.value)

def test_validate_model_name():
    """Test model name validation."""
    # Test valid models
    valid_models = ['gemini-1.0-pro', 'gemini-1.5-pro', 'gemini-1.5-flash']
    for model in valid_models:
        assert validate_model_name(model) == model
    
    # Test invalid model (should return default)
    assert validate_model_name("invalid-model") == "gemini-1.5-flash"

def test_load_configuration():
    """Test configuration loading."""
    # Test with API key from args
    mock_args = Mock(api_key="test-key")
    with patch('commi.run.config') as mock_config:
        def config_side_effect(key, default=None):
            if key == "MODEL_NAME":
                return "gemini-1.5-flash"
            return default
        mock_config.side_effect = config_side_effect
        
        api_key, model = load_configuration(mock_args)
        assert api_key == "test-key"
        assert model == "gemini-1.5-flash"
    
    # Test with API key from env
    mock_args = Mock(api_key=None)
    with patch('commi.run.config') as mock_config:
        def config_side_effect(key, default=None):
            if key == "COMMI_API_KEY":
                return "env-key"
            elif key == "MODEL_NAME":
                return "gemini-1.5-flash"
            return default
        mock_config.side_effect = config_side_effect
        
        api_key, model = load_configuration(mock_args)
        assert api_key == "env-key"
        assert model == "gemini-1.5-flash"
    
    # Test missing API key
    mock_args = Mock(api_key=None)
    with patch('commi.run.config') as mock_config:
        def config_side_effect(key, default=None):
            if key == "MODEL_NAME":
                return "gemini-1.5-flash"
            elif key == "COMMI_API_KEY":
                return None
            return default
        mock_config.side_effect = config_side_effect
        
        with pytest.raises(CommiError) as exc_info:
            load_configuration(mock_args)
        assert "COMMI_API_KEY is not set" in str(exc_info.value)

@pytest.fixture
def mock_dependencies():
    """Fixture to mock all dependencies for integration tests."""
    with patch('git.Repo') as mock_repo, \
         patch('google.generativeai.GenerativeModel') as mock_genai, \
         patch('google.generativeai.configure') as mock_configure, \
         patch('pyperclip.copy') as mock_copy, \
         patch('decouple.config') as mock_config:
        
        # Setup mock repo
        repo = Mock()
        repo.git_dir = "/mock/.git"
        repo.git.diff.return_value = "mock changes"
        mock_repo.return_value = repo
        
        # Setup mock AI
        model = Mock()
        response = Mock()
        mock_text = MagicMock()
        lines = "feat: test change\n\n- Change 1\n- Change 2".split('\n')
        mock_text.splitlines.return_value = lines
        mock_text.strip.return_value = "feat: test change\n\n- Change 1\n- Change 2"
        mock_text.__getitem__.side_effect = lambda idx: lines[idx]
        response.text = mock_text
        model.generate_content.return_value = response
        mock_genai.return_value = model

        # Setup mock config
        def config_side_effect(key, default=None):
            if key == "MODEL_NAME":
                return "gemini-1.5-flash"
            return default
        mock_config.side_effect = config_side_effect
        
        yield {
            'repo': repo,
            'genai': model,
            'configure': mock_configure,
            'copy': mock_copy,
            'config': mock_config
        }

def test_main_flow(mock_dependencies):
    """Test the main execution flow."""
    from commi.run import main
    
    with patch('sys.argv', ['commi', '--copy', '--api-key', 'test-key']), \
         patch('commi.cmd.CommiCommands') as mock_cmd:
        
        # Setup mock args
        args = Mock(
            repo=None,
            api_key="test-key",
            cached=False,
            copy=True,
            commit=False,
            co_author=None
        )
        mock_cmd.return_value.get_args.return_value = args
        
        # Run main
        main()
        
        # Verify flow
        mock_dependencies['configure'].assert_called_once()
        mock_dependencies['repo'].git.diff.assert_called_once()
        mock_dependencies['genai'].generate_content.assert_called_once()
        mock_dependencies['copy'].assert_called_once()
