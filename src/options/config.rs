use crate::options::{MatchedFlags, Vars};
use crate::output::color_scale::ColorScaleOptions;
use crate::theme::UiStyles;
use dirs;
use serde_yaml;
use std::{ffi::OsStr, path::PathBuf};

use super::{flags, OptionsError};

#[derive(Debug, Default, Eq, PartialEq)]
pub struct ThemeConfig {
    pub location: ConfigLoc,
    pub theme: UiStyles,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum ConfigLoc {
    #[default]
    Default, // $XDG_CONFIG_HOME/eza/config|theme.yml
    Env(PathBuf), // $EZA_CONFIG_DIR
    Arg(PathBuf), // --config path/to/config|theme.yml
}

impl ThemeConfig {
    pub fn write_default_theme_file(path: Option<&OsStr>) -> std::io::Result<()> {
        if path.is_some_and(|path| std::path::Path::new(path).is_dir()) {
            let path = std::path::Path::new(path.unwrap());
            let path = path.join("theme.yml");
            let file = std::fs::File::create(path.clone())?;
            println!("Writing default theme to {:?}", path);
            serde_yaml::to_writer(file, &UiStyles::default())
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        } else {
            let default_path = std::env::var("EZA_CONFIG_DIR")
                .map(|dir| PathBuf::from(&dir))
                .unwrap_or(dirs::config_dir().unwrap_or_default().join("eza"));
            if !default_path.exists() {
                std::fs::create_dir_all(&default_path)?;
            }
            println!("Writing default theme to {:?}", default_path);
            let default_file = default_path.join("theme.yml");
            let file = std::fs::File::create(default_file)?;
            let default = UiStyles::default();
            serde_yaml::to_writer(file, &default)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        }
    }

    pub fn theme_from_yaml(file: Option<&str>) -> UiStyles {
        if let Some(file) = file {
            let file = std::fs::File::open(file);
            if let Err(e) = file {
                eprintln!("Could not open theme file: {e}");
                return UiStyles::default();
            }
            let file = file.expect("Could not open theme file");
            let theme: UiStyles = serde_yaml::from_reader(file).unwrap_or_else(|e| {
                eprintln!("Could not parse theme file: {e}");
                UiStyles::default()
            });
            theme
        } else {
            UiStyles::default()
        }
    }
    pub fn deduce<V: Vars>(
        matches: &MatchedFlags<'_>,
        vars: &V,
        opts: ColorScaleOptions,
    ) -> Result<ThemeConfig, crate::options::OptionsError> {
        println!("Deducing theme");
        if matches.has(&flags::WRITE_THEME)? {
            let path = matches.get(&flags::WRITE_THEME)?;
            println!("Writing default theme to {:?}", path);
            let err = Self::write_default_theme_file(path).map_err(|e| e.to_string());
            if let Err(err) = err {
                return Err(OptionsError::WriteTheme(err));
            }
        }
        let theme_file = if matches.has(&flags::THEME)? {
            let path = matches.get(&flags::THEME)?;
            // passing --config will require a value as we will check default location
            if path.is_none() {
                return Err(OptionsError::BadArgument(&flags::THEME, "no value".into()));
            }
            path.map(|p| p.to_string_lossy().to_string())
        } else {
            None
        };
        Ok(Self::find_with_fallback(theme_file, vars, opts))
    }

    pub fn find_with_fallback<V: Vars>(
        path: Option<String>,
        vars: &V,
        opts: ColorScaleOptions,
    ) -> Self {
        if let Some(path) = path {
            let path = std::path::PathBuf::from(path);
            if path.is_dir() && path.exists() {
                let path = path
                    .join("theme.yml")
                    .exists()
                    .then(|| path.join("theme.yml"));
                match path {
                    Some(path) => {
                        let file = std::fs::read_to_string(&path).unwrap_or_default();
                        let uistyles: Option<UiStyles> = serde_yaml::from_str(&file).ok();
                        return Self {
                            location: ConfigLoc::Arg(path),
                            theme: uistyles.unwrap_or(UiStyles::default_theme(opts)),
                        };
                    }
                    None => return Self::default(),
                }
            }
        } else if vars.get("EZA_CONFIG_DIR").is_some() {
            let path = std::path::PathBuf::from(&format!(
                "{}/theme.yml",
                vars.get("EZA_CONFIG_DIR").unwrap().to_string_lossy()
            ));
            if path.exists() {
                let file = std::fs::read_to_string(&path).unwrap_or_default();
                let uistyles: Option<UiStyles> = serde_yaml::from_str(&file).ok();
                return Self {
                    location: ConfigLoc::Env(path),
                    theme: uistyles.unwrap_or(UiStyles::default_theme(opts)),
                };
            }
            return Self::default();
        };
        Self::default()
    }
}
