// SPDX-FileCopyrightText: 2026 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2026 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use nu_ansi_term::Style;

use crate::{loc::Language, output::cell::TextCell};

pub trait Render {
    fn render(self, style: Style) -> TextCell;
    fn render_json(self) -> Option<String>;
}

impl Render for Option<&Language> {
    fn render(self, style: Style) -> TextCell {
        match self {
            Some(lang) => TextCell::paint(style, lang.name.to_string()),
            None => TextCell::paint(style, "-".to_string()),
        }
    }

    fn render_json(self) -> Option<String> {
        self.map(|lang| lang.name.to_string())
    }
}
