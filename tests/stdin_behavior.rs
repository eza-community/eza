use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn test_stdin_ignored_by_default_without_flag() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_eza"))
        .arg("--oneline")
        .arg("--color=never")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn eza");

    {
        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        stdin.write_all(b"non_existent_file_xyz_123\n").unwrap();
    }

    let output = child.wait_with_output().expect("Failed to read stdout");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Cargo.toml") || stdout.contains("src"));
    assert!(!stdout.contains("non_existent_file_xyz_123"));
}

#[test]
fn test_stdin_empty_input_without_flag_lists_current_dir() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_eza"))
        .arg("--oneline")
        .arg("--color=never")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn eza");

    {
        let _ = child.stdin.take();
    }

    let output = child.wait_with_output().expect("Failed to read stdout");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Cargo.toml") || stdout.contains("src"));
}

#[test]
fn test_stdin_explicit_flag_reads_paths() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_eza"))
        .arg("--stdin")
        .arg("--oneline")
        .arg("--color=never")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn eza");

    {
        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        stdin.write_all(b"src\nCargo.toml\n").unwrap();
    }

    let output = child.wait_with_output().expect("Failed to read stdout");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("src"));
    assert!(stdout.contains("Cargo.toml"));
}

#[test]
fn test_stdin_explicit_flag_with_empty_input_lists_nothing() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_eza"))
        .arg("--stdin")
        .arg("--oneline")
        .arg("--color=never")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn eza");

    {
        let _ = child.stdin.take();
    }

    let output = child.wait_with_output().expect("Failed to read stdout");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.trim().is_empty());
}
