// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use nu_ansi_term::Style;

use crate::fs::fields as f;
use crate::output::cell::TextCell;
use crate::output::table::FlagsFormat;

impl f::Flags {
    pub fn render(self, style: Style, _format: FlagsFormat) -> TextCell {
        TextCell::paint(style, "-".to_string())
    }
}
