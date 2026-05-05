#![cfg(unix)]

mod cli_tests_helpers;

use cli_tests_helpers::TestDirectory;

use crate::cli_tests_helpers::AllocateFileSize;

#[test]
fn cli_tests_unix_basic() {
    let test_dir = TestDirectory::new("unix", "basic");

    test_dir.create_files(&["file.txt"]);
    test_dir.create_dirs(&["dir"]);

    test_dir.run_tests();
}

#[cfg(feature = "git")]
#[test]
fn cli_tests_unix_git_repos() {
    let test_dir = TestDirectory::new("unix", "git_repos");

    let (repo1, repo2, repo3) = ("dir-git-repo1", "dir-git-repo2", "dir-git-repo3");
    test_dir.create_dirs(&[repo1, repo2, repo3]);

    // dir-git-repo1
    test_dir.run("git", &["init", repo1]);
    test_dir.configure_git(repo1);

    // dir-git-repo2
    test_dir.run("git", &["init", repo2, "--initial-branch", "main"]);
    test_dir.configure_git(repo2);
    test_dir.run(
        "git",
        &[
            "-C",
            repo2,
            "commit",
            "--message=\"initial commit\"",
            "--allow-empty",
        ],
    );

    // dir-git-repo3
    test_dir.run("git", &["init", repo3, "--initial-branch", "add-bépo"]);
    test_dir.configure_git(repo3);
    test_dir.run(
        "git",
        &[
            "-C",
            repo3,
            "commit",
            "--message=\"initial commit\"",
            "--allow-empty",
        ],
    );

    test_dir.run_tests();
}

#[cfg(feature = "git")]
#[test]
fn cli_tests_unix_git_status() {
    let test_dir = TestDirectory::new("unix", "git_status");

    use std::io::Write;

    test_dir.run("git", &["init", "."]);
    test_dir.configure_git(".");
    test_dir.create_dirs(&[
        "dir-empty",
        "dir-ignored",
        "dir-ignored-with-file-unmodified",
        "dir-modified",
        "dir-modified-staged",
        "dir-new",
        "dir-new-staged",
        "dir-new-staged-new",
    ]);

    test_dir.create_files(&[
        "dir-ignored/file_new",
        "dir-ignored-with-file-unmodified/file-unmodified",
        "dir-new-staged-new/file-new-staged",
        "dir-new/file-ignored",
        "dir-new/file-new",
        "file-ignored",
        "file-new",
        "file-new-staged",
    ]);
    let mut dir_modified = test_dir.create_file("dir-modified/file-modified");
    let mut dir_modified_staged = test_dir.create_file("dir-modified-staged/file-modified-staged");
    let mut modified = test_dir.create_file("file-modified");
    let mut modified_staged = test_dir.create_file("file-modified-staged");
    let mut modified_staged_modified = test_dir.create_file("file-modified-staged-modified");
    let mut new_staged_modified = test_dir.create_file("file-new-staged-modified");

    test_dir.symlink("file-modified", "symlink-new");
    test_dir.symlink("file-non-existing", "symlink-broken");

    let mut gitignore = test_dir.create_file(".gitignore");
    gitignore.write_all(b"dir-ignored*\nfile-ignored*").unwrap();

    test_dir.run(
        "git",
        &[
            "add",
            "dir-modified",
            "dir-modified-staged",
            "file-modified",
            "file-modified-staged",
            "file-modified-staged-modified",
        ],
    );
    test_dir.run(
        "git",
        &[
            "add",
            "-f",
            "dir-ignored-with-file-unmodified/file-unmodified",
        ],
    );
    test_dir.run("git", &["commit", "-m", "initial commit"]);

    modified.write_all(b"a").unwrap();
    modified_staged.write_all(b"a").unwrap();
    modified_staged_modified.write_all(b"a").unwrap();
    dir_modified.write_all(b"a").unwrap();
    dir_modified_staged.write_all(b"a").unwrap();

    test_dir.run(
        "git",
        &[
            "add",
            "dir-modified-staged",
            "dir-new-staged",
            "dir-new-staged-new",
            "file-modified-staged",
            "file-modified-staged-modified",
            "file-new-staged",
            "file-new-staged-modified",
            "dir-new-staged-new/file-new-staged",
        ],
    );

    test_dir.create_file("dir-new-staged-new/file-new");

    new_staged_modified.write_all(b"a").unwrap();
    modified_staged_modified.write_all(b"a").unwrap();

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
fn cli_tests_any_unix_dirs() {
    let test_dir = TestDirectory::new("unix", "size_dirs");

    let dirs = ["dir1", "dir2", "dir1/dir3"];
    test_dir.create_dirs(&dirs);

    for i in 9..11 {
        for dir in dirs {
            test_dir.create_file(format!("{dir}/{i}Kib")).fill(i * 1024);
        }
    }

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
