use std::fs;
use std::path;
use std::time::SystemTime;

struct TransientDirectory {
    path: path::PathBuf
}

impl TransientDirectory {
    fn create(platform: &str, group: &str) -> Self {
        let path_str = format!("tests/data/{platform}/{group}");
        let path = path::PathBuf::from(&path_str);
        fs::create_dir_all(&path).unwrap();
        TransientDirectory { path }
    }

    fn create_files(&self, files: &[&str]) {
        for file_name in files {
            let file =  fs::File::create(self.path.join(file_name)).unwrap();
            file.set_modified(SystemTime::UNIX_EPOCH).unwrap();
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

impl AsRef<path::Path> for TransientDirectory {
    fn as_ref(&self) -> &path::Path {
        &self.path
    }
}

impl std::ops::Deref for TransientDirectory {
    type Target = path::PathBuf;

    fn deref(&self) -> &path::PathBuf {
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
