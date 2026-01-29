// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
#[cfg(target_os = "windows")]
pub use self::cell::TextCell;
pub use self::escape::escape;

pub mod color_scale;
pub mod details;
pub mod file_name;
pub mod grid;
pub mod grid_details;
pub mod icons;
pub mod lines;
pub mod render;
pub mod table;
pub mod time;

mod cell;
mod escape;
mod tree;

/// The **view** contains all information about how to format output.
#[derive(Debug)]
pub struct View {
    pub mode: Mode,
    pub width: TerminalWidth,
    pub space_between_columns: SpacingBetweenColumns,
    pub file_style: file_name::Options,
    pub deref_links: bool,
    pub follow_links: bool,
    pub total_size: bool,
}

/// The **mode** is the “type” of output.
#[derive(PartialEq, Eq, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum Mode {
    Grid(grid::Options),
    Details(details::Options),
    GridDetails(grid_details::Options),
    Lines,
}

/// The width of the terminal requested by the user.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum TerminalWidth {
    /// The user requested this specific number of columns.
    Set(usize),

    /// Look up the terminal size at runtime.
    Automatic,
}

impl TerminalWidth {
    #[must_use]
    pub fn actual_terminal_width(self) -> Option<usize> {
        // All of stdin, stdout, and stderr could not be connected to a
        // terminal, but we’re only interested in stdout because it’s
        // where the output goes.

        #[cfg(unix)]
        let stdout_term_width =
            { terminal_size::terminal_size_of(std::io::stdout()).map(|(w, _h)| w.0 as _) };
        #[cfg(windows)]
        let stdout_term_width = {
            use std::os::windows::io::BorrowedHandle;
            use windows_sys::Win32::System::Console::{GetStdHandle, STD_OUTPUT_HANDLE};
            terminal_size::terminal_size_of(unsafe {
                BorrowedHandle::borrow_raw(GetStdHandle(STD_OUTPUT_HANDLE))
            })
            .map(|(w, _h)| w.0 as _)
        };

        #[rustfmt::skip]
        return match self {
            Self::Set(width)  => Some(width),
            Self::Automatic   => stdout_term_width,
        };
    }
}

/// The default spacing mode for different view types.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum SpacingMode {
    /// Grid mode default (2 spaces)
    Grid,
    /// Details/tree mode default (1 space)
    Details,
}

impl SpacingMode {
    /// Get the default number of spaces for this mode.
    #[must_use]
    pub fn default_spaces(self) -> usize {
        match self {
            Self::Grid => 2,
            Self::Details => 1,
        }
    }
}

/// The spacing between columns requested by the user.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum SpacingBetweenColumns {
    /// The user requested this specific number of spaces.
    Set(usize),
    /// Use the default spacing based on the current mode.
    Default,
}

impl SpacingBetweenColumns {
    /// Get the actual number of spaces to use between columns.
    /// Takes the spacing mode to determine the correct default.
    #[must_use]
    pub fn spaces(self, mode: SpacingMode) -> usize {
        match self {
            Self::Set(spaces) => spaces,
            Self::Default => mode.default_spaces(),
        }
    }
}
