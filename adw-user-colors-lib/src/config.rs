// SPDX-License-Identifier: MPL-2.0-only

use crate::{colors::ColorOverrides, NAME, THEME_DIR};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{prelude::*, BufReader},
    path::PathBuf,
};

/// Cosmic Theme config
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub enum Config {
    DarkLight {
        /// whether high contrast mode is activated
        is_high_contrast: bool,
        /// active
        is_dark: bool,
        /// Selected light theme name
        light: String,
        /// Selected dark theme name
        dark: String,
    },
    Static {
        name: String,
        apply_all: bool,
    },
}

impl Default for Config {
    fn default() -> Self {
        // TODO load gsettings to determine if dark light or high contrast?
        Self::DarkLight {
            is_dark: true,
            light: Default::default(),
            dark: Default::default(),
            is_high_contrast: Default::default(),
        }
    }
}

pub const CONFIG_NAME: &'static str = "config";

impl Config {
    /// create a new cosmic theme config
    pub fn new_dark_light(is_dark: bool, high_contrast: bool, light: String, dark: String) -> Self {
        Self::DarkLight {
            is_dark,
            light,
            dark,
            is_high_contrast: high_contrast,
        }
    }

    /// create a new cosmic theme config
    pub fn new_static(name: String, apply_all: bool) -> Self {
        Self::Static { name, apply_all }
    }

    /// save the cosmic theme config
    pub fn save(&self) -> Result<()> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix(NAME)?;
        if let Ok(path) = xdg_dirs.place_config_file(PathBuf::from(format!("{CONFIG_NAME}.ron"))) {
            let mut f = File::create(path)?;
            let ron = ron::ser::to_string_pretty(&self, Default::default())?;
            f.write_all(ron.as_bytes())?;
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
        let path = xdg_dirs.find_config_file(PathBuf::from(format!("{CONFIG_NAME}.ron")));
        if path.is_none() {
            let s = Self::default();
            s.save()?;
        }
        if let Some(path) = xdg_dirs.find_config_file(PathBuf::from(format!("{CONFIG_NAME}.ron"))) {
            let mut f = File::open(&path)?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(ron::from_str(s.as_str())?)
        } else {
            anyhow::bail!("Failed to load config")
        }
    }

    #[cfg(feature = "gtk4")]
    /// applies the active config to to xdg-config-dir/gtk-4.0/cosmic.css
    pub fn apply_gtk4(&self) -> anyhow::Result<()> {
        let active = match self.active_name() {
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
        let colors = match &self {
            Config::DarkLight {
                is_high_contrast: high_contrast,
                ..
            } if *high_contrast => colors.to_high_contrast(),
            _ => colors,
        };

        let user_color_css = &mut colors.as_gtk_css().to_string();
        let xdg_dirs = xdg::BaseDirectories::with_prefix("gtk-4.0")?;
        let path = xdg_dirs.place_config_file(PathBuf::from("cosmic.css"))?;
        // write out css
        let _ = std::fs::write(&path, &user_color_css)?;

        let import = "@import url(\"cosmic.css\");";

        match self {
            Config::Static { apply_all, .. } if *apply_all => {
                // import if necessary
                if let Some(f) = xdg_dirs.find_config_file(PathBuf::from("gtk.css")) {
                    // let gtk_css_import = &format!("\n{import}\n");
                    let import_missing = {
                        let file = File::open(&f)?;
                        let reader = BufReader::new(file);
                        reader
                            .lines()
                            .find(|l| {
                                l.as_ref()
                                    .ok()
                                    .and_then(|l| if l.contains(import) { Some(()) } else { None })
                                    .is_some()
                            })
                            .is_none()
                    };
                    if import_missing {
                        let mut file = OpenOptions::new().write(true).append(true).open(f)?;

                        writeln!(file, "\n{import}")?;
                    }
                } else if let Ok(f) = xdg_dirs.place_config_file(PathBuf::from("gtk.css")) {
                    let mut file = OpenOptions::new().write(true).append(true).open(f)?;

                    writeln!(file, "\n{import}")?;
                }

                Ok(())
            }
            _ => Config::unimport(),
        }
    }

    pub fn unimport() -> anyhow::Result<()> {
        let import = "@import url(\"cosmic.css\");";
        let xdg_dirs = xdg::BaseDirectories::with_prefix("gtk-4.0")?;

        if let Some(f) = xdg_dirs.find_config_file(PathBuf::from("gtk.css")) {
            // let gtk_css_import = &format!("\n{import}\n");
            let mut changed = false;
            let new_contents: Vec<String> = {
                let file = File::open(&f)?;
                let reader = BufReader::new(file);
                reader
                    .lines()
                    .filter_map(|l| {
                        l.ok().and_then(|mut l| {
                            if l == import {
                                changed = true;
                                None
                            } else if let Some(start_index) = l.find(import) {
                                changed = true;
                                l.replace_range(start_index..start_index + import.len(), "");
                                Some(l)
                            } else {
                                Some(l)
                            }
                        })
                    })
                    .collect()
            };
            if changed {
                let mut file = OpenOptions::new().write(true).open(f)?;
                let new_contents = new_contents.join("\n");
                write!(file, "{new_contents}")?;
                return Ok(());
            }
        }
        Ok(())
    }

    /// get the name of the active theme
    pub fn active_name(&self) -> Option<String> {
        match self {
            Config::DarkLight {
                light,
                dark,
                is_dark,
                ..
            } => {
                if *is_dark && !dark.is_empty() {
                    Some(dark.clone())
                } else if !is_dark && !dark.is_empty() {
                    Some(light.clone())
                } else {
                    None
                }
                // if *high_contrast {
                //     if let Some(palette) = palette.take() {
                //         // TODO enforce high contrast constraints
                //         *palette = palette.to_high_contrast();
                //         todo!()
                //     }
                // }
            }
            Config::Static { name, .. } => Some(name.clone()),
        }
    }

    pub fn get_active(&self) -> anyhow::Result<ColorOverrides> {
        let active = match self.active_name() {
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
        Ok(colors)
    }

    pub fn set_active_light(new: &str) -> Result<()> {
        let mut self_ = Self::load()?;
        match self_ {
            Config::DarkLight { ref mut light, .. } => {
                *light = new.to_string();
            }
            Config::Static { ref mut name, .. } => {
                *name = new.to_string();
            }
        };
        Ok(self_.save()?)
    }

    pub fn set_active_dark(new: &str) -> Result<()> {
        let mut self_ = Self::load()?;
        match self_ {
            Config::DarkLight { ref mut dark, .. } => {
                *dark = new.to_string();
            }
            Config::Static { ref mut name, .. } => {
                *name = new.to_string();
            }
        };
        Ok(self_.save()?)
    }
}

impl From<(ColorOverrides, ColorOverrides)> for Config {
    fn from((light, dark): (ColorOverrides, ColorOverrides)) -> Self {
        Self::DarkLight {
            is_dark: true,
            is_high_contrast: false,
            light: light.name,
            dark: dark.name,
        }
    }
}

impl From<ColorOverrides> for Config {
    fn from(t: ColorOverrides) -> Self {
        Self::Static {
            name: t.name,
            apply_all: false,
        }
    }
}
