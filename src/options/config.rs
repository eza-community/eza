use crate::theme::UiStyles;
use serde_yaml;
use std::path::PathBuf;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct ThemeConfig {
    // This is rather bare for now, will be expanded with config file
    pub location: ConfigLoc,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum ConfigLoc {
    #[default]
    Default, // $XDG_CONFIG_HOME/eza/config|theme.yml
    Env(PathBuf), // $EZA_CONFIG_DIR
}

impl ThemeConfig {
    pub fn to_theme(&self) -> Option<UiStyles> {
        match &self.location {
            ConfigLoc::Default => {
                let path = dirs::config_dir()?.join("eza").join("theme.yml");
                let file = std::fs::File::open(path).ok()?;
                serde_yaml::from_reader(&file).ok()
            }
            ConfigLoc::Env(path) => {
                let file = std::fs::File::open(path).ok()?;
                serde_yaml::from_reader(&file).ok()
            }
        }
    }
}
