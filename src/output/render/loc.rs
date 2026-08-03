// SPDX-FileCopyrightText: 2026 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2026 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use locale::Numeric as NumericLocale;
use nu_ansi_term::Style;

use crate::{loc::LocCounts, options::parser::CodeContent, output::cell::TextCell};

pub trait Render {
    fn render(
        self,
        style: Style,
        placeholder_style: Style,
        content: CodeContent,
        loc_total: Option<usize>,
        numeric_format: &NumericLocale,
    ) -> TextCell;
    fn render_json(
        self,
        content: CodeContent,
        loc_total: Option<usize>,
        numeric_format: &NumericLocale,
    ) -> Option<String>;
}

impl Render for Option<LocCounts> {
    fn render(
        self,
        style: Style,
        placeholder_style: Style,
        content: CodeContent,
        loc_total: Option<usize>,
        numeric_format: &NumericLocale,
    ) -> TextCell {
        let Some(counts) = self else {
            return TextCell::paint(placeholder_style, "-".to_string());
        };
        // Quantities take the same colour as file sizes, so the Code column
        // reads consistently next to Size.
        match content {
            CodeContent::Percent => match loc_total {
                Some(total) if total > 0 => {
                    let pct = (counts.code as f64) * 100.0 / (total as f64);
                    TextCell::paint(style, format!("{pct:.1}%"))
                }
                _ => TextCell::paint(placeholder_style, "-".to_string()),
            },
            _ => TextCell::paint(style, numeric_format.format_int(counts.code)),
        }
    }

    fn render_json(
        self,
        content: CodeContent,
        loc_total: Option<usize>,
        numeric_format: &NumericLocale,
    ) -> Option<String> {
        let counts = self?;
        match content {
            CodeContent::Percent => match loc_total {
                Some(total) if total > 0 => {
                    let pct = (counts.code as f64) * 100.0 / (total as f64);
                    Some(format!("{pct:.1}%"))
                }
                _ => None,
            },
            _ => Some(numeric_format.format_int(counts.code)),
        }
    }
}
