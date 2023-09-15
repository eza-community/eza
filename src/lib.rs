#[macro_use]
extern crate lazy_static;

use crate::fs::mounts::MountedFs;

#[cfg(target_os = "linux")]
use proc_mounts::MountList;
use std::collections::HashMap;
use std::path::PathBuf;

// A lazily initialised static map of all mounted file systems.
//
// The map contains a mapping from the mounted directory path to the
// corresponding mount information. On Linux systems, this map is populated
// using the `proc-mounts` crate. If there's an error retrieving the mount
// list or if we're not running on Linux, the map will be empty.
//
// Initialise this at application start so we don't have to look the details
// up for every directory. Ideally this would only be done if the --mounts
// option is specified which will be significantly easier once the move
// to `clap` is complete.
lazy_static! {
    static ref ALL_MOUNTS: HashMap<PathBuf, MountedFs> = {
        #[cfg(target_os = "linux")]
        match MountList::new() {
            Ok(mount_list) => {
                let mut m = HashMap::new();
                mount_list.0.iter().for_each(|mount| {
                    m.insert(
                        mount.dest.clone(),
                        MountedFs {
                            dest: mount.dest.to_string_lossy().into_owned(),
                            fstype: mount.fstype.clone(),
                            source: mount.source.to_string_lossy().into(),
                        },
                    );
                });
                m
            }
            Err(_) => HashMap::new(),
        }
        #[cfg(not(target_os = "linux"))]
        HashMap::new()
    };
}

#[allow(unused)]
pub mod fs;
#[allow(unused)]
pub mod info;
#[allow(unused)]
pub mod logger;
#[allow(unused)]
pub mod options;
#[allow(unused)]
pub mod output;
#[allow(unused)]
pub mod theme;
