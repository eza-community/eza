#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "linux")]
pub use linux::mounts;
#[cfg(target_os = "macos")]
pub use macos::mounts;

/// Details of a mounted filesystem.
#[derive(Clone)]
pub struct MountedFs {
    pub dest: std::path::PathBuf,
    pub fstype: String,
    pub source: String,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    #[cfg(target_os = "macos")]
    GetFSStatError(i32),
    #[cfg(target_os = "linux")]
    IOError(std::io::Error)
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(target_os = "macos")]
            Error::GetFSStatError(err) => write!(f, "getfsstat failed: {err}"),
            #[cfg(target_os = "linux")]
            Error::IOError(err) => write!(f, "failed to read /proc/mounts: {err}")
        }
    }
}