#[cfg(unix)]
use std::fs::Permissions;
use std::fs::{self, File, FileTimes};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

struct TransientDirectory {
    path: PathBuf,
}

// We set up permissions to shield ourselves from an unusual umask
#[cfg(unix)]
const FILE_PERMISSIONS: u32 = 0o644;
#[cfg(unix)]
const DIR_PERMISSIONS: u32 = 0o755;

impl TransientDirectory {
    fn create(platform: &str, group: &str) -> Self {
        let path_str = format!("tests/data/{platform}/{group}");
        let path = PathBuf::from(&path_str);
        let _ = fs::remove_dir_all(&path_str);
        fs::create_dir_all(&path).unwrap();
        Self::set_dir_permissions(&path);
        TransientDirectory { path }
    }

    fn create_file<P: AsRef<Path> + std::fmt::Debug>(&self, file_name: P) -> File {
        let file = File::create(self.path.join(file_name)).unwrap();

        #[cfg(unix)]
        file.set_permissions(Permissions::from_mode(FILE_PERMISSIONS))
            .unwrap();

        let times = FileTimes::new()
            .set_accessed(SystemTime::UNIX_EPOCH)
            .set_modified(SystemTime::UNIX_EPOCH);

        file.set_times(times).unwrap();
        file
    }

    fn create_files(&self, files: &[&str]) {
        for file_name in files {
            self.create_file(file_name);
        }
    }

    fn create_dirs(&self, dirs: &[&str]) {
        for dir_name in dirs {
            let dir_path = self.path.join(dir_name);
            fs::create_dir(&dir_path).unwrap();
            Self::set_dir_permissions(&dir_path);
        }
    }

    fn set_dir_permissions<P: AsRef<Path>>(dir_path: P) {
        if cfg!(unix) {
            let dir = File::open(dir_path).unwrap();
            dir.set_permissions(Permissions::from_mode(DIR_PERMISSIONS))
                .unwrap();
        }
    }

    fn run(&self, command: &str, args: &[&str]) {
        Command::new(command)
            .args(args)
            .current_dir(&self.path)
            .output()
            .unwrap();
    }

    #[cfg(unix)]
    fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(&self, source: P, target: Q) {
        use std::os::unix::fs;

        fs::symlink(source, self.path.join(target)).unwrap();
    }
}

impl Drop for TransientDirectory {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.path).unwrap();
    }
}

impl AsRef<Path> for TransientDirectory {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

impl std::ops::Deref for TransientDirectory {
    type Target = PathBuf;

    fn deref(&self) -> &PathBuf {
        &self.path
    }
}

#[test]
fn cli_tests_any_basic() {
    let test_dir = TransientDirectory::create("any", "basic");
    test_dir.create_files(&["file.txt"]);
    test_dir.create_dirs(&["dir"]);

    trycmd::TestCases::new().case("tests/cmd/any/basic/*.toml");
}

#[test]
fn cli_tests_any_sort() {
    let test_dir = TransientDirectory::create("any", "sort");

    test_dir.create_files(&["a.txt", "abc.mp3", "ab"]);

    test_dir.create_dirs(&["test", "abc", "01.city", "02.apple"]);

    trycmd::TestCases::new().case("tests/cmd/any/sort/*.toml");
}

#[test]
#[cfg(unix)]
#[cfg(feature = "git")]
fn cli_tests_any_git() {
    use std::io::Write;

    let test_dir = TransientDirectory::create("unix", "git");
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

    trycmd::TestCases::new().case("tests/cmd/unix/git/*.toml");
}

#[test]
#[cfg(not(feature = "git"))]
fn cli_tests_any_no_git() {
    let test_dir = TransientDirectory::create("any", "no-git");
    test_dir.run("git", &["init", "."]);

    trycmd::TestCases::new().case("tests/cmd/any/no-git/*.toml");
}

#[test]
// Run in docker because it needs specific locales.
#[cfg(feature = "docker-tests")]
fn cli_tests_any_date() {
    use chrono::{Local, TimeZone};
    use std::thread;
    use std::time::Duration;

    let test_dir = TransientDirectory::create("any", "dates");

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

    trycmd::TestCases::new().case("tests/cmd/any/dates/*.toml");
}
