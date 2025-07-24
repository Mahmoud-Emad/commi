use assert_cmd::Command;
use predicates::prelude::*;

/// Tests for help system and command documentation
#[test]
fn test_main_help_command() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.arg("--help");
    cmd.assert().success().stdout(predicate::str::contains(
        "AI-Powered Git Commit Message Generator",
    ));
}

#[test]
fn test_version_command() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("4.0.0"));
}

#[test]
fn test_new_help_command_structure() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Commands:"))
        .stdout(predicate::str::contains("generate"))
        .stdout(predicate::str::contains("config"))
        .stdout(predicate::str::contains("completion"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("status"));
}

#[test]
fn test_generate_subcommand_help() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["generate", "--help"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Generate commit messages"))
        .stdout(predicate::str::contains("--cached"))
        .stdout(predicate::str::contains("--copy"))
        .stdout(predicate::str::contains("--commit"));
}

#[test]
fn test_config_subcommand_help() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["config", "--help"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Manage configuration"))
        .stdout(predicate::str::contains("set"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("reset"));
}

#[test]
fn test_completion_subcommand_help() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["completion", "--help"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Generate shell completion"))
        .stdout(predicate::str::contains("bash"))
        .stdout(predicate::str::contains("zsh"))
        .stdout(predicate::str::contains("fish"));
}

#[test]
fn test_update_subcommand_help() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["update", "--help"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Update management"))
        .stdout(predicate::str::contains("check"))
        .stdout(predicate::str::contains("install"));
}

#[test]
fn test_status_subcommand_help() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["status", "--help"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Show repository and tool status"))
        .stdout(predicate::str::contains("--repo"))
        .stdout(predicate::str::contains("--verbose"));
}

#[test]
fn test_config_reset_help() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["config", "reset", "--help"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Reset configuration"))
        .stdout(predicate::str::contains("--yes"));
}

#[test]
fn test_update_install_help() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["update", "install", "--help"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Install available updates"))
        .stdout(predicate::str::contains("--force"));
}
