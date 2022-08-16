// SPDX-License-Identifier: MPL-2.0-only

use crate::{colors::ColorOverrides, NAME, THEME_DIR};
use adw::StyleManager;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::PathBuf,
};

/// Cosmic Theme config
#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Config {
    /// Selected light theme name
    pub light: String,
    /// Selected dark theme name
    pub dark: String,
}

pub const CONFIG_NAME: &'static str = "config";

impl Config {
    /// create a new cosmic theme config
    pub fn new(light: String, dark: String) -> Self {
        Self { light, dark }
    }

    /// save the cosmic theme config
    pub fn save(&self) -> Result<()> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix(NAME)?;
        if let Ok(path) = xdg_dirs.place_config_file(PathBuf::from(format!("{CONFIG_NAME}.toml"))) {
            let mut f = File::create(path)?;
            let toml = toml::ser::to_string_pretty(&self)?;
            f.write_all(toml.as_bytes())?;
            Ok(())
        } else {
            bail!("failed to save theme config")
        }
    }

    pub fn init() -> anyhow::Result<PathBuf> {
        let base_dirs = xdg::BaseDirectories::new()?;
        Ok(base_dirs.create_config_directory(NAME)?)
    }

    /// load the cosmic theme config
    pub fn load() -> Result<Self> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix(NAME)?;
        let path = xdg_dirs.get_config_home();
        std::fs::create_dir_all(&path)?;
        let path = xdg_dirs.find_config_file(PathBuf::from(format!("{CONFIG_NAME}.toml")));
        if path.is_none() {
            let s = Self::default();
            s.save()?;
        }
        if let Some(path) = xdg_dirs.find_config_file(PathBuf::from(format!("{CONFIG_NAME}.toml")))
        {
            let mut f = File::open(&path)?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(toml::from_str(s.as_str())?)
        } else {
            anyhow::bail!("Failed to load config")
        }
    }

    pub fn apply(&self, style_manager: Option<&StyleManager>) -> anyhow::Result<()> {
        let active = match self.active_name(style_manager) {
            Some(n) => n,
            _ => anyhow::bail!("No configured active overrides"),
        };
        let css_path: PathBuf = [NAME, THEME_DIR].iter().collect();
        let css_dirs = xdg::BaseDirectories::with_prefix(css_path)?;
        let active_theme_path = match css_dirs.find_data_file(format!("{active}.ron")) {
            Some(p) => p,
            _ => anyhow::bail!("Could not find theme"),
        };
        let active_theme_file = File::open(active_theme_path)?;
        let reader = BufReader::new(active_theme_file);
        let colors = ron::de::from_reader::<_, ColorOverrides>(reader)?;
        let user_color_css = &mut colors.as_css().to_string();
        user_color_css.push_str(&format!("\n@import url(\"custom.css\");\n"));
        let xdg_dirs = xdg::BaseDirectories::with_prefix("gtk-4.0")?;
        let path = xdg_dirs.place_config_file(PathBuf::from("gtk.css"))?;
        let _ = std::fs::write(&path, &user_color_css)?;
        Ok(())
    }

    /// get the name of the active theme
    pub fn active_name(&self, style_manager: Option<&StyleManager>) -> Option<String> {
        if !adw::is_initialized() {
            None
        } else {
            let is_dark = style_manager.map(|sm| sm.is_dark()).unwrap_or_else(|| {
                let manager = StyleManager::default();
                manager.is_dark()
            });
            if is_dark {
                Some(self.dark.clone())
            } else {
                Some(self.light.clone())
            }
        }
    }

    pub fn set_active_light(new: &str) -> Result<()> {
        let mut self_ = Self::load()?;
        self_.light = new.to_string();
        Ok(self_.save()?)
    }

    pub fn set_active_dark(new: &str) -> Result<()> {
        let mut self_ = Self::load()?;
        self_.dark = new.to_string();
        Ok(self_.save()?)
    }
}

impl From<(ColorOverrides, ColorOverrides)> for Config {
    fn from((light, dark): (ColorOverrides, ColorOverrides)) -> Self {
        Self {
            light: light.name,
            dark: dark.name,
        }
    }
}

impl From<ColorOverrides> for Config {
    fn from(t: ColorOverrides) -> Self {
        Self {
            light: t.clone().name,
            dark: t.name,
        }
    }
}
