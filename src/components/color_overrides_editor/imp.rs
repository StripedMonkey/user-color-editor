// SPDX-License-Identifier: MPL-2.0-only

use crate::components::theme_import_button::ThemeImportButton;
use adw::StyleManager;
use gtk4::{glib, subclass::prelude::*, Box, Button, CssProvider, Entry};
use once_cell::sync::OnceCell;
use std::{cell::RefCell, rc::Rc};
use user_colors::colors::ColorOverrides;

// Object holding the state
#[derive(Default)]
pub struct ColorOverridesEditor {
    pub name: Rc<OnceCell<Entry>>,
    pub save: Rc<OnceCell<Button>>,
    pub file_button: OnceCell<ThemeImportButton>,
    pub theme: Rc<RefCell<ColorOverrides>>,
    pub css_provider: Rc<OnceCell<CssProvider>>,
    pub color_editor: Rc<OnceCell<Box>>,
    pub style_manager: Rc<OnceCell<StyleManager>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for ColorOverridesEditor {
    const NAME: &'static str = "ColorOverridesEditorWidget";
    type Type = super::ColorOverridesEditor;
    type ParentType = gtk4::Box;
}

// Trait shared by all GObjects
impl ObjectImpl for ColorOverridesEditor {}

// Trait shared by all widgets
impl WidgetImpl for ColorOverridesEditor {}

// Trait shared by all boxes
impl BoxImpl for ColorOverridesEditor {}
