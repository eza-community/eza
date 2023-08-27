use std::iter;

use nu_ansi_term::{AnsiString, Style};

use crate::fs::fields as f;
use crate::output::cell::{TextCell, DisplayWidth};
use crate::output::render::FiletypeColors;

pub trait PermissionsPlusRender {
    fn render<C: Colors+FiletypeColors>(&self, colors: &C) -> TextCell;
}

#[cfg(unix)]
impl PermissionsPlusRender for Option<f::PermissionsPlus> {
    fn render<C: Colors+FiletypeColors>(&self, colors: &C) -> TextCell {
        match self {
            Some(p) => {
                let mut chars = vec![ p.file_type.render(colors) ];
                let permissions = p.permissions;
                chars.extend(Some(permissions).render(colors, p.file_type.is_regular_file()));

                if p.xattrs {
                   chars.push(colors.attribute().paint("@"));
                }

                // As these are all ASCII characters, we can guarantee that they’re
                // all going to be one character wide, and don’t need to compute the
                // cell’s display width.
                TextCell {
                    width:    DisplayWidth::from(chars.len()),
                    contents: chars.into(),
                }
            },
            None => {
                let chars: Vec<_> = iter::repeat(colors.dash().paint("-")).take(10).collect();
                TextCell {
                    width:    DisplayWidth::from(chars.len()),
                    contents: chars.into(),
                }
            }
        }
    }

    #[cfg(windows)]
    pub fn render<C: Colors+FiletypeColors>(&self, colors: &C) -> TextCell {
        let mut chars = vec![ self.attributes.render_type(colors) ];
        chars.extend(self.attributes.render(colors));

        TextCell {
            width:    DisplayWidth::from(chars.len()),
            contents: chars.into(),
        }
    }
}

pub trait RenderPermissions {
    fn render<C: Colors>(&self, colors: &C, is_regular_file: bool) -> Vec<AnsiString<'static>>;
}

impl RenderPermissions for Option<f::Permissions> {
    fn render<C: Colors>(&self, colors: &C, is_regular_file: bool) -> Vec<AnsiString<'static>> {
        match self {
            Some(p) => {
                let bit = |bit, chr: &'static str, style: Style| {
                    if bit { style.paint(chr) }
                      else { colors.dash().paint("-") }
                };

                vec![
                    bit(p.user_read,   "r", colors.user_read()),
                    bit(p.user_write,  "w", colors.user_write()),
                    p.user_execute_bit(colors, is_regular_file),
                    bit(p.group_read,  "r", colors.group_read()),
                    bit(p.group_write, "w", colors.group_write()),
                    p.group_execute_bit(colors),
                    bit(p.other_read,  "r", colors.other_read()),
                    bit(p.other_write, "w", colors.other_write()),
                    p.other_execute_bit(colors)
                ]
            },
            None => {
                iter::repeat(colors.dash().paint("-")).take(9).collect()
            }
        }
    }
}

impl f::Permissions {
    fn user_execute_bit<C: Colors>(&self, colors: &C, is_regular_file: bool) -> AnsiString<'static> {
        match (self.user_execute, self.setuid, is_regular_file) {
            (false, false, _)      => colors.dash().paint("-"),
            (true,  false, false)  => colors.user_execute_other().paint("x"),
            (true,  false, true)   => colors.user_execute_file().paint("x"),
            (false, true,  _)      => colors.special_other().paint("S"),
            (true,  true,  false)  => colors.special_other().paint("s"),
            (true,  true,  true)   => colors.special_user_file().paint("s"),
        }
    }

    fn group_execute_bit<C: Colors>(&self, colors: &C) -> AnsiString<'static> {
        match (self.group_execute, self.setgid) {
            (false, false)  => colors.dash().paint("-"),
            (true,  false)  => colors.group_execute().paint("x"),
            (false, true)   => colors.special_other().paint("S"),
            (true,  true)   => colors.special_other().paint("s"),
        }
    }

    fn other_execute_bit<C: Colors>(&self, colors: &C) -> AnsiString<'static> {
        match (self.other_execute, self.sticky) {
            (false, false)  => colors.dash().paint("-"),
            (true,  false)  => colors.other_execute().paint("x"),
            (false, true)   => colors.special_other().paint("T"),
            (true,  true)   => colors.special_other().paint("t"),
        }
    }
}

#[cfg(windows)]
impl f::Attributes {
    pub fn render<C: Colors+FiletypeColors>(&self, colors: &C) -> Vec<AnsiString<'static>> {
        let bit = |bit, chr: &'static str, style: Style| {
            if bit { style.paint(chr) }
              else { colors.dash().paint("-") }
        };

        vec![
            bit(self.archive,   "a", colors.normal()),
            bit(self.readonly,  "r", colors.user_read()),
            bit(self.hidden,    "h", colors.special_user_file()),
            bit(self.system,    "s", colors.special_other()),
        ]
    }

    pub fn render_type<C: Colors+FiletypeColors>(&self, colors: &C) -> AnsiString<'static> {
        if self.reparse_point {
            return colors.pipe().paint("l")
        }
        else if self.directory {
            return colors.directory().paint("d")
        }
        else {
            return colors.dash().paint("-")
        }
    }
}

pub trait Colors {
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


#[cfg(test)]
#[allow(unused_results)]
pub mod test {
    use super::{Colors, RenderPermissions};
    use crate::output::cell::TextCellContents;
    use crate::fs::fields as f;

    use nu_ansi_term::Color::*;
    use nu_ansi_term::Style;


    struct TestColors;

    impl Colors for TestColors {
        fn dash(&self)                -> Style { Fixed(11).normal() }
        fn user_read(&self)           -> Style { Fixed(101).normal() }
        fn user_write(&self)          -> Style { Fixed(102).normal() }
        fn user_execute_file(&self)   -> Style { Fixed(103).normal() }
        fn user_execute_other(&self)  -> Style { Fixed(113).normal() }
        fn group_read(&self)          -> Style { Fixed(104).normal() }
        fn group_write(&self)         -> Style { Fixed(105).normal() }
        fn group_execute(&self)       -> Style { Fixed(106).normal() }
        fn other_read(&self)          -> Style { Fixed(107).normal() }
        fn other_write(&self)         -> Style { Fixed(108).normal() }
        fn other_execute(&self)       -> Style { Fixed(109).normal() }
        fn special_user_file(&self)   -> Style { Fixed(110).normal() }
        fn special_other(&self)       -> Style { Fixed(111).normal() }
        fn attribute(&self)           -> Style { Fixed(112).normal() }
    }


    #[test]
    fn negate() {
        let bits = Some(f::Permissions {
            user_read:  false,  user_write:  false,  user_execute:  false,  setuid: false,
            group_read: false,  group_write: false,  group_execute: false,  setgid: false,
            other_read: false,  other_write: false,  other_execute: false,  sticky: false,
        });

        let expected = TextCellContents::from(vec![
            Fixed(11).paint("-"),  Fixed(11).paint("-"),  Fixed(11).paint("-"),
            Fixed(11).paint("-"),  Fixed(11).paint("-"),  Fixed(11).paint("-"),
            Fixed(11).paint("-"),  Fixed(11).paint("-"),  Fixed(11).paint("-"),
        ]);

        assert_eq!(expected, bits.render(&TestColors, false).into())
    }


    #[test]
    fn affirm() {
        let bits = Some(f::Permissions {
            user_read:  true,  user_write:  true,  user_execute:  true,  setuid: false,
            group_read: true,  group_write: true,  group_execute: true,  setgid: false,
            other_read: true,  other_write: true,  other_execute: true,  sticky: false,
        });

        let expected = TextCellContents::from(vec![
            Fixed(101).paint("r"),  Fixed(102).paint("w"),  Fixed(103).paint("x"),
            Fixed(104).paint("r"),  Fixed(105).paint("w"),  Fixed(106).paint("x"),
            Fixed(107).paint("r"),  Fixed(108).paint("w"),  Fixed(109).paint("x"),
        ]);

        assert_eq!(expected, bits.render(&TestColors, true).into())
    }


    #[test]
    fn specials() {
        let bits = Some(f::Permissions {
            user_read:  false,  user_write:  false,  user_execute:  true,  setuid: true,
            group_read: false,  group_write: false,  group_execute: true,  setgid: true,
            other_read: false,  other_write: false,  other_execute: true,  sticky: true,
        });

        let expected = TextCellContents::from(vec![
            Fixed(11).paint("-"),  Fixed(11).paint("-"),  Fixed(110).paint("s"),
            Fixed(11).paint("-"),  Fixed(11).paint("-"),  Fixed(111).paint("s"),
            Fixed(11).paint("-"),  Fixed(11).paint("-"),  Fixed(111).paint("t"),
        ]);

        assert_eq!(expected, bits.render(&TestColors, true).into())
    }


    #[test]
    fn extra_specials() {
        let bits = Some(f::Permissions {
            user_read:  false,  user_write:  false,  user_execute:  false,  setuid: true,
            group_read: false,  group_write: false,  group_execute: false,  setgid: true,
            other_read: false,  other_write: false,  other_execute: false,  sticky: true,
        });

        let expected = TextCellContents::from(vec![
            Fixed(11).paint("-"),  Fixed(11).paint("-"),  Fixed(111).paint("S"),
            Fixed(11).paint("-"),  Fixed(11).paint("-"),  Fixed(111).paint("S"),
            Fixed(11).paint("-"),  Fixed(11).paint("-"),  Fixed(111).paint("T"),
        ]);

        assert_eq!(expected, bits.render(&TestColors, true).into())
    }
}
