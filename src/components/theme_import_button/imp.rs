// SPDX-License-Identifier: MPL-2.0-only

use gtk4::{
    glib::{self},
    subclass::prelude::*,
    Box, Button, FileChooserNative,
};
use std::{cell::RefCell, rc::Rc};

// Object holding the state
#[derive(Default)]
pub struct ThemeImportButton {
    pub button: Rc<RefCell<Button>>,
    pub file_chooser: Rc<RefCell<FileChooserNative>>,
}

#[glib::object_subclass]
impl ObjectSubclass for ThemeImportButton {
    const NAME: &'static str = "ThemeImportButton";
    type Type = super::ThemeImportButton;
    type ParentType = Box;
}

// Trait shared by all GObjects
impl ObjectImpl for ThemeImportButton {}

// Trait shared by all widgets
impl WidgetImpl for ThemeImportButton {}

// Trait shared by all boxes
impl BoxImpl for ThemeImportButton {}
