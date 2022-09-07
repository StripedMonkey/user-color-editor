// SPDX-License-Identifier: MPL-2.0-only

use std::fmt::Write as _;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
}; // import without risk of name clashing

use serde::{Deserialize, Serialize};

use crate::{NAME, THEME_DIR};

#[derive(Debug, Default, Deserialize, Serialize, Clone, Hash, PartialEq, Eq)]
pub struct ColorOverrides {
    /// name
    pub name: String,
    pub accent_bg_color: Option<String>,
    pub accent_fg_color: Option<String>,
    pub accent_color: Option<String>,

    // destructive-action buttons
    pub destructive_bg_color: Option<String>,
    pub destructive_fg_color: Option<String>,
    pub destructive_color: Option<String>,

    // Levelbars, entries, labels and infobars. These don't need text colors
    pub success_color: Option<String>,
    pub success_bg_color: Option<String>,
    pub success_fg_color: Option<String>,

    pub warning_color: Option<String>,
    pub warning_bg_color: Option<String>,
    pub warning_fg_color: Option<String>,

    pub error_color: Option<String>,
    pub error_bg_color: Option<String>,
    pub error_fg_color: Option<String>,

    // Main window background
    pub window_bg_color: Option<String>,
    pub window_fg_color: Option<String>,

    // Content areas, e.g. text views
    pub view_bg_color: Option<String>,
    pub view_fg_color: Option<String>,

    // Header bar, search bar, tab bar
    pub headerbar_bg_color: Option<String>,
    pub headerbar_fg_color: Option<String>,
    pub headerbar_border_color: Option<String>,
    pub headerbar_backdrop_color: Option<String>,
    pub headerbar_shade_color: Option<String>,

    // Cards, boxed lists
    pub card_bg_color: Option<String>,
    pub card_fg_color: Option<String>,
    pub card_shade_color: Option<String>,

    // Popovers
    pub popover_bg_color: Option<String>,
    pub popover_fg_color: Option<String>,

    // Miscellaneous
    pub scrollbar_outline_color: Option<String>,
    pub shade_color: Option<String>,
}

impl ColorOverrides {
    pub fn save(&self) -> anyhow::Result<()> {
        let ron_path: PathBuf = [NAME, THEME_DIR].iter().collect();
        let ron_dirs = xdg::BaseDirectories::with_prefix(ron_path)?;
        let ron_name = format!("{}.ron", &self.name);

        if let Ok(p) = ron_dirs.place_data_file(ron_name) {
            let mut f = File::create(p)?;
            f.write_all(ron::ser::to_string(self)?.as_bytes())?;
        } else {
            anyhow::bail!("Failed to write RON theme.");
        }
        Ok(())
    }

    pub fn init() -> anyhow::Result<PathBuf> {
        let ron_path: PathBuf = [NAME, THEME_DIR].iter().collect();
        let base_dirs = xdg::BaseDirectories::new()?;
        Ok(base_dirs.create_data_directory(ron_path)?)
    }

    pub fn load_from_name(name: &str) -> anyhow::Result<Self> {
        let ron_path: PathBuf = [NAME, THEME_DIR].iter().collect();
        let ron_dirs = xdg::BaseDirectories::with_prefix(ron_path)?;

        let ron_name = format!("{}.ron", name);
        if let Some(p) = ron_dirs.find_data_file(ron_name) {
            let f = File::open(p)?;
            Ok(ron::de::from_reader(f)?)
        } else {
            anyhow::bail!("Failed to write RON theme.");
        }
    }

    pub fn load(p: &dyn AsRef<Path>) -> anyhow::Result<Self> {
        let f = File::open(p)?;
        Ok(ron::de::from_reader(f)?)
    }

    pub fn light_default() -> Self {
        ron::de::from_bytes(include_bytes!("light_default.ron")).unwrap()
    }

    pub fn dark_default() -> Self {
        ron::de::from_bytes(include_bytes!("dark_default.ron")).unwrap()
    }

    /// ensures that all colors in the palette meet high-contrast constraints
    pub fn to_high_contrast(self) -> Self {
        // TODO
        self
    }

    pub fn set_key(&mut self, key: &str, value: Option<String>) -> anyhow::Result<()> {
        match key {
            "accent_bg_color" => self.accent_bg_color = value,
            "accent_fg_color" => self.accent_fg_color = value,
            "accent_color" => self.accent_color = value,

            // destructive-action buttons
            "destructive_bg_color" => self.destructive_bg_color = value,
            "destructive_fg_color" => self.destructive_fg_color = value,
            "destructive_color" => self.destructive_color = value,

            "success_color" => self.success_color = value,
            "success_bg_color" => self.success_color = value,
            "success_fg_color" => self.success_color = value,

            "warning_color" => self.warning_color = value,
            "warning_bg_color" => self.warning_color = value,
            "warning_fg_color" => self.warning_color = value,

            "error_color" => self.error_color = value,
            "error_bg_color" => self.error_color = value,
            "error_fg_color" => self.error_color = value,

            // Content areas, e.g. text views
            "view_bg_color" => self.view_bg_color = value,
            "view_fg_color" => self.view_fg_color = value,

            // Main window background
            "window_bg_color" => self.window_bg_color = value,
            "window_fg_color" => self.window_fg_color = value,

            // Header bar, search bar, tab bar
            "headerbar_bg_color" => self.headerbar_bg_color = value,
            "headerbar_fg_color" => self.headerbar_fg_color = value,
            "headerbar_border_color" => self.headerbar_border_color = value,
            "headerbar_backdrop_color" => self.headerbar_backdrop_color = value,
            "headerbar_shade_color" => self.headerbar_shade_color = value,

            // Cards, boxed lists
            "card_bg_color" => self.card_bg_color = value,
            "card_fg_color" => self.card_fg_color = value,
            "card_shade_color" => self.card_shade_color = value,

            // Popovers
            "popover_bg_color" => self.popover_bg_color = value,
            "popover_fg_color" => self.popover_fg_color = value,

            // Miscellaneous
            "scrollbar_outline_color" => self.scrollbar_outline_color = value,
            "shade_color" => self.shade_color = value,
            _ => anyhow::bail!("Invalid key"),
        }
        Ok(())
    }

    pub fn get_key(&self, key: &str) -> Option<String> {
        match key {
            "accent_bg_color" => self.accent_bg_color.clone(),
            "accent_fg_color" => self.accent_fg_color.clone(),
            "accent_color" => self.accent_color.clone(),

            // destructive-action buttons
            "destructive_bg_color" => self.destructive_bg_color.clone(),
            "destructive_fg_color" => self.destructive_fg_color.clone(),
            "destructive_color" => self.destructive_color.clone(),

            "success_color" => self.success_color.clone(),
            "success_bg_color" => self.success_color.clone(),
            "success_fg_color" => self.success_color.clone(),

            "warning_color" => self.warning_color.clone(),
            "warning_bg_color" => self.warning_color.clone(),
            "warning_fg_color" => self.warning_color.clone(),

            "error_color" => self.error_color.clone(),
            "error_bg_color" => self.error_color.clone(),
            "error_fg_color" => self.error_color.clone(),

            // Content areas.clone(), e.g. text views
            "view_bg_color" => self.view_bg_color.clone(),
            "view_fg_color" => self.view_fg_color.clone(),

            // Main window background
            "window_bg_color" => self.window_bg_color.clone(),
            "window_fg_color" => self.window_fg_color.clone(),

            // Header bar.clone(), search bar.clone(), tab bar
            "headerbar_bg_color" => self.headerbar_bg_color.clone(),
            "headerbar_fg_color" => self.headerbar_fg_color.clone(),
            "headerbar_border_color" => self.headerbar_border_color.clone(),
            "headerbar_backdrop_color" => self.headerbar_backdrop_color.clone(),
            "headerbar_shade_color" => self.headerbar_shade_color.clone(),

            // Cards.clone(), boxed lists
            "card_bg_color" => self.card_bg_color.clone(),
            "card_fg_color" => self.card_fg_color.clone(),
            "card_shade_color" => self.card_shade_color.clone(),

            // Popovers
            "popover_bg_color" => self.popover_bg_color.clone(),
            "popover_fg_color" => self.popover_fg_color.clone(),

            // Miscellaneous
            "scrollbar_outline_color" => self.scrollbar_outline_color.clone(),
            "shade_color" => self.shade_color.clone(),
            _ => None,
        }
    }

    pub fn as_gtk_css(&self) -> String {
        let mut user_color_css = String::new();
        if let Some(accent_bg_color) = self.accent_bg_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color accent_bg_color {};",
                &accent_bg_color
            );
        }
        if let Some(accent_fg_color) = self.accent_fg_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color accent_fg_color {};",
                &accent_fg_color
            );
        }
        if let Some(accent_color) = self.accent_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color accent_color {};",
                &accent_color
            );
        }

        if let Some(destructive_bg_color) = self.destructive_bg_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color destructive_bg_color {};",
                &destructive_bg_color
            );
        }
        if let Some(destructive_fg_color) = self.destructive_fg_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color destructive_fg_color {};",
                &destructive_fg_color
            );
        }
        if let Some(destructive_color) = self.destructive_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color destructive_color {};",
                &destructive_color
            );
        }

        if let Some(success_color) = self.success_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color success_color {};",
                &success_color
            );
        }
        if let Some(success_bg_color) = self.success_bg_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color success_bg_color {};",
                &success_bg_color
            );
        }
        if let Some(success_fg_color) = self.success_fg_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color success_fg_color {};",
                &success_fg_color
            );
        }
        if let Some(warning_color) = self.warning_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color warning_color {};",
                &warning_color
            );
        }
        if let Some(warning_bg_color) = self.warning_bg_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color warning_bg_color {};",
                &warning_bg_color
            );
        }
        if let Some(warning_fg_color) = self.warning_fg_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color warning_fg_color {};",
                &warning_fg_color
            );
        }
        if let Some(error_color) = self.error_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color error_color {};",
                &error_color
            );
        }
        if let Some(error_bg_color) = self.error_bg_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color error_bg_color {};",
                &error_bg_color
            );
        }
        if let Some(error_fg_color) = self.error_fg_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color error_fg_color {};",
                &error_fg_color
            );
        }

        if let Some(window_bg_color) = self.window_bg_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color window_bg_color {};",
                &window_bg_color
            );
        }
        if let Some(window_fg_color) = self.window_fg_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color window_fg_color {};",
                &window_fg_color
            );
        }

        if let Some(view_bg_color) = self.view_bg_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color view_bg_color {};",
                &view_bg_color
            );
        }
        if let Some(view_fg_color) = self.view_fg_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color view_fg_color {};",
                &view_fg_color
            );
        }
        if let Some(shade_color) = self.shade_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color shade_color {};",
                &shade_color
            );
        }

        if let Some(headerbar_bg_color) = self.headerbar_bg_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color headerbar_bg_color {};",
                &headerbar_bg_color
            );
        }
        if let Some(headerbar_fg_color) = self.headerbar_fg_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color headerbar_fg_color {};",
                &headerbar_fg_color
            );
        }
        if let Some(headerbar_border_color) = self.headerbar_border_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color headerbar_border_color {};",
                &headerbar_border_color
            );
        }
        if let Some(headerbar_backdrop_color) = self.headerbar_backdrop_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color headerbar_backdrop_color {};",
                &headerbar_backdrop_color
            );
        }
        if let Some(headerbar_shade_color) = self.headerbar_shade_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color headerbar_shade_color {};",
                &headerbar_shade_color
            );
        }

        if let Some(card_bg_color) = self.card_bg_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color card_bg_color {};",
                &card_bg_color
            );
        }
        if let Some(card_fg_color) = self.card_fg_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color card_fg_color {};",
                &card_fg_color
            );
        }
        if let Some(card_shade_color) = self.card_shade_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color card_shade_color {};",
                &card_shade_color
            );
        }

        if let Some(popover_bg_color) = self.popover_bg_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color popover_bg_color {};",
                &popover_bg_color
            );
        }
        if let Some(popover_fg_color) = self.popover_fg_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color popover_fg_color {};",
                &popover_fg_color
            );
        }

        if let Some(scrollbar_outline_color) = self.scrollbar_outline_color.as_ref() {
            let _ = writeln!(
                user_color_css,
                "@define-color scrollbar_outline_color {};",
                &scrollbar_outline_color
            );
        }
        user_color_css
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn light_default() {
        super::ColorOverrides::light_default();
    }

    #[test]
    fn dark_default() {
        super::ColorOverrides::dark_default();
    }
}
