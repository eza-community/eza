use crate::options::parser::{Opts, ShowWhen};
use crate::options::{vars, OptionsError, Vars};
use crate::output::color_scale::ColorScaleOptions;
use crate::theme::{Definitions, Options, UseColours};
use std::path::PathBuf;

use super::config::ThemeConfig;

impl Options {
    pub fn deduce<V: Vars>(matches: &Opts, vars: &V) -> Result<Self, OptionsError> {
        let use_colours = UseColours::deduce(matches, vars);
        let colour_scale = ColorScaleOptions::deduce(matches, vars)?;
        let theme_config = ThemeConfig::deduce(vars);

        let definitions = if use_colours == UseColours::Never {
            Definitions::default()
        } else {
            Definitions::deduce(vars)
        };

        Ok(Self {
            use_colours,
            colour_scale,
            definitions,
            theme_config,
        })
    }
}

impl ThemeConfig {
    fn deduce<V: Vars>(vars: &V) -> Option<Self> {
        if let Some(path) = vars.get("EZA_CONFIG_DIR") {
            let path = PathBuf::from(path);
            let path = path.join("theme.yml");
            if path.exists() {
                Some(ThemeConfig::from_path(&path.to_string_lossy()))
            } else {
                None
            }
        } else {
            let path = dirs::config_dir().unwrap_or_default();
            let path = path.join("eza").join("theme.yml");
            if path.exists() {
                Some(ThemeConfig::default())
            } else {
                None
            }
        }
    }
}

impl UseColours {
    fn deduce<V: Vars>(matches: &Opts, vars: &V) -> Self {
        let default_value = match vars.get(vars::NO_COLOR) {
            Some(_) => Self::Never,
            None => Self::Automatic,
        };

        match matches.color {
            ShowWhen::Auto => default_value,
            ShowWhen::Always => Self::Always,
            ShowWhen::Never => Self::Never,
        }
    }
}

impl Definitions {
    fn deduce<V: Vars>(vars: &V) -> Self {
        let ls = vars
            .get(vars::LS_COLORS)
            .map(|e| e.to_string_lossy().to_string());
        let exa = vars
            .get_with_fallback(vars::EZA_COLORS, vars::EXA_COLORS)
            .map(|e| e.to_string_lossy().to_string());
        Self { ls, exa }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::vars::MockVars;
    use std::ffi::OsString;

    #[test]
    fn deduce_definitions() {
        let vars = MockVars {
            ..MockVars::default()
        };

        assert_eq!(
            Definitions::deduce(&vars),
            Definitions {
                ls: None,
                exa: None,
            }
        );
    }

    #[test]
    fn deduce_definitions_colors() {
        let mut vars = MockVars {
            ..MockVars::default()
        };

        vars.set(vars::LS_COLORS, &OsString::from("uR=1;34"));

        assert_eq!(
            Definitions::deduce(&vars),
            Definitions {
                ls: Some("uR=1;34".to_string()),
                exa: Some("uR=1;34".to_string()),
            }
        );
    }

    #[test]
    fn deduce_use_colors_no_color_env() {
        let vars = MockVars {
            no_colors: OsString::from("1"),
            ..MockVars::default()
        };

        assert_eq!(
            UseColours::deduce(&Opts::default(), &vars),
            UseColours::Never
        );
    }

    #[test]
    fn deduce_use_colors_no_color_arg() {
        let vars = MockVars {
            ..MockVars::default()
        };

        assert_eq!(
            UseColours::deduce(
                &Opts {
                    color: ShowWhen::Never,
                    ..Opts::default()
                },
                &vars
            ),
            UseColours::Never
        );
    }

    #[test]
    fn deduce_use_colors_always() {
        let vars = MockVars {
            ..MockVars::default()
        };

        let options = Opts {
            color: ShowWhen::Always,
            ..Opts::default()
        };

        assert_eq!(UseColours::deduce(&options, &vars), UseColours::Always);
    }

    #[test]
    fn deduce_use_colors_auto() {
        let vars = MockVars {
            ..MockVars::default()
        };

        let options = Opts {
            color: ShowWhen::Auto,
            ..Opts::default()
        };

        assert_eq!(UseColours::deduce(&options, &vars), UseColours::Automatic);
    }
}
