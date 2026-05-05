#![cfg(windows)]

mod cli_tests_helpers;

use cli_tests_helpers::TestDirectory;

#[test]
fn cli_tests_windows_attributes() {
    use windows_sys::Win32::Storage::FileSystem::{
        FILE_ATTRIBUTE_ARCHIVE, FILE_ATTRIBUTE_HIDDEN, FILE_ATTRIBUTE_NORMAL,
        FILE_ATTRIBUTE_READONLY, FILE_ATTRIBUTE_SYSTEM,
    };
    let test_dir = TestDirectory::new("windows", "attributes");

    test_dir.create_files(&["readonly", "normal", "hidden", "system", "archive", "all"]);
    test_dir.create_dirs(&["dir"]);
    test_dir.set_windows_attributes("readonly", FILE_ATTRIBUTE_READONLY);
    test_dir.set_windows_attributes("normal", FILE_ATTRIBUTE_NORMAL);
    test_dir.set_windows_attributes("hidden", FILE_ATTRIBUTE_HIDDEN);
    test_dir.set_windows_attributes("archive", FILE_ATTRIBUTE_ARCHIVE);
    test_dir.set_windows_attributes("system", FILE_ATTRIBUTE_SYSTEM);
    test_dir.set_windows_attributes(
        "all",
        FILE_ATTRIBUTE_READONLY
            | FILE_ATTRIBUTE_HIDDEN
            | FILE_ATTRIBUTE_ARCHIVE
            | FILE_ATTRIBUTE_SYSTEM,
    );

    test_dir.run_tests();
}

#[test]
fn cli_tests_windows_hidden() {
    use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_HIDDEN;
    let test_dir = TestDirectory::new("windows", "hidden");

    test_dir.create_files(&[".dot", "_underscore", "hidden", "file"]);
    test_dir.create_dirs(&[".dot_dir", "_underscore_dir", "hidden_dir"]);
    test_dir.set_windows_attributes("hidden", FILE_ATTRIBUTE_HIDDEN);
    test_dir.set_windows_attributes("hidden_dir", FILE_ATTRIBUTE_HIDDEN);

    test_dir.run_tests();
}
