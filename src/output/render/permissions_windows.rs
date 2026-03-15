// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use crate::fs::fields as f;
use crate::output::cell::{DisplayWidth, TextCell};
use crate::output::render::FiletypeColours;

use super::{PermissionsColours as Colours, PermissionsPlusRender};

use nu_ansi_term::{AnsiString as ANSIString, Style};

impl PermissionsPlusRender for Option<f::PermissionsPlus> {
    fn render<C: Colours + FiletypeColours>(&self, colours: &C) -> TextCell {
        match self {
            Some(p) => {
                let mut chars = vec![p.attributes.render_type(colours)];
                chars.extend(p.attributes.render(colours));

                TextCell {
                    width: DisplayWidth::from(chars.len()),
                    contents: chars.into(),
                }
            }
            None => TextCell {
                width: DisplayWidth::from(0),
                contents: vec![].into(),
            },
        }
    }
}

impl f::Attributes {
    pub fn render<C: Colours + FiletypeColours>(self, colours: &C) -> Vec<ANSIString<'static>> {
        let bit = |bit, chr: &'static str, style: Style| {
            if bit {
                style.paint(chr)
            } else {
                colours.dash().paint("-")
            }
        };

        vec![
            bit(self.archive, "a", colours.normal()),
            bit(self.readonly, "r", colours.user_read()),
            bit(self.hidden, "h", colours.special_user_file()),
            bit(self.system, "s", colours.special_other()),
        ]
    }

    pub fn render_type<C: Colours + FiletypeColours>(self, colours: &C) -> ANSIString<'static> {
        if self.reparse_point {
            return colours.pipe().paint("l");
        } else if self.directory {
            return colours.directory().paint("d");
        }
        colours.dash().paint("-")
    }
}
