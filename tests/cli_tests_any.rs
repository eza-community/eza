mod cli_tests_helpers;

use std::fs::FileTimes;
use std::time::{Duration, SystemTime};

use cli_tests_helpers::{AllocateFileSize, TestDirectory};

#[test]
fn cli_tests_any_basic() {
    let test_dir = TestDirectory::new("any", "basic");

    test_dir.create_files(&["file.txt"]);
    test_dir.create_dirs(&["dir"]);

    test_dir.run_tests();
}

#[test]
fn cli_tests_any_date_relative() {
    let test_dir = TestDirectory::new("any", "date_relative");

    let current_time = SystemTime::now();
    let old_date: SystemTime = current_time - Duration::new(3600, 0);
    let med_date: SystemTime = current_time - Duration::new(3600 * 24 * 45, 0);
    let new_date: SystemTime = current_time - Duration::new(3600 * 24 * 365 * 2, 0);

    let peach = test_dir.create_file("peach");
    let peach_times = FileTimes::new().set_modified(med_date);
    peach.set_times(peach_times).unwrap();

    let plum = test_dir.create_file("plum");
    let plum_times = FileTimes::new().set_modified(new_date);
    plum.set_times(plum_times).unwrap();

    let pear = test_dir.create_file("pear");
    let pear_times = FileTimes::new().set_modified(old_date);
    pear.set_times(pear_times).unwrap();

    test_dir.run_tests();
}

#[test]
fn cli_tests_any_dotfiles() {
    let test_dir = TestDirectory::new("any", "dotfiles");

    test_dir.create_files(&[".file"]);
    test_dir.create_dirs(&[".dir"]);

    test_dir.run_tests();
}

#[test]
fn cli_tests_any_file_exts() {
    let test_dir = TestDirectory::new("any", "file_exts");

    test_dir.create_files(&[
        "Makefile",
        "IMAGE.PNG",
        "image.svg",
        "VIDEO.AVI",
        "video.wmv",
        "music.mp3",
        "MUSIC.OGG",
        "lossless.flac",
        "lossless.wav",
        "crypto.asc",
        "crypto.signature",
        "document.pdf",
        "DOCUMENT.XLSX",
        "COMPRESSED.ZIP",
        "compressed.tar.gz",
        "compressed.tgz",
        "compressed.tar.xz",
        "compressed.txz",
        "compressed.deb",
        "backup~",
        "#SAVEFILE#",
        "file.tmp",
        "compiled.class",
        "compiled.o",
        "compiled.js",
        "compiled.coffee",
    ]);

    test_dir.run_tests();
}

#[test]
fn cli_tests_any_files_and_dirs() {
    let test_dir = TestDirectory::new("any", "files_and_dirs");

    test_dir.create_files(&["a.txt", "abc.mp3", "ab"]);
    test_dir.create_dirs(&["test", "abc", "01.city", "02.apple"]);

    test_dir.run_tests();
}

#[test]
#[cfg(not(feature = "git"))]
fn cli_tests_any_no_git() {
    let test_dir = TestDirectory::new("any", "no_git");
    test_dir.run_tests();
}

#[test]
fn cli_tests_any_size() {
    let test_dir = TestDirectory::new("any", "size");

    for i in 9..11 {
        test_dir.create_file(format!("{i}bytes")).fill(i);
        test_dir.create_file(format!("{i}Kib")).fill(i * 1024);
        test_dir.create_file(format!("{i}Kb")).fill(i * 1000);
        test_dir
            .create_file(format!("{i}MiB"))
            .fill(i * 1024 * 1024);
        test_dir.create_file(format!("{i}MB")).fill(i * 1000 * 1000);
    }

    test_dir.run_tests();
}
