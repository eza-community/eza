// SPDX-FileCopyrightText: 2026 eza contributors
//
// SPDX-License-Identifier: EUPL-1.2

#[cfg(unix)]
mod tests {
    use std::fs;
    use std::process::{Command, Stdio};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir() -> std::path::PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("eza-stdin-dev-null-{unique}"));
        fs::create_dir(&path).expect("should create temp directory");
        path
    }

    #[test]
    fn defaults_to_current_directory_when_stdin_is_empty_and_not_explicitly_requested() {
        let path = temp_dir();
        fs::create_dir(path.join("alpha")).expect("should create alpha directory");
        fs::write(path.join("file.txt"), "").expect("should create file");

        let output = Command::new(env!("CARGO_BIN_EXE_eza"))
            .args(["--oneline", "--icons=never", "--color=never"])
            .current_dir(&path)
            .stdin(Stdio::null())
            .output()
            .expect("eza should run");

        fs::remove_dir_all(&path).expect("should remove temp directory");

        assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
        assert_eq!(String::from_utf8_lossy(&output.stdout), "alpha\nfile.txt\n");
    }

    #[test]
    fn explicit_stdin_still_uses_stdin_even_when_it_is_empty() {
        let path = temp_dir();
        fs::create_dir(path.join("alpha")).expect("should create alpha directory");
        fs::write(path.join("file.txt"), "").expect("should create file");

        let output = Command::new(env!("CARGO_BIN_EXE_eza"))
            .args(["--stdin", "--oneline", "--icons=never", "--color=never"])
            .current_dir(&path)
            .stdin(Stdio::null())
            .output()
            .expect("eza should run");

        fs::remove_dir_all(&path).expect("should remove temp directory");

        assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
        assert!(output.stdout.is_empty(), "stdout: {}", String::from_utf8_lossy(&output.stdout));
    }
}
