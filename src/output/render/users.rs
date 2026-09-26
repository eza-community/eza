// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use nu_ansi_term::Style;
use uzers::Users;

use crate::fs::fields as f;
use crate::output::cell::TextCell;
use crate::output::table::UserFormat;

pub trait Render {
    fn render<C: Colours, U: Users>(self, colours: &C, users: &U, format: UserFormat) -> TextCell;
}

impl Render for Option<f::User> {
    fn render<C: Colours, U: Users>(self, colours: &C, users: &U, format: UserFormat) -> TextCell {
        #[rustfmt::skip]
        let uid = match self {
            Some(u) => u.0,
            None    => return TextCell::blank(colours.no_user()),
        };
        #[rustfmt::skip]
        let user_name = match (format, users.get_user_by_uid(uid)) {
            (_, None)                               => uid.to_string(),
            (UserFormat::Numeric, _)                => uid.to_string(),
            (UserFormat::Name, Some(user))          => user.name().to_string_lossy().into(),
            (UserFormat::Truncated(n), Some(user))  => {
                let name: String = user.name().to_string_lossy().into();
                if name.chars().count() > n {
                    let truncated: String = name.chars().take(n).collect();
                    format!("{truncated}…")
                } else {
                    name
                }
            }
        };

        let style = if users.get_current_uid() == uid {
            colours.you()
        } else if uid == 0 {
            colours.root()
        } else {
            colours.other()
        };
        TextCell::paint(style, user_name)
    }
}

pub trait Colours {
    fn you(&self) -> Style;
    fn other(&self) -> Style;
    fn root(&self) -> Style;
    fn no_user(&self) -> Style;
}

#[cfg(test)]
#[allow(unused_results)]
pub mod test {
    use super::{Colours, Render};
    use crate::fs::fields as f;
    use crate::output::cell::TextCell;
    use crate::output::table::UserFormat;

    use nu_ansi_term::Color::*;
    use nu_ansi_term::Style;
    use uzers::User;
    use uzers::mock::MockUsers;

    struct TestColours;

    #[rustfmt::skip]
    impl Colours for TestColours {
        fn you(&self)          -> Style { Red.bold() }
        fn other(&self) -> Style { Blue.underline() }
        fn root(&self)         -> Style { Blue.underline() }
        fn no_user(&self)      -> Style { Black.italic() }
    }

    #[test]
    fn named() {
        let mut users = MockUsers::with_current_uid(1000);
        users.add_user(User::new(1000, "enoch", 100));

        let user = Some(f::User(1000));
        let expected = TextCell::paint_str(Red.bold(), "enoch");
        #[rustfmt::skip]
        assert_eq!(expected, user.render(&TestColours, &users, UserFormat::Name));

        let expected = TextCell::paint_str(Red.bold(), "1000");
        #[rustfmt::skip]
        assert_eq!(expected, user.render(&TestColours, &users, UserFormat::Numeric));
    }

    #[test]
    fn unnamed() {
        let users = MockUsers::with_current_uid(1000);

        let user = Some(f::User(1000));
        let expected = TextCell::paint_str(Red.bold(), "1000");
        #[rustfmt::skip]
        assert_eq!(expected, user.render(&TestColours, &users, UserFormat::Name));
        #[rustfmt::skip]
        assert_eq!(expected, user.render(&TestColours, &users, UserFormat::Numeric));
    }

    #[test]
    fn different_named() {
        let mut users = MockUsers::with_current_uid(0);
        users.add_user(User::new(1000, "enoch", 100));

        let user = Some(f::User(1000));
        let expected = TextCell::paint_str(Blue.underline(), "enoch");
        assert_eq!(
            expected,
            user.render(&TestColours, &users, UserFormat::Name)
        );
    }

    #[test]
    fn different_unnamed() {
        let user = Some(f::User(1000));
        let expected = TextCell::paint_str(Blue.underline(), "1000");
        assert_eq!(
            expected,
            user.render(
                &TestColours,
                &MockUsers::with_current_uid(0),
                UserFormat::Numeric
            )
        );
    }

    #[test]
    fn overflow() {
        let user = Some(f::User(2_147_483_648));
        let expected = TextCell::paint_str(Blue.underline(), "2147483648");
        assert_eq!(
            expected,
            user.render(
                &TestColours,
                &MockUsers::with_current_uid(0),
                UserFormat::Numeric
            )
        );
    }

    #[test]
    fn truncated_short() {
        let mut users = MockUsers::with_current_uid(1000);
        users.add_user(User::new(1000, "ada", 100));

        let user = Some(f::User(1000));
        let expected = TextCell::paint_str(Red.bold(), "ada");
        assert_eq!(
            expected,
            user.render(&TestColours, &users, UserFormat::Truncated(8))
        );
    }

    #[test]
    fn truncated_exact() {
        let mut users = MockUsers::with_current_uid(1000);
        users.add_user(User::new(1000, "enoch", 100));

        let user = Some(f::User(1000));
        let expected = TextCell::paint_str(Red.bold(), "enoch");
        assert_eq!(
            expected,
            user.render(&TestColours, &users, UserFormat::Truncated(5))
        );
    }

    #[test]
    fn truncated_long() {
        let mut users = MockUsers::with_current_uid(1000);
        users.add_user(User::new(1000, "roland.wohlfahrt", 100));

        let user = Some(f::User(1000));
        let expected = TextCell::paint_str(Red.bold(), "roland…");
        assert_eq!(
            expected,
            user.render(&TestColours, &users, UserFormat::Truncated(6))
        );
    }

    #[test]
    fn truncated_unicode() {
        let mut users = MockUsers::with_current_uid(1000);
        users.add_user(User::new(1000, "こんにちは", 100));

        let user = Some(f::User(1000));
        // truncated at char boundary, not byte boundary
        let expected = TextCell::paint_str(Red.bold(), "こんに…");
        assert_eq!(
            expected,
            user.render(&TestColours, &users, UserFormat::Truncated(3))
        );
    }

    #[test]
    fn truncated_unknown_uid() {
        let users = MockUsers::with_current_uid(0);

        let user = Some(f::User(1000));
        // unknown UID falls through to numeric regardless of format
        let expected = TextCell::paint_str(Blue.underline(), "1000");
        assert_eq!(
            expected,
            user.render(&TestColours, &users, UserFormat::Truncated(4))
        );
    }
}
