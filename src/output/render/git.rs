// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use nu_ansi_term::{AnsiString as ANSIString, Style};

use crate::fs::fields as f;
use crate::output::cell::{DisplayWidth, TextCell};

impl f::Git {
    pub fn render(self, colours: &dyn Colours) -> TextCell {
        TextCell {
            width: DisplayWidth::from(2),
            contents: vec![self.staged.render(colours), self.unstaged.render(colours)].into(),
        }
    }
}

impl f::GitStatus {
    fn render(self, colours: &dyn Colours) -> ANSIString<'static> {
        #[rustfmt::skip]
        return match self {
            Self::NotModified  => colours.not_modified().paint("-"),
            Self::New          => colours.new().paint("N"),
            Self::Modified     => colours.modified().paint("M"),
            Self::Deleted      => colours.deleted().paint("D"),
            Self::Renamed      => colours.renamed().paint("R"),
            Self::TypeChange   => colours.type_change().paint("T"),
            Self::Ignored      => colours.ignored().paint("I"),
            Self::Conflicted   => colours.conflicted().paint("U"),
        };
    }
}

pub trait Colours {
    fn not_modified(&self) -> Style;
    // FIXME: this amount of allows needed to keep clippy happy should be enough
    // of an argument that new needs to be renamed.
    #[allow(clippy::new_ret_no_self, clippy::wrong_self_convention)]
    fn new(&self) -> Style;
    fn modified(&self) -> Style;
    fn deleted(&self) -> Style;
    fn renamed(&self) -> Style;
    fn type_change(&self) -> Style;
    fn ignored(&self) -> Style;
    fn conflicted(&self) -> Style;
}

impl f::SubdirGitRepo {
    pub fn render(self, colours: &dyn RepoColours) -> TextCell {
        let branch_name = match self.branch {
            Some(name) => match name.as_ref() {
                "main" | "master" => colours.branch_main().paint(name),
                _ => colours.branch_other().paint(name),
            },
            None => colours.no_repo().paint("-"),
        };

        if let Some(status) = self.status {
            TextCell {
                width: DisplayWidth::from(2) + DisplayWidth::from(branch_name.as_str()),
                contents: vec![
                    status.render(colours),
                    Style::default().paint(" "),
                    branch_name,
                ]
                .into(),
            }
        } else {
            TextCell {
                width: DisplayWidth::from(branch_name.as_str()),
                contents: vec![branch_name].into(),
            }
        }
    }
}

impl f::SubdirGitRepoStatus {
    pub fn render(self, colours: &dyn RepoColours) -> ANSIString<'static> {
        match self {
            Self::NoRepo => colours.no_repo().paint("-"),
            Self::GitClean => colours.git_clean().paint("|"),
            Self::GitDirty => colours.git_dirty().paint("+"),
        }
    }
}

pub trait RepoColours {
    fn branch_main(&self) -> Style;
    fn branch_other(&self) -> Style;
    fn no_repo(&self) -> Style;
    fn git_clean(&self) -> Style;
    fn git_dirty(&self) -> Style;
}

#[cfg(test)]
pub mod test {
    use super::Colours;
    use crate::fs::fields as f;
    use crate::output::cell::{DisplayWidth, TextCell};

    use nu_ansi_term::Color::*;
    use nu_ansi_term::Style;

    struct TestColours;

    impl Colours for TestColours {
        fn not_modified(&self) -> Style {
            Fixed(90).normal()
        }
        fn new(&self) -> Style {
            Fixed(91).normal()
        }
        fn modified(&self) -> Style {
            Fixed(92).normal()
        }
        fn deleted(&self) -> Style {
            Fixed(93).normal()
        }
        fn renamed(&self) -> Style {
            Fixed(94).normal()
        }
        fn type_change(&self) -> Style {
            Fixed(95).normal()
        }
        fn ignored(&self) -> Style {
            Fixed(96).normal()
        }
        fn conflicted(&self) -> Style {
            Fixed(97).normal()
        }
    }

    #[test]
    fn git_blank() {
        let stati = f::Git {
            staged: f::GitStatus::NotModified,
            unstaged: f::GitStatus::NotModified,
        };

        let expected = TextCell {
            width: DisplayWidth::from(2),
            contents: vec![Fixed(90).paint("-"), Fixed(90).paint("-")].into(),
        };

        assert_eq!(expected, stati.render(&TestColours));
    }

    #[test]
    fn git_new_changed() {
        let stati = f::Git {
            staged: f::GitStatus::New,
            unstaged: f::GitStatus::Modified,
        };

        let expected = TextCell {
            width: DisplayWidth::from(2),
            contents: vec![Fixed(91).paint("N"), Fixed(92).paint("M")].into(),
        };

        assert_eq!(expected, stati.render(&TestColours));
    }
}
