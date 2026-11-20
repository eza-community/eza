use std::process::Command;

#[test]
fn cli_all_tests() {
    trycmd::TestCases::new().case("tests/cmd/*_all.toml");
}

#[test]
#[cfg(unix)]
fn recurse_level_is_relative_to_target_directory() {
    let output = Command::new(env!("CARGO_BIN_EXE_eza"))
        .args(["tests/itest/vagrant", "--recurse", "--level=2", "--oneline"])
        .output()
        .expect("failed to run eza");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
    assert!(stdout.contains("tests/itest/vagrant/debug:"));
    assert!(stdout.contains("tests/itest/vagrant/dev:"));
    assert!(!stdout.contains("tests/itest/vagrant/log/run:"));
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
