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

    // `is_control` covers C0, DEL and C1 control characters, all of which
    // a terminal may interpret instead of printing.
    if !string.chars().any(char::is_control) {
        bits.push(good.paint(string));
    } else {
        for c in string.chars() {
            // The `escape_default` method on `char` is *almost* what we want here, but
            // it still escapes non-ASCII UTF-8 characters, which are still printable.

            // TODO: This allocates way too much,
            // hence the check above.
            if !c.is_control() {
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

const HYPERLINK_ESCAPE_CHARS: &AsciiSet = &CONTROLS.add(b' ');
const HYPERLINK_OPENING_START: &str = "\x1B]8;;";
const HYPERLINK_OPENING_END: &str = "\x1B\x5C";
// Combination of both above tags
pub const HYPERLINK_CLOSING: &str = "\x1B]8;;\x1B\x5C";

pub fn get_hyperlink_start_tag(abs_path: &str) -> String {
    let abs_path = utf8_percent_encode(abs_path, HYPERLINK_ESCAPE_CHARS).to_string();

    // On Windows, `std::fs::canonicalize` adds the Win32 File prefix, which we need to remove
    #[cfg(target_os = "windows")]
    let abs_path = abs_path.strip_prefix("\\\\?\\").unwrap_or(&abs_path);

    format!("{HYPERLINK_OPENING_START}file://{abs_path}{HYPERLINK_OPENING_END}")
}

#[cfg(test)]
mod test {
    use super::*;

    fn escaped(string: &str) -> String {
        let mut bits = Vec::new();
        escape(
            string.into(),
            &mut bits,
            Style::default(),
            Style::default(),
            QuoteStyle::NoQuotes,
        );
        bits.iter().map(|bit| bit.to_string()).collect()
    }

    #[test]
    fn escape_control_characters() {
        assert_eq!(escaped("a\tb"), "a\\tb");
        assert_eq!(escaped("a\x7fb"), "a\\u{7f}b");
        // C1 control characters, e.g. U+009F, start terminal control sequences too
        assert_eq!(escaped("b\u{9f}"), "b\\u{9f}");
        assert_eq!(escaped("b\u{85}c"), "b\\u{85}c");
        // printable non-ASCII characters are kept as they are
        assert_eq!(escaped("café ☕"), "café ☕");
    }

    #[test]
    fn hyperlink_start_tag_escape_spaces() {
        assert_eq!(
            get_hyperlink_start_tag("/folder name/file name").to_string(),
            format!(
                "{HYPERLINK_OPENING_START}file:///folder%20name/file%20name{HYPERLINK_OPENING_END}"
            ),
        );
    }
}
