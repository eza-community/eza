use std::fs::{self, File, FileTimes};
use std::time::Duration;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::thread;

struct TransientDirectory {
    path: PathBuf
}

impl TransientDirectory {
    fn create(platform: &str, group: &str) -> Self {
        let path_str = format!("tests/data/{platform}/{group}");
        let path = PathBuf::from(&path_str);
        fs::create_dir_all(&path).unwrap();
        TransientDirectory { path }
    }

    fn create_file<P: AsRef<Path>>(&self, file_name: P) -> File {
        let file =  File::create(self.path.join(file_name)).unwrap();

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
            fs::create_dir(self.path.join(dir_name)).unwrap();
        }
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

    test_dir.create_files(&[
        "a.txt",
        "abc.mp3",
        "ab"
    ]);

    test_dir.create_dirs(&[
        "test",
        "abc",
        "01.city",
        "02.apple",
    ]);

    trycmd::TestCases::new().case("tests/cmd/any/sort/*.toml");
}

#[test]
#[cfg(feature = "git")]
fn cli_tests_any_git() {
    use std::process::Command;

    let test_dir = TransientDirectory::create("any", "git");
    Command::new("git").args(["init", test_dir.to_str().unwrap()]).output().unwrap();

    trycmd::TestCases::new().case("tests/cmd/any/git/*.toml");
}

#[test]
#[cfg(not(feature = "git"))]
fn cli_tests_any_no_git() {
    use std::process::Command;

    let test_dir = TransientDirectory::create("any", "no-git");
    Command::new("git").args(["init", test_dir.to_str().unwrap()]).output().unwrap();

    trycmd::TestCases::new().case("tests/cmd/any/no-git/*.toml");
}

#[test]
#[cfg(feature = "docker-tests")]
fn cli_tests_any_date() {
    use chrono::{TimeZone, Local};
    let test_dir = TransientDirectory::create("any", "dates");

    let old_date: SystemTime = Local.with_ymd_and_hms(2003, 3, 3, 0, 0, 0).unwrap().into();
    let med_date: SystemTime = Local.with_ymd_and_hms(2006, 6, 15, 23, 14, 29).unwrap().into();
    let new_date: SystemTime = Local.with_ymd_and_hms(2009, 12, 22, 10, 38, 53).unwrap().into();

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
