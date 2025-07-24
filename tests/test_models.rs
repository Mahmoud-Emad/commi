use assert_cmd::Command;
use predicates::prelude::*;

/// Tests for model management functionality
#[test]
fn test_model_list_command() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["model", "list"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Supported AI Models"))
        .stdout(predicate::str::contains("gemini-1.5-flash"))
        .stdout(predicate::str::contains("gemini-2.5-flash"))
        .stdout(predicate::str::contains("gemini-2.5-flash-lite"))
        .stdout(predicate::str::contains("gemini-2.5-pro"));
}

#[test]
fn test_model_list_shows_current_model() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.env("COMMI_TEST_MODE", "1"); // Use test config file
    cmd.args(["model", "list"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Current model:"));
}

#[test]
fn test_model_list_shows_change_instructions() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["model", "list"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("To change model:"))
        .stdout(predicate::str::contains("commi config set model"));
}

#[test]
fn test_model_help_command() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["model", "--help"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("List and manage AI models"))
        .stdout(predicate::str::contains("list"));
}

#[test]
fn test_model_command_in_main_help() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("model"))
        .stdout(predicate::str::contains("List and manage AI models"));
}

#[test]
fn test_model_command_in_simple_help() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("model"))
        .stdout(predicate::str::contains("List and manage AI models"));
}

#[test]
fn test_model_integration_with_config() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.env("COMMI_TEST_MODE", "1"); // Use test config file

    // First set a model
    cmd.args(["config", "set", "model", "gemini-2.5-pro"]);
    cmd.assert().success();

    // Then check if model list shows the current model correctly
    let mut list_cmd = Command::cargo_bin("commi").unwrap();
    list_cmd.env("COMMI_TEST_MODE", "1"); // Use test config file
    list_cmd.args(["model", "list"]);
    list_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("gemini-2.5-pro"));
}

#[test]
fn test_model_list_shows_all_supported_models() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["model", "list"]);

    // Check that all expected models are listed
    let expected_models = [
        "gemini-1.5-flash",
        "gemini-2.5-flash",
        "gemini-2.5-flash-lite",
        "gemini-2.5-pro",
    ];

    let mut assertion = cmd.assert().success();
    for model in expected_models {
        assertion = assertion.stdout(predicate::str::contains(model));
    }
}

#[test]
fn test_model_list_shows_descriptions() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["model", "list"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Fast and efficient"))
        .stdout(predicate::str::contains("Latest fast model"))
        .stdout(predicate::str::contains("Lightweight version"))
        .stdout(predicate::str::contains("Most capable model"));
}
