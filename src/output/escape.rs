// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use super::file_name::QuoteStyle;
use nu_ansi_term::{AnsiString as ANSIString, Style};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

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
