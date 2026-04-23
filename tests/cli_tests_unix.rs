#![cfg(unix)]

mod cli_tests_helpers;

use cli_tests_helpers::TestDirectory;

#[test]
#[cfg(feature = "git")]
fn cli_tests_unix_git_status() {
    use std::io::Write;

    let test_dir = TestDirectory::new("unix", "git_status");
    test_dir.run("git", &["init", "."]);
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

    test_dir.run("git", &["config", "user.name", "Eza"]);
    test_dir.run("git", &["config", "user.email", "eza@eza.test"]);

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
#[cfg(feature = "git")]
fn cli_tests_unix_git_repos() {
    let test_dir = TestDirectory::new("unix", "git_repos");
    test_dir.create_dirs(&["dir-git-repo"]);

    test_dir.run("git", &["init", "dir-git-repo"]);

    test_dir.run_tests();
}
