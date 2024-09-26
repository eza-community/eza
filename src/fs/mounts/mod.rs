// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::OnceLock;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "linux")]
use linux::mounts;
#[cfg(target_os = "macos")]
use macos::mounts;

/// Details of a mounted filesystem.
#[derive(Clone)]
pub struct MountedFs {
    pub dest: PathBuf,
    pub fstype: String,
    pub source: String,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    #[cfg(target_os = "macos")]
    GetFSStatError(i32),
    #[cfg(target_os = "linux")]
    IOError(std::io::Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Allow unreachable_patterns for windows build
        #[allow(unreachable_patterns)]
        match self {
            #[cfg(target_os = "macos")]
            Error::GetFSStatError(err) => write!(f, "getfsstat failed: {err}"),
            #[cfg(target_os = "linux")]
            Error::IOError(err) => write!(f, "failed to read /proc/mounts: {err}"),
            _ => write!(f, "Unknown error"),
        }
    }
}

// A lazily initialised static map of all mounted file systems.
//
// The map contains a mapping from the mounted directory path to the
// corresponding mount information. If there's an error retrieving the mount
// list or if we're not running on Linux or Mac, the map will be empty.
//
// Initialise this at application start so we don't have to look the details
// up for every directory. Ideally this would only be done if the --mounts
// option is specified which will be significantly easier once the move
// to `clap` is complete.
pub(super) fn all_mounts() -> &'static HashMap<PathBuf, MountedFs> {
    static ALL_MOUNTS: OnceLock<HashMap<PathBuf, MountedFs>> = OnceLock::new();

    ALL_MOUNTS.get_or_init(|| {
        // Allow unused_mut for windows build
        #[allow(unused_mut)]
        let mut mount_map: HashMap<PathBuf, MountedFs> = HashMap::new();

        #[cfg(any(target_os = "linux", target_os = "macos"))]
        if let Ok(mounts) = mounts() {
            for mount in mounts {
                mount_map.insert(mount.dest.clone(), mount);
            }
        }

        mount_map
    })
}
