//! Parsing the options for `DirAction`.

use crate::options::OptionsError;

use crate::fs::dir_action::{DirAction, RecurseOptions};
use crate::options::parser::Opts;

impl DirAction {
    /// Determine which action to perform when trying to list a directory.
    /// There are three possible actions, and they overlap somewhat: the
    /// `--tree` flag is another form of recursion, so those two are allowed
    /// to both be present, but the `--list-dirs` flag is used separately.
    pub fn deduce(matches: &Opts, can_tree: bool, strictness: bool) -> Result<Self, OptionsError> {
        let recurse = matches.recurse > 0;
        let as_file = matches.list_dirs > 0;
        let tree = matches.tree > 0;

        if strictness {
            // Early check for --level when it wouldn’t do anything
            if !recurse && !tree && matches.level.is_some() {
                return Err(OptionsError::Useless2(
                    "--level".to_string(),
                    "--recurse".to_string(),
                    "--tree".to_string(),
                ));
            } else if recurse && as_file {
                return Err(OptionsError::Conflict(
                    "--recurse".to_string(),
                    "--list-dirs".to_string(),
                ));
            } else if tree && as_file {
                return Err(OptionsError::Conflict(
                    "--tree".to_string(),
                    "--list-dirs".to_string(),
                ));
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
    pub fn deduce(matches: &Opts, tree: bool) -> Self {
        Self {
            tree,
            max_depth: matches.level,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deduces_list() {
        let matches = Opts { ..Opts::default() };

        assert_eq!(
            DirAction::deduce(&matches, false, false).unwrap(),
            DirAction::List
        );
    }

    #[test]
    fn deduce_recurse() {
        let matches = Opts {
            recurse: 1,
            ..Opts::default()
        };
        assert_eq!(
            DirAction::deduce(&matches, false, false).unwrap(),
            DirAction::Recurse(RecurseOptions {
                tree: false,
                max_depth: None,
            })
        );
    }

    #[test]
    fn deduce_recurse_tree() {
        let matches = Opts {
            tree: 1,
            ..Opts::default()
        };
        assert_eq!(
            DirAction::deduce(&matches, true, false).unwrap(),
            DirAction::Recurse(RecurseOptions {
                tree: true,
                max_depth: None,
            })
        );
    }

    #[test]
    fn deduce_as_file() {
        let matches = Opts {
            list_dirs: 1,
            ..Opts::default()
        };
        assert_eq!(
            DirAction::deduce(&matches, false, false).unwrap(),
            DirAction::AsFile
        );
    }

    #[test]
    fn deduce_strict_unuseful_level() {
        let matches = Opts {
            level: Some(2),
            ..Opts::default()
        };

        assert!(DirAction::deduce(&matches, false, true).is_err());
    }

    #[test]
    fn deduce_strict_recurse_and_as_file_option() {
        let matches = Opts {
            recurse: 1,
            list_dirs: 1,
            ..Opts::default()
        };

        assert!(DirAction::deduce(&matches, false, true).is_err());
    }

    #[test]
    fn deduce_strict_tree_and_as_file_option() {
        let matches = Opts {
            tree: 1,
            list_dirs: 1,
            ..Opts::default()
        };

        assert!(DirAction::deduce(&matches, false, true).is_err());
    }

    #[test]
    fn deduce_recurse_options() {
        let matches = Opts {
            recurse: 1,
            tree: 1,
            level: Some(2),
            ..Opts::default()
        };

        assert_eq!(
            DirAction::deduce(&matches, true, false).unwrap(),
            DirAction::Recurse(RecurseOptions {
                tree: true,
                max_depth: Some(2),
            })
        );
    }
}
