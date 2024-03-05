use crate::options::parser::{AbsoluteArgs, Opts, ShowWhen};
use crate::options::vars::{self, Vars};
use crate::options::{NumberSource, OptionsError};

use crate::output::file_name::{
    Absolute, Classify, EmbedHyperlinks, Options, QuoteStyle, ShowIcons,
};

impl Options {
    pub fn deduce<V: Vars>(matches: &Opts, vars: &V, is_a_tty: bool) -> Result<Self, OptionsError> {
        let classify = Classify::deduce(matches);
        let show_icons = ShowIcons::deduce(matches, vars)?;

        let quote_style = QuoteStyle::deduce(matches);
        let embed_hyperlinks = EmbedHyperlinks::deduce(matches);

        let absolute = Absolute::deduce(matches);

        Ok(Self {
            classify,
            show_icons,
            quote_style,
            embed_hyperlinks,
            absolute,
            is_a_tty,
        })
    }
}

impl Classify {
    fn deduce(matches: &Opts) -> Self {
        if matches.classify {
            Self::AddFileIndicators
        } else {
            Self::JustFilenames
        }
    }
}

impl ShowIcons {
    pub fn deduce<V: Vars>(matches: &Opts, vars: &V) -> Result<Self, OptionsError> {
        enum AlwaysOrAuto {
            Always,
            Automatic,
        }

        let force_icons = vars.get(vars::EZA_ICONS_AUTO).is_some();
        let mode_opt = &matches.icons;
        if !force_icons && mode_opt.is_none() {
            return Ok(Self::Never);
        }

        let mode = match mode_opt {
            Some(word) => match word {
                ShowWhen::Always => AlwaysOrAuto::Always,
                ShowWhen::Auto => AlwaysOrAuto::Automatic,
                ShowWhen::Never => return Ok(Self::Never),
            },
            None => AlwaysOrAuto::Automatic,
        };

        let width = if let Some(columns) = vars
            .get_with_fallback(vars::EXA_ICON_SPACING, vars::EZA_ICON_SPACING)
            .map(|s| s.to_string_lossy().to_string())
        {
            match columns.parse() {
                Ok(width) => width,
                Err(e) => {
                    let source = NumberSource::Env(
                        vars.source(vars::EXA_ICON_SPACING, vars::EZA_ICON_SPACING)
                            .unwrap_or("1"),
                    );
                    return Err(OptionsError::FailedParse(columns.to_string(), source, e));
                }
            }
        } else {
            1
        };

        match mode {
            AlwaysOrAuto::Always => Ok(Self::Always(width)),
            AlwaysOrAuto::Automatic => Ok(Self::Automatic(width)),
        }
    }
}

impl QuoteStyle {
    pub fn deduce(matches: &Opts) -> Self {
        if matches.no_quotes {
            Self::NoQuotes
        } else {
            Self::QuoteSpaces
        }
    }
}

impl EmbedHyperlinks {
    fn deduce(matches: &Opts) -> Self {
        if matches.hyperlink {
            Self::On
        } else {
            Self::Off
        }
    }
}

impl Absolute {
    fn deduce(matches: &Opts) -> Self {
        match matches.absolute {
            Some(AbsoluteArgs::Yes | AbsoluteArgs::On) => Self::On,
            Some(AbsoluteArgs::Follow) => Self::Follow,
            Some(AbsoluteArgs::Off | AbsoluteArgs::No) | None => Self::Off,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::parser::ShowWhen;
    use crate::options::vars::MockVars;
    use clap::ValueEnum;
    use std::ffi::OsString;
    use std::num::ParseIntError;
    #[test]
    fn deduce_classify_file_indicators() {
        let options = Opts {
            classify: true,
            ..Opts::default()
        };

        assert_eq!(Classify::deduce(&options), Classify::AddFileIndicators);
    }

    #[test]
    fn deduce_classify_just_filenames() {
        let options = Opts { ..Opts::default() };

        assert_eq!(Classify::deduce(&options), Classify::JustFilenames);
    }

    #[test]
    fn deduce_quote_style_no_quotes() {
        let options = Opts {
            no_quotes: true,
            ..Opts::default()
        };

        assert_eq!(QuoteStyle::deduce(&options), QuoteStyle::NoQuotes);
    }

    #[test]
    fn deduce_quote_style_quote_spaces() {
        let options = Opts { ..Opts::default() };

        assert_eq!(QuoteStyle::deduce(&options), QuoteStyle::QuoteSpaces);
    }

    #[test]
    fn deduce_embed_hyperlinks_on() {
        let options = Opts {
            hyperlink: true,
            ..Opts::default()
        };

        assert_eq!(EmbedHyperlinks::deduce(&options), EmbedHyperlinks::On);
    }

    #[test]
    fn deduce_embed_hyperlinks_off() {
        let options = Opts { ..Opts::default() };

        assert_eq!(EmbedHyperlinks::deduce(&options), EmbedHyperlinks::Off);
    }

    #[test]
    fn deduce_show_icons_never_no_arg() {
        let options = Opts { ..Opts::default() };

        let vars = MockVars {
            ..MockVars::default()
        };

        assert_eq!(ShowIcons::deduce(&options, &vars), Ok(ShowIcons::Never));
    }

    #[test]
    fn deduce_show_icons_never_no_arg_env() {
        let options = Opts { ..Opts::default() };

        let mut vars = MockVars {
            ..MockVars::default()
        };

        vars.set(vars::EZA_ICONS_AUTO, &OsString::from("1"));

        assert_eq!(
            ShowIcons::deduce(&options, &vars),
            Ok(ShowIcons::Automatic(1))
        );
    }

    #[test]
    fn deduce_show_icon_always() {
        let options = Opts {
            icons: Some(ShowWhen::Always),
            ..Opts::default()
        };

        let vars = MockVars {
            ..MockVars::default()
        };

        assert_eq!(ShowIcons::deduce(&options, &vars), Ok(ShowIcons::Always(1)));
    }

    #[test]
    fn deduce_show_icons_never() {
        let options = Opts {
            icons: Some(ShowWhen::Never),
            ..Opts::default()
        };

        let vars = MockVars {
            ..MockVars::default()
        };

        assert_eq!(ShowIcons::deduce(&options, &vars), Ok(ShowIcons::Never));
    }

    #[test]
    fn deduce_show_icons_auto() {
        let options = Opts {
            icons: Some(ShowWhen::from_str("auto", false).unwrap()),
            ..Opts::default()
        };

        let vars = MockVars {
            ..MockVars::default()
        };

        assert_eq!(
            ShowIcons::deduce(&options, &vars),
            Ok(ShowIcons::Automatic(1))
        );
    }

    #[test]
    fn deduce_show_icons_error() {
        assert_eq!(
            ShowWhen::from_str("foo", false)
                .map_err(|err| OptionsError::BadArgument("icons", err.into())),
            Err(OptionsError::BadArgument("icons", OsString::from("foo")))
        );
    }

    #[test]
    fn deduce_show_icons_width() {
        let options = Opts {
            icons: Some(ShowWhen::from_str("", false).unwrap()),
            ..Opts::default()
        };

        let mut vars = MockVars {
            ..MockVars::default()
        };

        vars.set(vars::EZA_ICON_SPACING, &OsString::from("3"));

        assert_eq!(
            ShowIcons::deduce(&options, &vars),
            Ok(ShowIcons::Automatic(3))
        );
    }

    #[test]
    fn deduce_show_icons_width_error() {
        let options = Opts {
            icons: Some(ShowWhen::from_str("auto", false).unwrap()),
            ..Opts::default()
        };

        let mut vars = MockVars {
            ..MockVars::default()
        };

        vars.set(vars::EZA_ICON_SPACING, &OsString::from("foo"));

        let e: Result<i64, ParseIntError> = vars
            .get(vars::EZA_ICON_SPACING)
            .unwrap()
            .to_string_lossy()
            .parse();

        assert_eq!(
            ShowIcons::deduce(&options, &vars),
            Err(OptionsError::FailedParse(
                String::from("foo"),
                NumberSource::Env(vars::EXA_ICON_SPACING),
                e.unwrap_err()
            ))
        );
    }

    #[test]
    fn deduce_options() {
        let options = Opts { ..Opts::default() };

        let vars = MockVars {
            ..MockVars::default()
        };

        assert_eq!(
            Options::deduce(&options, &vars, true),
            Ok(Options {
                classify: Classify::JustFilenames,
                show_icons: ShowIcons::Never,
                quote_style: QuoteStyle::QuoteSpaces,
                embed_hyperlinks: EmbedHyperlinks::Off,
                absolute: Absolute::Off,
                is_a_tty: true,
            })
        );
    }
}
