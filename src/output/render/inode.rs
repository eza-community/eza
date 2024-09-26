// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use nu_ansi_term::Style;

use crate::fs::fields as f;
use crate::output::cell::TextCell;

impl f::Inode {
    pub fn render(self, style: Style) -> TextCell {
        TextCell::paint(style, self.0.to_string())
    }
}

#[cfg(test)]
pub mod test {
    use crate::fs::fields as f;
    use crate::output::cell::TextCell;

    use nu_ansi_term::Color::*;

    #[test]
    fn blocklessness() {
        let io = f::Inode(1_414_213);
        let expected = TextCell::paint_str(Cyan.underline(), "1414213");
        assert_eq!(expected, io.render(Cyan.underline()));
    }
}
