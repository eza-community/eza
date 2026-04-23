mod cli_tests_helpers;

use cli_tests_helpers::TestDirectory;

#[test]
fn cli_tests_any_basic() {
    let test_dir = TestDirectory::new("any", "basic");

    test_dir.create_files(&["file.txt"]);
    test_dir.create_dirs(&["dir"]);

    test_dir.run_tests();
}

#[test]
fn cli_tests_any_dotfiles() {
    let test_dir = TestDirectory::new("any", "dotfiles");

    test_dir.create_files(&[".file"]);
    test_dir.create_dirs(&[".dir"]);

    test_dir.run_tests();
}

#[test]
#[cfg(not(feature = "git"))]
fn cli_tests_any_no_git() {
    let test_dir = TestDirectory::new("any", "no-git");
    test_dir.run_tests();
}

#[test]
fn cli_tests_any_sort() {
    let test_dir = TestDirectory::new("any", "sort");

    test_dir.create_files(&["a.txt", "abc.mp3", "ab"]);
    test_dir.create_dirs(&["test", "abc", "01.city", "02.apple"]);

    test_dir.run_tests();
}
