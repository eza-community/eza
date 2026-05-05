#![cfg(target_os = "linux")]

mod cli_tests_helpers;

use cli_tests_helpers::TestDirectory;

use std::fs::FileTimes;
use std::time::SystemTime;

use crate::cli_tests_helpers::AllocateFileSize;

#[test]
fn cli_tests_linux_date() {
    let test_dir = TestDirectory::new("linux", "date");

    use std::thread;
    use std::time::Duration;

    use chrono::{Local, TimeZone};

    let old_date: SystemTime = Local.with_ymd_and_hms(2003, 3, 3, 0, 0, 0).unwrap().into();
    let med_date: SystemTime = Local
        .with_ymd_and_hms(2006, 6, 15, 23, 14, 29)
        .unwrap()
        .into();
    let new_date: SystemTime = Local
        .with_ymd_and_hms(2009, 12, 22, 10, 38, 53)
        .unwrap()
        .into();

    // Sleep between each create as we can not modified the created time
    let peach = test_dir.create_file("peach");
    let peach_times = FileTimes::new()
        .set_modified(med_date)
        .set_accessed(new_date);
    peach.set_times(peach_times).unwrap();

    thread::sleep(Duration::from_millis(100));

    let plum = test_dir.create_file("plum");
    let plum_times = FileTimes::new()
        .set_modified(new_date)
        .set_accessed(old_date);
    plum.set_times(plum_times).unwrap();

    thread::sleep(Duration::from_millis(100));

    let pear = test_dir.create_file("pear");
    let pear_times = FileTimes::new()
        .set_modified(old_date)
        .set_accessed(med_date);
    pear.set_times(pear_times).unwrap();
}

#[test]
fn cli_tests_linux_date_current_year() {
    let test_dir = TestDirectory::new("linux", "date_current_year");

    use chrono::{Datelike, Local, TimeZone};

    let current_year = Local::now().year();

    let old_date: SystemTime = Local
        .with_ymd_and_hms(current_year, 3, 3, 0, 0, 0)
        .unwrap()
        .into();
    let med_date: SystemTime = Local
        .with_ymd_and_hms(current_year, 6, 15, 23, 14, 29)
        .unwrap()
        .into();
    let new_date: SystemTime = Local
        .with_ymd_and_hms(current_year, 12, 22, 10, 38, 53)
        .unwrap()
        .into();

    // Sleep between each create as we can not modified the created time
    let peach = test_dir.create_file("peach");
    let peach_times = FileTimes::new().set_modified(med_date);
    peach.set_times(peach_times).unwrap();

    let plum = test_dir.create_file("plum");
    let plum_times = FileTimes::new().set_modified(new_date);
    plum.set_times(plum_times).unwrap();

    let pear = test_dir.create_file("pear");
    let pear_times = FileTimes::new().set_modified(old_date);
    pear.set_times(pear_times).unwrap();
}

#[test]
fn cli_tests_linux_groups() {
    let test_dir = TestDirectory::new("linux", "groups");

    test_dir.create_files(&["eza_test", "eza_group", "eza_group2"]);

    test_dir.chown("eza_test", None, Some(5677));
    test_dir.chown("eza_group", None, Some(5678));
    test_dir.chown("eza_group2", None, Some(5679));
}

#[test]
fn cli_tests_linux_size() {
    let test_dir = TestDirectory::new("linux", "size");

    for i in 9..11 {
        test_dir.create_file(format!("{i}bytes")).fill(i);
        test_dir.create_file(format!("{i}Kib")).fill(i * 1024);
        test_dir.create_file(format!("{i}Kb")).fill(i * 1000);
        test_dir
            .create_file(format!("{i}MiB"))
            .fill(i * 1024 * 1024);
        test_dir.create_file(format!("{i}MB")).fill(i * 1000 * 1000);
    }
}

#[test]
fn cli_tests_linux_views() {
    let test_dir = TestDirectory::new("linux", "views");

    test_dir.create_dirs(&["dir1", "dir2", "dir2/subdir"]);

    test_dir.create_files(
        (1..=16u32)
            .map(|n: u32| format!("file{n}"))
            .collect::<Vec<_>>()
            .as_ref(),
    );

    test_dir.create_files(&[
        "dir2/file1",
        "dir2/file2",
        "dir2/subdir/file1",
        "dir2/subdir/file2",
    ]);

    test_dir.symlink("dir2", "symlink-dir");
    test_dir.symlink("file16", "symlink-file");
    test_dir.symlink("file17", "symlink-file-broken");

}

#[test]
fn cli_tests_linux_weird_filenames() {
    use std::ffi::OsStr;

    use nu_ansi_term::Color;

    let test_dir = TestDirectory::new("linux", "weird_filenames");

    test_dir.create_dirs(&["new-line-dir: [\n]"]);

    let mut ansi_string = "ansi: ".to_owned();
    ansi_string.push_str(Color::Blue.paint("blue").as_str());

    test_dir.create_files(&[
        "ascii: hello",
        "emoji: [🆒]",
        "utf-8: pâté",
        "bell: \u{07}",
        "backspace: \u{08}",
        "tab: \t",
        "vertical tab: \u{0b}",
        "form-feed: \u{0c}",
        "carriage return: \r",
        "escape: \u{1b}",
        "new-line: [\n]",
        &ansi_string,
    ]);

    test_dir.symlink("new-line: [\n]", "new-line: [\n]-symlink");
    test_dir.symlink("broken-target: [\n]", "broken-target: [\n]-symlink");

    let mut invalid_utf8_1 = "invalid-utf8-1: ".as_bytes().to_vec();
    invalid_utf8_1.extend_from_slice(&[0xff]);
    let mut invalid_utf8_2 = "invalid-utf8-2: ".as_bytes().to_vec();
    invalid_utf8_2.extend_from_slice(&[0xc3, 0x28]);
    let mut invalid_utf8_3 = "invalid-utf8-3: ".as_bytes().to_vec();
    invalid_utf8_3.extend_from_slice(&[0xe2, 0x82, 0x28]);
    let mut invalid_utf8_4 = "invalid-utf8-4: ".as_bytes().to_vec();
    invalid_utf8_4.extend_from_slice(&[0xf0, 0x28, 0x8c, 0x28]);

    unsafe {
        test_dir.create_files(&[
            OsStr::from_encoded_bytes_unchecked(&invalid_utf8_1),
            OsStr::from_encoded_bytes_unchecked(&invalid_utf8_2),
            OsStr::from_encoded_bytes_unchecked(&invalid_utf8_3),
            OsStr::from_encoded_bytes_unchecked(&invalid_utf8_4),
        ]);
    }
}

#[test]
fn cli_tests_linux_xattr() {
    let test_dir = TestDirectory::new("linux", "xattr");

    test_dir.create_files(&["file_no_attributes", "file_attributes", "file_selinux"]);
    test_dir.create_dirs(&["dir_no_attributes", "dir_attributes", "dir_selinux"]);

    // command from package `attr`
    test_dir.run(
        "setfattr",
        &[
            "--name=user.xdg.tags",
            "--value=foo,bar,baz",
            "file_attributes",
        ],
    );
    test_dir.run(
        "setfattr",
        &[
            "--name=user.xdg.tags",
            "--value=foo,bar,baz",
            "dir_attributes",
        ],
    );

    test_dir.run(
        "sudo",
        &[
            "setfattr",
            "--name=security.selinux",
            "--value=unconfined_u:object_r:user_home_t:s0",
            "file_selinux",
        ],
    );
    test_dir.run(
        "sudo",
        &[
            "setfattr",
            "--name=security.selinux",
            "--value=unconfined_u:object_r:user_home_t:s0",
            "dir_selinux",
        ],
    );
}
