// SPDX-FileCopyrightText: 2024 Christina Sørensen, eza contributors
// SPDX-License-Identifier: EUPL-1.2

#![cfg(unix)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

fn restore_and_cleanup(dir: &std::path::Path, child: &std::path::Path) {
    let _ = fs::set_permissions(child, fs::Permissions::from_mode(0o755));
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn permission_denied_on_listed_path_exits_13() {
    let root = std::env::temp_dir().join(format!("eza-perm-denied-{}", std::process::id()));
    let denied = root.join("denied");
    fs::create_dir_all(&denied).expect("create denied dir");
    fs::set_permissions(&denied, fs::Permissions::from_mode(0o000)).expect("chmod 000");

    let output = Command::new(env!("CARGO_BIN_EXE_eza"))
        .arg(&denied)
        .output();

    restore_and_cleanup(&root, &denied);

    let output = output.expect("run eza");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        output.status.code(),
        Some(13),
        "expected exit 13 on permission denied, got {:?}; stderr={stderr}",
        output.status.code()
    );
}

#[test]
fn permission_denied_during_recurse_exits_13() {
    let root = std::env::temp_dir().join(format!("eza-perm-denied-r-{}", std::process::id()));
    let denied = root.join("denied");
    fs::create_dir_all(&denied).expect("create denied dir");
    fs::set_permissions(&denied, fs::Permissions::from_mode(0o000)).expect("chmod 000");

    let output = Command::new(env!("CARGO_BIN_EXE_eza"))
        .args(["-R"])
        .arg(&root)
        .output();

    restore_and_cleanup(&root, &denied);

    let output = output.expect("run eza");
    assert_eq!(
        output.status.code(),
        Some(13),
        "expected exit 13 on recursive permission denied, got {:?}; stderr={}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
}
