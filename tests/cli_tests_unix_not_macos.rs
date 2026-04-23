#![cfg(all(unix, not(target_os = "macos")))]

mod cli_tests_helpers;

use cli_tests_helpers::TestDirectory;

#[test]
fn cli_tests_any_weird_filenames() {
    use std::ffi::OsStr;

    use nu_ansi_term::Color;

    let test_dir = TestDirectory::new("unix", "weird_filenames");

    test_dir.create_dirs(&["new-line-dir: [\n]"]);

    let mut ansi_string = "ansi: ".to_owned();
    ansi_string.push_str(Color::Blue.paint("blue").as_str());

    test_dir.create_files(&[
        "ascii: hello",
        "emoji: [🆒]",
        "utf-8: pâté",
        "bell: \u{07}",
        "backspace: \u{08}",
        "tab: \t",
        "vertical tab: \u{0b}",
        "form-feed: \u{0c}",
        "carriage return: \r",
        "escape: \u{1b}",
        "new-line: [\n]",
        &ansi_string,
    ]);

    test_dir.symlink("new-line: [\n]", "new-line: [\n]-symlink");
    test_dir.symlink("broken-target: [\n]", "broken-target: [\n]-symlink");

    let mut invalid_utf8_1 = "invalid-utf8-1: ".as_bytes().to_vec();
    invalid_utf8_1.extend_from_slice(&[0xff]);
    let mut invalid_utf8_2 = "invalid-utf8-2: ".as_bytes().to_vec();
    invalid_utf8_2.extend_from_slice(&[0xc3, 0x28]);
    let mut invalid_utf8_3 = "invalid-utf8-3: ".as_bytes().to_vec();
    invalid_utf8_3.extend_from_slice(&[0xe2, 0x82, 0x28]);
    let mut invalid_utf8_4 = "invalid-utf8-4: ".as_bytes().to_vec();
    invalid_utf8_4.extend_from_slice(&[0xf0, 0x28, 0x8c, 0x28]);

    unsafe {
        test_dir.create_files(&[
            OsStr::from_encoded_bytes_unchecked(&invalid_utf8_1),
            OsStr::from_encoded_bytes_unchecked(&invalid_utf8_2),
            OsStr::from_encoded_bytes_unchecked(&invalid_utf8_3),
            OsStr::from_encoded_bytes_unchecked(&invalid_utf8_4),
        ]);
    }

    test_dir.run_tests();
}
