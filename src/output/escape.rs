// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use super::file_name::QuoteStyle;
use nu_ansi_term::{AnsiString as ANSIString, Style};
use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};

pub fn escape(
    string: String,
    bits: &mut Vec<ANSIString<'_>>,
    good: Style,
    bad: Style,
    quote_style: QuoteStyle,
) {
    let bits_starting_length = bits.len();
    let needs_quotes = string.contains(' ') || string.contains('\'');
    let quote_bit = good.paint(if string.contains('\'') { "\"" } else { "\'" });

    if string
        .chars()
        .all(|c| c >= 0x20 as char && c != 0x7f as char)
    {
        bits.push(good.paint(string));
    } else {
        for c in string.chars() {
            // The `escape_default` method on `char` is *almost* what we want here, but
            // it still escapes non-ASCII UTF-8 characters, which are still printable.

            // TODO: This allocates way too much,
            // hence the `all` check above.
            if c >= 0x20 as char && c != 0x7f as char {
                bits.push(good.paint(c.to_string()));
            } else {
                bits.push(bad.paint(c.escape_default().to_string()));
            }
        }
    }

    if quote_style != QuoteStyle::NoQuotes && needs_quotes {
        bits.insert(bits_starting_length, quote_bit.clone());
        bits.push(quote_bit);
    }
}

const HYPERLINK_ESCAPE_CHARS: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'%')
    .add(b'<')
    .add(b'>')
    .add(b'?')
    .add(b'[')
    .add(b'\\')
    .add(b']')
    .add(b'^')
    .add(b'`')
    .add(b'{')
    .add(b'|')
    .add(b'}');
const HYPERLINK_OPENING_START: &str = "\x1B]8;;";
const HYPERLINK_OPENING_END: &str = "\x1B\x5C";
// Combination of both above tags
pub const HYPERLINK_CLOSING: &str = "\x1B]8;;\x1B\x5C";

#[cfg(any(target_os = "windows", test))]
fn encode_windows_path_for_file_uri(abs_path: &str) -> String {
    let (path, is_unc) = if let Some(path) = abs_path.strip_prefix(r"\\?\UNC\") {
        (path, true)
    } else {
        let path = abs_path.strip_prefix(r"\\?\").unwrap_or(abs_path);
        if let Some(path) = path.strip_prefix(r"\\") {
            (path, true)
        } else {
            (path, false)
        }
    };

    let mut path = path.replace('\\', "/");
    // Drive paths need an empty authority (`file:///C:/...`), while UNC paths
    // use their server name as the authority (`file://server/share/...`).
    if !is_unc && !path.starts_with('/') {
        path.insert(0, '/');
    }

    utf8_percent_encode(&path, HYPERLINK_ESCAPE_CHARS).to_string()
}

pub fn get_hyperlink_start_tag(abs_path: &str) -> String {
    #[cfg(target_os = "windows")]
    let abs_path = encode_windows_path_for_file_uri(abs_path);

    #[cfg(not(target_os = "windows"))]
    let abs_path = utf8_percent_encode(abs_path, HYPERLINK_ESCAPE_CHARS).to_string();

    format!("{HYPERLINK_OPENING_START}file://{abs_path}{HYPERLINK_OPENING_END}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hyperlink_start_tag_escape_spaces() {
        assert_eq!(
            get_hyperlink_start_tag("/folder name/file name").to_string(),
            format!(
                "{HYPERLINK_OPENING_START}file:///folder%20name/file%20name{HYPERLINK_OPENING_END}"
            ),
        );
    }

    #[test]
    fn hyperlink_start_tag_escapes_uri_path_characters() {
        assert_eq!(
            get_hyperlink_start_tag(r#"/folder/file#?%[]"<>^`{|}.txt"#),
            format!(
                "{HYPERLINK_OPENING_START}file:///folder/file%23%3F%25%5B%5D%22%3C%3E%5E%60%7B%7C%7D.txt{HYPERLINK_OPENING_END}"
            ),
        );
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn hyperlink_start_tag_escapes_backslashes_in_unix_file_names() {
        assert_eq!(
            get_hyperlink_start_tag(r"/folder/file\name.txt"),
            format!(
                "{HYPERLINK_OPENING_START}file:///folder/file%5Cname.txt{HYPERLINK_OPENING_END}"
            ),
        );
    }

    #[test]
    fn windows_paths_are_normalized_before_uri_escaping() {
        for (path, expected) in [
            (
                r"\\?\C:\folder name\file#.txt",
                "file:///C:/folder%20name/file%23.txt",
            ),
            (
                r"C:\folder name\file#.txt",
                "file:///C:/folder%20name/file%23.txt",
            ),
            (
                r"\\?\UNC\server\share\folder name\file#.txt",
                "file://server/share/folder%20name/file%23.txt",
            ),
            (
                r"\\server\share\folder name\file#.txt",
                "file://server/share/folder%20name/file%23.txt",
            ),
        ] {
            let encoded_path = encode_windows_path_for_file_uri(path);
            assert_eq!(format!("file://{encoded_path}"), expected);
        }
    }
}
