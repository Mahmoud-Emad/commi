use assert_cmd::Command;
use predicates::prelude::*;

/// Tests for error handling and "did you mean?" functionality
#[test]
fn test_invalid_subcommand_with_suggestion() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.arg("confgi"); // Typo for "config"
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"))
        .stderr(predicate::str::contains("config"));
}

#[test]
fn test_invalid_subcommand_generate_typo() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.arg("generaet"); // Typo for "generate"
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"))
        .stderr(predicate::str::contains("generate"));
}

#[test]
fn test_invalid_subcommand_status_typo() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.arg("statu"); // Typo for "status"
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"))
        .stderr(predicate::str::contains("status"));
}

#[test]
fn test_invalid_subcommand_update_typo() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.arg("updat"); // Typo for "update"
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"))
        .stderr(predicate::str::contains("update"));
}

#[test]
fn test_invalid_subcommand_completion_typo() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.arg("completio"); // Typo for "completion"
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"))
        .stderr(predicate::str::contains("completion"));
}

#[test]
fn test_completely_invalid_subcommand() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.arg("xyz123invalid");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
}

#[test]
fn test_invalid_flag_suggestion() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["generate", "--cach"]); // Typo for "--cached"
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unexpected argument"));
}

#[test]
fn test_missing_required_argument() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["config", "set"]); // Missing key and value
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_invalid_shell_for_completion() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["completion", "invalid-shell"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
}

#[test]
fn test_error_message_formatting() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.arg("invalid-command");
    let output = cmd.assert().failure().get_output().stderr.clone();
    let error_message = String::from_utf8(output).unwrap();

    // Verify error message is well-formatted
    assert!(error_message.contains("error:"));
    assert!(!error_message.is_empty());
}

#[test]
fn test_help_suggestion_on_error() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.arg("invalid-command");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("help").or(predicate::str::contains("--help")));
}
