// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use crate::options::parser::MatchedFlags;
use crate::options::vars::EZA_STDIN_SEPARATOR;
use crate::options::{flags, OptionsError, Vars};
use std::ffi::OsString;

#[derive(Debug, PartialEq)]
pub enum FilesInput {
    Stdin(OsString),
    Args,
}

impl FilesInput {
    pub fn deduce<V: Vars>(matches: &MatchedFlags<'_>, vars: &V) -> Result<Self, OptionsError> {
        Ok(
            if matches.has(&flags::STDIN)? {
                let separator = vars
                    .get(EZA_STDIN_SEPARATOR)
                    .unwrap_or(OsString::from("\n"));
                FilesInput::Stdin(separator)
            } else {
                FilesInput::Args
            },
        )
    }
}
