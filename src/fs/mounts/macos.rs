// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use crate::fs::mounts::{Error, MountedFs};
use libc::{__error, getfsstat, statfs, MNT_NOWAIT};
use std::ffi::{CStr, OsStr};
use std::os::raw::{c_char, c_int};
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;
use std::{mem, ptr};

/// Get a list of all mounted filesystem
pub fn mounts() -> Result<Vec<MountedFs>, Error> {
    // SAFETY:
    // Calling external "C" function getfsstat.  Passing a null pointer and zero
    // bufsize will return the number of mounts.
    let mut count: i32 = unsafe { getfsstat(ptr::null_mut(), 0, MNT_NOWAIT) };
    let mut mntbuf = Vec::<statfs>::new();
    if count > 0 {
        // SAFETY: Zero out buffer memory as we allocate.
        mntbuf.resize_with(count as usize, || unsafe { mem::zeroed() });
        let bufsize = mntbuf.len() * mem::size_of::<statfs>();
        // SAFETY:
        // Calling external "C" function getfsstate with actual buffer now.  The
        // function takes a buffer size to not overflow.  If the mount table
        // changes size between calls we are protected by bufsize
        count = unsafe { getfsstat(mntbuf.as_mut_ptr(), bufsize as c_int, MNT_NOWAIT) };
        // Resize if the mount table has shrunk since last call
        if count >= 0 {
            mntbuf.truncate(count as usize);
        }
    }
    if count < 0 {
        // SAFETY: Calling external "C" errno function to get the error number
        return Err(Error::GetFSStatError(unsafe { *__error() }));
    }

    let mut mounts = Vec::with_capacity(count as usize);
    for mnt in &mntbuf {
        let mount_point = OsStr::from_bytes(
            // SAFETY: Converting null terminated "C" string
            unsafe { CStr::from_ptr(mnt.f_mntonname.as_ptr().cast::<c_char>()) }.to_bytes(),
        );
        let dest = PathBuf::from(mount_point);
        // SAFETY: Converting null terminated "C" string
        let fstype = unsafe { CStr::from_ptr(mnt.f_fstypename.as_ptr().cast::<c_char>()) }
            .to_string_lossy()
            .into();
        // SAFETY: Converting null terminated "C" string
        let source = unsafe { CStr::from_ptr(mnt.f_mntfromname.as_ptr().cast::<c_char>()) }
            .to_string_lossy()
            .into();
        mounts.push(MountedFs {
            dest,
            fstype,
            source,
        });
    }

    Ok(mounts)
}
