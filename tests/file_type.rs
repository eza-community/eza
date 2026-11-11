// SPDX-FileCopyrightText: 2026 eza contributors
// SPDX-License-Identifier: EUPL-1.2

#![cfg(unix)]

use std::fs;
use std::os::unix::fs::{PermissionsExt, symlink};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use eza::fs::fields::{Size, Type};
use eza::fs::filter::{
    FileFilter, FileFilterFlags, GitIgnore, IgnorePatterns, SortCase, SortField,
};
use eza::fs::{DotFilter, File};

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!(
            "file-type-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&path).unwrap();
        fs::create_dir(path.join("directory")).unwrap();
        fs::write(path.join("regular"), b"target contents").unwrap();
        fs::set_permissions(path.join("regular"), fs::Permissions::from_mode(0o640)).unwrap();
        symlink("regular", path.join("link")).unwrap();
        symlink("directory", path.join("dir-link")).unwrap();
        symlink("missing", path.join("broken-link")).unwrap();
        let fifo =
            std::ffi::CString::new(path.join("pipe").as_os_str().as_encoded_bytes()).unwrap();
        // SAFETY: fifo is a valid, NUL-terminated pathname for this fixture.
        assert_eq!(unsafe { libc::mkfifo(fifo.as_ptr(), 0o600) }, 0);
        Self(path)
    }

    fn file(&self, name: &str, hint: &str, deref: bool) -> File<'static> {
        File::from_args(
            self.0.join(name),
            None,
            None,
            deref,
            false,
            Some(fs::symlink_metadata(self.0.join(hint)).unwrap().file_type()),
        )
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

fn filter(flags: Vec<FileFilterFlags>) -> FileFilter {
    FileFilter {
        sort_field: SortField::Name(SortCase::AaBbCc),
        flags,
        dot_filter: DotFilter::JustFiles,
        ignore_patterns: IgnorePatterns::empty(),
        git_ignore: GitIgnore::Off,
        no_symlinks: false,
        show_symlinks: false,
    }
}

#[test]
fn metadata_overrides_directory_entry_type() {
    let fixture = Fixture::new();
    for (name, hint, expected) in [
        ("directory", "pipe", Type::Directory),
        ("link", "regular", Type::Link),
        ("broken-link", "regular", Type::Link),
        ("regular", "directory", Type::File),
    ] {
        let file = fixture.file(name, hint, false);
        // Exercise the hint before metadata is fetched, as a short view might.
        let _ = file.type_char();
        assert!(file.metadata.get().is_none());
        file.metadata().unwrap();
        assert!(file.type_char() == expected, "incorrect type for {name}");
    }
}

#[test]
fn matching_types_and_name_only_filtering_remain_lazy() {
    let fixture = Fixture::new();
    let mut files: Vec<_> = ["regular", "directory", "link", "pipe"]
        .iter()
        .map(|name| fixture.file(name, name, false))
        .collect();
    filter(vec![]).filter_child_files(false, &mut files);
    filter(vec![]).sort_files(&mut files);
    for file in files {
        let hint = file.type_char();
        assert!(file.metadata.get().is_none());
        file.metadata().unwrap();
        assert!(file.type_char() == hint);
    }
}

#[test]
fn failed_metadata_keeps_directory_entry_fallback() {
    let fixture = Fixture::new();
    let file = fixture.file("missing", "directory", false);
    assert!(file.metadata().is_err());
    assert!(file.is_directory());
    let unknown = File::from_args(fixture.0.join("missing"), None, None, false, false, None);
    assert!(!unknown.is_directory());
    assert!(!unknown.is_file());
    assert!(!unknown.is_link());
}

#[test]
fn type_filters_validate_entries_before_selecting_files() {
    use FileFilterFlags::{NoSymlinks, OnlyDirs, OnlyFiles, ShowSymlinks};
    let fixture = Fixture::new();
    for (flags, expected) in [
        (vec![OnlyDirs], vec!["directory"]),
        (vec![OnlyFiles], vec!["regular"]),
        (vec![NoSymlinks], vec!["directory", "regular"]),
        (vec![OnlyDirs, ShowSymlinks], vec!["directory", "dir-link"]),
        (
            vec![OnlyFiles, ShowSymlinks],
            vec!["regular", "link", "broken-link"],
        ),
    ] {
        let mut files: Vec<_> = ["directory", "regular", "link", "dir-link", "broken-link"]
            .iter()
            .map(|name| {
                fixture.file(
                    name,
                    if *name == "directory" {
                        "pipe"
                    } else {
                        "regular"
                    },
                    false,
                )
            })
            .collect();
        filter(flags).filter_child_files(false, &mut files);
        assert_eq!(
            files
                .iter()
                .map(|file| file.name.as_str())
                .collect::<Vec<_>>(),
            expected
        );
    }
}

#[test]
fn recursion_validates_even_without_type_filters() {
    let fixture = Fixture::new();
    let mut files = vec![
        fixture.file("directory", "pipe", false),
        fixture.file("dir-link", "regular", false),
    ];
    filter(vec![]).filter_child_files(true, &mut files);
    assert!(files[0].is_directory());
    assert!(!files[1].is_directory());
    assert!(files[1].is_link());
    assert!(files[1].points_to_directory());
}

#[test]
fn directory_grouping_validates_before_sorting() {
    let fixture = Fixture::new();
    let mut files = vec![
        fixture.file("link", "regular", false),
        fixture.file("directory", "pipe", false),
    ];
    filter(vec![FileFilterFlags::ListDirsLast]).sort_files(&mut files);
    assert_eq!(files[0].name, "link");
    assert_eq!(files[1].name, "directory");
}

#[test]
fn dereferencing_validates_before_reading_target_fields() {
    let fixture = Fixture::new();
    let link = fixture.file("link", "regular", true);
    assert!(link.metadata.get().is_none());
    assert!(matches!(link.size(), Size::Some(15)));
    assert!(link.is_link());
    let permissions = link.permissions().unwrap();
    assert!(!permissions.user_execute);
    assert!(!permissions.other_read);
    let broken = fixture.file("broken-link", "regular", true);
    assert!(matches!(broken.size(), Size::None));
    assert!(broken.permissions().is_none());
}

#[test]
fn long_view_resolves_links_even_without_metadata_columns() {
    struct NoVars;
    impl eza::options::Vars for NoVars {
        fn get(&self, _: &'static str) -> Option<std::ffi::OsString> {
            None
        }
    }

    let fixture = Fixture::new();
    for columns in [
        vec![],
        vec!["--no-permissions"],
        vec!["--no-permissions", "--no-filesize"],
    ] {
        let matches = eza::options::parser::get_command()
            .try_get_matches_from(
                ["eza", "-l", "--no-user", "--no-time", "--color=never"]
                    .into_iter()
                    .chain(columns),
            )
            .unwrap();
        let options = eza::options::Options::deduce(&matches, &NoVars).unwrap();
        let theme = options.theme.to_theme(false);
        let eza::output::Mode::Details(details) = &options.view.mode else {
            panic!("expected a long view");
        };
        let table =
            eza::output::table::Table::new(details.table.as_ref().unwrap(), None, &theme, false);
        for (name, target) in [("link", "regular"), ("broken-link", "missing")] {
            let file = fixture.file(name, "regular", false);
            let _ = table.row_for_file(&file, false, None);
            let cell = options
                .view
                .file_style
                .for_file(&file, &theme)
                .with_link_paths()
                .paint();
            assert_eq!(
                cell.strings().to_string(),
                format!("{} -> {target}", file.path.display())
            );
        }
    }
}
