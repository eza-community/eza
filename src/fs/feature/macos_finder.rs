// SPDX-FileCopyrightText: 2024 eza contributors
// SPDX-License-Identifier: MIT
//! macOS Finder tag colors.

use nu_ansi_term::{Color, Style};

use crate::fs::feature::xattr::Attribute;

const TAG_XATTR: &str = "com.apple.metadata:_kMDItemUserTags";

/// Returns the Finder tag color index (1–7) for the first colored tag found,
/// by parsing the binary plist in `_kMDItemUserTags`.
pub fn finder_tag_color(attrs: &[Attribute]) -> Option<u8> {
    let data = attr_value(attrs, TAG_XATTR)?;
    parse_tag_color(data)
}

/// Maps a Finder tag color index to an ANSI terminal style.
pub fn finder_tag_style(color_idx: u8) -> Style {
    match color_idx {
        1 => Style::new().fg(Color::DarkGray),
        2 => Style::new().fg(Color::Green),
        3 => Style::new().fg(Color::Purple),
        4 => Style::new().fg(Color::Blue),
        5 => Style::new().fg(Color::Yellow),
        6 => Style::new().fg(Color::Red),
        7 => Style::new().fg(Color::Fixed(208)), // Orange
        _ => Style::default(),
    }
}

fn attr_value<'a>(attrs: &'a [Attribute], name: &str) -> Option<&'a [u8]> {
    attrs
        .iter()
        .find(|a| a.name == name)
        .and_then(|a| a.value.as_deref())
}

/// Read Finder's binary plist using the shared plist decoder, including UTF-16
/// tag names and extended-length strings/arrays. Preserve Finder tag order.
fn parse_tag_color(data: &[u8]) -> Option<u8> {
    if !data.starts_with(b"bplist00") {
        return None;
    }
    let value = plist::Value::from_reader(std::io::Cursor::new(data)).ok()?;
    value.as_array()?.iter().find_map(|tag| {
        let (_, suffix) = tag.as_string()?.rsplit_once('\n')?;
        match suffix.as_bytes() {
            [digit @ b'1'..=b'7'] => Some(digit - b'0'),
            _ => None,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::{TAG_XATTR, finder_tag_color, finder_tag_style, parse_tag_color};
    use crate::fs::feature::xattr::Attribute;
    use nu_ansi_term::{Color, Style};

    fn attr(name: &str, value: &[u8]) -> Attribute {
        Attribute {
            name: name.to_string(),
            value: Some(value.to_vec()),
        }
    }

    // Real binary plists encoded independently with Python's plistlib.
    const BPLIST_TAG2: &[u8] = &[
        98, 112, 108, 105, 115, 116, 48, 48, 161, 1, 87, 71, 114, 101, 101, 110, 10, 50, 8, 10, 0,
        0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        18,
    ];
    const BPLIST_NO_DIGIT: &[u8] = &[
        98, 112, 108, 105, 115, 116, 48, 48, 161, 1, 84, 110, 111, 110, 101, 8, 10, 0, 0, 0, 0, 0,
        0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15,
    ];

    // --- parse_tag_color ---

    #[test]
    fn tag_color_valid() {
        assert_eq!(parse_tag_color(BPLIST_TAG2), Some(2));
    }

    #[test]
    fn tag_color_wrong_magic() {
        let mut bad = BPLIST_TAG2.to_vec();
        bad[0] = b'x';
        assert_eq!(parse_tag_color(&bad), None);
    }

    #[test]
    fn tag_color_too_short() {
        assert_eq!(parse_tag_color(&BPLIST_TAG2[..39]), None);
    }

    #[test]
    fn tag_color_no_digit() {
        assert_eq!(parse_tag_color(BPLIST_NO_DIGIT), None);
    }

    fn encoded_tags(tags: &[&str]) -> Vec<u8> {
        let value = plist::Value::Array(
            tags.iter()
                .map(|tag| plist::Value::String((*tag).into()))
                .collect(),
        );
        let mut bytes = Vec::new();
        value.to_writer_binary(&mut bytes).unwrap();
        bytes
    }

    #[test]
    fn tag_color_unicode_and_long_names() {
        for name in [
            "红色",
            "工作🛠️",
            "A custom label longer than fifteen characters",
        ] {
            for color in 1..=7 {
                let tag = format!("{name}\n{color}");
                assert_eq!(parse_tag_color(&encoded_tags(&[&tag])), Some(color));
            }
        }
    }

    #[test]
    fn tag_color_extended_array_and_first_colored_tag() {
        let mut tags = vec!["Uncolored\n0"; 20];
        tags.extend(["绿色\n2", "Red\n6"]);
        assert_eq!(parse_tag_color(&encoded_tags(&tags)), Some(2));
    }

    #[test]
    fn tag_color_requires_valid_suffix() {
        for tag in ["Project2", "2", "Tag\n0", "Tag\n8", "Tag\n12", "Tag\n2x"] {
            assert_eq!(parse_tag_color(&encoded_tags(&[tag])), None);
        }
        assert_eq!(parse_tag_color(&encoded_tags(&[])), None);
    }

    #[test]
    fn tag_color_truncated_plists() {
        let bytes = encoded_tags(&["红色\n6"]);
        for length in 0..bytes.len() {
            assert_eq!(parse_tag_color(&bytes[..length]), None);
        }
        let mut bytes = Vec::new();
        plist::Value::String("Red\n6".into())
            .to_writer_binary(&mut bytes)
            .unwrap();
        assert_eq!(parse_tag_color(&bytes), None);
    }

    // --- finder_tag_color (via Attribute slice) ---

    #[test]
    fn finder_tag_color_present() {
        let attrs = vec![attr(TAG_XATTR, BPLIST_TAG2)];
        assert_eq!(finder_tag_color(&attrs), Some(2));
    }

    #[test]
    fn finder_tag_color_absent() {
        assert_eq!(finder_tag_color(&[]), None);
    }

    // --- finder_tag_style ---

    #[test]
    fn tag_style_gray() {
        assert_eq!(finder_tag_style(1), Style::new().fg(Color::DarkGray));
    }

    #[test]
    fn tag_style_green() {
        assert_eq!(finder_tag_style(2), Style::new().fg(Color::Green));
    }

    #[test]
    fn tag_style_red() {
        assert_eq!(finder_tag_style(6), Style::new().fg(Color::Red));
    }

    #[test]
    fn tag_style_orange() {
        assert_eq!(finder_tag_style(7), Style::new().fg(Color::Fixed(208)));
    }

    #[test]
    fn tag_style_unknown() {
        assert_eq!(finder_tag_style(0), Style::default());
        assert_eq!(finder_tag_style(8), Style::default());
    }
}
