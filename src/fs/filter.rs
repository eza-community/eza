// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
//! Filtering and sorting the list of files before displaying them.

use std::cmp::Ordering;
use std::iter::FromIterator;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

use crate::fs::DotFilter;
use crate::fs::File;

/// Flags used to manage the **file filter** process
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum FileFilterFlags {
    /// Whether to reverse the sorting order. This would sort the largest
    /// files first, or files starting with Z, or the most-recently-changed
    /// ones, depending on the sort field.
    Reverse,

    /// Whether to only show directories.
    OnlyDirs,

    /// Whether to only show files.
    OnlyFiles,

    /// Whether to ignore symlinks
    NoSymlinks,

    /// Whether to explicitly show symlinks
    ShowSymlinks,

    /// Whether directories should be listed first, and other types of file
    /// second. Some users prefer it like this.
    ListDirsFirst,

    /// Whether directories should be listed as the last items, after other
    /// types of file. Some users prefer it like this.
    ListDirsLast,
}

/// The **file filter** processes a list of files before displaying them to
/// the user, by removing files they don’t want to see, and putting the list
/// in the desired order.
///
/// Usually a user does not want to see *every* file in the list. The most
/// common case is to remove files starting with `.`, which are designated
/// as ‘hidden’ files.
///
/// The special files `.` and `..` files are not actually filtered out, but
/// need to be inserted into the list, in a special case.
///
/// The filter also governs sorting the list. After being filtered, pairs of
/// files are compared and sorted based on the result, with the sort field
/// performing the comparison.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FileFilter {
    /// The metadata field to sort by.
    pub sort_field: SortField,

    // Flags that the file filtering process follow
    pub flags: Vec<FileFilterFlags>,

    /// Which invisible “dot” files to include when listing a directory.
    ///
    /// Files starting with a single “.” are used to determine “system” or
    /// “configuration” files that should not be displayed in a regular
    /// directory listing, and the directory entries “.” and “..” are
    /// considered extra-special.
    ///
    /// This came about more or less by a complete historical accident,
    /// when the original `ls` tried to hide `.` and `..`:
    ///
    /// [Linux History: How Dot Files Became Hidden Files](https://linux-audit.com/linux-history-how-dot-files-became-hidden-files/)
    pub dot_filter: DotFilter,

    /// Glob patterns to ignore. Any file name that matches *any* of these
    /// patterns won’t be displayed in the list.
    pub ignore_patterns: IgnorePatterns,

    /// Whether to ignore Git-ignored patterns.
    pub git_ignore: GitIgnore,

    /// Whether to ignore symlinks
    pub no_symlinks: bool,

    /// Whether to explicitly show symlinks
    pub show_symlinks: bool,
}

impl FileFilter {
    /// Remove every file in the given vector that does *not* pass the
    /// filter predicate for files found inside a directory.
    pub fn filter_child_files(&self, is_recurse: bool, files: &mut Vec<File<'_>>) {
        use FileFilterFlags::{NoSymlinks, OnlyDirs, OnlyFiles, ShowSymlinks};

        let only_dirs = self.flags.contains(&OnlyDirs);
        let only_files = self.flags.contains(&OnlyFiles);
        let no_symlinks = self.flags.contains(&NoSymlinks);
        let show_symlinks = self.flags.contains(&ShowSymlinks);

        files.retain(|f| !self.ignore_patterns.is_ignored(&f.name));
        // Each active flag applies its own constraint independently, and a file
        // must satisfy all of them (rather than matching on flag combinations,
        // which silently let unlisted combinations through unfiltered).
        files.retain(|f| {
            if only_dirs {
                let is_dir = if show_symlinks {
                    f.points_to_directory()
                } else {
                    f.is_directory()
                };
                if !is_dir {
                    return false;
                }
            }
            if only_files && !is_recurse {
                let is_file = if show_symlinks {
                    f.is_file() || (f.is_link() && !f.points_to_directory())
                } else {
                    f.is_file()
                };
                if !is_file {
                    return false;
                }
            }
            if no_symlinks && f.is_link() {
                return false;
            }
            true
        });
    }

    /// Remove every file in the given vector that does *not* pass the
    /// filter predicate for file names specified on the command-line.
    ///
    /// The rules are different for these types of files than the other
    /// type because the ignore rules can be used with globbing. For
    /// example, running `exa -I='*. tmp' .vimrc` shouldn’t filter out the
    /// dotfile, because it’s been directly specified. But running
    /// `exa -I='*.ogg' music/*` should filter out the ogg files obtained
    /// from the glob, even though the globbing is done by the shell!
    pub fn filter_argument_files(&self, files: &mut Vec<File<'_>>) {
        files.retain(|f| !self.ignore_patterns.is_ignored(&f.name));
    }

    /// Sort the files in the given vector based on the sort field option.
    pub fn sort_files<'a, F>(&self, files: &mut [F])
    where
        F: AsRef<File<'a>>,
    {
        files.sort_by(|a, b| self.sort_field.compare_files(a.as_ref(), b.as_ref()));

        if self.flags.contains(&FileFilterFlags::Reverse) {
            files.reverse();
        }

        if self.flags.contains(&FileFilterFlags::ListDirsFirst) {
            // This relies on the fact that `sort_by` is *stable*: it will keep
            // adjacent elements next to each other.
            files.sort_by(|a, b| {
                b.as_ref()
                    .points_to_directory()
                    .cmp(&a.as_ref().points_to_directory())
            });
        } else if self.flags.contains(&FileFilterFlags::ListDirsLast) {
            files.sort_by(|a, b| {
                a.as_ref()
                    .points_to_directory()
                    .cmp(&b.as_ref().points_to_directory())
            });
        }
    }
}

/// User-supplied field to sort by.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum SortField {
    /// Don’t apply any sorting. This is usually used as an optimisation in
    /// scripts, where the order doesn’t matter.
    Unsorted,

    /// The file name. This is the default sorting.
    Name(SortCase),

    /// The file’s extension, with extensionless files being listed first.
    Extension(SortCase),

    /// The file’s size, in bytes.
    Size,

    /// The file’s inode, which usually corresponds to the order in which
    /// files were created on the filesystem, more or less.
    #[cfg(unix)]
    FileInode,

    /// The time the file was modified (the “mtime”).
    ///
    /// As this is stored as a Unix timestamp, rather than a local time
    /// instance, the time zone does not matter and will only be used to
    /// display the timestamps, not compare them.
    ModifiedDate,

    /// The time the file was accessed (the “atime”).
    ///
    /// Oddly enough, this field rarely holds the *actual* accessed time.
    /// Recording a read time means writing to the file each time it’s read
    /// slows the whole operation down, so many systems will only update the
    /// timestamp in certain circumstances. This has become common enough that
    /// it’s now expected behaviour!
    /// <https://unix.stackexchange.com/a/8842>
    AccessedDate,

    /// The time the file was changed (the “ctime”).
    ///
    /// This field is used to mark the time when a file’s metadata
    /// changed — its permissions, owners, or link count.
    ///
    /// In original Unix, this was, however, meant as creation time.
    /// <https://www.bell-labs.com/usr/dmr/www/cacm.html>
    ChangedDate,

    /// The time the file was created (the “btime” or “birthtime”).
    CreatedDate,

    /// The type of the file: directories, links, pipes, regular, files, etc.
    ///
    /// Files are ordered according to the `PartialOrd` implementation of
    /// `fs::fields::Type`, so changing that will change this.
    FileType,

    /// The “age” of the file, which is the time it was modified sorted
    /// backwards. The reverse of the `ModifiedDate` ordering!
    ///
    /// It turns out that listing the most-recently-modified files first is a
    /// common-enough use case that it deserves its own variant. This would be
    /// implemented by just using the modified date and setting the reverse
    /// flag, but this would make reversing *that* output not work, which is
    /// bad, even though that’s kind of nonsensical. So it’s its own variant
    /// that can be reversed like usual.
    ModifiedAge,

    /// The file's name, however if the name of the file begins with `.`
    /// ignore the leading `.` and then sort as Name
    NameMixHidden(SortCase),
}

/// Whether a field should be sorted case-sensitively or case-insensitively.
/// This determines which of the `natord` functions to use.
///
/// I kept on forgetting which one was sensitive and which one was
/// insensitive. Would a case-sensitive sort put capital letters first because
/// it takes the case of the letters into account, or intermingle them with
/// lowercase letters because it takes the difference between the two cases
/// into account? I gave up and just named these two variants after the
/// effects they have.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum SortCase {
    /// Sort files case-sensitively with uppercase first, with ‘A’ coming
    /// before ‘a’.
    ABCabc,

    /// Sort files case-insensitively, with ‘A’ being equal to ‘a’.
    AaBbCc,
}

impl SortField {
    /// Compares two files to determine the order they should be listed in,
    /// depending on the search field.
    ///
    /// The `natord` crate is used here to provide a more *natural* sorting
    /// order than just sorting character-by-character. This splits filenames
    /// into groups between letters and numbers, and then sorts those blocks
    /// together, so `file10` will sort after `file9`, instead of before it
    /// because of the `1`.
    pub fn compare_files(self, a: &File<'_>, b: &File<'_>) -> Ordering {
        use self::SortCase::{ABCabc, AaBbCc};

        #[rustfmt::skip]
        return match self {
            Self::Unsorted  => Ordering::Equal,

            Self::Name(ABCabc)  => natord::compare(&a.name, &b.name),
            Self::Name(AaBbCc)  => natord::compare_ignore_case(&a.name, &b.name),

            Self::Size          => a.length().cmp(&b.length()),

            #[cfg(unix)]
            Self::FileInode     => {
                a.metadata().map_or(0, MetadataExt::ino)
                    .cmp(&b.metadata().map_or(0, MetadataExt::ino))
            }
            Self::ModifiedDate  => a.modified_time().cmp(&b.modified_time()),
            Self::AccessedDate  => a.accessed_time().cmp(&b.accessed_time()),
            Self::ChangedDate   => a.changed_time().cmp(&b.changed_time()),
            Self::CreatedDate   => a.created_time().cmp(&b.created_time()),
            Self::ModifiedAge   => b.modified_time().cmp(&a.modified_time()),  // flip b and a
            Self::FileType => match a.type_char().cmp(&b.type_char()) { // todo: this recomputes
                Ordering::Equal  => natord::compare(&a.name, &b.name),
                order            => order,
            },

            Self::Extension(case) => {
                // Ignore extensions for directories when sorting.
                let left = if a.is_directory() { &None } else { &a.ext };
                let right = if b.is_directory() { &None } else { &b.ext };
                match (left.cmp(right), case) {
                    (Ordering::Equal, ABCabc)  => natord::compare(&a.name, &b.name),
                    (Ordering::Equal, AaBbCc)  => natord::compare_ignore_case(&a.name, &b.name),
                    (order, _)                 => order,
                }
            },

            Self::NameMixHidden(ABCabc) => natord::compare(
                Self::strip_dot(&a.name),
                Self::strip_dot(&b.name)
            ),
            Self::NameMixHidden(AaBbCc) => natord::compare_ignore_case(
                Self::strip_dot(&a.name),
                Self::strip_dot(&b.name)
            ),
        };
    }

    fn strip_dot(n: &str) -> &str {
        match n.strip_prefix('.') {
            Some(s) => s,
            None => n,
        }
    }
}

/// The **ignore patterns** are a list of globs that are tested against
/// each filename, and if any of them match, that file isn’t displayed.
/// This lets a user hide, say, text files by ignoring `*.txt`.
#[derive(PartialEq, Eq, Default, Debug, Clone)]
pub struct IgnorePatterns {
    patterns: Vec<glob::Pattern>,
}

impl FromIterator<glob::Pattern> for IgnorePatterns {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = glob::Pattern>,
    {
        let patterns = iter.into_iter().collect();
        Self { patterns }
    }
}

impl IgnorePatterns {
    /// Create a new list from the input glob strings, turning the inputs that
    /// are valid glob patterns into an `IgnorePatterns`. The inputs that
    /// don’t parse correctly are returned separately.
    pub fn parse_from_iter<'a, I: IntoIterator<Item = &'a str>>(
        iter: I,
    ) -> (Self, Vec<glob::PatternError>) {
        let iter = iter.into_iter();

        // Almost all glob patterns are valid, so it’s worth pre-allocating
        // the vector with enough space for all of them.
        let mut patterns = match iter.size_hint() {
            (_, Some(count)) => Vec::with_capacity(count),
            _ => Vec::new(),
        };

        // Similarly, assume there won’t be any errors.
        let mut errors = Vec::new();

        for input in iter {
            match glob::Pattern::new(input) {
                Ok(pat) => patterns.push(pat),
                Err(e) => errors.push(e),
            }
        }

        (Self { patterns }, errors)
    }

    /// Create a new empty set of patterns that matches nothing.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    /// Test whether the given file should be hidden from the results.
    fn is_ignored(&self, file: &str) -> bool {
        self.patterns.iter().any(|p| p.matches(file))
    }
}

/// Whether to ignore or display files that Git would ignore.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum GitIgnore {
    /// Ignore files that Git would ignore.
    CheckAndIgnore,

    /// Display files, even if Git would ignore them.
    Off,
}

#[cfg(test)]
#[cfg(unix)]
mod test_filter_child_files {
    use super::*;
    use std::fs;
    use std::os::unix::fs::symlink;
    use std::path::PathBuf;

    /// A self-cleaning temp directory, since this crate has no existing
    /// dependency on a tempfile crate.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new(name: &str) -> Self {
            let dir =
                std::env::temp_dir().join(format!("eza-filter-test-{name}-{}", std::process::id()));
            let _ = fs::remove_dir_all(&dir);
            fs::create_dir_all(&dir).unwrap();
            Self(dir)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn make_filter(flags: Vec<FileFilterFlags>) -> FileFilter {
        FileFilter {
            sort_field: SortField::Unsorted,
            flags,
            dot_filter: DotFilter::default(),
            ignore_patterns: IgnorePatterns::empty(),
            git_ignore: GitIgnore::Off,
            no_symlinks: false,
            show_symlinks: false,
        }
    }

    fn names<'a>(files: &'a [File<'a>]) -> Vec<&'a str> {
        let mut names: Vec<&str> = files.iter().map(|f| f.name.as_str()).collect();
        names.sort_unstable();
        names
    }

    /// Builds a real directory containing a real subdirectory (`dir`), a
    /// real regular file (`file`), and a real symlink to that subdirectory
    /// (`link`), and returns `File`s for all three (in that order).
    fn fixture(temp: &TempDir) -> Vec<File<'static>> {
        let dir_path = temp.0.join("dir");
        let file_path = temp.0.join("file");
        let link_path = temp.0.join("link");
        fs::create_dir(&dir_path).unwrap();
        fs::write(&file_path, b"hi").unwrap();
        symlink(&dir_path, &link_path).unwrap();

        vec![
            File::from_args(dir_path, None, None, false, false, None),
            File::from_args(file_path, None, None, false, false, None),
            File::from_args(link_path, None, None, false, false, None),
        ]
    }

    #[test]
    fn only_files_with_no_symlinks_excludes_dirs_and_symlinks() {
        // Regression test for https://github.com/eza-community/eza/issues/1927:
        // this combination used to fall through to the catch-all `_ => true`
        // arm and disable filtering entirely.
        let temp = TempDir::new("only-files-no-symlinks");
        let mut files = fixture(&temp);
        let filter = make_filter(vec![
            FileFilterFlags::OnlyFiles,
            FileFilterFlags::NoSymlinks,
        ]);

        filter.filter_child_files(false, &mut files);

        assert_eq!(names(&files), vec!["file"]);
    }

    #[test]
    fn only_dirs_with_no_symlinks_and_show_symlinks_excludes_the_symlink() {
        // NoSymlinks always wins over ShowSymlinks: a symlink-to-dir is not
        // shown even though OnlyDirs+ShowSymlinks alone would include it.
        let temp = TempDir::new("only-dirs-no-symlinks-show-symlinks");
        let mut files = fixture(&temp);
        let filter = make_filter(vec![
            FileFilterFlags::OnlyDirs,
            FileFilterFlags::NoSymlinks,
            FileFilterFlags::ShowSymlinks,
        ]);

        filter.filter_child_files(false, &mut files);

        assert_eq!(names(&files), vec!["dir"]);
    }

    #[test]
    fn no_symlinks_with_show_symlinks_still_excludes_symlinks() {
        let temp = TempDir::new("no-symlinks-show-symlinks");
        let mut files = fixture(&temp);
        let filter = make_filter(vec![
            FileFilterFlags::NoSymlinks,
            FileFilterFlags::ShowSymlinks,
        ]);

        filter.filter_child_files(false, &mut files);

        assert_eq!(names(&files), vec!["dir", "file"]);
    }

    #[test]
    fn only_dirs_with_show_symlinks_includes_symlink_to_dir() {
        let temp = TempDir::new("only-dirs-show-symlinks");
        let mut files = fixture(&temp);
        let filter = make_filter(vec![
            FileFilterFlags::OnlyDirs,
            FileFilterFlags::ShowSymlinks,
        ]);

        filter.filter_child_files(false, &mut files);

        assert_eq!(names(&files), vec!["dir", "link"]);
    }
}

#[cfg(test)]
mod test_ignores {
    use super::*;

    #[test]
    fn empty_matches_nothing() {
        let pats = IgnorePatterns::empty();
        assert!(!pats.is_ignored("nothing"));
        assert!(!pats.is_ignored("test.mp3"));
    }

    #[test]
    fn ignores_a_glob() {
        let (pats, fails) = IgnorePatterns::parse_from_iter(vec!["*.mp3"]);
        assert!(fails.is_empty());
        assert!(!pats.is_ignored("nothing"));
        assert!(pats.is_ignored("test.mp3"));
    }

    #[test]
    fn ignores_an_exact_filename() {
        let (pats, fails) = IgnorePatterns::parse_from_iter(vec!["nothing"]);
        assert!(fails.is_empty());
        assert!(pats.is_ignored("nothing"));
        assert!(!pats.is_ignored("test.mp3"));
    }

    #[test]
    fn ignores_both() {
        let (pats, fails) = IgnorePatterns::parse_from_iter(vec!["nothing", "*.mp3"]);
        assert!(fails.is_empty());
        assert!(pats.is_ignored("nothing"));
        assert!(pats.is_ignored("test.mp3"));
    }
}
