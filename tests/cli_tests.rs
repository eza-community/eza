#[test]
fn cli_all_tests() {
    trycmd::TestCases::new().case("tests/cmd/*_all.toml");
}

#[test]
#[cfg(unix)]
fn cli_unix_tests() {
    trycmd::TestCases::new().case("tests/cmd/*_unix.toml");
}

#[test]
#[cfg(windows)]
fn cli_windows_tests() {
    trycmd::TestCases::new().case("tests/cmd/*_windows.toml");
}

#[test]
#[cfg(feature = "nix-local")]
fn cli_nix_local_tests() {
    trycmd::TestCases::new().case("tests/cmd/*_nix_local.toml");
}

#[test]
#[cfg(feature = "powertest")]
fn cli_powertest_tests() {
    trycmd::TestCases::new().case("tests/ptests/*.toml");
}

#[test]
#[cfg(feature = "nix")]
fn cli_nix_generated_tests() {
    trycmd::TestCases::new().case("tests/gen/*.toml");
}

#[test]
#[cfg(unix)]
fn recurse_level_with_absolute_path() {
    // --level should count depth from the given directory, not from the
    // filesystem root, so an absolute path recurses as deep as a relative one.
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/itest/vagrant");
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_eza"))
        .arg(&dir)
        .args(["--recurse", "--level=2"])
        .stdin(std::process::Stdio::null())
        .output()
        .expect("failed to run eza");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    let headers: Vec<&str> = stdout.lines().filter(|l| l.ends_with(':')).collect();
    let expected: Vec<String> = ["debug", "dev", "log"]
        .iter()
        .map(|d| format!("{}:", dir.join(d).display()))
        .collect();
    assert_eq!(headers, expected);
}
