//! Parsing the options for `DirAction`.

use crate::options::parser::Opts;
use crate::options::OptionsError;

use crate::fs::dir_action::{DirAction, RecurseOptions};

impl DirAction {
    /// Determine which action to perform when trying to list a directory.
    /// There are three possible actions, and they overlap somewhat: the
    /// `--tree` flag is another form of recursion, so those two are allowed
    /// to both be present, but the `--list-dirs` flag is used separately.
    pub fn deduce(matches: &Opts, can_tree: bool, strict: bool) -> Result<Self, OptionsError> {
        let recurse = matches.recurse > 0;
        let as_file = matches.list_dirs > 0;
        let tree = matches.tree > 0;

        if strict {
            // Early check for --level when it wouldn’t do anything
            if !recurse && !tree && matches.level.is_some() {
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
    pub fn deduce(matches: &Opts, tree: bool) -> Self {
        if let Some(level) = matches.level {
            Self {
                tree,
                max_depth: Some(level),
            }
        } else {
            Self {
                tree,
                max_depth: None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deduce_recurse_options_level() {
        let options = Opts {
            level: Some(3),
            ..Opts::default()
        };

        assert_eq!(
            RecurseOptions::deduce(&options, false),
            RecurseOptions {
                tree: false,
                max_depth: Some(3),
            }
        );
    }

    #[test]
    fn deduce_recurse_options_no_level() {
        let options = Opts {
            level: None,
            ..Opts::default()
        };

        assert_eq!(
            RecurseOptions::deduce(&options, true),
            RecurseOptions {
                tree: true,
                max_depth: None,
            }
        );
    }

    #[test]
    fn deduce_dir_action_list() {
        let options = Opts { ..Opts::default() };

        assert_eq!(
            DirAction::deduce(&options, false, false),
            Ok(DirAction::List)
        );
    }

    #[test]
    fn deduce_dir_action_as_file() {
        let options = Opts {
            list_dirs: 1,
            ..Opts::default()
        };

        assert_eq!(
            DirAction::deduce(&options, false, false),
            Ok(DirAction::AsFile)
        );
    }

    #[test]
    fn deduce_dir_action_recurse() {
        let options = Opts {
            recurse: 1,
            ..Opts::default()
        };

        assert_eq!(
            DirAction::deduce(&options, false, false),
            Ok(DirAction::Recurse(RecurseOptions {
                tree: false,
                max_depth: None,
            }))
        );
    }

    #[test]
    fn deduce_dir_action_tree() {
        let options = Opts {
            tree: 1,
            ..Opts::default()
        };

        assert_eq!(
            DirAction::deduce(&options, true, false),
            Ok(DirAction::Recurse(RecurseOptions {
                tree: true,
                max_depth: None,
            }))
        );
    }

    #[test]
    fn deduce_dir_action_tree_level() {
        let options = Opts {
            tree: 1,
            level: Some(3),
            ..Opts::default()
        };

        assert_eq!(
            DirAction::deduce(&options, true, false),
            Ok(DirAction::Recurse(RecurseOptions {
                tree: true,
                max_depth: Some(3),
            }))
        );
    }

    #[test]
    fn deduce_dir_action_tree_level_conflict() {
        let options = Opts {
            level: Some(3),
            ..Opts::default()
        };

        assert_eq!(
            DirAction::deduce(&options, false, true),
            Err(OptionsError::Useless2("level", "recurse", "tree"))
        );
    }

    #[test]
    fn deduce_dir_action_recurse_as_file_conflict() {
        let options = Opts {
            recurse: 1,
            list_dirs: 1,
            ..Opts::default()
        };

        assert_eq!(
            DirAction::deduce(&options, false, true),
            Err(OptionsError::Conflict("recurse", "list-dirs"))
        );
    }

    #[test]
    fn deduce_dir_action_tree_as_file_conflict() {
        let options = Opts {
            tree: 1,
            list_dirs: 1,
            ..Opts::default()
        };

        assert_eq!(
            DirAction::deduce(&options, true, true),
            Err(OptionsError::Conflict("tree", "list-dirs"))
        );
    }
}
