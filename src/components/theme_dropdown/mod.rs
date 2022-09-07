// SPDX-License-Identifier: MPL-2.0-only

use std::path::PathBuf;

use gtk4::{
    ffi::GTK_INVALID_LIST_POSITION,
    gio::{File, ListStore},
    glib::{self, clone},
    prelude::*,
    DropDown, Label, SignalListItemFactory,
};
use itertools::Itertools;
use user_colors::{config::Config, NAME, THEME_DIR};
mod imp;

glib::wrapper! {
    pub struct ThemeDropdown(ObjectSubclass<imp::ThemeDropdown>)
        @extends gtk4::Box, gtk4::Widget,
    @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget, gtk4::Orientable;
}

pub enum Watch {
    Light,
    Dark,
    Static,
}

impl ThemeDropdown {
    pub fn new(watch: Option<Watch>) -> Self {
        let self_: Self = glib::Object::new(&[]).expect("Failed to create Theme Dropdown");

        let model = ListStore::new(File::static_type());

        let factory = SignalListItemFactory::new();
        factory.connect_setup(move |_, list_item| {
            let label = Label::new(None);
            list_item.set_child(Some(&label));
        });

        factory.connect_bind(move |_, list_item| {
            // Get `IntegerObject` from `ListItem`
            let file = list_item
                .item()
                .expect("The item has to exist.")
                .downcast::<File>()
                .expect("The item has to be an `GString`.");

            // Get `Label` from `ListItem`
            let label = list_item
                .child()
                .expect("The child has to exist.")
                .downcast::<Label>()
                .expect("The child has to be a `Label`.");

            // Set "label" to "number"
            let basename = file.basename().unwrap_or_default();
            let name = basename
                .file_stem()
                .map(|s| s.to_string_lossy())
                .unwrap_or_default();

            label.set_label(&name);
        });

        let dropdown = DropDown::builder()
            .model(&model)
            .factory(&factory)
            // TODO searchable ?
            // .enable_search()
            // .expression()
            .margin_bottom(4)
            .margin_top(4)
            .margin_start(4)
            .margin_end(4)
            .build();

        dropdown.connect_activate(clone!(@weak model => move |_dropdown| {
            let ron_path: PathBuf = [NAME, THEME_DIR].iter().collect();
            // TODO more error handling
            if let Some(mut themes) = xdg::BaseDirectories::with_prefix(ron_path).ok().map(|ron_dirs| ron_dirs.list_data_files(PathBuf::new())) {
                themes.sort_by(|a, b| {
                    let name_a = a.file_stem().map(|s| s.to_string_lossy()).unwrap_or_default();
                    let name_b = b.file_stem().map(|s| s.to_string_lossy()).unwrap_or_default();
                    name_a.cmp(&name_b)
                });
                model.splice(0, model.n_items(), &themes.iter().map(File::for_path).collect_vec());
            }
        }));

        let ron_path: PathBuf = [NAME, THEME_DIR].iter().collect();
        // TODO more error handling
        if let Some(mut themes) = xdg::BaseDirectories::with_prefix(ron_path)
            .ok()
            .map(|ron_dirs| ron_dirs.list_data_files(PathBuf::new()))
        {
            themes.sort_by(|a, b| {
                let name_a = a
                    .file_stem()
                    .map(|s| s.to_string_lossy())
                    .unwrap_or_default();
                let name_b = b
                    .file_stem()
                    .map(|s| s.to_string_lossy())
                    .unwrap_or_default();
                name_a.cmp(&name_b)
            });
            model.splice(
                0,
                model.n_items(),
                &themes.iter().map(File::for_path).collect_vec(),
            );
        }

        dropdown.connect_selected_notify(clone!(@weak self_ => move|dropdown| {
            if let Some(selected_item) = dropdown.selected_item() {
                let file = selected_item
                .downcast::<File>()
                .expect("The item has to be an `File`.");

                self_.emit_by_name::<()>("theme-selected", &[&file]);
            }
        }));

        self_.append(&dropdown);

        if let (Some(watch), Ok(config)) = (watch, Config::load()) {
            let selected = match (watch, config) {
                (Watch::Light, Config::DarkLight { light, .. }) => light,
                (Watch::Dark, Config::DarkLight { dark, .. }) => dark,
                (Watch::Static, Config::Static { name, .. }) => name,
                _ => return self_,
            };

            if let Some(selected) = (0..model.n_items()).position(|i| {
                if let Some(o) = model.item(i) {
                    let file = o
                        .downcast::<File>()
                        .expect("The item has to be an `GString`.");

                    // Set "label" to "number"
                    let basename = file.basename().unwrap_or_default();
                    let name = basename
                        .file_stem()
                        .map(|s| s.to_string_lossy())
                        .unwrap_or_default();
                    name == selected
                } else {
                    false
                }
            }) {
                dropdown.set_selected(selected.try_into().unwrap());
            } else {
                dropdown.set_selected(GTK_INVALID_LIST_POSITION);
            }
            // TODO watch config for changes
        } else {
            dropdown.set_selected(GTK_INVALID_LIST_POSITION);
        }

        self_
    }
}
