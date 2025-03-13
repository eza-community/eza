// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use nu_ansi_term::Style;

use crate::fs::fields as f;
use crate::output::cell::TextCell;

#[allow(unused)]
pub trait Render {
    fn render(&self, style: Style) -> TextCell;
}

impl Render for Option<f::OctalPermissions> {
    fn render(&self, style: Style) -> TextCell {
        match self {
            Some(p) => {
                let perm = &p.permissions;
                #[rustfmt::skip]
                let octal_sticky = f::OctalPermissions::bits_to_octal(
                    perm.setuid,
                    perm.setgid,
                    perm.sticky
                );
                let octal_owner = f::OctalPermissions::bits_to_octal(
                    perm.user_read,
                    perm.user_write,
                    perm.user_execute,
                );
                let octal_group = f::OctalPermissions::bits_to_octal(
                    perm.group_read,
                    perm.group_write,
                    perm.group_execute,
                );
                let octal_other = f::OctalPermissions::bits_to_octal(
                    perm.other_read,
                    perm.other_write,
                    perm.other_execute,
                );

                TextCell::paint(
                    style,
                    format!("{octal_sticky}{octal_owner}{octal_group}{octal_other}"),
                )
            }
            None => TextCell::paint(style, "----".into()),
        }
    }
}

impl f::OctalPermissions {
    fn bits_to_octal(r: bool, w: bool, x: bool) -> u8 {
        u8::from(r) * 4 + u8::from(w) * 2 + u8::from(x)
    }
}

#[cfg(test)]
pub mod test {
    use super::Render;
    use crate::fs::fields as f;
    use crate::output::cell::TextCell;

    use nu_ansi_term::Color::*;

    #[test]
    fn normal_folder() {
        let bits = f::Permissions {
            user_read: true,
            user_write: true,
            user_execute: true,
            setuid: false,
            group_read: true,
            group_write: false,
            group_execute: true,
            setgid: false,
            other_read: true,
            other_write: false,
            other_execute: true,
            sticky: false,
        };

        let octal = Some(f::OctalPermissions { permissions: bits });

        let expected = TextCell::paint_str(Purple.bold(), "0755");
        assert_eq!(expected, octal.render(Purple.bold()));
    }

    #[test]
    fn normal_file() {
        let bits = f::Permissions {
            user_read: true,
            user_write: true,
            user_execute: false,
            setuid: false,
            group_read: true,
            group_write: false,
            group_execute: false,
            setgid: false,
            other_read: true,
            other_write: false,
            other_execute: false,
            sticky: false,
        };

        let octal = Some(f::OctalPermissions { permissions: bits });

        let expected = TextCell::paint_str(Purple.bold(), "0644");
        assert_eq!(expected, octal.render(Purple.bold()));
    }

    #[test]
    fn secret_file() {
        let bits = f::Permissions {
            user_read: true,
            user_write: true,
            user_execute: false,
            setuid: false,
            group_read: false,
            group_write: false,
            group_execute: false,
            setgid: false,
            other_read: false,
            other_write: false,
            other_execute: false,
            sticky: false,
        };

        let octal = Some(f::OctalPermissions { permissions: bits });

        let expected = TextCell::paint_str(Purple.bold(), "0600");
        assert_eq!(expected, octal.render(Purple.bold()));
    }

    #[test]
    fn sticky1() {
        let bits = f::Permissions {
            user_read: true,
            user_write: true,
            user_execute: true,
            setuid: true,
            group_read: true,
            group_write: true,
            group_execute: true,
            setgid: false,
            other_read: true,
            other_write: true,
            other_execute: true,
            sticky: false,
        };

        let octal = Some(f::OctalPermissions { permissions: bits });

        let expected = TextCell::paint_str(Purple.bold(), "4777");
        assert_eq!(expected, octal.render(Purple.bold()));
    }

    #[test]
    fn sticky2() {
        let bits = f::Permissions {
            user_read: true,
            user_write: true,
            user_execute: true,
            setuid: false,
            group_read: true,
            group_write: true,
            group_execute: true,
            setgid: true,
            other_read: true,
            other_write: true,
            other_execute: true,
            sticky: false,
        };

        let octal = Some(f::OctalPermissions { permissions: bits });

        let expected = TextCell::paint_str(Purple.bold(), "2777");
        assert_eq!(expected, octal.render(Purple.bold()));
    }

    #[test]
    fn sticky3() {
        let bits = f::Permissions {
            user_read: true,
            user_write: true,
            user_execute: true,
            setuid: false,
            group_read: true,
            group_write: true,
            group_execute: true,
            setgid: false,
            other_read: true,
            other_write: true,
            other_execute: true,
            sticky: true,
        };

        let octal = Some(f::OctalPermissions { permissions: bits });

        let expected = TextCell::paint_str(Purple.bold(), "1777");
        assert_eq!(expected, octal.render(Purple.bold()));
    }
}
