// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use crate::output::cell::TextCell;
use crate::output::time::TimeFormat;

use chrono::prelude::*;
use nu_ansi_term::Style;

pub trait Render {
    fn render(self, style: Style, time_offset: FixedOffset, time_format: TimeFormat) -> TextCell;
}

impl Render for Option<NaiveDateTime> {
    fn render(self, style: Style, time_offset: FixedOffset, time_format: TimeFormat) -> TextCell {
        let datestamp = if let Some(time) = self {
            time_format.format(&DateTime::<FixedOffset>::from_naive_utc_and_offset(
                time,
                time_offset,
            ))
        } else {
            String::from("-")
        };

        TextCell::paint(style, datestamp)
    }
}
