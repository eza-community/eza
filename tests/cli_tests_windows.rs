#![cfg(windows)]

mod cli_tests_helpers;

use cli_tests_helpers::TestDirectory;

#[test]
fn cli_tests_windows_attributes() {
    let test_dir = TestDirectory::new("windows", "attributes");

    test_dir.create_files(&["readonly", "hidden", "system", "archive"]);
    test_dir.create_dirs(&["dir"]);
    test_dir.set_attributes("hidden");

    test_dir.run_tests();
}
