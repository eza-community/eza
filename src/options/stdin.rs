// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use clap::ArgMatches;

use crate::options::Vars;
use crate::options::vars::EZA_STDIN_SEPARATOR;
use std::ffi::OsString;
use std::io;

#[derive(Debug, PartialEq)]
pub enum FilesInput {
    Stdin(OsString),
    Args,
}

/// Returns true only when stdin is a pipe or FIFO — not a terminal, not /dev/null
/// or other character devices. This prevents eza from blocking or silently
/// producing no output when stdin is redirected to /dev/null in sandboxed
/// environments (bubblewrap, Docker, etc.).
#[cfg(unix)]
fn stdin_is_pipe() -> bool {
    use std::os::unix::io::AsRawFd;
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    unsafe {
        libc::fstat(io::stdin().as_raw_fd(), &mut stat) == 0
            && (stat.st_mode & libc::S_IFMT) == libc::S_IFIFO
    }
}

#[cfg(not(unix))]
fn stdin_is_pipe() -> bool {
    use std::io::IsTerminal;

    !io::stdin().is_terminal()
}

impl FilesInput {
    pub fn deduce<V: Vars>(matches: &ArgMatches, vars: &V) -> Self {
        if matches.get_flag("stdin") || stdin_is_pipe() {
            let separator = vars
                .get(EZA_STDIN_SEPARATOR)
                .unwrap_or(OsString::from("\n"));
            FilesInput::Stdin(separator)
        } else {
            FilesInput::Args
        }
    }
}
