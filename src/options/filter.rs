// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
//! Parsing the options for `FileFilter`.

use clap::ArgMatches;

use crate::fs::filter::{
    FileFilter, FileFilterFlags, GitIgnore, IgnorePatterns, SortCase, SortField,
};
use crate::fs::DotFilter;

use crate::options::OptionsError;

impl FileFilter {
    /// Determines which of all the file filter options to use.
    pub fn deduce(matches: &ArgMatches, strict: bool) -> Result<Self, OptionsError> {
        use FileFilterFlags as FFF;
        let mut filter_flags: Vec<FileFilterFlags> = vec![];

        for (flag, filter_flag) in &[
            ("reverse", FFF::Reverse),
            ("only-dirs", FFF::OnlyDirs),
            ("only-files", FFF::OnlyFiles),
            ("no-symlinks", FFF::NoSymlinks),
            ("show-symlinks", FFF::ShowSymlinks),
            ("dirs-last", FFF::ListDirsLast),
            ("dirs-first", FFF::ListDirsFirst),
        ] {
            if matches.get_flag(flag) {
                filter_flags.push(filter_flag.clone());
            }
        }

        #[rustfmt::skip]
        return Ok(Self {
            no_symlinks:      matches.get_flag("no-symlinks"),
            show_symlinks:    matches.get_flag("show-symlinks"),
            flags: filter_flags,
            sort_field:       *matches.get_one("sort").unwrap(),
            dot_filter:       DotFilter::deduce(matches, strict)?,
            ignore_patterns:  IgnorePatterns::deduce(matches)?,
            git_ignore:       GitIgnore::deduce(matches),
        });
    }
}

// impl SortField {
//     /// Determines which sort field to use based on the `--sort` argument.
//     /// This argument’s value can be one of several flags, listed above.
//     /// Returns the default sort field if none is given, or `Err` if the
//     /// value doesn’t correspond to a sort field we know about.
//     fn deduce(matches: &ArgMatches) -> Result<Self, OptionsError> {
//         let Some(&word) = matches.get_one::<SortField>("sort") else {
//             return Ok(Self::default());
//         };

//         // Get String because we can’t match an OsStr
//         let Some(word) = word.into() else {
//             return Err(OptionsError::BadArgument("sort", word.to_possible_value()));
//         };

//         let field = match word {
//             SortField::Name => Self::Name(SortCase::AaBbCc),
//             SortField::NameCaps => Self::Name(SortCase::ABCabc),
//             SortField::DotName => Self::NameMixHidden(SortCase::AaBbCc),
//             SortField::DotNameCaps => Self::NameMixHidden(SortCase::ABCabc),
//             SortField::Size => Self::Size,
//             SortField::Extension => Self::Extension(SortCase::AaBbCc),
//             SortField::ExtensionCaps => Self::Extension(SortCase::ABCabc),

//             // “new” sorts oldest at the top and newest at the bottom; “old”
//             // sorts newest at the top and oldest at the bottom. I think this
//             // is the right way round to do this: “size” puts the smallest at
//             // the top and the largest at the bottom, doesn’t it?
//             SortField::Modified => Self::ModifiedDate,

//             // Similarly, “age” means that files with the least age (the
//             // newest files) get sorted at the top, and files with the most
//             // age (the oldest) at the bottom.
//             "age" | "old" | "oldest" => Self::ModifiedAge,

//             "ch" | "changed" => Self::ChangedDate,
//             "acc" | "accessed" => Self::AccessedDate,
//             "cr" | "created" => Self::CreatedDate,
//             #[cfg(unix)]
//             "inode" => Self::FileInode,
//             "type" => Self::FileType,
//             "none" => Self::Unsorted,
//             _ => {
//                 return Err(OptionsError::BadArgument("sort", word.into()));
//             }
//         };

//         Ok(field)
//     }
// }

// I’ve gone back and forth between whether to sort case-sensitively or
// insensitively by default. The default string sort in most programming
// languages takes each character’s ASCII value into account, sorting
// “Documents” before “apps”, but there’s usually an option to ignore
// characters’ case, putting “apps” before “Documents”.
//
// The argument for following case is that it’s easy to forget whether an item
// begins with an uppercase or lowercase letter and end up having to scan both
// the uppercase and lowercase sub-lists to find the item you want. If you
// happen to pick the sublist it’s not in, it looks like it’s missing, which
// is worse than if you just take longer to find it.
// (https://ux.stackexchange.com/a/79266)
//
// The argument for ignoring case is that it makes exa sort files differently
// from shells. A user would expect a directory’s files to be in the same
// order if they used “exa ~/directory” or “exa ~/directory/*”, but exa sorts
// them in the first case, and the shell in the second case, so they wouldn’t
// be exactly the same if exa does something non-conventional.
//
// However, exa already sorts files differently: it uses natural sorting from
// the natord crate, sorting the string “2” before “10” because the number’s
// smaller, because that’s usually what the user expects to happen. Users will
// name their files with numbers expecting them to be treated like numbers,
// rather than lists of numeric characters.
//
// In the same way, users will name their files with letters expecting the
// order of the letters to matter, rather than each letter’s character’s ASCII
// value. So exa breaks from tradition and ignores case while sorting:
// “apps” first, then “Documents”.
//
// You can get the old behaviour back by sorting with `--sort=Name`.
impl Default for SortField {
    fn default() -> Self {
        Self::Name(SortCase::AaBbCc)
    }
}

impl DotFilter {
    /// Determines the dot filter based on how many `--all` options were
    /// given: one will show dotfiles, but two will show `.` and `..` too.
    /// --almost-all is equivalent to --all, included for compatibility with
    /// `ls -A`.
    ///
    /// It also checks for the `--tree` option, because of a special case
    /// where `--tree --all --all` won’t work: listing the parent directory
    /// in tree mode would loop onto itself!
    ///
    /// `--almost-all` binds stronger than multiple `--all` as we currently do not take the order
    /// of arguments into account and it is the safer option (does not clash with `--tree`)
    pub fn deduce(matches: &ArgMatches, strict: bool) -> Result<Self, OptionsError> {
        let all_count = matches.get_count("all");
        let has_almost_all = matches.get_flag("almost-all");

        match (all_count, has_almost_all) {
            (0, false) => Ok(Self::JustFiles),

            // either a single --all or at least one --almost-all is given
            (1, _) | (0, true) => Ok(Self::Dotfiles),
            // more than one --all
            (c, _) => {
                if matches.get_flag("tree") {
                    Err(OptionsError::TreeAllAll)
                } else if strict && c > 2 {
                    Err(OptionsError::Conflict("all", "all"))
                } else {
                    Ok(Self::DotfilesAndDots)
                }
            }
        }
    }
}

impl IgnorePatterns {
    /// Determines the set of glob patterns to use based on the
    /// `--ignore-glob` argument’s value. This is a list of strings
    /// separated by pipe (`|`) characters, given in any order.
    pub fn deduce(matches: &ArgMatches) -> Result<Self, OptionsError> {
        // If there are no inputs, we return a set of patterns that doesn’t
        // match anything, rather than, say, `None`.
        let Some(inputs) = matches.get_one::<String>("ignore-glob") else {
            return Ok(Self::empty());
        };

        // Awkwardly, though, a glob pattern can be invalid, and we need to
        // deal with invalid patterns somehow.
        let (patterns, mut errors) = Self::parse_from_iter(inputs.split('|'));

        // It can actually return more than one glob error,
        // but we only use one. (TODO)
        match errors.pop() {
            Some(e) => Err(e.into()),
            None => Ok(patterns),
        }
    }
}

impl GitIgnore {
    pub fn deduce(matches: &ArgMatches) -> Self {
        if matches.get_flag("git-ignore") {
            Self::CheckAndIgnore
        } else {
            Self::Off
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use super::*;
    use crate::options::parser::test::{mock_cli, mock_cli_try};

    #[test]
    fn deduce_git_ignore_off() {
        assert_eq!(GitIgnore::deduce(&mock_cli(vec![""])), GitIgnore::Off);
    }

    #[test]
    fn deduce_git_ignore_on() {
        assert_eq!(
            GitIgnore::deduce(&mock_cli(vec!["--git-ignore"])),
            GitIgnore::CheckAndIgnore
        );
    }

    #[test]
    fn deduce_ignore_patterns_empty() {
        assert_eq!(
            IgnorePatterns::deduce(&mock_cli(vec![""])),
            Ok(IgnorePatterns::empty())
        );
    }

    #[test]
    fn deduce_ignore_patterns_one() {
        let pattern = OsString::from("*.o");
        let (res, _) = IgnorePatterns::parse_from_iter(pattern.to_string_lossy().split('|'));

        assert_eq!(
            IgnorePatterns::deduce(&mock_cli(vec!["--ignore-glob", "*.o"])),
            Ok(res)
        );
    }

    #[test]
    fn deduce_ignore_patterns_error() {
        let pattern = OsString::from("[");
        let (_, mut e) = IgnorePatterns::parse_from_iter(pattern.to_string_lossy().split('|'));
        assert_eq!(
            IgnorePatterns::deduce(&mock_cli(vec!["--ignore-glob", "["])),
            Err(e.pop().unwrap().into())
        );
    }

    #[test]
    fn deduce_dot_filter_just_files() {
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec![""]), false),
            Ok(DotFilter::JustFiles)
        );
    }

    #[test]
    fn deduce_dot_filter_dotfiles() {
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["--all"]), false),
            Ok(DotFilter::Dotfiles)
        );
    }

    #[test]
    fn deduce_dot_filter_dotfiles_and_dots() {
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["--all", "--all"]), false),
            Ok(DotFilter::DotfilesAndDots)
        );
    }

    #[test]
    fn deduce_dot_filter_tree_all_all() {
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["--all", "--all", "--tree"]), false),
            Err(OptionsError::TreeAllAll)
        );
    }

    #[test]
    fn deduce_dot_filter_all_all() {
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["--all", "--all", "--all"]), true),
            Err(OptionsError::Conflict("all", "all"))
        );
    }

    #[test]
    fn deduce_dot_filter_almost_all() {
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["--almost-all"]), false),
            Ok(DotFilter::Dotfiles)
        );
    }

    #[test]
    fn deduce_sort_field_default() {
        assert_eq!(
            mock_cli(vec![""]).get_one::<SortField>("sort"),
            Some(&SortField::default())
        );
    }

    #[test]
    fn deduce_sort_field_name() {
        assert_eq!(
            mock_cli(vec!["--sort", "name"]).get_one::<SortField>("sort"),
            Some(&SortField::Name(SortCase::AaBbCc))
        );
    }

    #[test]
    fn deduce_sort_field_name_case() {
        assert_eq!(
            mock_cli(vec!["--sort", "Name"]).get_one::<SortField>("sort"),
            Some(&SortField::Name(SortCase::ABCabc))
        );
    }

    #[test]
    fn deduce_sort_field_name_mix_hidden() {
        assert_eq!(
            mock_cli(vec!["--sort", ".name"]).get_one::<SortField>("sort"),
            Some(&SortField::NameMixHidden(SortCase::AaBbCc))
        );
    }

    #[test]
    fn deduce_sort_field_name_mix_hidden_case() {
        assert_eq!(
            mock_cli(vec!["--sort", ".Name"]).get_one::<SortField>("sort"),
            Some(&SortField::NameMixHidden(SortCase::ABCabc))
        );
    }

    #[test]
    fn deduce_sort_field_size() {
        assert_eq!(
            mock_cli(vec!["--sort", "size"]).get_one::<SortField>("sort"),
            Some(&SortField::Size)
        );
    }

    #[test]
    fn deduce_sort_field_extension() {
        assert_eq!(
            mock_cli(vec!["--sort", "ext"]).get_one::<SortField>("sort"),
            Some(&SortField::Extension(SortCase::AaBbCc))
        );
    }

    #[test]
    fn deduce_sort_field_extension_case() {
        assert_eq!(
            mock_cli(vec!["--sort", "Ext"]).get_one::<SortField>("sort"),
            Some(&SortField::Extension(SortCase::ABCabc))
        );
    }

    #[test]
    fn deduce_sort_field_date() {
        assert_eq!(
            mock_cli(vec!["--sort", "date"]).get_one::<SortField>("sort"),
            Some(&SortField::ModifiedDate)
        );
    }

    #[test]
    fn deduce_sort_field_time() {
        assert_eq!(
            mock_cli(vec!["--sort", "time"]).get_one::<SortField>("sort"),
            Some(&SortField::ModifiedDate)
        );
    }

    #[test]
    fn deduce_sort_field_age() {
        assert_eq!(
            mock_cli(vec!["--sort", "age"]).get_one::<SortField>("sort"),
            Some(&SortField::ModifiedAge)
        );
    }

    #[test]
    fn deduce_sort_field_old() {
        assert_eq!(
            mock_cli(vec!["--sort", "old"]).get_one::<SortField>("sort"),
            Some(&SortField::ModifiedAge)
        );
    }

    #[test]
    fn deduce_sort_field_ch() {
        assert_eq!(
            mock_cli(vec!["--sort", "ch"]).get_one::<SortField>("sort"),
            Some(&SortField::ChangedDate)
        );
    }

    #[test]
    fn deduce_sort_field_acc() {
        assert_eq!(
            mock_cli(vec!["--sort", "acc"]).get_one::<SortField>("sort"),
            Some(&SortField::AccessedDate)
        );
    }

    #[test]
    fn deduce_sort_field_cr() {
        assert_eq!(
            mock_cli(vec!["--sort", "cr"]).get_one::<SortField>("sort"),
            Some(&SortField::CreatedDate)
        );
    }

    #[test]
    fn deduce_sort_field_err() {
        assert!(mock_cli_try(vec!["--sort", "foo"]).is_err());
    }

    #[test]
    fn deduce_file_filter_default() {
        assert_eq!(
            FileFilter::deduce(&mock_cli(vec![""]), false),
            Ok(FileFilter {
                flags: vec![],
                sort_field: SortField::default(),
                dot_filter: DotFilter::JustFiles,
                ignore_patterns: IgnorePatterns::empty(),
                git_ignore: GitIgnore::Off,
                no_symlinks: false,
                show_symlinks: false,
            })
        );
    }

    #[test]
    fn deduce_file_filter_reverse() {
        assert_eq!(
            FileFilter::deduce(&mock_cli(vec!["--reverse"]), false),
            Ok(FileFilter {
                flags: vec![FileFilterFlags::Reverse],
                sort_field: SortField::default(),
                dot_filter: DotFilter::JustFiles,
                ignore_patterns: IgnorePatterns::empty(),
                git_ignore: GitIgnore::Off,
                no_symlinks: false,
                show_symlinks: false,
            })
        );
    }

    #[test]
    fn deduce_file_filter_only_dirs() {
        assert_eq!(
            FileFilter::deduce(&mock_cli(vec!["--only-dirs"]), false),
            Ok(FileFilter {
                flags: vec![FileFilterFlags::OnlyDirs],
                sort_field: SortField::default(),
                dot_filter: DotFilter::JustFiles,
                ignore_patterns: IgnorePatterns::empty(),
                git_ignore: GitIgnore::Off,
                no_symlinks: false,
                show_symlinks: false,
            })
        );
    }

    #[test]
    fn deduce_file_filter_only_files() {
        assert_eq!(
            FileFilter::deduce(&mock_cli(vec!["--only-files"]), false),
            Ok(FileFilter {
                flags: vec![FileFilterFlags::OnlyFiles],
                sort_field: SortField::default(),
                dot_filter: DotFilter::JustFiles,
                ignore_patterns: IgnorePatterns::empty(),
                git_ignore: GitIgnore::Off,
                no_symlinks: false,
                show_symlinks: false,
            })
        );
    }
}
