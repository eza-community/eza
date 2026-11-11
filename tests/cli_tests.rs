#[test]
fn cli_all_tests() {
    trycmd::TestCases::new().case("tests/cmd/*_all.toml");
}

#[test]
#[cfg(unix)]
fn cli_unix_tests() {
    trycmd::TestCases::new().case("tests/cmd/*_unix.toml");
}

/// Makes a directory unreadable to the current user; access is restored on drop.
struct DeniedDir(std::path::PathBuf);

impl DeniedDir {
    fn deny(path: &std::path::Path) -> Self {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o000)).unwrap();
        }
        #[cfg(windows)]
        {
            let user = std::env::var("USERNAME").unwrap();
            let status = std::process::Command::new("icacls")
                .arg(path)
                .arg("/deny")
                .arg(format!("{user}:(R)"))
                .status()
                .unwrap();
            assert!(status.success(), "icacls /deny failed");
        }
        Self(path.to_path_buf())
    }
}

impl Drop for DeniedDir {
    fn drop(&mut self) {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(&self.0, std::fs::Permissions::from_mode(0o755));
        }
        #[cfg(windows)]
        {
            let user = std::env::var("USERNAME").unwrap_or_default();
            let _ = std::process::Command::new("icacls")
                .arg(&self.0)
                .arg("/remove:d")
                .arg(user)
                .status();
        }
    }
}

#[test]
fn permission_denied_exits_13() {
    let base = std::env::temp_dir().join(format!("eza-perm-denied-{}", std::process::id()));
    let denied = base.join("denied");
    std::fs::create_dir_all(&denied).unwrap();

    {
        let _restore = DeniedDir::deny(&denied);

        // Privileged users (e.g. root) can still read the directory; there is no denial to test then.
        if std::fs::read_dir(&denied).is_err() {
            let output = std::process::Command::new(env!("CARGO_BIN_EXE_eza"))
                .arg(&denied)
                .output()
                .unwrap();
            assert_eq!(
                output.status.code(),
                Some(13),
                "expected exit code 13 on permission denied, got {:?}",
                output.status.code()
            );
        }
    }

    std::fs::remove_dir_all(&base).unwrap();
}

#[test]
fn permission_denied_recursive_exits_13() {
    let base = std::env::temp_dir().join(format!("eza-perm-denied-rec-{}", std::process::id()));
    let denied = base.join("denied");
    std::fs::create_dir_all(&denied).unwrap();

    {
        let _restore = DeniedDir::deny(&denied);

        // Privileged users (e.g. root) can still read the directory; there is no denial to test then.
        if std::fs::read_dir(&denied).is_err() {
            let output = std::process::Command::new(env!("CARGO_BIN_EXE_eza"))
                .arg("-R")
                .arg(&base)
                .output()
                .unwrap();
            assert_eq!(
                output.status.code(),
                Some(13),
                "expected exit code 13 on permission denied under -R, got {:?}",
                output.status.code()
            );
        }
    }

    std::fs::remove_dir_all(&base).unwrap();
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
