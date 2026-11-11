// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
//! Filtering and sorting the list of files before displaying them.

use std::cmp::Ordering;
use std::ffi::OsStr;
use std::iter::FromIterator;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
use std::path::{Component, Path, PathBuf};

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

    /// Paths under which the Git-ignore filter is turned off, given with
    /// `--no-git-ignore`. Files listed within one of these directories are
    /// shown even when Git would ignore them.
    pub no_git_ignore: NoGitIgnore,

    /// Whether to ignore symlinks
    pub no_symlinks: bool,

    /// Whether to explicitly show symlinks
    pub show_symlinks: bool,
}

impl FileFilter {
    /// Remove every file in the given vector that does *not* pass the
    /// filter predicate for files found inside a directory.
    #[rustfmt::skip]
    pub fn filter_child_files(&self, is_recurse: bool, files: &mut Vec<File<'_>>) {
        use FileFilterFlags::{NoSymlinks, OnlyDirs, OnlyFiles, ShowSymlinks};

        files.retain(|f| !self.ignore_patterns.is_ignored(&f.name));
        files.retain(|f| {
            match (
                self.flags.contains(&OnlyDirs),
                self.flags.contains(&OnlyFiles),
                self.flags.contains(&NoSymlinks),
                self.flags.contains(&ShowSymlinks),
            ) {
                (true, false, false, false) => f.is_directory(),
                (true, false, true, false) => f.is_directory(),
                (true, false, false, true) => f.is_directory() || f.points_to_directory(),
                (false, true, false, false) => if is_recurse { true } else {f.is_file() },
                (false, true, false, true) => if is_recurse { true } else { f.is_file() || f.is_link() && !f.points_to_directory()
                },
                (false, false, true, false) => !f.is_link(),
                _ => true,
            }
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

/// The set of paths given with `--no-git-ignore`, under which the
/// `--git-ignore` feature is disabled: files within them are listed even
/// when Git would ignore them. This exists for directories that live inside
/// a repository but whose contents should always stay visible, such as
/// Homebrew’s `/opt/homebrew` prefix.
///
/// The paths are made absolute when they’re parsed. Each one is kept in two
/// forms: its lexical path with any `.` or `..` components folded away but
/// symlinked names left intact, so the name of a symlinked exclusion still
/// matches its entry in a relative directory listing, and the fully
/// resolved path, so listings reached through symlinks still match. The
/// resolved form is kept only when it differs from the anchored one.
#[derive(PartialEq, Eq, Debug, Default, Clone)]
pub struct NoGitIgnore {
    paths: Vec<PathBuf>,
}

impl NoGitIgnore {
    /// Create a set of exclusions from the user-supplied paths.
    pub fn new<I, P>(inputs: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        Self {
            paths: inputs.into_iter().fold(Vec::new(), |mut paths, input| {
                let path = anchor_path(input.as_ref());
                let canonical = absolutise(&path);
                if canonical != path {
                    paths.push(canonical);
                }
                paths.push(path);
                paths
            }),
        }
    }

    /// Whether no paths were given, letting callers skip the checks entirely.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.paths.is_empty()
    }

    /// The number of paths this set was built from.
    #[must_use]
    pub fn len(&self) -> usize {
        self.paths.len()
    }

    /// Whether the directory at the given (absolutised) path lies within one
    /// of the excluded paths, in which case every file listed from it should
    /// stay visible.
    #[must_use]
    pub fn covers_dir(&self, dir: &Path) -> bool {
        self.paths.iter().any(|p| dir.starts_with(p))
    }

    /// Whether the child entry with the given name, listed from the directory
    /// at the given (absolutised) path, is or contains one of the excluded
    /// paths — covering the case where the exclusion itself would be
    /// Git-ignored in the parent listing.
    #[must_use]
    pub fn covers_child(&self, dir: &Path, name: &OsStr) -> bool {
        self.paths.iter().any(|p| {
            p.strip_prefix(dir)
                .is_ok_and(|rest| rest.components().next().map(|c| c.as_os_str()) == Some(name))
        })
    }
}

/// Make a path absolute, resolving symlinks if the filesystem will let us,
/// and falling back to the path unchanged if it won’t.
pub fn absolutise(path: &Path) -> PathBuf {
    std::fs::canonicalize(path)
        .or_else(|_| std::path::absolute(path))
        .unwrap_or_else(|_| path.to_path_buf())
}

/// Make a path absolute and fold away any `.` or `..` components purely
/// lexically, so `strip_prefix` against a canonicalised listing directory
/// can succeed, while symlinked names stay intact so a path still matches
/// the entry it was named by. Where the `..` would have crossed a symlink
/// and led somewhere the fold doesn’t predict, the fully resolved form kept
/// alongside the anchored one covers the match instead.
fn anchor_path(path: &Path) -> PathBuf {
    let absolute = std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf());

    // all of the below can be replaced by Path::normalize_lexically when that stabilizes
    let mut anchored = PathBuf::new();
    for component in absolute.components() {
        match component {
            Component::ParentDir => {
                anchored.pop();
            }
            Component::CurDir => {}
            other => anchored.push(other),
        }
    }
    anchored
}

#[cfg(test)]
mod test_no_git_ignore {
    use super::*;

    fn exclusions(paths: &[&str]) -> NoGitIgnore {
        NoGitIgnore {
            paths: paths.iter().map(PathBuf::from).collect(),
        }
    }

    /// A fresh scratch directory, canonicalised so symlinks earlier in the path
    /// don’t skew comparisons against absolutised exemptions.
    #[cfg(unix)]
    fn scratch_base(tag: &str) -> PathBuf {
        let base =
            std::env::temp_dir().join(format!("eza-no-git-ignore-{}-{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).unwrap();
        std::fs::canonicalize(&base).unwrap()
    }

    #[test]
    fn empty_covers_nothing() {
        let ex = NoGitIgnore::default();
        assert!(ex.is_empty());
        assert!(!ex.covers_dir(Path::new("/opt/homebrew")));
        assert!(!ex.covers_child(Path::new("/opt"), OsStr::new("homebrew")));
    }

    #[test]
    fn covers_exact_and_nested_paths() {
        let ex = exclusions(&["/opt/homebrew"]);
        assert!(ex.covers_dir(Path::new("/opt/homebrew")));
        assert!(ex.covers_dir(Path::new("/opt/homebrew/Library/Cask")));
        assert!(!ex.covers_dir(Path::new("/opt")));
        assert!(!ex.covers_dir(Path::new("/opt/homebrew-other")));
    }

    #[test]
    fn covers_child_before_recursing() {
        let ex = exclusions(&["/opt/homebrew"]);
        // The `homebrew` entry in a listing of `/opt` is covered itself, so
        // it’s not filtered out of the parent listing.
        assert!(ex.covers_child(Path::new("/opt"), OsStr::new("homebrew")));
        assert!(!ex.covers_child(Path::new("/opt"), OsStr::new("other")));
        assert!(!ex.covers_child(Path::new("/opt/homebrew"), OsStr::new("Library")));
    }

    #[test]
    fn multiple_exclusions() {
        let ex = exclusions(&["/a/b", "/c"]);
        assert!(ex.covers_dir(Path::new("/a/b/x")));
        assert!(ex.covers_child(Path::new("/"), OsStr::new("c")));
        assert!(!ex.covers_dir(Path::new("/d")));
    }

    #[test]
    fn new_makes_relative_paths_absolute() {
        let ex = NoGitIgnore::new(["some/relative/dir"]);
        let first = ex.paths.first().unwrap();
        assert!(first.is_absolute());
        assert!(first.ends_with("some/relative/dir"));
        // The path doesn’t exist, so there’s nothing resolved to keep.
        assert!(ex.paths.len() == 1);
    }

    #[cfg(unix)]
    #[test]
    fn new_folds_dot_and_parent_components() {
        let ex = NoGitIgnore::new(["./some/../relative/dir"]);
        // Folded lexically: no `.` or `..` survive to break `strip_prefix`,
        // and the anchored form is the one child names are matched from.
        let anchored = ex.paths.first().unwrap();
        let as_str = anchored.to_str().unwrap();
        assert!(
            anchored.ends_with(Path::new("relative/dir")),
            "got {anchored:?}"
        );
        assert!(!as_str.contains("/../"), "got {anchored:?}");
        assert!(!as_str.contains("/./"), "got {anchored:?}");
    }

    #[cfg(windows)]
    #[test]
    fn new_folds_dot_and_parent_components() {
        let ex = NoGitIgnore::new(["./some/../relative/dir"]);
        // Folded lexically: no `.` or `..` survive to break `strip_prefix`,
        // and the anchored form is the one child names are matched from.
        let anchored = ex.paths.first().unwrap();
        let as_str = anchored.to_str().unwrap();
        assert!(
            anchored.ends_with(Path::new("relative/dir")),
            "got {anchored:?}"
        );
        assert!(!as_str.contains("\\..\\"), "got {anchored:?}");
        assert!(!as_str.contains("\\.\\"), "got {anchored:?}");
    }

    /// An exemption naming a symlink must survive the parent listing under
    /// the link’s own name: canonicalising eagerly resolves the link away,
    /// leaving `covers_child` with only the target, which it can’t match.
    #[cfg(unix)]
    #[test]
    fn new_keeps_symlink_name_for_child_matching() {
        let base = scratch_base("child");

        std::fs::create_dir(base.join("target")).unwrap();
        std::os::unix::fs::symlink(base.join("target"), base.join("link")).unwrap();

        let ex = NoGitIgnore::new([base.join("link")]);

        // The `link` entry in a listing of `base` must stay visible, under
        // the name the user gave rather than the target's...
        assert!(
            ex.covers_child(&base, OsStr::new("link")),
            "link’s own name missing from paths {:?}",
            ex.paths
        );
        // The target’s own entry matches as well whenever it’s a sibling
        // in the same listing: the resolved form still names it, keeping the
        // exempted contents visible under either spelling.
        assert!(ex.covers_child(&base, OsStr::new("target")));
        // …and everything under it stays visible once we descend into it,
        // where the directory path is canonicalised to the target.
        let canonical_link = std::fs::canonicalize(base.join("link")).unwrap();
        assert!(ex.covers_dir(&canonical_link));

        std::fs::remove_dir_all(&base).unwrap();
    }

    /// An exemption *under* a symlinked directory must still match both
    /// listings: the parent listing that sees the link’s name, and listings
    /// reached through the link, whose paths resolve to the target.
    #[cfg(unix)]
    #[test]
    fn new_matches_nested_exemptions_through_symlinks() {
        let base = scratch_base("nested");
        std::fs::create_dir_all(base.join("target/sub")).unwrap();
        std::os::unix::fs::symlink(base.join("target"), base.join("link")).unwrap();

        let ex = NoGitIgnore::new([base.join("link/sub")]);

        assert!(ex.covers_child(&base, OsStr::new("link")));
        assert!(ex.covers_dir(&base.join("target/sub")));
        assert!(!ex.covers_dir(&base.join("other")));

        std::fs::remove_dir_all(&base).unwrap();
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
