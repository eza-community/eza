use super::file_name::QuoteStyle;
use crate::fs::File;
use ansiterm::{ANSIString, Style};

#[derive(Eq, Copy, PartialEq, Clone)]
pub enum Quotes {
    Single,
    Double,
}

pub fn check_quote(files: &[File<'_>]) -> Quotes {
    for file in files {
        if file.name.contains('\'') {
            return Quotes::Double;
        }
    }
    Quotes::Single
}

pub fn escape(
    string: String,
    bits: &mut Vec<ANSIString<'_>>,
    good: Style,
    bad: Style,
    quote_style: QuoteStyle,
    quotes: Quotes,
) {
    let bits_starting_length = bits.len();
    let needs_quotes = string.contains(' ') || string.contains('\'');
    let quote_bit = good.paint(if quotes == Quotes::Double { "\"" } else { "\'" });

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
