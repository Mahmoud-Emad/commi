use assert_cmd::Command;
use predicates::prelude::*;

/// Tests for configuration management functionality
#[test]
fn test_config_list_command() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.env("COMMI_TEST_MODE", "1"); // Use test config file
    cmd.args(["config", "list"]);
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Configuration values"))
        .stdout(predicate::str::contains("api-key"))
        .stdout(predicate::str::contains("model"));
}

#[test]
fn test_config_get_api_key() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.env("COMMI_TEST_MODE", "1"); // Use test config file
    cmd.args(["config", "get", "api-key"]);
    // Should succeed when API key is in config file
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("api-key:"));
}

#[test]
fn test_config_get_nonexistent_key() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.env("COMMI_TEST_MODE", "1"); // Use test config file
    cmd.args(["config", "get", "nonexistent-key"]);
    // Should fail with unknown key error
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Unknown configuration key"));
}

#[test]
fn test_config_set_validation() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.env("COMMI_TEST_MODE", "1"); // Use test config file
    cmd.args([
        "config",
        "set",
        "api-key",
        "AIzaSyDjzOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO",
    ]);
    // Should succeed with valid key format
    cmd.assert().success();
}

#[test]
fn test_config_reset_requires_confirmation() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.env("COMMI_TEST_MODE", "1"); // Use test config file
    cmd.args(["config", "reset"]);
    // Should fail without --yes flag (requires interactive confirmation)
    cmd.assert().failure();
}

#[test]
fn test_config_reset_with_yes_flag() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.env("COMMI_TEST_MODE", "1"); // Use test config file
    cmd.args(["config", "reset", "--yes"]);
    // Should succeed with --yes flag
    cmd.assert().success();
}

#[test]
fn test_config_reset_specific_key() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.env("COMMI_TEST_MODE", "1"); // Use test config file
    cmd.args(["config", "reset", "--key", "api-key", "--yes"]);
    // Should succeed when resetting specific key
    cmd.assert().success().stderr(predicate::str::contains(
        "Configuration key 'api-key' reset successfully",
    ));
}

#[test]
fn test_config_validate_format_setting() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.env("COMMI_TEST_MODE", "1"); // Use test config file
    cmd.args(["config", "set", "validate-format", "false"]);
    cmd.assert().success();

    // Check that it was set correctly
    let mut get_cmd = Command::cargo_bin("commi").unwrap();
    get_cmd.env("COMMI_TEST_MODE", "1"); // Use test config file
    get_cmd.args(["config", "get", "validate-format"]);
    get_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("false"));
}
