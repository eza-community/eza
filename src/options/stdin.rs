// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use clap::ArgMatches;

use crate::options::vars::EZA_STDIN_SEPARATOR;
use crate::options::Vars;
use std::ffi::OsString;
use std::io;
use std::io::IsTerminal;

#[derive(Debug, PartialEq)]
pub enum FilesInput {
    Stdin(OsString),
    Args,
}

impl FilesInput {
    pub fn deduce<V: Vars>(matches: &ArgMatches, vars: &V) -> Self {
        if io::stdin().is_terminal() || !matches.get_flag("stdin") {
            FilesInput::Args
        } else if matches.get_flag("stdin") && !io::stdin().is_terminal() {
            let separator = vars
                .get(EZA_STDIN_SEPARATOR)
                .unwrap_or(OsString::from("\n"));
            FilesInput::Stdin(separator)
        } else {
            FilesInput::Args
        }
    }
}
