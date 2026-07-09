use clap::ArgMatches;

// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use crate::options::parser::ShowWhen;
use crate::options::{vars, Vars};
use crate::output::color_scale::ColorScaleOptions;
use crate::theme::{Definitions, Options, UseColours};

use super::config::{config_dir_from_env, ThemeConfig};

impl Options {
    pub fn deduce<V: Vars>(matches: &ArgMatches, vars: &V) -> Self {
        let use_colours = UseColours::deduce(matches, vars);
        let colour_scale = ColorScaleOptions::deduce(matches, vars);
        let theme_config = ThemeConfig::deduce(vars);

        let definitions = if use_colours == UseColours::Never {
            Definitions::default()
        } else {
            Definitions::deduce(vars)
        };

        Self {
            use_colours,
            colour_scale,
            definitions,
            theme_config,
        }
    }
}

impl ThemeConfig {
    fn deduce<V: Vars>(vars: &V) -> Option<Self> {
        let path = config_dir_from_env(
            vars.get(vars::EZA_CONFIG_DIR),
            vars.get(vars::XDG_CONFIG_HOME),
        );

        let theme = path.join("theme.yml");
        if theme.exists() {
            return Some(ThemeConfig::from_path(theme));
        }
        let theme = path.join("theme.yaml");
        if theme.exists() {
            return Some(ThemeConfig::from_path(theme));
        }
        None
    }
}

impl UseColours {
    fn deduce<V: Vars>(matches: &ArgMatches, vars: &V) -> Self {
        let default_value = match vars.get(vars::NO_COLOR) {
            Some(_) => Self::Never,
            None => Self::Automatic,
        };

        match matches.get_one("color").unwrap() {
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
    use crate::options::{parser::test::mock_cli, vars::test::MockVars};
    use std::ffi::OsString;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("eza-{name}-{nanos}"));
        fs::create_dir_all(&path).unwrap();
        path
    }

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
    fn deduce_theme_from_eza_config_dir() {
        let config_dir = temp_dir("eza-config-dir");
        let theme = config_dir.join("theme.yml");
        fs::write(&theme, "filekinds:\n  normal: { foreground: red }\n").unwrap();

        let mut vars = MockVars {
            ..MockVars::default()
        };
        vars.set(vars::EZA_CONFIG_DIR, &config_dir.as_os_str().to_os_string());

        assert_eq!(ThemeConfig::deduce(&vars), Some(ThemeConfig::from_path(theme)));
    }

    #[test]
    fn deduce_theme_from_xdg_config_home() {
        let xdg_config_home = temp_dir("xdg-config-home");
        let config_dir = xdg_config_home.join("eza");
        fs::create_dir_all(&config_dir).unwrap();
        let theme = config_dir.join("theme.yml");
        fs::write(&theme, "filekinds:\n  normal: { foreground: red }\n").unwrap();

        let mut vars = MockVars {
            ..MockVars::default()
        };
        vars.set(
            vars::XDG_CONFIG_HOME,
            &xdg_config_home.as_os_str().to_os_string(),
        );

        assert_eq!(ThemeConfig::deduce(&vars), Some(ThemeConfig::from_path(theme)));
    }

    #[test]
    fn eza_config_dir_takes_precedence_over_xdg_config_home() {
        let eza_config_dir = temp_dir("eza-config-dir-priority");
        let eza_theme = eza_config_dir.join("theme.yml");
        fs::write(&eza_theme, "filekinds:\n  normal: { foreground: red }\n").unwrap();

        let xdg_config_home = temp_dir("xdg-config-home-priority");
        let xdg_config_dir = xdg_config_home.join("eza");
        fs::create_dir_all(&xdg_config_dir).unwrap();
        fs::write(
            xdg_config_dir.join("theme.yml"),
            "filekinds:\n  normal: { foreground: blue }\n",
        )
        .unwrap();

        let mut vars = MockVars {
            ..MockVars::default()
        };
        vars.set(
            vars::EZA_CONFIG_DIR,
            &eza_config_dir.as_os_str().to_os_string(),
        );
        vars.set(
            vars::XDG_CONFIG_HOME,
            &xdg_config_home.as_os_str().to_os_string(),
        );

        assert_eq!(
            ThemeConfig::deduce(&vars),
            Some(ThemeConfig::from_path(eza_theme))
        );
    }

    #[test]
    fn deduce_use_colors_no_color_env() {
        let vars = MockVars {
            no_colors: OsString::from("1"),
            ..MockVars::default()
        };

        assert_eq!(
            UseColours::deduce(&mock_cli(vec![""]), &vars),
            UseColours::Never
        );
    }

    #[test]
    fn deduce_use_colors_no_color_arg() {
        let vars = MockVars {
            ..MockVars::default()
        };

        assert_eq!(
            UseColours::deduce(&mock_cli(vec!["--color", "never"]), &vars),
            UseColours::Never
        );
    }

    #[test]
    fn deduce_use_colors_always() {
        let vars = MockVars {
            ..MockVars::default()
        };

        assert_eq!(
            UseColours::deduce(&mock_cli(vec!["--color", "always"]), &vars),
            UseColours::Always
        );
    }

    #[test]
    fn deduce_use_colors_auto() {
        let vars = MockVars {
            ..MockVars::default()
        };

        assert_eq!(
            UseColours::deduce(&mock_cli(vec!["--color", "auto"]), &vars),
            UseColours::Automatic
        );
    }
}
