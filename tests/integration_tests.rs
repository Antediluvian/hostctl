//! Integration tests - Test complete functionality of hostctl

use std::process::Command;
use std::str;

#[cfg(test)]
pub mod additional_tests;

/// Test basic CLI commands
#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("hostctl"));
    assert!(stdout.contains("Manage hosts file with different environments"));
}

/// Test list command (when no environments exist)
#[test]
fn test_cli_list_empty() {
    let output = Command::new("cargo")
        .args(["run", "--", "list"])
        .output()
        .expect("Failed to execute command");

    // list command should succeed even without environments
    assert!(output.status.success());

    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("No environments configured."));
}

/// Test current command (when no current environment)
#[test]
fn test_cli_current_no_env() {
    let output = Command::new("cargo")
        .args(["run", "--", "current"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("No environment is currently active."));
}

/// Test show command (when no environment exists)
#[test]
fn test_cli_show_no_env() {
    let output = Command::new("cargo")
        .args(["run", "--", "show", "nonexistent"])
        .output()
        .expect("Failed to execute command");

    // show command should fail when environment is not specified
    assert!(!output.status.success());

    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("not found"));
}

/// Test invalid command
#[test]
fn test_cli_invalid_command() {
    let output = Command::new("cargo")
        .args(["run", "--", "invalid-command"])
        .output()
        .expect("Failed to execute command");

    // invalid command should fail
    assert!(!output.status.success());

    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("error:"));
}

/// Test add command argument validation
#[test]
fn test_cli_add_missing_args() {
    let output = Command::new("cargo")
        .args(["run", "--", "add"])
        .output()
        .expect("Failed to execute command");

    // missing required arguments should fail
    assert!(!output.status.success());

    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("error:"));
}

/// Test switch command argument validation
#[test]
fn test_cli_switch_missing_name() {
    let output = Command::new("cargo")
        .args(["run", "--", "switch"])
        .output()
        .expect("Failed to execute command");

    // missing required arguments should fail
    assert!(!output.status.success());

    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("error:"));
}

/// Test remove command argument validation
#[test]
fn test_cli_remove_missing_args() {
    let output = Command::new("cargo")
        .args(["run", "--", "remove"])
        .output()
        .expect("Failed to execute command");

    // missing required arguments should fail
    assert!(!output.status.success());

    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("error:"));
}

/// Test add-entry command argument validation
#[test]
fn test_cli_add_entry_missing_args() {
    let output = Command::new("cargo")
        .args(["run", "--", "add-entry"])
        .output()
        .expect("Failed to execute command");

    // missing required arguments should fail
    assert!(!output.status.success());

    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("error:"));
}

/// Test remove-entry command argument validation
#[test]
fn test_cli_remove_entry_missing_args() {
    let output = Command::new("cargo")
        .args(["run", "--", "remove-entry"])
        .output()
        .expect("Failed to execute command");

    // missing required arguments should fail
    assert!(!output.status.success());

    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("error:"));
}
