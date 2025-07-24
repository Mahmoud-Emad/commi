use assert_cmd::Command;
use predicates::prelude::*;

/// Tests for status command functionality
#[test]
fn test_status_command() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.arg("status");
    cmd.assert().success().stdout(
        predicate::str::contains("Repository Status")
            .or(predicate::str::contains("No changes detected")),
    );
}

#[test]
fn test_status_with_verbose_flag() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["status", "--verbose"]);
    cmd.assert().success().stdout(
        predicate::str::contains("Repository Status")
            .or(predicate::str::contains("No changes detected")),
    );
}

#[test]
fn test_status_with_repo_flag() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["status", "--repo", "/tmp"]);
    // /tmp is not a git repo, so this should fail
    cmd.assert().failure();
}

#[test]
fn test_global_verbose_flag() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["--verbose", "status"]);
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Repository path"));
}

#[test]
fn test_global_no_color_flag() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["--no-color", "status"]);
    cmd.assert().success();
}

#[test]
fn test_status_shows_repository_info() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["status", "--verbose"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Repository").or(predicate::str::contains("File")));
}

#[test]
fn test_status_handles_non_git_directory() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["status", "--repo", "/tmp"]);
    // Should fail when pointing to non-git directory
    cmd.assert().failure();
}
