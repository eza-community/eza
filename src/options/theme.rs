use crate::options::{flags, vars, Vars, OptionsError};
use crate::options::parser::MatchedFlags;
use crate::theme::{Options, UseColors, ColorScale, Definitions};


impl Options {
    pub fn deduce<V: Vars>(matches: &MatchedFlags<'_>, vars: &V) -> Result<Self, OptionsError> {
        let use_colors = UseColors::deduce(matches, vars)?;
        let color_scale = ColorScale::deduce(matches)?;

        let definitions = if use_colors == UseColors::Never {
                Definitions::default()
            }
            else {
                Definitions::deduce(vars)
            };

        Ok(Self { use_colors, color_scale, definitions })
    }
}


impl UseColors {
    fn deduce<V: Vars>(matches: &MatchedFlags<'_>, vars: &V) -> Result<Self, OptionsError> {
        let default_value = match vars.get(vars::NO_COLOR) {
            Some(_) => Self::Never,
            None => Self::Automatic,
        };

        let Some(word) = matches.get_where(|f| f.matches(&flags::COLOR) || f.matches(&flags::COLOUR))? else { return Ok(default_value) };

        if word == "always" {
            Ok(Self::Always)
        }
        else if word == "auto" || word == "automatic" {
            Ok(Self::Automatic)
        }
        else if word == "never" {
            Ok(Self::Never)
        }
        else {
            Err(OptionsError::BadArgument(&flags::COLOR, word.into()))
        }
    }
}


impl ColorScale {
    fn deduce(matches: &MatchedFlags<'_>) -> Result<Self, OptionsError> {
        if matches.has_where(|f| f.matches(&flags::COLOR_SCALE) || f.matches(&flags::COLOUR_SCALE))?.is_some() {
            Ok(Self::Gradient)
        }
        else {
            Ok(Self::Fixed)
        }
    }
}


impl Definitions {
    fn deduce<V: Vars>(vars: &V) -> Self {
        let ls =  vars.get(vars::LS_COLORS) .map(|e| e.to_string_lossy().to_string());
        let exa = vars.get(vars::EXA_COLORS).map(|e| e.to_string_lossy().to_string());
        Self { ls, exa }
    }
}


#[cfg(test)]
mod terminal_test {
    use super::*;
    use std::ffi::OsString;
    use crate::options::flags;
    use crate::options::parser::{Flag, Arg};

    use crate::options::test::parse_for_test;
    use crate::options::test::Strictnesses::*;

    static TEST_ARGS: &[&Arg] = &[ &flags::COLOR,       &flags::COLOUR,
                                   &flags::COLOR_SCALE, &flags::COLOUR_SCALE, ];

    macro_rules! test {
        ($name:ident:  $type:ident <- $inputs:expr;  $stricts:expr => $result:expr) => {
            #[test]
            fn $name() {
                for result in parse_for_test($inputs.as_ref(), TEST_ARGS, $stricts, |mf| $type::deduce(mf)) {
                    assert_eq!(result, $result);
                }
            }
        };

        ($name:ident:  $type:ident <- $inputs:expr, $env:expr;  $stricts:expr => $result:expr) => {
            #[test]
            fn $name() {
                let env = $env;
                for result in parse_for_test($inputs.as_ref(), TEST_ARGS, $stricts, |mf| $type::deduce(mf, &env)) {
                    assert_eq!(result, $result);
                }
            }
        };

        ($name:ident:  $type:ident <- $inputs:expr;  $stricts:expr => err $result:expr) => {
            #[test]
            fn $name() {
                for result in parse_for_test($inputs.as_ref(), TEST_ARGS, $stricts, |mf| $type::deduce(mf)) {
                    assert_eq!(result.unwrap_err(), $result);
                }
            }
        };

        ($name:ident:  $type:ident <- $inputs:expr, $env:expr;  $stricts:expr => err $result:expr) => {
            #[test]
            fn $name() {
                let env = $env;
                for result in parse_for_test($inputs.as_ref(), TEST_ARGS, $stricts, |mf| $type::deduce(mf, &env)) {
                    assert_eq!(result.unwrap_err(), $result);
                }
            }
        };
    }

    struct MockVars {
        ls: &'static str,
        exa: &'static str,
        no_color: &'static str,
    }

    impl MockVars {
        fn empty() -> MockVars {
            MockVars {
                ls: "",
                exa: "",
                no_color: "",
            }
        }
        fn with_no_color() -> MockVars {
            MockVars {
                ls: "",
                exa: "",
                no_color: "true",
            }
        }
    }

    // Test impl that just returns the value it has.
    impl Vars for MockVars {
        fn get(&self, name: &'static str) -> Option<OsString> {
            if name == vars::LS_COLORS && ! self.ls.is_empty() {
                Some(OsString::from(self.ls))
            }
            else if name == vars::EXA_COLORS && ! self.exa.is_empty() {
                Some(OsString::from(self.exa))
            }
            else if name == vars::NO_COLOR && ! self.no_color.is_empty() {
                Some(OsString::from(self.no_color))
            }
            else {
                None
            }
        }
    }



    // Default
    test!(empty:               UseColors <- [], MockVars::empty();                     Both => Ok(UseColors::Automatic));
    test!(empty_with_no_color: UseColors <- [], MockVars::with_no_color();             Both => Ok(UseColors::Never));

    // --colour
    test!(u_always:      UseColors <- ["--colour=always"], MockVars::empty();    Both => Ok(UseColors::Always));
    test!(u_auto:        UseColors <- ["--colour", "auto"], MockVars::empty();   Both => Ok(UseColors::Automatic));
    test!(u_never:       UseColors <- ["--colour=never"], MockVars::empty();     Both => Ok(UseColors::Never));

    // --color
    test!(no_u_always:   UseColors <- ["--color", "always"], MockVars::empty();  Both => Ok(UseColors::Always));
    test!(no_u_auto:     UseColors <- ["--color=auto"], MockVars::empty();       Both => Ok(UseColors::Automatic));
    test!(no_u_never:    UseColors <- ["--color", "never"], MockVars::empty();   Both => Ok(UseColors::Never));

    // Errors
    test!(no_u_error:    UseColors <- ["--color=upstream"], MockVars::empty();   Both => err OptionsError::BadArgument(&flags::COLOR, OsString::from("upstream"))); // the error is for --color
    test!(u_error:       UseColors <- ["--colour=lovers"], MockVars::empty();    Both => err OptionsError::BadArgument(&flags::COLOR, OsString::from("lovers"))); // and so is this one!

    // Overriding
    test!(overridden_1:  UseColors <- ["--colour=auto", "--colour=never"], MockVars::empty();  Last => Ok(UseColors::Never));
    test!(overridden_2:  UseColors <- ["--color=auto",  "--colour=never"], MockVars::empty();  Last => Ok(UseColors::Never));
    test!(overridden_3:  UseColors <- ["--colour=auto", "--color=never"], MockVars::empty();   Last => Ok(UseColors::Never));
    test!(overridden_4:  UseColors <- ["--color=auto",  "--color=never"], MockVars::empty();   Last => Ok(UseColors::Never));

    test!(overridden_5:  UseColors <- ["--colour=auto", "--colour=never"], MockVars::empty();  Complain => err OptionsError::Duplicate(Flag::Long("colour"), Flag::Long("colour")));
    test!(overridden_6:  UseColors <- ["--color=auto",  "--colour=never"], MockVars::empty();  Complain => err OptionsError::Duplicate(Flag::Long("color"),  Flag::Long("colour")));
    test!(overridden_7:  UseColors <- ["--colour=auto", "--color=never"], MockVars::empty();   Complain => err OptionsError::Duplicate(Flag::Long("colour"), Flag::Long("color")));
    test!(overridden_8:  UseColors <- ["--color=auto",  "--color=never"], MockVars::empty();   Complain => err OptionsError::Duplicate(Flag::Long("color"),  Flag::Long("color")));

    test!(scale_1:  ColorScale <- ["--color-scale", "--colour-scale"];   Last => Ok(ColorScale::Gradient));
    test!(scale_2:  ColorScale <- ["--color-scale",                 ];   Last => Ok(ColorScale::Gradient));
    test!(scale_3:  ColorScale <- [                 "--colour-scale"];   Last => Ok(ColorScale::Gradient));
    test!(scale_4:  ColorScale <- [                                 ];   Last => Ok(ColorScale::Fixed));

    test!(scale_5:  ColorScale <- ["--color-scale", "--colour-scale"];   Complain => err OptionsError::Duplicate(Flag::Long("color-scale"),  Flag::Long("colour-scale")));
    test!(scale_6:  ColorScale <- ["--color-scale",                 ];   Complain => Ok(ColorScale::Gradient));
    test!(scale_7:  ColorScale <- [                 "--colour-scale"];   Complain => Ok(ColorScale::Gradient));
    test!(scale_8:  ColorScale <- [                                 ];   Complain => Ok(ColorScale::Fixed));
}
