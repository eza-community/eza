use ansiterm::{ANSIString, Style};

use super::file_name::QuoteStyle;


pub fn escape(string: String, bits: &mut Vec<ANSIString<'_>>, good: Style, bad: Style, quote_style: QuoteStyle) {
    let needs_quotes = string.contains(' ') || string.contains('\'');
    let quote_bit = good.paint(if string.contains('\'') { "\"" } else { "\'" });

    if string.chars().all(|c| c >= 0x20 as char && c != 0x7f as char) {

        bits.push(good.paint(string));
    }
    else {
        for c in string.chars() {
            // The `escape_default` method on `char` is *almost* what we want here, but
            // it still escapes non-ASCII UTF-8 characters, which are still printable.

            // TODO: This allocates way too much,
            // hence the `all` check above.
            if c >= 0x20 as char && c != 0x7f as char {
                bits.push(good.paint(c.to_string()));
            }
            else {
                bits.push(bad.paint(c.escape_default().to_string()));
            }
        }
    }

    if quote_style != QuoteStyle::NoQuotes && needs_quotes {
        bits.insert(0, quote_bit.clone());
        bits.push(quote_bit);

    // the lengthier string of non control character can’t be bigger than the whole string
    let mut regular_char_buff = String::with_capacity(string.len());
    for c in string.chars() {
        // The `escape_default` method on `char` is *almost* what we want here, but
        // it still escapes non-ASCII UTF-8 characters, which are still printable.

        if c.is_control() {
            if !regular_char_buff.is_empty() {
                bits.push(good.paint(std::mem::take(&mut regular_char_buff)));
            }
            regular_char_buff.extend(c.escape_default());
            // biased towards regular characters, we push control characters immediately
            bits.push(bad.paint(std::mem::take(&mut regular_char_buff)));
        } else {
            regular_char_buff.push(c);
        }
    }
    // if last character was not a control character, the buffer is not empty!
    if !regular_char_buff.is_empty() {
        bits.push(good.paint(std::mem::take(&mut regular_char_buff)));
    }
}
