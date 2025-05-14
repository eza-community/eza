// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
//! Parsing the options for `DirAction`.

use crate::options::OptionsError;

use crate::fs::dir_action::{DirAction, RecurseOptions};

use clap::ArgMatches;

impl DirAction {
    /// Determine which action to perform when trying to list a directory.
    /// There are three possible actions, and they overlap somewhat: the
    /// `--tree` flag is another form of recursion, so those two are allowed
    /// to both be present, but the `--list-dirs` flag is used separately.
    pub fn deduce(
        matches: &ArgMatches,
        can_tree: bool,
        strict: bool,
    ) -> Result<Self, OptionsError> {
        let recurse = matches.get_flag("recurse");
        let as_file = matches.get_flag("list-dirs");
        let tree = matches.get_flag("tree");

        if strict {
            // Early check for --level when it wouldn’t do anything
            if !recurse && !tree && matches.get_one::<usize>("level").is_some() {
                return Err(OptionsError::Useless2("level", "recurse", "tree"));
            } else if recurse && as_file {
                return Err(OptionsError::Conflict("recurse", "list-dirs"));
            } else if tree && as_file {
                return Err(OptionsError::Conflict("tree", "list-dirs"));
            }
        }

        if tree && can_tree {
            // Tree is only appropriate in details mode, so this has to
            // examine the View, which should have already been deduced by now
            Ok(Self::Recurse(RecurseOptions::deduce(matches, true)))
        } else if recurse {
            Ok(Self::Recurse(RecurseOptions::deduce(matches, false)))
        } else if as_file {
            Ok(Self::AsFile)
        } else {
            Ok(Self::List)
        }
    }
}

impl RecurseOptions {
    /// Determine which files should be recursed into, based on the `--level`
    /// flag’s value, and whether the `--tree` flag was passed, which was
    /// determined earlier. The maximum level should be a number, and this
    /// will fail with an `Err` if it isn’t.
    pub fn deduce(matches: &ArgMatches, tree: bool) -> Self {
        Self {
            tree,
            max_depth: matches.get_one("level").copied(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::parser::test::{mock_cli, mock_cli_try};

    #[test]
    fn deduce_dir_action_list() {
        assert_eq!(
            DirAction::deduce(&mock_cli(vec![""]), false, false),
            Ok(DirAction::List)
        );
    }

    #[test]
    fn deduce_recurse_options_level() {
        assert_eq!(
            RecurseOptions::deduce(&mock_cli(vec!["--level", "3"]), false),
            RecurseOptions {
                tree: false,
                max_depth: Some(3),
            }
        );
    }

    #[test]
    fn deduce_recurse_options_no_level() {
        assert_eq!(
            DirAction::deduce(&mock_cli(vec!["--recurse"]), true, true),
            Ok(DirAction::Recurse(RecurseOptions {
                tree: false,
                max_depth: None,
            }))
        );
    }

    #[test]
    fn deduce_dir_action_as_file() {
        assert_eq!(
            DirAction::deduce(&mock_cli(vec!["--list-dirs"]), false, false),
            Ok(DirAction::AsFile)
        );
    }

    #[test]
    fn deduce_dir_action_recurse() {
        assert_eq!(
            DirAction::deduce(&mock_cli(vec!["--recurse"]), false, false),
            Ok(DirAction::Recurse(RecurseOptions {
                tree: false,
                max_depth: None,
            }))
        );
    }

    #[test]
    fn deduce_dir_action_tree() {
        assert_eq!(
            DirAction::deduce(&mock_cli(vec!["--tree"]), true, false),
            Ok(DirAction::Recurse(RecurseOptions {
                tree: true,
                max_depth: None,
            }))
        );
    }

    #[test]
    fn deduce_dir_action_tree_level() {
        assert_eq!(
            DirAction::deduce(&mock_cli(vec!["--tree", "--level", "3"]), true, false),
            Ok(DirAction::Recurse(RecurseOptions {
                tree: true,
                max_depth: Some(3),
            }))
        );
    }

    #[test]
    fn deduce_dir_action_tree_level_conflict() {
        assert_eq!(
            DirAction::deduce(&mock_cli(vec!["--level", "3"]), false, true),
            Err(OptionsError::Useless2("level", "recurse", "tree"))
        );
    }

    #[test]
    fn deduce_dir_action_recurse_as_file_conflict() {
        assert!(mock_cli_try(vec!["--recurse", "--list-dirs"]).is_err());
    }

    #[test]
    fn deduce_dir_action_tree_as_file_conflict() {
        assert!(mock_cli_try(vec!["--tree", "--list-dirs"]).is_err());
    }
}
