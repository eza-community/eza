//! Parsing the options for `FileFilter`.

use crate::fs::filter::{
    FileFilter, FileFilterFlags, GitIgnore, IgnorePatterns, SortCase, SortField,
};
use crate::fs::DotFilter;

use crate::options::OptionsError;

use super::parser::Opts;


impl FileFilter {
    /// Determines which of all the file filter options to use.
    pub fn deduce(matches: &Opts, strictness: bool) -> Result<Self, OptionsError> {
        Ok(Self {
            list_dirs_first:  matches.dirs_first > 0,
            reverse:          matches.reverse > 0,
            only_dirs:        matches.only_dirs > 0,
            sort_field:       SortField::deduce(matches)?,
            dot_filter:       DotFilter::deduce(matches, strictness)?,
            ignore_patterns:  IgnorePatterns::deduce(matches)?,
            git_ignore:       GitIgnore::deduce(matches)?,
        });
    }
}

impl SortField {
    /// Determines which sort field to use based on the `--sort` argument.
    /// This argument’s value can be one of several flags, listed above.
    /// Returns the default sort field if none is given, or `Err` if the
    /// value doesn’t correspond to a sort field we know about.
    fn deduce(matches: &Opts) -> Result<Self, OptionsError> {
        let Some(ref word) = matches.sort else { return Ok(Self::default()) };

        // Get String because we can’t match an OsStr
        let Some(word) = word.to_str() else { return Err(OptionsError::BadArgument("SORT".to_string(), word.to_string_lossy().to_string())) };

        let field = match word {
            "name" | "filename" => Self::Name(SortCase::AaBbCc),
            "Name" | "Filename" => Self::Name(SortCase::ABCabc),
            ".name" | ".filename" => Self::NameMixHidden(SortCase::AaBbCc),
            ".Name" | ".Filename" => Self::NameMixHidden(SortCase::ABCabc),
            "size" | "filesize" => Self::Size,
            "ext" | "extension" => Self::Extension(SortCase::AaBbCc),
            "Ext" | "Extension" => Self::Extension(SortCase::ABCabc),

            // “new” sorts oldest at the top and newest at the bottom; “old”
            // sorts newest at the top and oldest at the bottom. I think this
            // is the right way round to do this: “size” puts the smallest at
            // the top and the largest at the bottom, doesn’t it?
            "date" | "time" | "mod" | "modified" | "new" | "newest" => Self::ModifiedDate,

            // Similarly, “age” means that files with the least age (the
            // newest files) get sorted at the top, and files with the most
            // age (the oldest) at the bottom.
            "age" | "old" | "oldest" => Self::ModifiedAge,

            "ch" | "changed" => Self::ChangedDate,
            "acc" | "accessed" => Self::AccessedDate,
            "cr" | "created" => Self::CreatedDate,
            #[cfg(unix)]
            "inode" => Self::FileInode,
            "type" => Self::FileType,
            "none" => Self::Unsorted,
            _ => {
                return Err(OptionsError::BadArgument("SORT".to_string(), word.into()));
            }
        };

        Ok(field)
    }
}

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
    pub fn deduce(matches: &Opts, strictness: bool) -> Result<Self, OptionsError> {
        let all_count = matches.all;
        let has_almost_all = matches.almost_all;

        match (all_count, has_almost_all) {
            (0, 0) => Ok(Self::JustFiles),

            // either a single --all or at least one --almost-all is given
            (1 | 0, _) => Ok(Self::Dotfiles),
            // more than one --all
            (_, _) => if matches.tree > 0 {
                Err(OptionsError::TreeAllAll)
            } else if strictness {
                Err(OptionsError::Conflict("ALL".to_string(), "ALL".to_string()))
            } else {
                Ok(Self::DotfilesAndDots)
            },
        }
    }
}

impl IgnorePatterns {
    /// Determines the set of glob patterns to use based on the
    /// `--ignore-glob` argument’s value. This is a list of strings
    /// separated by pipe (`|`) characters, given in any order.
    pub fn deduce(matches: &Opts) -> Result<Self, OptionsError> {

        // If there are no inputs, we return a set of patterns that doesn’t
        // match anything, rather than, say, `None`.
        let Some(ref inputs) = matches.ignore_glob else { return Ok(Self::empty()) };

        // Awkwardly, though, a glob pattern can be invalid, and we need to
        // deal with invalid patterns somehow.
        let (patterns, mut errors) = Self::parse_from_iter(inputs.to_string_lossy().split('|'));

        // It can actually return more than one glob error,
        // but we only use one. (TODO)
        match errors.pop() {
            Some(e) => Err(e.into()),
            None => Ok(patterns),
        }
    }
}

impl GitIgnore {
    pub fn deduce(matches: &Opts) -> Result<Self, OptionsError> {
        if matches.git_ignore > 0 && matches.no_git == 0 {
            return Ok(Self::CheckAndIgnore);
        } else if matches.git_ignore > 0 && matches.no_git > 0 {
            return Err(OptionsError::Conflict("GIT_IGNORE".to_string(), "NO_GIT".to_string()));
        }
        Ok(Self::Off)
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn deduce_git_ignore_ok() {
        let opts = Opts {
            git_ignore: 1,
            ..Opts::default()
        };
        assert_eq!(GitIgnore::deduce(&opts).unwrap(), GitIgnore::CheckAndIgnore);
    }

    #[test]
    fn deduce_git_ignore_conflict() {
        let opts = Opts {
            git_ignore: 1,
            no_git: 1,
            ..Opts::default()
        };
        assert_eq!(GitIgnore::deduce(&opts).unwrap_err(), OptionsError::Conflict("GIT_IGNORE".to_string(), "NO_GIT".to_string()));
    }

    #[test]
    fn deduce_ignore_patterns() {
        let opts = Opts {
            ..Opts::default()
        };
        assert_eq!(IgnorePatterns::deduce(&opts).unwrap(), IgnorePatterns::empty());
    }

    #[test]
    fn deduce_dot_filter_just_files() {
        let opts = Opts {
            ..Opts::default()
        };
        assert_eq!(DotFilter::deduce(&opts, false).unwrap(), DotFilter::JustFiles);
    }

    #[test]
    fn deduce_dot_filter_dotfiles() {
        let opts = Opts {
            all: 1,
            ..Opts::default()
        };
        assert_eq!(DotFilter::deduce(&opts, false).unwrap(), DotFilter::Dotfiles);
    }

    #[test]
    fn deduce_dot_filter_dotfiles_and_dots() {
        let opts = Opts {
            all: 2,
            ..Opts::default()
        };
        assert_eq!(DotFilter::deduce(&opts, false).unwrap(), DotFilter::DotfilesAndDots);
    }

    #[test]
    fn deduce_dot_filter_tree_all_all() {
        let opts = Opts {
            all: 2,
            tree: 1,
            ..Opts::default()
        };
        assert_eq!(DotFilter::deduce(&opts, false).unwrap_err(), OptionsError::TreeAllAll);
    }

    #[test]
    fn deduce_dot_filter_all_all() {
        let opts = Opts {
            all: 2,
            ..Opts::default()
        };
        assert_eq!(DotFilter::deduce(&opts, true).unwrap_err(), OptionsError::Conflict("ALL".to_string(), "ALL".to_string()));
    }
    
    #[test]
    fn deduce_sort_field_name() {
        let opts = Opts {
            sort: Some("name".into()),
            ..Opts::default()
        };
        assert_eq!(SortField::deduce(&opts).unwrap(), SortField::Name(SortCase::AaBbCc));
    }

    #[test]
    fn deduce_sort_field_name_mix_hidden() {
        let opts = Opts {
            sort: Some(".name".into()),
            ..Opts::default()
        };
        assert_eq!(SortField::deduce(&opts).unwrap(), SortField::NameMixHidden(SortCase::AaBbCc));
    }

    #[test]
    fn deduce_sort_field_size() {
        let opts = Opts {
            sort: Some("size".into()),
            ..Opts::default()
        };
        assert_eq!(SortField::deduce(&opts).unwrap(), SortField::Size);
    }

    #[test]
    fn deduce_sort_field_extension() {
        let opts = Opts {
            sort: Some("ext".into()),
            ..Opts::default()
        };
        assert_eq!(SortField::deduce(&opts).unwrap(), SortField::Extension(SortCase::AaBbCc));
    }

    #[test]
    fn deduce_sort_field_modified_date() {
        let opts = Opts {
            sort: Some("date".into()),
            ..Opts::default()
        };
        assert_eq!(SortField::deduce(&opts).unwrap(), SortField::ModifiedDate);
    }

    #[test]
    fn deduce_sort_field_modified_age() {
        let opts = Opts {
            sort: Some("age".into()),
            ..Opts::default()
        };
        assert_eq!(SortField::deduce(&opts).unwrap(), SortField::ModifiedAge);
    }

    #[test]
    fn deduce_sort_field_changed_date() {
        let opts = Opts {
            sort: Some("ch".into()),
            ..Opts::default()
        };
        assert_eq!(SortField::deduce(&opts).unwrap(), SortField::ChangedDate);
    }

    #[test]
    fn deduce_sort_field_accessed_date() {
        let opts = Opts {
            sort: Some("acc".into()),
            ..Opts::default()
        };
        assert_eq!(SortField::deduce(&opts).unwrap(), SortField::AccessedDate);
    }

    #[test]
    fn deduce_sort_field_created_date() {
        let opts = Opts {
            sort: Some("cr".into()),
            ..Opts::default()
        };
        assert_eq!(SortField::deduce(&opts).unwrap(), SortField::CreatedDate);
    }

    #[cfg(unix)]
    #[test]
    fn deduce_sort_field_inode() {
        let opts = Opts {
            sort: Some("inode".into()),
            ..Opts::default()
        };
        assert_eq!(SortField::deduce(&opts).unwrap(), SortField::FileInode);
    }

    #[test]
    fn deduce_sort_field_file_type() {
        let opts = Opts {
            sort: Some("type".into()),
            ..Opts::default()
        };
        assert_eq!(SortField::deduce(&opts).unwrap(), SortField::FileType);
    }

    #[test]
    fn deduce_sort_field_unsorted() {
        let opts = Opts {
            sort: Some("none".into()),
            ..Opts::default()
        };
        assert_eq!(SortField::deduce(&opts).unwrap(), SortField::Unsorted);
    }

    #[test]
    fn deduce_sort_field_bad_argument() {
        let opts = Opts {
            sort: Some("bad".into()),
            ..Opts::default()
        };
        assert_eq!(SortField::deduce(&opts).unwrap_err(), OptionsError::BadArgument("SORT".to_string(), "bad".to_string()));
    }
}
