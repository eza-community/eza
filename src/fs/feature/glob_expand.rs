// SPDX-FileCopyrightText: 2026 Konstantin Glukhov
// SPDX-License-Identifier: MIT
#![cfg(windows)]

use glob::{MatchOptions, glob_with};
use std::borrow::Cow;
use std::ffi::OsStr;

fn has_wildcard(s: &str) -> bool {
    s.contains('*') || s.contains('?') || s.contains('[')
}

/// Expand wildcard arguments (globs) into matching filesystem paths on demand.
///
/// Returns a `Vec<Cow<'a, OsStr>>`:
/// - `Cow::Borrowed`: arguments kept as-is (not expanded, no matches, or invalid pattern).
/// - `Cow::Owned`: paths produced by glob expansion.
///
/// Behavior:
/// - Only arguments containing '*', '?', or '[' are expanded using the `glob` crate.
///   If a pattern yields no matches, nothing is returned; if it is invalid, the literal
///   argument is preserved.
/// - Matching is case-insensitive.
pub fn expand_args<'a>(args: &[&'a OsStr]) -> Vec<Cow<'a, OsStr>> {
    let mut out: Vec<Cow<'a, OsStr>> = Vec::new();

    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    for &a in args {
        let path_str = a.to_string_lossy();

        if !has_wildcard(&path_str) {
            out.push(Cow::Borrowed(a));
            continue;
        }

        match glob_with(&path_str, options) {
            Ok(iter) => {
                // If it's a valid glob pattern, we pull matches into our vector
                out.extend(
                    iter.filter_map(Result::ok)
                        .map(|p| Cow::Owned(p.into_os_string())),
                );
                // If a valid pattern yields no matches, let it return nothing.
            }
            // If the pattern structure is invalid (PatternError), the literal argument is preserved.
            Err(_) => out.push(Cow::Borrowed(a)),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::ffi::OsStr;
    use std::fs::File;
    use tempfile::TempDir;

    #[test]
    fn no_expand_when_disabled() {
        let args: Vec<&OsStr> = vec![OsStr::new("alpha"), OsStr::new("beta")];
        let refs = expand_args(&args);
        assert_eq!(refs.len(), 2);
        assert_eq!(refs[0].to_string_lossy(), "alpha");
        assert_eq!(refs[1].to_string_lossy(), "beta");
    }

    #[test]
    fn expands_matching_glob() {
        let td = TempDir::new().expect("tempdir");

        // 1. Single source of truth: define all test files in an array
        let file_names = ["one.txt", "two.txt"];

        // 2. Loop through the array to construct paths and create the files on disk
        for name in &file_names {
            let path = td.path().join(name);
            File::create(&path).unwrap();
        }

        // 3. Prepare the glob wildcard pattern and run the argument expansion
        let pattern = format!("{}/{}.txt", td.path().display(), "*");
        let args = vec![OsStr::new(&pattern)];
        let refs = expand_args(&args);

        // 4. Extract just the resulting filenames into a HashSet for order-independent comparison
        let returned: HashSet<String> = refs
            .iter()
            .map(|r| {
                std::path::Path::new(r)
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();

        // 5. Build the expected HashSet directly from the original array
        let expected: HashSet<String> = file_names.iter().map(|&s| s.to_string()).collect();

        // 6. Assert that all expected files were found
        assert!(
            returned.is_superset(&expected),
            "returned {:?} does not contain expected {:?}",
            returned,
            expected
        );
    }

    #[test]
    fn handles_mixed_arguments() {
        let td = TempDir::new().expect("tempdir");
        let file_names = ["alpha.txt"];
        for name in &file_names {
            File::create(td.path().join(name)).unwrap();
        }

        let pattern = format!("{}/*.txt", td.path().display());
        // One literal arg, one wildcard arg
        let args = vec![OsStr::new("--verbose"), OsStr::new(&pattern)];
        let refs = expand_args(&args);

        assert_eq!(refs.len(), 2);
        assert_eq!(refs[0].to_string_lossy(), "--verbose"); // Borrowed as-is
        assert!(refs[1].to_string_lossy().contains("alpha.txt")); // Expanded
    }
    #[test]
    fn expands_different_wildcard_characters() {
        let td = TempDir::new().expect("tempdir");
        // Create files that match different types of wildcards
        let file_names = ["file1.txt", "fileA.txt"];
        for name in &file_names {
            File::create(td.path().join(name)).unwrap();
        }

        // '?' matches exactly one character
        let pattern = format!("{}/file?.txt", td.path().display());
        let args = vec![OsStr::new(&pattern)];
        let refs = expand_args(&args);

        assert_eq!(refs.len(), 2);
    }

    #[test]
    fn non_existent_directory_returns_nothing() {
        let pattern = "C:\\this\\folder\\does\\not\\exist\\*.txt";
        let args = vec![OsStr::new(pattern)];
        let refs = expand_args(&args);

        assert_eq!(refs.len(), 0);
    }

    #[test]
    fn invalid_pattern_preserves_literal() {
        // A deliberately invalid pattern to cause glob::glob to return Err
        let args = vec![OsStr::new("[[")];
        let refs = expand_args(&args);
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].to_string_lossy(), "[[");
    }
    #[test]
    fn no_matches_returns_nothing() {
        let td = TempDir::new().expect("tempdir");
        let pattern = format!("{}/no-such-*.zzz", td.path().display());
        let args = vec![OsStr::new(&pattern)];
        let refs = expand_args(&args);

        assert_eq!(refs.len(), 0);
    }
}
