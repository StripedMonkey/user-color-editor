use gtk4::{
    gio::File,
    glib::{self, subclass::Signal},
    prelude::StaticType,
    subclass::prelude::*,
};
use once_cell::sync::Lazy;

// Object holding the state
#[derive(Default)]
pub struct ThemeDropdown {}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for ThemeDropdown {
    const NAME: &'static str = "ThemeDropdownWidget";
    type Type = super::ThemeDropdown;
    type ParentType = gtk4::Box;
}

// Trait shared by all GObjects
impl ObjectImpl for ThemeDropdown {
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder("theme-selected")
                .param_types(&[File::static_type()])
                .return_type::<()>()
                .build()]
        });
        SIGNALS.as_ref()
    }
}

// Trait shared by all widgets
impl WidgetImpl for ThemeDropdown {}

// Trait shared by all boxes
impl BoxImpl for ThemeDropdown {}
