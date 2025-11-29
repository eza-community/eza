// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use crate::options::parser::ShowWhen;
use crate::options::vars::{self, Vars};
use crate::options::{NumberSource, OptionsError};

use crate::output::file_name::{Classify, EmbedHyperlinks, Options, QuoteStyle, ShowIcons};

use clap::ArgMatches;

impl Options {
    pub fn deduce<V: Vars>(
        matches: &ArgMatches,
        vars: &V,
        is_a_tty: bool,
    ) -> Result<Self, OptionsError> {
        let classify = Classify::deduce(matches);
        let show_icons = ShowIcons::deduce(matches, vars)?;

        let quote_style = QuoteStyle::deduce(matches);
        let embed_hyperlinks = EmbedHyperlinks::deduce(matches);

        let absolute = *matches.get_one("absolute").unwrap();

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
    fn deduce(matches: &ArgMatches) -> Self {
        match matches.get_one("classify") {
            Some(ShowWhen::Auto) => Self::AutomaticAddFileIndicators,
            Some(ShowWhen::Always) => Self::AddFileIndicators,
            None | Some(ShowWhen::Never) => Self::JustFilenames,
        }
    }
}

impl ShowIcons {
    pub fn deduce<V: Vars>(matches: &ArgMatches, vars: &V) -> Result<Self, OptionsError> {
        let force_icons = vars.get(vars::EZA_ICONS_AUTO).is_some();
        let mode_opt = &matches.get_one("icons");
        if !force_icons && mode_opt.is_none() {
            return Ok(Self::Never);
        }

        match mode_opt {
            Some(ShowWhen::Never) => Ok(Self::Never),
            Some(ShowWhen::Always) => Ok(Self::Always(Self::get_width(vars)?)),
            Some(ShowWhen::Auto) | None => Ok(Self::Automatic(Self::get_width(vars)?)),
        }
    }

    fn get_width<V: Vars>(vars: &V) -> Result<u32, OptionsError> {
        if let Some(columns) = vars
            .get_with_fallback(vars::EXA_ICON_SPACING, vars::EZA_ICON_SPACING)
            .map(|s| s.to_string_lossy().to_string())
        {
            match columns.parse() {
                Ok(width) => Ok(width),
                Err(e) => {
                    let source = NumberSource::Env(
                        vars.source(vars::EXA_ICON_SPACING, vars::EZA_ICON_SPACING)
                            .unwrap_or("1"),
                    );
                    Err(OptionsError::FailedParse(columns.to_string(), source, e))
                }
            }
        } else {
            Ok(1)
        }
    }
}

impl QuoteStyle {
    pub fn deduce(matches: &ArgMatches) -> Self {
        if matches.get_flag("no-quotes") {
            Self::NoQuotes
        } else {
            Self::QuoteSpaces
        }
    }
}

impl EmbedHyperlinks {
    fn deduce(matches: &ArgMatches) -> Self {
        if matches.get_flag("hyperlink") {
            Self::On
        } else {
            Self::Off
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;
    use std::num::ParseIntError;

    use super::*;
    use crate::options::parser::test::mock_cli;
    use crate::options::parser::ShowWhen;
    use crate::options::vars::test::MockVars;
    use crate::output::file_name::Absolute;

    use clap::ValueEnum;

    #[test]
    fn deduce_classify_file_indicators() {
        assert_eq!(
            Classify::deduce(&mock_cli(vec!["--classify"])),
            Classify::AutomaticAddFileIndicators
        );
    }

    #[test]
    fn deduce_classify_just_filenames() {
        assert_eq!(
            Classify::deduce(&mock_cli(vec![""])),
            Classify::JustFilenames
        );
    }

    #[test]
    fn deduce_quote_style_no_quotes() {
        assert_eq!(
            QuoteStyle::deduce(&mock_cli(vec!["--no-quotes"])),
            QuoteStyle::NoQuotes
        );
    }

    #[test]
    fn deduce_quote_style_quote_spaces() {
        assert_eq!(
            QuoteStyle::deduce(&mock_cli(vec![""])),
            QuoteStyle::QuoteSpaces
        );
    }

    #[test]
    fn deduce_embed_hyperlinks_on() {
        assert_eq!(
            EmbedHyperlinks::deduce(&mock_cli(vec!["--hyperlink"])),
            EmbedHyperlinks::On
        );
    }

    #[test]
    fn deduce_embed_hyperlinks_off() {
        assert_eq!(
            EmbedHyperlinks::deduce(&mock_cli(vec![""])),
            EmbedHyperlinks::Off
        );
    }

    #[test]
    fn deduce_show_icons_never_no_arg() {
        assert_eq!(
            ShowIcons::deduce(&mock_cli(vec![""]), &MockVars::default()),
            Ok(ShowIcons::Never)
        );
    }

    #[test]
    fn deduce_show_icons_never_no_arg_env() {
        let mut vars = MockVars::default();
        vars.set(vars::EZA_ICONS_AUTO, &OsString::from("1"));
        assert_eq!(
            ShowIcons::deduce(&mock_cli(vec![""]), &vars),
            Ok(ShowIcons::Automatic(1))
        );
    }

    #[test]
    fn deduce_show_icon_always() {
        assert_eq!(
            ShowIcons::deduce(&mock_cli(vec!["--icons", "always"]), &MockVars::default()),
            Ok(ShowIcons::Always(1)),
        );
    }

    #[test]
    fn deduce_show_icons_never() {
        assert_eq!(
            ShowIcons::deduce(&mock_cli(vec!["--icons", "never"]), &MockVars::default()),
            Ok(ShowIcons::Never)
        );
    }

    #[test]
    fn deduce_show_icons_auto() {
        assert_eq!(
            ShowIcons::deduce(&mock_cli(vec!["--icons", "auto"]), &MockVars::default()),
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
        let mut vars = MockVars::default();
        vars.set(vars::EZA_ICON_SPACING, &OsString::from("3"));
        assert_eq!(
            ShowIcons::deduce(&mock_cli(vec!["--icons"]), &vars),
            Ok(ShowIcons::Automatic(3))
        );
    }

    #[test]
    fn deduce_show_icons_width_error() {
        let mut vars = MockVars::default();
        vars.set(vars::EZA_ICON_SPACING, &OsString::from("foo"));

        let e: Result<i64, ParseIntError> = vars
            .get(vars::EZA_ICON_SPACING)
            .unwrap()
            .to_string_lossy()
            .parse();

        assert_eq!(
            ShowIcons::deduce(&mock_cli(vec!["--icons", "auto"]), &vars),
            Err(OptionsError::FailedParse(
                String::from("foo"),
                NumberSource::Env(vars::EXA_ICON_SPACING),
                e.unwrap_err()
            ))
        );
    }

    #[test]
    fn deduce_options() {
        assert_eq!(
            Options::deduce(&mock_cli(vec![""]), &MockVars::default(), true),
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
