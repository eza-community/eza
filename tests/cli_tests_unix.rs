#![cfg(unix)]

mod cli_tests_helpers;

use cli_tests_helpers::TestDirectory;

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
