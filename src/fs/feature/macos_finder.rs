// SPDX-FileCopyrightText: 2024 eza contributors
// SPDX-License-Identifier: MIT
//! macOS Finder metadata: tag colors and SF Symbol folder overlays.

use nu_ansi_term::{Color, Style};
use phf::phf_map;

use crate::fs::feature::xattr::Attribute;

const TAG_XATTR: &str = "com.apple.metadata:_kMDItemUserTags";
const SYMBOL_XATTR: &str = "com.apple.icon.folder#S";

/// Returns the Finder tag color index (1–7) for the first colored tag found,
/// by parsing the binary plist in `_kMDItemUserTags`.
pub fn finder_tag_color(attrs: &[Attribute]) -> Option<u8> {
    let data = attr_value(attrs, TAG_XATTR)?;
    parse_tag_color(data)
}

/// Returns a Unicode char representing the folder's SF Symbol overlay,
/// or `'◆'` if the symbol name has no known mapping.
/// Returns `None` if no symbol xattr is present.
pub fn finder_symbol_char(attrs: &[Attribute]) -> Option<char> {
    let data = attr_value(attrs, SYMBOL_XATTR)?;
    let text = std::str::from_utf8(data).ok()?;
    let sym = extract_sym_field(text)?;
    Some(SF_SYMBOL_MAP.get(sym).copied().unwrap_or('◆'))
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

/// Binary plist parser for `_kMDItemUserTags`.
///
/// macOS stores tags as a bplist00 containing an array of UTF-8 strings of the
/// form "TagName\nN" where N is a color index 1–7. The array holds object
/// *references* (not inline objects), so we must follow the 32-byte trailer to
/// locate the offset table and resolve each reference to its string object.
fn parse_tag_color(data: &[u8]) -> Option<u8> {
    // 32-byte trailer + magic + at least one object
    if data.len() < 40 || &data[..8] != b"bplist00" {
        return None;
    }

    // --- trailer (last 32 bytes) ---
    let t = data.len() - 32;
    let offset_size = data[t + 6] as usize; // bytes per offset-table entry
    let ref_size = data[t + 7] as usize; // bytes per object reference
    let num_objects = read_uint_be(data.get(t + 8..t + 16)?)? as usize;
    let top_object = read_uint_be(data.get(t + 16..t + 24)?)? as usize;
    let ot_start = read_uint_be(data.get(t + 24..t + 32)?)? as usize;

    if offset_size == 0 || ref_size == 0 || offset_size > 8 || ref_size > 8 {
        return None;
    }

    // Resolve object index → byte offset in data
    let obj_offset = |idx: usize| -> Option<usize> {
        if idx >= num_objects {
            return None;
        }
        let pos = ot_start + idx * offset_size;
        read_uint_be(data.get(pos..pos + offset_size)?).map(|v| v as usize)
    };

    // Root object must be an array
    let root = obj_offset(top_object)?;
    let array_marker = *data.get(root)?;
    if (array_marker & 0xF0) != 0xA0 {
        return None;
    }
    let count = (array_marker & 0x0F) as usize;

    // Walk array elements: each is a ref_size-byte index into the object table
    for i in 0..count {
        let ref_pos = root + 1 + i * ref_size;
        let obj_ref = read_uint_be(data.get(ref_pos..ref_pos + ref_size)?)? as usize;
        let str_off = obj_offset(obj_ref)?;

        let str_marker = *data.get(str_off)?;
        if (str_marker & 0xF0) != 0x50 {
            continue; // not a UTF-8 string
        }
        let str_len = (str_marker & 0x0F) as usize;
        let str_bytes = data.get(str_off + 1..str_off + 1 + str_len)?;

        // Tag string is "TagName\nN" — color index is the last byte
        if let Some(&digit) = str_bytes.last()
            && (b'1'..=b'7').contains(&digit)
        {
            return Some(digit - b'0');
        }
    }
    None
}

/// Read a 1-, 2-, 4-, or 8-byte big-endian unsigned integer.
fn read_uint_be(bytes: &[u8]) -> Option<u64> {
    Some(match bytes.len() {
        1 => bytes[0] as u64,
        2 => u16::from_be_bytes(bytes.try_into().ok()?) as u64,
        4 => u32::from_be_bytes(bytes.try_into().ok()?) as u64,
        8 => u64::from_be_bytes(bytes.try_into().ok()?),
        _ => return None,
    })
}

/// Extracts the value of the `sym` field from `{"sym":"<name>"}`.
/// Handles the simple single-field JSON that macOS writes.
fn extract_sym_field(json: &str) -> Option<&str> {
    let key = "\"sym\"";
    let key_pos = json.find(key)?;
    let after_key = &json[key_pos + key.len()..];
    let colon = after_key.find(':')? + 1;
    let after_colon = after_key[colon..].trim_start();
    if let Some(inner) = after_colon.strip_prefix('"') {
        let end = inner.find('"')?;
        Some(&inner[..end])
    } else {
        None
    }
}

static SF_SYMBOL_MAP: phf::Map<&'static str, char> = phf_map! {
    // People & Body
    "person.fill"                   => '👤',
    "person.2.fill"                 => '👥',
    "person.circle.fill"            => '👤',
    "person.crop.circle.fill"       => '👤',
    "figure.stand"                  => '🧍',
    "figure.walk"                   => '🚶',
    "figure.run"                    => '🏃',
    "figure.pregnant"               => '🤰',
    "figure.2.arms.open"            => '🤸',
    "brain"                         => '🧠',
    "brain.head.profile"            => '🧠',
    "eye"                           => '👁',
    "eye.fill"                      => '👁',
    "eye.slash"                     => '🙈',
    "mouth.fill"                    => '👄',
    "ear"                           => '👂',
    "ear.fill"                      => '👂',
    "nose"                          => '👃',
    "nose.fill"                     => '👃',
    "lungs"                         => '🫁',
    "lungs.fill"                    => '🫁',
    "hand.raised"                   => '✋',
    "hand.raised.fill"              => '✋',
    "hand.thumbsup"                 => '👍',
    "hand.thumbsup.fill"            => '👍',
    "hand.thumbsdown"               => '👎',
    "hand.thumbsdown.fill"          => '👎',
    "hand.point.up"                 => '👆',
    "hand.point.up.fill"            => '👆',
    "face.smiling"                  => '😊',
    "face.smiling.fill"             => '😊',
    "mustache"                      => '🥸',
    "mustache.fill"                 => '🥸',

    // Nature
    "leaf.fill"                     => '🍃',
    "leaf"                          => '🍃',
    "tree.fill"                     => '🌲',
    "tree"                          => '🌲',
    "sun.max.fill"                  => '☀',
    "sun.max"                       => '☀',
    "moon.fill"                     => '🌙',
    "moon"                          => '🌙',
    "cloud.fill"                    => '☁',
    "cloud.rain.fill"               => '🌧',
    "cloud.bolt.fill"               => '⛈',
    "cloud.snow.fill"               => '🌨',
    "snow"                          => '❄',
    "wind"                          => '💨',
    "tornado"                       => '🌪',
    "flame.fill"                    => '🔥',
    "flame"                         => '🔥',
    "drop.fill"                     => '💧',
    "drop"                          => '💧',
    "bolt.fill"                     => '⚡',
    "bolt"                          => '⚡',
    "ant.fill"                      => '🐜',
    "ant"                           => '🐜',
    "ladybug.fill"                  => '🐞',
    "ladybug"                       => '🐞',
    "fish.fill"                     => '🐟',
    "fish"                          => '🐟',
    "bird.fill"                     => '🐦',
    "bird"                          => '🐦',
    "pawprint.fill"                 => '🐾',
    "pawprint"                      => '🐾',
    "tortoise.fill"                 => '🐢',
    "tortoise"                      => '🐢',
    "hare.fill"                     => '🐇',
    "hare"                          => '🐇',
    "cat.fill"                      => '🐱',
    "cat"                           => '🐱',
    "dog.fill"                      => '🐶',
    "dog"                           => '🐶',
    "lizard.fill"                   => '🦎',
    "lizard"                        => '🦎',
    "snail.fill"                    => '🐌',
    "snail"                         => '🐌',
    "worm.fill"                     => '🪱',
    "worm"                          => '🪱',
    "teddybear.fill"                => '🧸',
    "teddybear"                     => '🧸',
    "globe"                         => '🌐',
    "globe.americas.fill"           => '🌎',
    "globe.europe.africa.fill"      => '🌍',
    "globe.asia.australia.fill"     => '🌏',
    "mountain.2.fill"               => '🏔',
    "mountain.2"                    => '🏔',

    // Objects & Tools
    "hammer.fill"                   => '🔨',
    "hammer"                        => '🔨',
    "wrench.fill"                   => '🔧',
    "wrench"                        => '🔧',
    "screwdriver.fill"              => '🪛',
    "screwdriver"                   => '🪛',
    "paintbrush.fill"               => '🖌',
    "paintbrush"                    => '🖌',
    "paintpalette.fill"             => '🎨',
    "paintpalette"                  => '🎨',
    "pencil"                        => '✏',
    "scissors"                      => '✂',
    "ruler.fill"                    => '📏',
    "ruler"                         => '📏',
    "magnifyingglass"               => '🔍',
    "camera.fill"                   => '📷',
    "camera"                        => '📷',
    "photo.fill"                    => '🖼',
    "photo"                         => '🖼',
    "flashlight.on.fill"            => '🔦',
    "flashlight.off.fill"           => '🔦',
    "phone.fill"                    => '📞',
    "phone"                         => '📞',
    "envelope.fill"                 => '✉',
    "envelope"                      => '✉',
    "message.fill"                  => '💬',
    "message"                       => '💬',
    "house.fill"                    => '🏠',
    "house"                         => '🏠',
    "building.fill"                 => '🏢',
    "building.2.fill"               => '🏘',
    "bicycle"                       => '🚲',
    "car.fill"                      => '🚗',
    "car"                           => '🚗',
    "bus.fill"                      => '🚌',
    "bus"                           => '🚌',
    "tram.fill"                     => '🚋',
    "tram"                          => '🚋',
    "airplane"                      => '✈',
    "boat.fill"                     => '⛵',
    "cart.fill"                     => '🛒',
    "cart"                          => '🛒',
    "bag.fill"                      => '👜',
    "bag"                           => '👜',
    "fork.knife"                    => '🍴',
    "cup.and.saucer.fill"           => '☕',
    "cup.and.saucer"                => '☕',
    "birthday.cake.fill"            => '🎂',
    "birthday.cake"                 => '🎂',
    "book.fill"                     => '📖',
    "book"                          => '📖',
    "books.vertical.fill"           => '📚',
    "books.vertical"                => '📚',
    "doc.fill"                      => '📄',
    "doc"                           => '📄',
    "folder.fill"                   => '📁',
    "folder"                        => '📁',
    "archivebox.fill"               => '📦',
    "archivebox"                    => '📦',
    "briefcase.fill"                => '💼',
    "briefcase"                     => '💼',
    "tv.fill"                       => '📺',
    "tv"                            => '📺',
    "desktopcomputer"               => '🖥',
    "laptopcomputer"                => '💻',
    "keyboard"                      => '⌨',
    "printer.fill"                  => '🖨',
    "printer"                       => '🖨',
    "externaldrive.fill"            => '💾',
    "externaldrive"                 => '💾',
    "internaldrive.fill"            => '💾',
    "music.note"                    => '🎵',
    "guitar.fill"                   => '🎸',
    "guitar"                        => '🎸',
    "piano.keys"                    => '🎹',
    "headphones"                    => '🎧',
    "speaker.wave.3.fill"           => '🔊',
    "speaker.wave.2.fill"           => '🔉',
    "speaker.wave.1.fill"           => '🔈',
    "speaker.slash.fill"            => '🔇',
    "gamecontroller.fill"           => '🎮',
    "gamecontroller"                => '🎮',
    "puzzlepiece.fill"              => '🧩',
    "puzzlepiece"                   => '🧩',
    "sportscourt.fill"              => '🏟',
    "soccerball"                    => '⚽',
    "football.fill"                 => '🏈',
    "football"                      => '🏈',
    "basketball.fill"               => '🏀',
    "basketball"                    => '🏀',
    "tennisball.fill"               => '🎾',
    "tennisball"                    => '🎾',
    "baseball.fill"                 => '⚾',
    "baseball"                      => '⚾',
    "trophy.fill"                   => '🏆',
    "trophy"                        => '🏆',
    "medal.fill"                    => '🏅',
    "medal"                         => '🏅',
    "bed.double.fill"               => '🛏',
    "sofa.fill"                     => '🛋',
    "sofa"                          => '🛋',
    "trash.fill"                    => '🗑',
    "trash"                         => '🗑',
    "lock.fill"                     => '🔒',
    "lock"                          => '🔒',
    "lock.open.fill"                => '🔓',
    "lock.open"                     => '🔓',
    "key.fill"                      => '🔑',
    "key"                           => '🔑',
    "shield.fill"                   => '🛡',
    "shield"                        => '🛡',
    "lightbulb.fill"                => '💡',
    "lightbulb"                     => '💡',
    "stethoscope"                   => '🩺',
    "cross.fill"                    => '✚',
    "cross"                         => '✚',
    "pills.fill"                    => '💊',
    "pills"                         => '💊',
    "syringe.fill"                  => '💉',
    "syringe"                       => '💉',
    "bandage.fill"                  => '🩹',
    "bandage"                       => '🩹',
    "gift.fill"                     => '🎁',
    "gift"                          => '🎁',
    "ticket.fill"                   => '🎟',
    "ticket"                        => '🎟',
    "creditcard.fill"               => '💳',
    "creditcard"                    => '💳',
    "clock.fill"                    => '🕐',
    "clock"                         => '🕐',
    "alarm.fill"                    => '⏰',
    "alarm"                         => '⏰',
    "stopwatch.fill"                => '⏱',
    "stopwatch"                     => '⏱',
    "timer"                         => '⏱',
    "map.fill"                      => '🗺',
    "map"                           => '🗺',
    "location.fill"                 => '📍',
    "location"                      => '📍',

    // Symbols & Misc
    "star.fill"                     => '★',
    "star"                          => '★',
    "heart.fill"                    => '❤',
    "heart"                         => '❤',
    "tag.fill"                      => '🏷',
    "tag"                           => '🏷',
    "flag.fill"                     => '🚩',
    "flag"                          => '🚩',
    "bookmark.fill"                 => '🔖',
    "bookmark"                      => '🔖',
    "ribbon.fill"                   => '🎀',
    "ribbon"                        => '🎀',
    "crown.fill"                    => '👑',
    "crown"                         => '👑',
    "graduationcap.fill"            => '🎓',
    "graduationcap"                 => '🎓',
    "film.fill"                     => '🎬',
    "film"                          => '🎬',
    "calendar"                      => '📅',
    "link"                          => '🔗',
    "gear"                          => '⚙',
    "gearshape.fill"                => '⚙',
    "gearshape"                     => '⚙',
    "chart.bar.fill"                => '📊',
    "chart.bar"                     => '📊',
    "chart.pie.fill"                => '📊',
    "chart.pie"                     => '📊',
    "chart.line.uptrend.xyaxis"     => '📈',
    "dollarsign.circle.fill"        => '💰',
    "eurosign.circle.fill"          => '💶',
    "sterlingsign.circle.fill"      => '💷',
    "yensign.circle.fill"           => '💴',
    "arrow.up"                      => '⬆',
    "arrow.down"                    => '⬇',
    "arrow.right"                   => '➡',
    "arrow.left"                    => '⬅',
    "arrow.up.arrow.down"           => '↕',
    "arrow.left.arrow.right"        => '↔',
    "arrow.clockwise"               => '🔃',
    "arrow.counterclockwise"        => '🔄',
    "checkmark.circle.fill"         => '✅',
    "checkmark.circle"              => '✅',
    "xmark.circle.fill"             => '❌',
    "xmark.circle"                  => '❌',
    "exclamationmark.circle.fill"   => '❗',
    "questionmark.circle.fill"      => '❓',
    "infinity"                      => '∞',
    "at"                            => '@',
    "sparkles"                      => '✨',
    "wand.and.stars.inverse"        => '✨',
    "wand.and.stars"                => '✨',
};

#[cfg(test)]
mod tests {
    use super::{
        SYMBOL_XATTR, TAG_XATTR, finder_symbol_char, finder_tag_color, finder_tag_style,
        parse_tag_color,
    };
    use crate::fs::feature::xattr::Attribute;
    use nu_ansi_term::{Color, Style};

    fn attr(name: &str, value: &[u8]) -> Attribute {
        Attribute {
            name: name.to_string(),
            value: Some(value.to_vec()),
        }
    }

    // Minimal bplist00: array of one string "2" (tag color Green)
    //
    // Layout (46 bytes):
    //   [0..8]  magic "bplist00"
    //   [8]     0xA1  array marker (1 element)
    //   [9]     0x01  element ref → object 1
    //   [10]    0x51  string marker (1 char)
    //   [11]    b'2'  string content
    //   [12]    0x08  offset table: obj 0 at byte 8
    //   [13]    0x0A  offset table: obj 1 at byte 10
    //   [14..46] 32-byte trailer
    const BPLIST_TAG2: &[u8] = &[
        b'b', b'p', b'l', b'i', b's', b't', b'0', b'0', // magic
        0xA1, 0x01, // array(1), ref=1
        0x51, b'2', // string(1) = "2"
        0x08, 0x0A, // offset table
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // trailer padding
        0x01, // offset_size = 1
        0x01, // ref_size = 1
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, // num_objects = 2
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // top_object = 0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, // ot_start = 12
    ];

    // Same layout but string content is "none" — no digit '1'–'7' as last byte
    const BPLIST_NO_DIGIT: &[u8] = &[
        b'b', b'p', b'l', b'i', b's', b't', b'0', b'0', 0xA1, 0x01, 0x54, b'n', b'o', b'n',
        b'e', // string(4) = "none"
        0x08, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x0C,
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

    // --- finder_symbol_char ---

    #[test]
    fn symbol_known() {
        let attrs = vec![attr(SYMBOL_XATTR, br#"{"sym":"hammer.fill"}"#)];
        assert_eq!(finder_symbol_char(&attrs), Some('🔨'));
    }

    #[test]
    fn symbol_unknown_falls_back_to_diamond() {
        let attrs = vec![attr(SYMBOL_XATTR, br#"{"sym":"nonexistent.symbol"}"#)];
        assert_eq!(finder_symbol_char(&attrs), Some('◆'));
    }

    #[test]
    fn symbol_malformed_json() {
        let attrs = vec![attr(SYMBOL_XATTR, b"not json at all")];
        assert_eq!(finder_symbol_char(&attrs), None);
    }

    #[test]
    fn symbol_missing_key() {
        let attrs = vec![attr(SYMBOL_XATTR, br#"{"other":"value"}"#)];
        assert_eq!(finder_symbol_char(&attrs), None);
    }

    #[test]
    fn symbol_absent() {
        assert_eq!(finder_symbol_char(&[]), None);
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
