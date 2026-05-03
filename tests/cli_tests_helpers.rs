#![allow(dead_code)]

use std::env;
use std::fs::{self, File, FileTimes};
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs as unix_fs;
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

        // This code is just here to prevent missing tests because of an incorrect path
        let spec_path_str = spec_path.to_str().unwrap();
        let mut test_case_files = glob::glob(&format!("{spec_path_str}/*.toml")).unwrap();
        if test_case_files.next().is_none() {
            panic!("No test cases (TOML file) in {spec_path_str}.")
        }

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

            let mut dir = self.open_dir(dir_name);
            Self::set_time_to_epoch(&mut dir);
        }
    }

    #[cfg(windows)]
    pub fn open_dir<P: AsRef<Path>>(&self, dir_name: P) -> File {
        // Taken from the source of std::fs::set_times as it is
        // still nightly. Windows needs specific options to open
        // a directory, trying to open one with File::open result
        // in a permission error.
        use std::fs::OpenOptions;
        use std::os::windows::fs::OpenOptionsExt;
        use windows_sys::Win32::Storage::FileSystem::{
            FILE_FLAG_BACKUP_SEMANTICS, FILE_WRITE_ATTRIBUTES,
        };

        OpenOptions::new()
            .access_mode(FILE_WRITE_ATTRIBUTES)
            .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
            .open(self.data_path.join(dir_name))
            .unwrap()
    }

    #[cfg(unix)]
    pub fn open_dir<P: AsRef<Path>>(&self, dir_name: P) -> File {
        File::open(self.data_path.join(dir_name)).unwrap()
    }

    fn set_time_to_epoch(f: &mut File) {
        let times = FileTimes::new()
            .set_accessed(SystemTime::UNIX_EPOCH)
            .set_modified(SystemTime::UNIX_EPOCH);
        f.set_times(times).unwrap();
    }

    #[cfg(windows)]
    pub fn set_attributes<P: AsRef<Path>>(&self, file_name: P) {
        use windows_sys::Win32::Storage::FileSystem::{
            FILE_ATTRIBUTE_HIDDEN,
            SetFileAttributesA,
        };
        use windows_sys::Win32::Foundation::GetLastError;
        let p = self.data_path.join(file_name)
            .as_os_str()
            .as_encoded_bytes()
            .as_ptr()
            .cast();
        let res = unsafe { SetFileAttributesA(p, FILE_ATTRIBUTE_HIDDEN) };
        dbg!(res);
        if res == 0 {
            println!("{}", unsafe { GetLastError() });
        }
    }

    // Not currently used on Windows
    pub fn run(&self, command: &str, args: &[&str]) {
        Command::new(command)
            .args(args)
            .current_dir(&self.data_path)
            .output()
            .unwrap();
    }

    pub fn configure_git(&self, dir: &str) {
        self.run("git", &["-C", dir, "config", "user.email", "eza@eza.test"]);
        self.run("git", &["-C", dir, "config", "user.name", "eza"]);
    }

    #[cfg(unix)]
    pub fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(&self, original: P, link: Q) {
        unix_fs::symlink(original, self.data_path.join(link)).unwrap();
    }

    #[cfg(unix)]
    pub fn chown<P: AsRef<Path>>(&self, dir: P, uid: Option<u32>, gid: Option<u32>) {
        unix_fs::chown(self.data_path.join(dir), uid, gid).unwrap();
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
