use ansi_term::Style;
use phf;

use crate::fs::File;

// See build.rs for FILENAME_ICONS and EXTENSION_ICONS
include!(concat!(env!("OUT_DIR"), "/icon_maps.rs"));

/// Converts the style used to paint a file name into the style that should be
/// used to paint an icon.
///
/// - The background colour should be preferred to the foreground colour, as
///   if one is set, it’s the more “obvious” colour choice.
/// - If neither is set, just use the default style.
/// - Attributes such as bold or underline should not be used to paint the
///   icon, as they can make it look weird.
pub fn iconify_style(style: Style) -> Style {
    style.background.or(style.foreground)
         .map(Style::from)
         .unwrap_or_default()
}

pub fn icon_for_file(file: &File<'_>) -> char {
    if let Some(icon) = FILENAME_ICONS.get(file.name.as_str()) {
        *icon
    } else if file.points_to_directory() {
        if file.is_empty_dir() {
            '\u{f115}' // 
        } else {
            '\u{f07b}' // 
        }
    } else if let Some(ext) = file.ext.as_ref() {
        *EXTENSION_ICONS.get(ext.as_str()).unwrap_or(&'\u{f15b}') // 
    } else {
        '\u{f016}' // 
    }
}
