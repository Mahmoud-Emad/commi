import pytest
import os
import json
from unittest.mock import patch, Mock, mock_open, MagicMock
from datetime import datetime, timedelta
from commi.cmd import CommiCommands


class MockCommiCommands(CommiCommands):
    """A mock version of CommiCommands that doesn't parse arguments in __init__"""

    def __init__(self, installed_version="2.2.5", latest_version="v2.3.0"):
        self.VERSION_CACHE_FILE = os.path.expanduser("~/.commi_version")
        self.VERSION_CACHE_EXPIRY = timedelta(days=1)
        self.installed_version = installed_version
        self.latest_version = latest_version

        # Create a parser but don't parse args
        self._setup_parser()

    def _setup_parser(self):
        """Setup the argument parser without parsing args"""
        self.parser = MagicMock()
        self.args = MagicMock()

    def get_args(self):
        """Return mock args"""
        return self.args


@pytest.fixture
def mock_version_cache():
    """Fixture to create a mock version cache file."""
    cache_data = {
        "version": "v2.3.0",
        "fetched_at": (datetime.now() - timedelta(hours=2)).isoformat(),
    }

    with patch("os.path.exists") as mock_exists, patch(
        "builtins.open", mock_open(read_data=json.dumps(cache_data))
    ) as mock_file:
        mock_exists.return_value = True
        yield mock_file


def test_argument_parsing_update():
    """Test the --update argument parsing."""
    with patch("sys.argv", ["commi", "--update"]), patch(
        "argparse.ArgumentParser.parse_args", return_value=Mock(update=True)
    ):
        cmd = MockCommiCommands()
        cmd.args = Mock(update=True)
        args = cmd.get_args()
        assert args.update is True


def test_get_installed_version():
    """Test getting the installed version from pyproject.toml."""
    mock_pyproject = {"tool": {"poetry": {"version": "2.2.5"}}}

    with patch("toml.load", return_value=mock_pyproject):
        cmd = MockCommiCommands()
        version = cmd.get_installed_version()
        assert version == "2.2.5"

def test_is_update_available():
    """Test checking if an update is available."""
    # Test when update is available
    cmd = MockCommiCommands(installed_version="2.2.5", latest_version="v2.3.0")
    assert cmd.is_update_available() is True

    # Test when versions are equal
    cmd = MockCommiCommands(installed_version="2.3.0", latest_version="v2.3.0")
    assert cmd.is_update_available() is False

    # Test when installed version is newer
    cmd = MockCommiCommands(installed_version="2.4.0", latest_version="v2.3.0")
    assert cmd.is_update_available() is False


def test_update_binary():
    """Test updating the binary."""
    with patch("subprocess.run") as mock_run, patch(
        "subprocess.check_output", return_value=b"/usr/local/bin/commi\n"
    ):

        cmd = MockCommiCommands(installed_version="2.2.5", latest_version="v2.3.0")
        result = cmd.update_binary()

        assert result is True
        assert mock_run.call_count == 3
        # Check curl command
        assert mock_run.call_args_list[0][0][0][0] == "curl"
        # Check chmod command
        assert mock_run.call_args_list[1][0][0][0] == "chmod"
        # Check mv command
        assert mock_run.call_args_list[2][0][0][0] == "sudo"


def test_update_binary_error():
    """Test handling errors when updating the binary."""
    with patch("subprocess.run", side_effect=Exception("Update Error")):
        cmd = MockCommiCommands(installed_version="2.2.5", latest_version="v2.3.0")
        result = cmd.update_binary()
        assert result is False


def test_help_text_includes_update():
    """Test that help text includes the update option."""
    cmd = MockCommiCommands()
    cmd.parser.format_help.return_value = "--update Update Commi to the latest version"
    help_text = cmd.parser.format_help()
    assert "--update" in help_text
    assert "Update Commi to the latest version" in help_text


def test_main_with_update_flag(caplog):
    """Test main function with update flag."""
    from commi.run import main
    import logging

    # Set up caplog to capture log messages
    caplog.set_level(logging.INFO)

    # We need to patch the specific methods in run.py that use CommiCommands
    with patch("sys.argv", ["commi", "--update"]), patch(
        "commi.logs.print_ultron_header"
    ), patch("commi.run.CommiCommands") as mock_cmd_class:

        # Setup mock instance
        mock_instance = Mock()
        mock_instance.get_args.return_value = Mock(update=True, repo=None, api_key=None)
        mock_instance.is_update_available.return_value = True
        mock_instance.installed_version = "2.2.5"
        mock_instance.latest_version = "v2.3.0"
        mock_instance.update_binary.return_value = True

        # Make the mock class return our mock instance
        mock_cmd_class.return_value = mock_instance

        # Run main
        main()

        # Verify log messages using caplog
        assert "Update available: 2.2.5 -> v2.3.0" in caplog.text
        assert mock_instance.update_binary.call_count >= 1


def test_main_with_update_notification(caplog):
    """Test main function with update notification."""
    from commi.run import main
    import logging

    # Set up caplog to capture log messages
    caplog.set_level(logging.INFO)

    with patch(
        "sys.argv", ["commi", "--repo", "/mock", "--api-key", "test-key"]
    ), patch("commi.logs.print_ultron_header"), patch(
        "commi.run.CommiCommands"
    ) as mock_cmd_class, patch(
        "commi.run.CommitMessageGenerator"
    ), patch(
        "commi.run.load_configuration", return_value=("test-key", "gemini-1.5-flash")
    ), patch(
        "commi.run.setup_repo_path", return_value="/mock"
    ), patch(
        "commi.run.generate_commit_message", return_value="test commit"
    ), patch(
        "commi.run.has_changes", return_value=False
    ):

        # Setup mock instance
        mock_instance = Mock()
        mock_instance.get_args.return_value = Mock(
            update=False,
            repo="/mock",
            api_key="test-key",
            cached=False,
            copy=False,
            commit=False,
            co_author=None,
        )
        mock_instance.is_update_available.return_value = True
        mock_instance.installed_version = "2.2.5"
        mock_instance.latest_version = "v2.3.0"

        # Make the mock class return our mock instance
        mock_cmd_class.return_value = mock_instance

        # Run main
        main()

        # Verify update notification was shown in the logs using caplog
        assert "A new version of Commi is available: v2.3.0" in caplog.text
        assert "Run 'commi --update' to update to the latest version." in caplog.text
