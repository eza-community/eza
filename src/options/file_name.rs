use crate::options::vars::{self, Vars};
use crate::options::{NumberSource, OptionsError};

use crate::options::parser::Opts;
use crate::output::file_name::{Classify, EmbedHyperlinks, Options, ShowIcons};

impl Options {
    pub fn deduce<V: Vars>(matches: &Opts, vars: &V) -> Result<Self, OptionsError> {
        let classify = Classify::deduce(matches);
        let show_icons = ShowIcons::deduce(matches, vars)?;
        let embed_hyperlinks = EmbedHyperlinks::deduce(matches);

        Ok(Self {
            classify,
            show_icons,
            embed_hyperlinks,
        })
    }
}

impl Classify {
    fn deduce(matches: &Opts) -> Self {
        if matches.classify > 0 {
            return Self::AddFileIndicators;
        }
        Self::JustFilenames
    }
}

impl ShowIcons {
    pub fn deduce<V: Vars>(matches: &Opts, vars: &V) -> Result<Self, OptionsError> {
        if matches.no_icons > 0 || matches.icons == 0 {
            Ok(Self::Off)
        } else if let Some(columns) = vars
            .get_with_fallback(vars::EZA_ICON_SPACING, vars::EXA_ICON_SPACING)
            .and_then(|s| s.into_string().ok())
        {
            match columns.parse() {
                Ok(width) => Ok(Self::On(width)),
                Err(e) => {
                    let source = NumberSource::Var(vars::EXA_ICON_SPACING.to_string());
                    Err(OptionsError::FailedParse(columns, source, e))
                }
            }
        } else {
            Ok(Self::On(1))
        }
    }
}

impl EmbedHyperlinks {
    fn deduce(matches: &Opts) -> Self {
        if matches.hyperlink > 0 {
            return Self::On;
        }
        Self::Off
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deduce_hyperlinks() {
        assert_eq!(
            EmbedHyperlinks::deduce(&Opts::default()),
            EmbedHyperlinks::Off
        );
    }

    #[test]
    fn deduce_hyperlinks_on() {
        let matches = Opts {
            hyperlink: 1,
            ..Opts::default()
        };

        assert_eq!(EmbedHyperlinks::deduce(&matches), EmbedHyperlinks::On);
    }

    #[test]
    fn deduce_classify() {
        let matches = Opts {
            classify: 1,
            ..Opts::default()
        };

        assert_eq!(Classify::deduce(&matches), Classify::AddFileIndicators);
    }

    #[test]
    fn deduce_classify_no_classify() {
        let matches = Opts { ..Opts::default() };

        assert_eq!(Classify::deduce(&matches), Classify::JustFilenames);
    }
}
