mod cli_tests_helpers;

use cli_tests_helpers::{AllocateFileSize, TestDirectory};

#[test]
fn cli_tests_any_basic() {
    let test_dir = TestDirectory::new("any", "basic");

    test_dir.create_files(&["file.txt"]);
    test_dir.create_dirs(&["dir"]);

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

    for i in 1..13 {
        let mut f = test_dir.create_file(format!("{}_bytes", i));
        f.write_sized(i);
        let mut f = test_dir.create_file(format!("{}_Kib", i));
        f.write_sized(i * 1024);
        let mut f = test_dir.create_file(format!("{}_MiB", i));
        f.write_sized(i * 1024 * 1024);
    }

    test_dir.run_tests();
}
