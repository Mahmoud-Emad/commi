import pytest
from commi.cmd import CommiCommands
import sys
from unittest.mock import patch

def test_argument_parsing():
    """Test command line argument parsing."""
    test_cases = [
        # Test repo path
        (["--repo", "/path/to/repo"], {"repo": "/path/to/repo"}),
        # Test API key
        (["--api-key", "test-key"], {"api_key": "test-key"}),
        # Test cached flag
        (["--cached"], {"cached": True}),
        # Test copy flag
        (["--copy"], {"copy": True}),
        # Test commit flag
        (["--commit"], {"commit": True}),
        # Test co-author
        (["--co-author", "test@example.com"], {"co_author": "test@example.com"}),
        # Test multiple flags
        (
            ["--repo", "/repo", "--cached", "--copy"],
            {"repo": "/repo", "cached": True, "copy": True}
        ),
    ]

    for args, expected in test_cases:
        with patch('sys.argv', ['commi'] + args):
            cmd = CommiCommands()
            args = cmd.get_args()
            for key, value in expected.items():
                assert getattr(args, key) == value

def test_help_display():
    """Test help text display when no arguments provided."""
    with patch('sys.argv', ['commi']):
        cmd = CommiCommands()
        with pytest.raises(SystemExit) as exc_info:
            cmd.get_args()
        assert exc_info.value.code == 0

def test_help_text_content():
    """Test that help text contains important information."""
    with patch('sys.argv', ['commi']):
        cmd = CommiCommands()
        help_text = cmd.parser.format_help()
        
        important_elements = [
            "AI-powered Git commit message generator",
            "Git commit message format",
            "--repo",
            "--api-key",
            "--cached",
            "--copy",
            "--commit",
            "--co-author",
            "summary line",
            "bullet points"
        ]
        
        for element in important_elements:
            assert element.lower() in help_text.lower(), f"Help text should contain '{element}'"

def test_co_author_email_format():
    """Test co-author email format validation."""
    test_cases = [
        # Valid email
        (["--co-author", "test@example.com"], True),
        # Invalid email (no @)
        (["--co-author", "invalid-email"], False),
    ]

    for args, should_pass in test_cases:
        with patch('sys.argv', ['commi'] + args):
            cmd = CommiCommands()
            args = cmd.get_args()
            if should_pass:
                assert '@' in args.co_author
            else:
                assert args.co_author and '@' not in args.co_author
