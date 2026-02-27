// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use crate::fs::mounts::{Error, MountedFs};
use proc_mounts::MountList;

/// Get a list of all mounted filesystems
pub fn mounts() -> Result<Vec<MountedFs>, Error> {
    Ok(MountList::new()
        .map_err(Error::IOError)?
        .0
        .iter()
        .map(|mount| MountedFs {
            dest: mount.dest.clone(),
            fstype: mount.fstype.clone(),
            source: mount.source.to_string_lossy().into(),
        })
        .collect())
}
