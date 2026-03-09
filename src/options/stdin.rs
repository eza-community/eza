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
use std::io::IsTerminal;

#[derive(Debug, PartialEq)]
pub enum FilesInput {
    Stdin(OsString),
    Args,
}

// Check if stdin is redirected to /dev/null.
// /dev/null is commonly used in sandboxes and background processes to mean "no input",
// so we should not try to read from it.
#[cfg(unix)]
fn is_stdin_dev_null() -> bool {
    use std::os::unix::fs::MetadataExt;

    let Ok(stdin_meta) = std::fs::metadata("/dev/stdin") else {
        return false;
    };
    let Ok(null_meta) = std::fs::metadata("/dev/null") else {
        return false;
    };

    // Compare device and inode numbers to check if stdin is /dev/null
    stdin_meta.dev() == null_meta.dev() && stdin_meta.ino() == null_meta.ino()
}

#[cfg(not(unix))]
fn is_stdin_dev_null() -> bool {
    // On non-Unix platforms, we can't reliably detect /dev/null
    false
}

impl FilesInput {
    pub fn deduce<V: Vars>(matches: &ArgMatches, vars: &V) -> Self {
        if matches.get_flag("stdin") || (!io::stdin().is_terminal() && !is_stdin_dev_null()) {
            let separator = vars
                .get(EZA_STDIN_SEPARATOR)
                .unwrap_or(OsString::from("\n"));
            FilesInput::Stdin(separator)
        } else {
            FilesInput::Args
        }
    }
}
