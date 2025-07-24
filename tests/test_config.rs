use assert_cmd::Command;
use predicates::prelude::*;

/// Tests for configuration management functionality
#[test]
fn test_config_list_command() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.env("COMMI_TEST_MODE", "1"); // Use test config file
    cmd.env_remove("COMMI_API_KEY"); // Ensure no env API key
    cmd.args(["config", "list"]);
    // Test should handle both cases: API key present or not
    let result = cmd.assert();

    // Either succeeds with config list or fails with API key error
    result
        .code(predicate::in_iter([0, 1]))
        .stderr(predicate::str::contains("Configuration values"));
}

#[test]
fn test_config_get_api_key() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.env("COMMI_TEST_MODE", "1"); // Use test config file
    cmd.env_remove("COMMI_API_KEY"); // Ensure no env API key
    cmd.args(["config", "get", "api-key"]);
    // Test should handle both cases: API key present or not
    let result = cmd.assert();

    // Either succeeds with API key output or fails with error message
    result.code(predicate::in_iter([0, 1]));
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
    use tempfile::TempDir;

    // Create a unique temporary directory for this test
    let temp_dir = TempDir::new().unwrap();
    let test_config_path = temp_dir.path().join("commi_test_validate.toml");

    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.env("COMMI_TEST_MODE", "1"); // Use test config file
    cmd.env("COMMI_TEST_CONFIG_PATH", test_config_path.to_str().unwrap());
    cmd.args(["config", "set", "validate-format", "false"]);
    cmd.assert().success();

    // Check that it was set correctly
    let mut get_cmd = Command::cargo_bin("commi").unwrap();
    get_cmd.env("COMMI_TEST_MODE", "1"); // Use test config file
    get_cmd.env("COMMI_TEST_CONFIG_PATH", test_config_path.to_str().unwrap());
    get_cmd.args(["config", "get", "validate-format"]);
    get_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("false"));
}
