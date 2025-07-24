use assert_cmd::Command;
use predicates::prelude::*;

/// Tests for shell completion functionality
#[test]
fn test_bash_completion_generation() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["completion", "bash"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("_commi"))
        .stdout(predicate::str::contains("complete"));
}

#[test]
fn test_zsh_completion_generation() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["completion", "zsh"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("#compdef"))
        .stdout(predicate::str::contains("_commi"));
}

#[test]
fn test_fish_completion_generation() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["completion", "fish"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("complete"))
        .stdout(predicate::str::contains("commi"));
}

#[test]
fn test_invalid_shell_completion() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["completion", "invalid-shell"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
}

#[test]
fn test_completion_output_is_valid() {
    // Test that bash completion output is syntactically valid
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["completion", "bash"]);
    let output = cmd.assert().success().get_output().stdout.clone();

    // Basic validation that it contains expected bash completion structure
    let completion_script = String::from_utf8(output).unwrap();
    assert!(completion_script.contains("_commi()"));
    assert!(completion_script.contains("COMPREPLY"));
}

#[test]
fn test_completion_includes_all_subcommands() {
    let mut cmd = Command::cargo_bin("commi").unwrap();
    cmd.args(["completion", "bash"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("generate"))
        .stdout(predicate::str::contains("config"))
        .stdout(predicate::str::contains("completion"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("status"));
}
