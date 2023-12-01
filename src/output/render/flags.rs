use ansiterm::Style;

use crate::fs::fields as f;
use crate::output::cell::TextCell;

impl f::Flags {
    pub fn render(self, style: Style) -> TextCell {
        TextCell::paint(style, "-".to_string())
    }
}
