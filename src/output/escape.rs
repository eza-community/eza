use ansiterm::{ANSIString, Style};


pub fn escape(string: String, bits: &mut Vec<ANSIString<'_>>, good: Style, bad: Style) {
    // if the string has no control character
    if string.chars().all(|c| !c.is_control()) {
        bits.push(good.paint(string));
        return;
    }

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
