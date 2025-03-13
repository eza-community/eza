// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
#[cfg(unix)]
use locale::Numeric as NumericLocale;
use nu_ansi_term::Style;

#[cfg(unix)]
use crate::fs::fields as f;
#[cfg(unix)]
use crate::output::cell::TextCell;

#[cfg(unix)]
impl f::Links {
    pub fn render<C: Colours>(&self, colours: &C, numeric: &NumericLocale) -> TextCell {
        let style = if self.multiple {
            colours.multi_link_file()
        } else {
            colours.normal()
        };

        TextCell::paint(style, numeric.format_int(self.count))
    }
}

#[allow(unused)]
pub trait Colours {
    fn normal(&self) -> Style;
    fn multi_link_file(&self) -> Style;
}

#[cfg(test)]
pub mod test {
    use super::Colours;
    #[cfg(unix)]
    use crate::fs::fields as f;
    #[cfg(unix)]
    use crate::output::cell::{DisplayWidth, TextCell};

    #[cfg(unix)]
    use locale;
    use nu_ansi_term::Color::*;
    use nu_ansi_term::Style;

    #[allow(dead_code)]
    struct TestColours;

    impl Colours for TestColours {
        fn normal(&self) -> Style {
            Blue.normal()
        }
        fn multi_link_file(&self) -> Style {
            Blue.on(Red)
        }
    }

    #[test]
    #[cfg(unix)]
    fn regular_file() {
        let stati = f::Links {
            count: 1,
            multiple: false,
        };

        let expected = TextCell {
            width: DisplayWidth::from(1),
            contents: vec![Blue.paint("1")].into(),
        };

        assert_eq!(
            expected,
            stati.render(&TestColours, &locale::Numeric::english())
        );
    }

    #[test]
    #[cfg(unix)]
    fn regular_directory() {
        let stati = f::Links {
            count: 3005,
            multiple: false,
        };

        let expected = TextCell {
            width: DisplayWidth::from(5),
            contents: vec![Blue.paint("3,005")].into(),
        };

        assert_eq!(
            expected,
            stati.render(&TestColours, &locale::Numeric::english())
        );
    }

    #[test]
    #[cfg(unix)]
    fn popular_file() {
        let stati = f::Links {
            count: 3005,
            multiple: true,
        };

        let expected = TextCell {
            width: DisplayWidth::from(5),
            contents: vec![Blue.on(Red).paint("3,005")].into(),
        };

        assert_eq!(
            expected,
            stati.render(&TestColours, &locale::Numeric::english())
        );
    }
}
