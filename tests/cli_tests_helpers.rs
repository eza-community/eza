use std::fs::{self, File, FileTimes};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

pub struct TransientDirectory {
    path: PathBuf,
}

impl TransientDirectory {
    pub fn create(platform: &str, group: &str) -> Self {
        let path_str = format!("tests/data/{platform}/{group}");
        let path = PathBuf::from(&path_str);
        let _ = fs::remove_dir_all(&path_str);
        fs::create_dir_all(&path).unwrap();
        TransientDirectory { path }
    }

    pub fn create_file<P: AsRef<Path> + std::fmt::Debug>(&self, file_name: P) -> File {
        let file = File::create(self.path.join(file_name)).unwrap();

        let times = FileTimes::new()
            .set_accessed(SystemTime::UNIX_EPOCH)
            .set_modified(SystemTime::UNIX_EPOCH);

        file.set_times(times).unwrap();
        file
    }

    pub fn create_files(&self, files: &[&str]) {
        for file_name in files {
            self.create_file(file_name);
        }
    }

    pub fn create_dirs(&self, dirs: &[&str]) {
        for dir_name in dirs {
            fs::create_dir(self.path.join(dir_name)).unwrap();
        }
    }

    #[allow(dead_code)]
    pub fn run(&self, command: &str, args: &[&str]) {
        Command::new(command)
            .args(args)
            .current_dir(&self.path)
            .output()
            .unwrap();
    }

    #[cfg(unix)]
    pub fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(&self, source: P, target: Q) {
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
