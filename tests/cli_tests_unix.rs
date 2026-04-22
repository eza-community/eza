mod cli_tests_helpers;

use cli_tests_helpers::TestDirectory;

#[test]
#[cfg(unix)]
// This test needs locales en_US, fr_FR and ja_JP.
fn cli_tests_unix_date() {
    use std::fs::FileTimes;
    use std::thread;
    use std::time::{Duration, SystemTime};

    use chrono::{Local, TimeZone};

    let test_dir = TestDirectory::create("unix", "date");

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
    thread::sleep(Duration::from_millis(100));
    let peach_times = FileTimes::new()
        .set_modified(med_date)
        .set_accessed(new_date);
    peach.set_times(peach_times).unwrap();

    let plum = test_dir.create_file("plum");
    thread::sleep(Duration::from_millis(100));
    let plum_times = FileTimes::new()
        .set_modified(new_date)
        .set_accessed(old_date);
    plum.set_times(plum_times).unwrap();

    let pear = test_dir.create_file("pear");
    let pear_times = FileTimes::new()
        .set_modified(old_date)
        .set_accessed(med_date);
    pear.set_times(pear_times).unwrap();

    test_dir.run_tests();
}

#[test]
#[cfg(unix)]
#[cfg(feature = "git")]
fn cli_tests_unix_git() {
    use std::io::Write;

    let test_dir = TestDirectory::create("unix", "git");
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
#[cfg(unix)]
fn cli_tests_unix_xattr() {
    let test_dir = TestDirectory::create("unix", "xattr");

    test_dir.create_files(&["file_no_attributes", "file_attributes"]);
    test_dir.create_dirs(&["dir_no_attributes", "dir_attributes"]);

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

    test_dir.run_tests();
}
