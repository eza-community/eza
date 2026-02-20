// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT

use crate::output::cell::TextCell;
use crate::output::render::FiletypeColours;

use nu_ansi_term::Style;

pub trait PermissionsPlusRender {
    fn render<C: Colours + FiletypeColours>(&self, colours: &C) -> TextCell;
}

pub trait Colours {
    fn dash(&self) -> Style;

    fn user_read(&self) -> Style;
    fn user_write(&self) -> Style;
    fn user_execute_file(&self) -> Style;
    fn user_execute_other(&self) -> Style;

    fn group_read(&self) -> Style;
    fn group_write(&self) -> Style;
    fn group_execute(&self) -> Style;

    fn other_read(&self) -> Style;
    fn other_write(&self) -> Style;
    fn other_execute(&self) -> Style;

    fn special_user_file(&self) -> Style;
    fn special_other(&self) -> Style;

    fn attribute(&self) -> Style;
}
