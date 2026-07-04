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
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use super::config::ThemeConfig;

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
        if let Some(path) = vars.get(vars::EZA_CONFIG_DIR) {
            let home = vars
                .get(vars::HOME)
                .filter(|home| !home.is_empty())
                .map(PathBuf::from)
                .or_else(dirs::home_dir);
            let path = expand_home_path(&path, home.as_deref());
            let theme = path.join("theme.yml");
            if theme.exists() {
                return Some(ThemeConfig::from_path(theme));
            }
            let theme = path.join("theme.yaml");
            if theme.exists() {
                return Some(ThemeConfig::from_path(theme));
            }
            None
        } else {
            let path = dirs::config_dir().unwrap_or_default();
            let path = path.join("eza");
            let theme = path.join("theme.yml");
            if theme.exists() {
                return Some(ThemeConfig::default());
            }
            let theme = path.join("theme.yaml");
            if theme.exists() {
                return Some(ThemeConfig::from_path(theme));
            }
            None
        }
    }
}

fn expand_home_path(path: &OsStr, home: Option<&Path>) -> PathBuf {
    let Some(home) = home else {
        return PathBuf::from(path);
    };
    let Some(path) = path.to_str() else {
        return PathBuf::from(path);
    };

    match path {
        "~" | "$HOME" | "${HOME}" => home.to_path_buf(),
        _ => path
            .strip_prefix("~/")
            .or_else(|| path.strip_prefix("~\\"))
            .or_else(|| path.strip_prefix("$HOME/"))
            .or_else(|| path.strip_prefix("$HOME\\"))
            .or_else(|| path.strip_prefix("${HOME}/"))
            .or_else(|| path.strip_prefix("${HOME}\\"))
            .map_or_else(|| PathBuf::from(path), |rest| home.join(rest)),
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

    struct ConfigVars<'a> {
        config_dir: &'a str,
        home: &'a Path,
    }

    impl Vars for ConfigVars<'_> {
        fn get(&self, name: &'static str) -> Option<OsString> {
            match name {
                "EZA_CONFIG_DIR" => Some(OsString::from(self.config_dir)),
                "HOME" => Some(self.home.as_os_str().to_os_string()),
                _ => None,
            }
        }
    }

    struct TestHome(PathBuf);

    impl TestHome {
        fn new(name: &str) -> Self {
            let path = std::env::temp_dir().join(format!("eza-theme-test-{name}-{}", std::process::id()));
            let _ = fs::remove_dir_all(&path);
            fs::create_dir_all(&path).expect("test home should be created");

            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }

        fn config_dir(&self) -> PathBuf {
            self.0.join(".config").join("eza")
        }

        fn write_theme(&self) -> PathBuf {
            let config_dir = self.config_dir();
            fs::create_dir_all(&config_dir).expect("test config dir should be created");
            let theme = config_dir.join("theme.yml");
            fs::write(&theme, "").expect("test theme should be written");
            theme
        }
    }

    impl Drop for TestHome {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn assert_theme_config_expands(config_dir: &str, name: &str) {
        let home = TestHome::new(name);
        let theme = home.write_theme();
        let vars = ConfigVars {
            config_dir,
            home: home.path(),
        };

        assert_eq!(
            ThemeConfig::deduce(&vars),
            Some(ThemeConfig::from_path(theme))
        );
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
    fn deduce_theme_config_expands_home_variable_in_eza_config_dir() {
        assert_theme_config_expands("$HOME/.config/eza", "home-variable");
    }

    #[test]
    fn deduce_theme_config_expands_braced_home_variable_in_eza_config_dir() {
        assert_theme_config_expands("${HOME}/.config/eza", "braced-home-variable");
    }

    #[test]
    fn deduce_theme_config_expands_tilde_in_eza_config_dir() {
        assert_theme_config_expands("~/.config/eza", "tilde");
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
