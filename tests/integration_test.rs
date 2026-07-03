//! Integration tests for the os-info command-line tool.
//! These tests verify that all command-line options work as expected across different platforms.

use std::process::Command;

/// Test the application without any command-line options.
/// It should return a success exit code and print non-empty output.
#[test]
fn test_no_options() {
    // env!("CARGO_BIN_EXE_os-info") provides the path to the compiled binary.
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_os-info"));
    let output = cmd.output().expect("failed to execute process");
    
    // Verify that the command executed successfully.
    assert!(output.status.success());
    
    // Ensure that some output was produced.
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.trim().is_empty());
    // Verify kernel information is present.
    assert!(stdout.contains("(kernel "));
}

/// Test the long help flag: --help.
/// It should print the NAME, SYNOPSIS, and DESCRIPTION sections.
#[test]
fn test_help_long() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_os-info"));
    cmd.arg("--help");
    let output = cmd.output().expect("failed to execute process");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Verify standard Unix-style help sections are present.
    assert!(stdout.contains("NAME"));
    assert!(stdout.contains("SYNOPSIS"));
    assert!(stdout.contains("DESCRIPTION"));
}

/// Test the short help flag: -h.
/// It should produce the same help output as the long flag.
#[test]
fn test_help_short() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_os-info"));
    cmd.arg("-h");
    let output = cmd.output().expect("failed to execute process");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("NAME"));
}

/// Test the --version flag.
/// It should print the program name followed by the version from Cargo.toml.
#[test]
fn test_version() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_os-info"));
    cmd.arg("--version");
    let output = cmd.output().expect("failed to execute process");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Check for the expected version format.
    assert!(stdout.starts_with("os-info "));
    // The version string should be the one defined in the crate's metadata.
    assert!(stdout.contains(env!("CARGO_PKG_VERSION")));
}

/// Test the long verbose flag: --verbose.
/// It should print detailed OS information on separate lines.
#[test]
fn test_verbose_long() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_os-info"));
    cmd.arg("--verbose");
    let output = cmd.output().expect("failed to execute process");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    // In verbose mode, these labels should be present regardless of the platform.
    assert!(stdout.contains("Type:"));
    assert!(stdout.contains("Version:"));
    assert!(stdout.contains("Kernel Version:"));
}

/// Test the short verbose flag: -v.
/// It should produce the same detailed output as the long flag.
#[test]
fn test_verbose_short() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_os-info"));
    cmd.arg("-v");
    let output = cmd.output().expect("failed to execute process");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Type:"));
    assert!(stdout.contains("Version:"));
    assert!(stdout.contains("Kernel Version:"));
}
