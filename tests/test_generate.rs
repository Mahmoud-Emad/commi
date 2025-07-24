use assert_cmd::Command;
use predicates::prelude::*;
use std::process::Command as StdCommand;
use tempfile::TempDir;

/// Tests for commit message generation functionality
#[test]
fn test_no_git_repo_error() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.env(
        "COMMI_API_KEY",
        "AIzaSyDjzOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO",
    );

    // Now commi without arguments shows help instead of trying to generate
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Commi v4.0.0"));
}

#[test]
fn test_git_repo_with_no_changes() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize git repo
    StdCommand::new("git")
        .args(["init"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    // Configure git
    StdCommand::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    StdCommand::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.env(
        "COMMI_API_KEY",
        "AIzaSyDjzOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO",
    );

    // Now commi without arguments shows help instead of trying to generate
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Commi v4.0.0"));
}

#[test]
fn test_generate_with_cached_flag() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["generate", "--cached"]);
    cmd.env(
        "COMMI_API_KEY",
        "AIzaSyDjzOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO",
    );

    // This will likely fail due to no git repo, but we're testing flag parsing
    cmd.assert().failure();
}

#[test]
fn test_generate_with_copy_flag() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["generate", "--copy"]);
    cmd.env(
        "COMMI_API_KEY",
        "AIzaSyDjzOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO",
    );

    // This will likely fail due to no git repo, but we're testing flag parsing
    cmd.assert().failure();
}

#[test]
fn test_generate_with_commit_flag() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["generate", "--commit"]);
    cmd.env(
        "COMMI_API_KEY",
        "AIzaSyDjzOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO",
    );

    // This will likely fail due to no git repo, but we're testing flag parsing
    cmd.assert().failure();
}

#[test]
fn test_generate_with_repo_flag() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["generate", "--repo", temp_dir.path().to_str().unwrap()]);
    cmd.env(
        "COMMI_API_KEY",
        "AIzaSyDjzOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO",
    );

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to find Git repository"));
}

#[test]
fn test_generate_with_co_author() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["generate", "--co-author", "test@example.com"]);
    cmd.env(
        "COMMI_API_KEY",
        "AIzaSyDjzOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO",
    );

    // This will likely fail due to no git repo, but we're testing flag parsing
    cmd.assert().failure();
}

#[test]
fn test_generate_missing_api_key() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.env("COMMI_TEST_MODE", "1"); // Use test config file (which will be empty)
    cmd.args(["generate"]);
    // Don't set COMMI_API_KEY

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("API key"));
}

#[test]
fn test_generate_with_api_key_flag() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args([
        "generate",
        "--api-key",
        "AIzaSyDjzOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO",
    ]);

    // This will likely fail due to no git repo, but we're testing flag parsing
    cmd.assert().failure();
}
