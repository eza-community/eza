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
fn recurse_level_relative_to_target_dir() {
    use std::fs;
    use std::process::Command;

    let base = std::env::temp_dir().join(format!("eza-recurse-level-{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    let target = base.join("nested").join("target");
    fs::create_dir_all(target.join("ABC").join("123").join("XYZ")).unwrap();
    fs::create_dir_all(target.join("ABC").join("456")).unwrap();
    fs::write(target.join("ABC").join("ABD.md"), "").unwrap();
    fs::write(
        target
            .join("ABC")
            .join("123")
            .join("XYZ")
            .join("ABC123XYZ.md"),
        "",
    )
    .unwrap();

    let run = |cwd: &std::path::Path, args: &[&str]| -> String {
        let out = Command::new(env!("CARGO_BIN_EXE_eza"))
            .current_dir(cwd)
            .args(args)
            .output()
            .expect("failed to run eza");
        assert!(
            out.status.success(),
            "stderr: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8(out.stdout).unwrap()
    };

    // Level 1: only immediate children of target (no recursion into ABC)
    let out_rel_1 = run(
        &base,
        &["nested/target", "--recurse", "--level=1", "--oneline"],
    );
    assert_eq!(out_rel_1, "ABC\n");

    let abs_target = target.to_str().unwrap();
    let out_abs_1 = run(&base, &[abs_target, "--recurse", "--level=1", "--oneline"]);
    assert_eq!(out_abs_1, "ABC\n");

    // Level 2: descends into ABC, stops before 123/456
    let out_rel_2 = run(
        &base,
        &["nested/target", "--recurse", "--level=2", "--oneline"],
    );
    assert!(out_rel_2.contains("123\n"));
    assert!(out_rel_2.contains("456\n"));
    assert!(out_rel_2.contains("ABD.md\n"));
    assert!(!out_rel_2.contains("XYZ"));

    let out_abs_2 = run(&base, &[abs_target, "--recurse", "--level=2", "--oneline"]);
    assert!(out_abs_2.contains("123\n"));
    assert!(out_abs_2.contains("ABD.md\n"));
    assert!(!out_abs_2.contains("XYZ"));

    // Level 3: descends into 123 and 456, stops before XYZ
    let out_rel_3 = run(
        &base,
        &["nested/target", "--recurse", "--level=3", "--oneline"],
    );
    assert!(out_rel_3.contains("XYZ\n"));
    assert!(!out_rel_3.contains("ABC123XYZ.md"));

    // Level 4: descends into XYZ
    let out_rel_4 = run(
        &base,
        &["nested/target", "--recurse", "--level=4", "--oneline"],
    );
    assert!(out_rel_4.contains("ABC123XYZ.md\n"));

    let _ = fs::remove_dir_all(&base);
}
