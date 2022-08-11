// SPDX-License-Identifier: MPL-2.0-only

mod imp;

use cascade::cascade;
use gtk4::{gio::File, glib, prelude::*, subclass::prelude::*, Button, FileChooserNative, Window};
use std::path::PathBuf;
use user_colors::{NAME, THEME_DIR};

glib::wrapper! {
    pub struct ThemeChooserButton(ObjectSubclass<imp::ThemeChooserButton>)
        @extends gtk4::Box, gtk4::Widget,
    @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget, gtk4::Orientable;
}

impl Default for ThemeChooserButton {
    fn default() -> Self {
        Self::new()
    }
}

impl ThemeChooserButton {
    fn connect_button_to_chooser_dialog(&self) {
        let imp = imp::ThemeChooserButton::from_instance(&self);
        imp.button.borrow().connect_clicked(
            glib::clone!(@weak imp.file_chooser as file_chooser, @weak self as self_ => move |_| {
                let window = self_
                    .root()
                    .map(|root| {
                        if let Ok(w) = root.downcast::<Window>() {
                            Some(w)
                        } else {
                            None
                        }
                    })
                    .unwrap_or_default();

                let file_chooser = FileChooserNative::new(
                    Some("Select Theme"),
                    window.as_ref(),
                    gtk4::FileChooserAction::Open,
                    None,
                    None,
                );

                let xdg_dirs = xdg::BaseDirectories::with_prefix(NAME).unwrap();
                let mut path: PathBuf = xdg_dirs.get_data_home();
                path.push(THEME_DIR);
                let _ = file_chooser.set_current_folder(Some(&File::for_path(path)));
                file_chooser.connect_response(
                    glib::clone!(@weak self_ => move |file_chooser, response| {
                        if response != gtk4::ResponseType::Accept {return};
                        if let Some(f) = file_chooser.file() {
                            self_.emit_by_name::<()>("file-selected", &[&f]);
                        }
                    }),
                );

                let filter = gtk4::FileFilter::new();
                filter.add_suffix("ron");
                file_chooser.add_filter(&filter);

                file_chooser.show();
                let imp = imp::ThemeChooserButton::from_instance(&self_);
                imp.file_chooser.replace(file_chooser);
            }),
        );
    }

    pub fn new() -> Self {
        let button = Button::with_label("Load theme");

        let self_: Self = glib::Object::new(&[]).expect("Failed to create `ThemeChooserButton`.");
        cascade! {
            &self_;
            ..append(&button);
            ..set_margin_top(4);
            ..set_margin_bottom(4);
            ..set_margin_start(4);
            ..set_margin_end(4);

        };
        let imp = imp::ThemeChooserButton::from_instance(&self_);

        imp.button.replace(button);

        self_.connect_button_to_chooser_dialog();

        self_
    }
}
