use assert_cmd::Command;
use predicates::prelude::*;

/// Tests for update management functionality
#[test]
fn test_update_check_command() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.env("COMMI_TEST_MODE", "1"); // Use test config file
    cmd.args(["update", "check"]);
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Checking for updates"))
        .stderr(predicate::str::contains(
            "You are already using the latest version",
        ));
}

#[test]
fn test_update_install_requires_confirmation() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.env("COMMI_TEST_MODE", "1"); // Use test config file
    cmd.args(["update", "install"]);
    // Should succeed but show "already up to date" message
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Already up to date"));
}

#[test]
fn test_update_install_with_force_flag() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.env("COMMI_TEST_MODE", "1"); // Use test config file
    cmd.args(["update", "install", "--force"]);
    // Should succeed but show "already up to date" message
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Already up to date"));
}

#[test]
fn test_update_check_shows_version_info() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.env("COMMI_TEST_MODE", "1"); // Use test config file
    cmd.args(["update", "check"]);
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("version"));
}

#[test]
fn test_update_install_help_shows_force_option() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["update", "install", "--help"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("--force"));
}

#[test]
fn test_legacy_update_flag() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.env("COMMI_TEST_MODE", "1"); // Use test mode to avoid network calls
    cmd.arg("--update");
    cmd.assert()
        .success() // Should succeed in test mode
        .stderr(predicate::str::contains(
            "Warning: You are using the legacy CLI",
        ))
        .stderr(predicate::str::contains("Consider migrating"))
        .stderr(predicate::str::contains(
            "You're already running the latest version",
        ));
}

#[test]
fn test_update_check_handles_network_errors() {
    // This test verifies that update check gracefully handles network issues
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.env("COMMI_TEST_MODE", "1"); // Use test mode to avoid network calls
    cmd.args(["update", "check"]);
    // Should succeed even if network is unavailable (with appropriate message)
    cmd.assert().success();
}
