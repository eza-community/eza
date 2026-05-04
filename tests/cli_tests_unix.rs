#![cfg(unix)]

mod cli_tests_helpers;

use cli_tests_helpers::TestDirectory;

#[test]
fn cli_tests_unix_basic() {
    let test_dir = TestDirectory::new("unix", "basic");

    test_dir.create_files(&["file.txt"]);
    test_dir.create_dirs(&["dir"]);

    test_dir.run_tests();
}

#[test]
fn cli_tests_unix_links() {
    let test_dir = TestDirectory::new("unix", "links");

    test_dir.create_dirs(&[
        "dir",
        "dir_with_subdirs/",
        "dir_with_subdirs/dir1",
        "dir_with_subdirs/dir2",
        "dir_with_subdirs/dir3",
        "dir_with_subdirs/dir4",
        "dir_with_subdirs/dir5",
        "dir_with_subdirs/dir6",
        "dir_with_subdirs/dir7",
        "dir_with_subdirs/dir8",
        "dir_with_subdirs/dir9",
    ]);

    test_dir.create_files(
        (1..=3u32)
            .map(|n: u32| format!("file{n}"))
            .collect::<Vec<_>>()
            .as_ref(),
    );

    test_dir.hard_link("file2", "file2-link");

    test_dir.hard_link("file3", "file3-link1");
    test_dir.hard_link("file3", "file3-link2");

    test_dir.run_tests();
}

#[test]
fn cli_tests_unix_views() {
    let test_dir = TestDirectory::new("unix", "views");

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

    test_dir.run_tests();
}
