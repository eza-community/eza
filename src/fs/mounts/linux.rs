// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use crate::fs::mounts::{Error, MountedFs};
use proc_mounts::MountList;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

/// Get a list of all mounted filesystems
pub fn mounts() -> Result<Vec<MountedFs>, Error> {
    // Try to read from `/proc/self/mountinfo` first
    if let Ok(mounts) = read_mountinfo() {
        return Ok(mounts);
    }

    // Fall back to `/proc/mounts` if `/proc/self/mountinfo` doesn't exist
    Ok(MountList::new()
        .map_err(Error::IOError)?
        .0
        .iter()
        .map(|mount| MountedFs {
            dest: mount.dest.clone(),
            fstype: mount.fstype.clone(),
            source: mount.source.to_string_lossy().into(),
            root: None,
        })
        .collect())
}

/// Read and parse the /proc/self/mountinfo file
fn read_mountinfo() -> Result<Vec<MountedFs>, Error> {
    let file = File::open("/proc/self/mountinfo").map_err(Error::IOError)?;
    let reader = BufReader::new(file);
    let mut mounts = Vec::new();

    for line in reader.lines() {
        let line = line.map_err(Error::IOError)?;
        if let Some(mount) = parse_mountinfo_line(&line) {
            mounts.push(mount);
        }
    }

    Ok(mounts)
}

/// Parse a line from the /proc/self/mountinfo file
fn parse_mountinfo_line(line: &str) -> Option<MountedFs> {
    // https://man7.org/linux/man-pages/man5/proc_pid_mountinfo.5.html
    // Format: 36 35 98:0 /mnt1 /mnt2 rw,noatime master:1 - ext3 /dev/root rw,errors=continue
    // Fields: (1)(2)(3)   (4)   (5)      (6)      (7)   (8) (9)   (10)         (11)
    // We need: (4) root, (5) mount point, (9) fstype, (10) source

    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 10 {
        return None;
    }

    // Find the separator "-" which marks the end of optional fields
    let separator_pos = parts.iter().position(|&p| p == "-")?;
    if separator_pos + 3 > parts.len() {
        return None;
    }

    let root = parts[3].to_string();
    let dest = PathBuf::from(parts[4]);
    let fstype = parts[separator_pos + 1].to_string();
    let source = parts[separator_pos + 2].to_string();

    Some(MountedFs {
        dest,
        fstype,
        source,
        root: Some(root),
    })
}
