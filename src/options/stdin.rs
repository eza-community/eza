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

#[derive(Debug, PartialEq, Eq)]
pub enum FilesInput {
    Stdin(OsString),
    Args,
}

impl FilesInput {
    pub fn deduce<V: Vars>(matches: &ArgMatches, vars: &V) -> Self {
        if matches.get_flag("stdin") {
            let separator = vars
                .get(EZA_STDIN_SEPARATOR)
                .unwrap_or(OsString::from("\n"));
            FilesInput::Stdin(separator)
        } else {
            FilesInput::Args
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::parser::test::mock_cli;
    use crate::options::vars::test::MockVars;

    #[test]
    fn deduce_stdin_disabled_by_default() {
        let vars = MockVars::default();
        assert_eq!(
            FilesInput::deduce(&mock_cli(vec![""]), &vars),
            FilesInput::Args
        );
    }

    #[test]
    fn deduce_stdin_enabled_with_flag() {
        let vars = MockVars::default();
        assert_eq!(
            FilesInput::deduce(&mock_cli(vec!["--stdin"]), &vars),
            FilesInput::Stdin(OsString::from("\n"))
        );
    }

    #[test]
    fn deduce_stdin_custom_separator() {
        let mut vars = MockVars::default();
        let sep = OsString::from("0");
        vars.set(EZA_STDIN_SEPARATOR, &sep);
        assert_eq!(
            FilesInput::deduce(&mock_cli(vec!["--stdin"]), &vars),
            FilesInput::Stdin(OsString::from("0"))
        );
    }
}
