#![allow(dead_code)]

use std::env;
use std::fs::{self, File, FileTimes};
use std::io::Write;
use std::path::{self, Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

pub struct TestDirectory {
    data_path: PathBuf,
    spec_path: PathBuf,
    initial_dir_path: PathBuf,
}

const TEST_DATA_DIR: &str = "tests/data";
const TEST_SPEC_DIR: &str = "tests/cmd";

impl TestDirectory {
    pub fn new(platform: &str, group: &str) -> Self {
        let data_path_str = format!("{TEST_DATA_DIR}/{platform}/{group}");
        let spec_path_str = format!("{TEST_SPEC_DIR}/{platform}/{group}");

        let data_path = path::absolute(&data_path_str).unwrap();
        let spec_path = path::absolute(&spec_path_str).unwrap();

        let _ = fs::remove_dir_all(&data_path_str);
        fs::create_dir_all(&data_path).unwrap();

        TestDirectory {
            data_path,
            spec_path,
            initial_dir_path: env::current_dir().unwrap(),
        }
    }

    pub fn create_file<P: AsRef<Path>>(&self, file_name: P) -> File {
        let mut file = File::create(self.data_path.join(file_name)).unwrap();
        Self::set_time_to_epoch(&mut file);
        file
    }

    pub fn create_files<P: AsRef<Path>>(&self, files: &[P]) {
        for file_name in files {
            self.create_file(file_name);
        }
    }

    pub fn create_dirs<P: AsRef<Path>>(&self, dirs: &[P]) {
        for dir_name in dirs {
            fs::create_dir(self.data_path.join(dir_name)).unwrap();
            let mut dir = File::open(self.data_path.join(dir_name)).unwrap();
            Self::set_time_to_epoch(&mut dir);
        }
    }

    fn set_time_to_epoch(f: &mut File) {
        let times = FileTimes::new()
            .set_accessed(SystemTime::UNIX_EPOCH)
            .set_modified(SystemTime::UNIX_EPOCH);
        f.set_times(times).unwrap();
    }

    // Not currently used on Windows
    pub fn run(&self, command: &str, args: &[&str]) {
        Command::new(command)
            .args(args)
            .current_dir(&self.data_path)
            .output()
            .unwrap();
    }

    #[cfg(unix)]
    pub fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(&self, source: P, target: Q) {
        use std::os::unix::fs;

        fs::symlink(source, self.data_path.join(target)).unwrap();
    }

    pub fn run_tests(&self) {
        let spec_path = self.spec_path.to_str().unwrap();

        // This is not thread safe!!!
        // This will change the CWD of the whole process,
        // so the tests must be run in single thread mode.
        env::set_current_dir(&self.data_path).unwrap();
        trycmd::TestCases::new()
            .case(format!("{spec_path}/*.toml"))
            .default_bin_name("eza")
            .run();
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        env::set_current_dir(&self.initial_dir_path).unwrap();
    }
}

impl AsRef<Path> for TestDirectory {
    fn as_ref(&self) -> &Path {
        &self.data_path
    }
}

impl std::ops::Deref for TestDirectory {
    type Target = PathBuf;

    fn deref(&self) -> &PathBuf {
        &self.data_path
    }
}

pub trait AllocateFileSize {
    fn write_sized(&mut self, size: usize);
}

impl AllocateFileSize for File {
    fn write_sized(&mut self, size: usize) {
        // Naive implementation, works fine for small files
        let buf = vec![0; size];
        self.write(buf.as_slice()).unwrap();
    }
}
