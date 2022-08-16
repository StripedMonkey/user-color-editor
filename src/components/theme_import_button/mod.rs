// SPDX-License-Identifier: MPL-2.0-only

mod imp;

use cascade::cascade;
use gtk4::{glib, prelude::*, subclass::prelude::*, Button, FileChooserNative, Window};
use std::path::PathBuf;
use user_colors::{NAME, THEME_DIR};

glib::wrapper! {
    pub struct ThemeImportButton(ObjectSubclass<imp::ThemeImportButton>)
        @extends gtk4::Box, gtk4::Widget,
    @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget, gtk4::Orientable;
}

impl Default for ThemeImportButton {
    fn default() -> Self {
        Self::new()
    }
}

impl ThemeImportButton {
    fn connect_button_to_chooser_dialog(&self) {
        let imp = imp::ThemeImportButton::from_instance(self);
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
                    Some("Import Theme"),
                    window.as_ref(),
                    gtk4::FileChooserAction::Open,
                    None,
                    None,
                );


                file_chooser.connect_response(
                    glib::clone!(@weak self_ => move |file_chooser, response| {
                        if response != gtk4::ResponseType::Accept {return};
                        if let Some(f) = file_chooser.file() {
                            let ron_path: PathBuf = [NAME, THEME_DIR].iter().collect();
                            let copy_err = xdg::BaseDirectories::with_prefix(ron_path).ok().and_then(|ron_dirs| ron_dirs.place_data_file(f.basename()?).ok()).and_then(|dest| {
                                let source = f.path()?;
                                std::fs::copy(source, dest).ok()
                            }).is_none();
                            if copy_err {
                                // TODO toast error
                            }
                            // TODO Toast success
                        }
                    }),
                );

                let filter = gtk4::FileFilter::new();
                filter.add_suffix("ron");
                file_chooser.add_filter(&filter);

                file_chooser.show();
                let imp = imp::ThemeImportButton::from_instance(&self_);
                imp.file_chooser.replace(file_chooser);
            }),
        );
    }

    pub fn new() -> Self {
        let button = Button::with_label("Import theme");

        let self_: Self = glib::Object::new(&[]).expect("Failed to create `ThemeImportButton`.");
        cascade! {
            &self_;
            ..append(&button);
            ..set_margin_top(4);
            ..set_margin_bottom(4);
            ..set_margin_start(4);
            ..set_margin_end(4);

        };
        let imp = imp::ThemeImportButton::from_instance(&self_);

        imp.button.replace(button);

        self_.connect_button_to_chooser_dialog();

        self_
    }
}
