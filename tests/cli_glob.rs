#![cfg(windows)]

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::File;
use tempfile::TempDir;

#[test]
fn cli_expands_wildcard_on_windows() {
    // Create a temporary directory with two files.
    let td = TempDir::new().expect("tempdir");
    File::create(td.path().join("one.txt")).expect("create one.txt");
    File::create(td.path().join("two.txt")).expect("create two.txt");

    // Pass a literal wildcard pattern as a single argument (no shell expansion).
    let pattern = format!("{}\\*.txt", td.path().display());

    // Run the built binary and assert both filenames appear in the output.
    let mut cmd = Command::cargo_bin("eza").expect("binary built as `eza`");
    cmd.arg(&pattern);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("one.txt").and(predicate::str::contains("two.txt")));
}
