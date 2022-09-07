// SPDX-License-Identifier: MPL-2.0-only

use crate::{
    components::{
        theme_dropdown::{ThemeDropdown, Watch},
        theme_import_button::ThemeImportButton,
    },
    fl,
    util::{hex_from_rgba, SRGBA},
};

use adw::{traits::ExpanderRowExt, ExpanderRow};
use cascade::cascade;
use gtk4::{
    gdk::{self, RGBA},
    gio::{self, File},
    glib::{self, closure_local},
    prelude::*,
    subclass::prelude::*,
    Align, Box, Button, ColorButton, CssProvider, Entry, Label, MessageDialog, Orientation,
    ScrolledWindow, Switch, Window,
};
use relm4_macros::view;
use std::fmt::Display;
use user_colors::{colors::ColorOverrides, config::Config};
mod imp;

glib::wrapper! {
    pub struct ColorOverridesEditor(ObjectSubclass<imp::ColorOverridesEditor>)
        @extends gtk4::Box, gtk4::Widget,
    @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget, gtk4::Orientable;
}

impl Default for ColorOverridesEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorOverridesEditor {
    pub fn new() -> Self {
        let self_: Self = glib::Object::new(&[]).expect("Failed to create Theme Editor Widget");

        let imp = imp::ColorOverridesEditor::from_instance(&self_);

        cascade! {
            &self_;
            ..set_orientation(Orientation::Vertical);
        };

        view! {
            inner = Box {
                set_orientation: Orientation::Vertical,
                set_spacing: 4,
                set_margin_top: 4,
                set_margin_bottom: 4,
                set_margin_start: 4,
                set_margin_end: 4,

                append: name = &Entry {
                    set_placeholder_text: Some("Theme Name"),
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,
                    set_width_request: 160,
                },

                append: color_box = &Box {
                    set_orientation: Orientation::Vertical,
                    set_spacing: 4,
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,
                },


                // TODO add the rest label for each section

                append: control_button_box = &Box {
                    set_orientation: Orientation::Horizontal,
                    set_spacing: 4,
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,

                    append: _box = &Box {
                        set_orientation: Orientation::Vertical,
                        append: _label = &Label {
                            set_text: &fl!("load-theme"),
                        },
                        append: load_dropdown = &ThemeDropdown::new(None),
                    },


                    append: file_button = &ThemeImportButton {},

                    append: save_button = &Button {
                        set_margin_top: 4,
                        set_margin_bottom: 4,
                        set_margin_start: 4,
                        set_margin_end: 4,
                        add_css_class: "suggested-action",
                        set_label: &fl!("save-theme")
                    },
                },
                append = &Box {
                    set_orientation: Orientation::Horizontal,
                    set_spacing: 4,
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,
                    append = &Label {
                        set_text: &fl!("dark-light-switch"),
                    },
                    append: dark_light_switch = &Switch {},
                },

                append: config_section = &Box {
                    set_orientation: Orientation::Vertical,
                },
            }
        };

        // if no valid config exists, create one
        let mut config = match Config::load() {
            Ok(c) => c,
            Err(_) => {
                let c = Config::default();
                c.save().unwrap();
                c
            }
        };
        // init state of switch
        match config {
            Config::DarkLight { .. } => {
                dark_light_switch.set_state(true);
            }
            Config::Static { .. } => {
                dark_light_switch.set_state(false);
            }
        }

        let settings_source = gio::SettingsSchemaSource::default();
        if let Some(dark_settings) = settings_source
            .as_ref()
            .and_then(|s| s.lookup("org.gnome.desktop.interface", true))
            .map(|_| gio::Settings::new("org.gnome.desktop.interface"))
        {
            dark_settings.connect_changed(
                Some("color-scheme"),
                glib::clone!(@weak self_ => move |settings, _| {
                    let dark = settings.string("color-scheme").as_str() != "prefer-light";
                    let mut config: Config = self_.imp().config.borrow().clone();
                    if match config {
                        Config::DarkLight { ref mut is_dark, .. } if *is_dark != dark => {
                            *is_dark = dark;
                            true
                        },
                        _ => false
                    } {
                        if let Some(name) = config.active_name() {
                            if let Ok(palette) = ColorOverrides::load_from_name(&name) {
                                self_.imp().theme.replace(palette);
                            }
                            let _ = config.apply_gtk4();
                        }
                        let _ = config.save();
                        self_.imp().config.replace(config);
                        self_.preview();
                    }
                }),
            );
            let dark = dark_settings.string("color-scheme").as_str() != "prefer-light";
            match config {
                Config::DarkLight {
                    ref mut is_dark, ..
                } if *is_dark != dark => {
                    *is_dark = dark;
                    let _ = match config.active_name() {
                        Some(n) if !n.is_empty() => config.apply_gtk4(),
                        _ => Ok(()),
                    };
                }
                _ => {}
            }
            imp.dark_settings.set(dark_settings).unwrap();
        };

        if let Some(hc_settings) = settings_source
            .and_then(|s| s.lookup("org.gnome.desktop.a11y.interface", true))
            .map(|_| gio::Settings::new("org.gnome.desktop.a11y.interface"))
        {
            hc_settings.connect_changed(Some("high-contrast"), glib::clone!(@weak self_ => move |settings, _| {
                let high_contrast = settings.boolean("high-contrast");

                let mut config: Config = self_.imp().config.borrow().clone();
                if match config {
                    Config::DarkLight { ref mut is_high_contrast, .. } if *is_high_contrast != high_contrast => {
                        *is_high_contrast = high_contrast;
                        if let Some(name) = config.active_name() {
                            if let Ok(palette) = ColorOverrides::load_from_name(&name) {
                                self_.imp().theme.replace(palette);
                            }
                            let _ = config.save();
                            let _ = config.apply_gtk4();
                        }
                        true
                    },
                    _ => false
                } {
                    self_.imp().config.replace(config);
                    self_.preview();
                }
            }));
            let high_contrast = hc_settings.boolean("high-contrast");
            match config {
                Config::DarkLight {
                    ref mut is_high_contrast,
                    ..
                } if *is_high_contrast != high_contrast => {
                    *is_high_contrast = high_contrast;
                    let _ = match config.active_name() {
                        Some(n) if !n.is_empty() => config.apply_gtk4(),
                        _ => Ok(()),
                    };
                }
                _ => {}
            }
            imp.high_contrast_settings.set(hc_settings).unwrap();
        };

        // init config widgets
        self_.set_config_widgets(&config_section, &config);

        dark_light_switch.connect_state_set(glib::clone!(@weak config_section, @weak self_=> @default-return gtk4::Inhibit(false), move |_, state| {
            // cleanup existing widgets
            while let Some(c) = config_section.first_child() {
                config_section.remove(&c);
            }

            // TODO set dark light & high contrast depending on gsettings
            if state {
                let config = Config::new_dark_light(true, false, "".into(), "".into());
                let _ = config.save();
                self_.set_config_widgets(&config_section, &config);
            } else {
                let config = Config::new_static("".into(), false);
                let _ = config.save();
                self_.set_config_widgets(&config_section, &config);
            }
            gtk4::Inhibit(false)
        }));

        load_dropdown.connect_closure(
            "theme-selected",
            false,
            closure_local!(@weak-allow-none imp.name as name, @weak-allow-none imp.theme as theme, @weak-allow-none self_ => move |_file_button: ThemeDropdown, f: File| {
                if let (Some(theme), Some(name), Some(Ok(t))) = (theme, name, f.path().as_ref().map(|p| ColorOverrides::load(p))) {
                    let name = name.get().unwrap();
                    name.set_text(&t.name);
                    theme.replace(t);
                    if let Some(self_) = self_ {
                        self_.set_buttons();
                        self_.preview();
                    }
                }
            }),
        );

        let scroll_window = ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .child(&inner)
            .build();

        self_.append(&scroll_window);

        let provider = CssProvider::new();
        if let Some(display) = gdk::Display::default() {
            gtk4::StyleContext::add_provider_for_display(
                &display,
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_USER,
            );
        }

        // set widget state
        imp.css_provider.set(provider).unwrap();
        imp.name.set(name).unwrap();
        imp.save.set(save_button).unwrap();
        imp.file_button.set(file_button).unwrap();
        imp.color_editor.set(color_box).unwrap();
        imp.config.replace(config);
        imp.dark_light_switch.set(dark_light_switch).unwrap();
        self_.set_buttons();
        self_.connect_name();
        self_.connect_control_buttons();

        self_
    }

    fn connect_name(&self) {
        let imp = imp::ColorOverridesEditor::from_instance(self);
        imp.name.get().unwrap().connect_changed(
            glib::clone!(@weak imp.theme as theme => move |name| {
                let name = name.text();
                theme.borrow_mut().name = String::from(name.as_str());
            }),
        );
    }

    fn set_config_widgets(&self, config_box: &Box, config: &Config) {
        match config {
            Config::DarkLight {
                is_dark,
                is_high_contrast,
                ..
            } => {
                view! {
                    light_box = Box {
                        set_orientation: Orientation::Horizontal,
                        set_spacing: 4,
                        set_margin_top: 4,
                        set_margin_bottom: 4,
                        set_margin_start: 4,
                        set_margin_end: 4,

                        append: light_theme_label = &Label {
                            set_text: &fl!("current-light-theme"),
                        },
                        append: light_dropdown = &ThemeDropdown::new(Some(Watch::Light)),

                    }
                };
                view! {
                    dark_box = Box {
                        set_orientation: Orientation::Horizontal,
                        set_spacing: 4,
                        set_margin_top: 4,
                        set_margin_bottom: 4,
                        set_margin_start: 4,
                        set_margin_end: 4,

                        append: dark_theme_label = &Label {
                            set_text: &fl!("current-dark-theme"),
                        },
                        append: dark_dropdown = &ThemeDropdown::new(Some(Watch::Dark)),
                    }
                };
                view! {
                    prefer_dark = Box {
                        set_orientation: Orientation::Horizontal,
                        set_spacing: 4,
                        set_margin_top: 4,
                        set_margin_bottom: 4,
                        set_margin_start: 4,
                        set_margin_end: 4,
                        append = &Label {
                            set_text: &fl!("set-dark-switch"),
                        },
                        append: dark_light_switch = &Switch {},
                    }
                };
                view! {
                    high_contrast = Box {
                        set_orientation: Orientation::Horizontal,
                        set_spacing: 4,
                        set_margin_top: 4,
                        set_margin_bottom: 4,
                        set_margin_start: 4,
                        set_margin_end: 4,
                        append = &Label {
                            set_text: &fl!("set-high-contrast-switch"),
                        },
                        append: high_contrast_switch = &Switch {},
                    }
                };
                cascade! {
                    config_box;
                    ..append(&light_box);
                    ..append(&dark_box);
                    ..append(&prefer_dark);
                    ..append(&high_contrast);
                };
                dark_light_switch.set_state(*is_dark);
                dark_light_switch.set_state(*is_high_contrast);

                // TODO init selection with config values
                light_dropdown.connect_closure(
                    "theme-selected",
                    false,
                    closure_local!(@weak-allow-none light_theme_label, @weak-allow-none self as self_  => move |_file_button: ThemeDropdown, f: File| {
                        if let (Some(_), Some(name), Some(self_)) = (light_theme_label, f.basename(), self_) {
                            let name = name.file_stem().unwrap().to_string_lossy();
                            let mut c = self_.imp().config.borrow().clone();
                            match c {
                                Config::DarkLight { ref mut light, .. } => {*light = name.to_string();}
                                _ => return,
                            };
                            if let Err(err) = {
                                c.active_name();
                                let _ = c.save();
                                c.apply_gtk4()
                            } {
                                if let Some(window) = self_.root().and_then(|root| {
                                    root.downcast::<Window>().ok()
                                }) {
                                    glib::MainContext::default().spawn_local(Self::dialog(window, format!("Warning to apply custom colors. {}", err)));
                                };
                            }
                            self_.imp().config.replace(c);
                        }
                    }),
                );

                dark_dropdown.connect_closure(
                    "theme-selected",
                    false,
                    closure_local!(@weak-allow-none dark_theme_label, @weak-allow-none self as self_ => move |_file_button: ThemeDropdown, f: File| {
                        if let (Some(_), Some(name), Some(self_)) = (dark_theme_label, f.basename(), self_) {
                            let name = name.file_stem().unwrap().to_string_lossy();
                            let mut c = self_.imp().config.borrow().clone();
                            match c {
                                Config::DarkLight { ref mut dark, .. } => {*dark = name.to_string();}
                                _ => return,
                            };
                            if let Err(err) = {
                                c.active_name();
                                let _ = c.save();
                                c.apply_gtk4()
                            } {
                                if let Some(window) = self_.root().and_then(|root| {
                                    root.downcast::<Window>().ok()
                                }) {
                                    glib::MainContext::default().spawn_local(Self::dialog(window, format!("Warning to apply custom colors. {}", err)));
                                };
                            }
                            self_.imp().config.replace(c);
                        }
                    }),
                );
                dark_light_switch.connect_state_set(glib::clone!(@weak self as self_ => @default-return gtk4::Inhibit(false), move |_, state| {
                    if let Some(settings) = self_.imp().dark_settings.get() {
                        let _ = settings.set_string("color-scheme", if state {"prefer-dark"} else {"prefer-light"});
                    }
                    gtk4::Inhibit(false)
                }));
                high_contrast_switch.connect_state_set(glib::clone!(@weak self as self_ => @default-return gtk4::Inhibit(false), move |_, state| {
                    if let Some(settings) = self_.imp().high_contrast_settings.get() {
                        let _ = settings.set_boolean("high-contrast", state);
                    }
                    gtk4::Inhibit(false)
                }));
            }
            Config::Static { .. } => {
                view! {
                    theme_box = Box {
                        set_orientation: Orientation::Horizontal,
                        set_spacing: 4,
                        set_margin_top: 4,
                        set_margin_bottom: 4,
                        set_margin_start: 4,
                        set_margin_end: 4,

                        append: theme_label = &Label {
                            set_text: &fl!("current-theme"),
                        },
                        append: dropdown = &ThemeDropdown::new(Some(Watch::Static)),
                    }
                };
                view! {
                    switch_box = Box {
                        set_orientation: Orientation::Horizontal,
                        set_spacing: 4,
                        set_margin_top: 4,
                        set_margin_bottom: 4,
                        set_margin_start: 4,
                        set_margin_end: 4,

                        append = &Label {
                            set_text: &fl!("apply-to-all-apps"),
                        },
                        append: switch = &Switch {},
                    }
                };
                cascade! {
                    config_box;
                    ..append(&theme_box);
                    ..append(&switch_box);
                };

                dropdown.connect_closure(
                    "theme-selected",
                    false,
                    closure_local!(@weak-allow-none theme_label, @weak-allow-none self as self_ => move |_file_button: ThemeDropdown, f: File| {
                        if let (Some(_), Some(name)) = (theme_label, f.basename()) {
                            let name = name.file_stem().unwrap().to_string_lossy();
                            user_colors::config::Config::set_active_light(&name).unwrap();
                            if let Err(err) = Config::load().and_then(|c| match c.active_name() {
                                Some(n) if !n.is_empty() => c.apply_gtk4(),
                                _ => Ok(()),
                            }) {
                                if let Some(window) = self_.and_then(|self_| self_.root()).and_then(|root| {
                                    root.downcast::<Window>().ok()
                                }) {
                                    glib::MainContext::default().spawn_local(Self::dialog(window, format!("Warning to apply custom colors. {}", err)));
                                };
                            }
                        }
                    }),
                );

                switch.connect_state_set(move |_, state| {
                    let mut c = match Config::load() {
                        Ok(c) => c,
                        Err(_) => return gtk4::Inhibit(false),
                    };
                    match c {
                        Config::DarkLight { .. } => return gtk4::Inhibit(false),
                        Config::Static {
                            ref mut apply_all, ..
                        } => {
                            *apply_all = state;
                        }
                    };

                    if c.active_name().is_some() {
                        let _ = c.save();
                        let _ = c.apply_gtk4();
                    }

                    gtk4::Inhibit(false)
                });
            }
        }
    }

    fn set_buttons(&self) {
        let imp = imp::ColorOverridesEditor::from_instance(self);

        let color_editor = imp.color_editor.get().unwrap();
        let mut c = color_editor.first_child();
        while let Some(child) = c {
            color_editor.remove(&child);
            c = color_editor.first_child();
        }

        let accent_section = ExpanderRow::builder()
            .name(&fl!("accent-Colors"))
            .expanded(true)
            .enable_expansion(true)
            .title(&fl!("accent-Colors"))
            .hexpand(true)
            .build();
        let accent_bg_color =
            Self::get_color_button(self, "accent_bg_color", &fl!("accent-background-color"));
        accent_section.add_row(&accent_bg_color);
        let accent_fg_color =
            Self::get_color_button(self, "accent_fg_color", &fl!("accent-foreground-color"));
        accent_section.add_row(&accent_fg_color);
        let accent_color = Self::get_color_button(self, "accent_color", &fl!("accent-color"));
        accent_section.add_row(&accent_color);

        let destructive_section = ExpanderRow::builder()
            .name(&fl!("destructive-colors"))
            .expanded(true)
            .enable_expansion(true)
            .title(&fl!("destructive-colors"))
            .hexpand(true)
            .build();
        let destructive_bg_color =
            Self::get_color_button(self, "destructive_bg_color", "destructive-background-color");
        destructive_section.add_row(&destructive_bg_color);
        let destructive_fg_color =
            Self::get_color_button(self, "destructive_fg_color", "destructive-foreground-color");
        destructive_section.add_row(&destructive_fg_color);
        let destructive_color =
            Self::get_color_button(self, "destructive_color", &fl!("destructive-color"));
        destructive_section.add_row(&destructive_color);

        let status_section = ExpanderRow::builder()
            .name(&fl!("status-colors"))
            .expanded(false)
            .enable_expansion(true)
            .title(&fl!("status-colors"))
            .hexpand(true)
            .build();
        let success_color = Self::get_color_button(self, "success_color", &fl!("success-color"));
        status_section.add_row(&success_color);
        let success_bg_color =
            Self::get_color_button(self, "success_bg_color", &fl!("success-background-color"));
        status_section.add_row(&success_bg_color);
        let success_fg_color =
            Self::get_color_button(self, "success_fg_color", &fl!("success-foreground-color"));
        status_section.add_row(&success_fg_color);

        let warning_color = Self::get_color_button(self, "warning_color", &fl!("warning-color"));
        status_section.add_row(&warning_color);
        let warning_bg_color =
            Self::get_color_button(self, "warning_bg_color", &fl!("warning-background-color"));
        status_section.add_row(&warning_bg_color);
        let warning_fg_color =
            Self::get_color_button(self, "warning_fg_color", &fl!("warning-foreground-color"));
        status_section.add_row(&warning_fg_color);

        let error_color = Self::get_color_button(self, "error_color", &fl!("error-color"));
        status_section.add_row(&error_color);
        let error_bg_color =
            Self::get_color_button(self, "error_bg_color", &fl!("error-background-color"));
        status_section.add_row(&error_bg_color);
        let error_fg_color =
            Self::get_color_button(self, "error_fg_color", &fl!("error-foreground-color"));
        status_section.add_row(&error_fg_color);

        let content_section = ExpanderRow::builder()
            .name(&fl!("content-colors"))
            .expanded(false)
            .enable_expansion(true)
            .title(&fl!("content-colors"))
            .hexpand(true)
            .build();
        let view_bg_color =
            Self::get_color_button(self, "view_bg_color", &fl!("widget-base-color"));
        content_section.add_row(&view_bg_color);
        let view_fg_color =
            Self::get_color_button(self, "view_fg_color", &fl!("widget-text-color"));
        content_section.add_row(&view_fg_color);

        let window_section = ExpanderRow::builder()
            .name(&fl!("window-colors"))
            .expanded(false)
            .enable_expansion(true)
            .title(&fl!("window-colors"))
            .hexpand(true)
            .build();
        let window_bg_color =
            Self::get_color_button(self, "window_bg_color", &fl!("window-background-color"));
        window_section.add_row(&window_bg_color);
        let window_fg_color =
            Self::get_color_button(self, "window_fg_color", &fl!("window-foreground-color"));
        window_section.add_row(&window_fg_color);

        let headerbar_section = ExpanderRow::builder()
            .name(&fl!("headerbar-colors"))
            .expanded(false)
            .enable_expansion(true)
            .title(&fl!("headerbar-colors"))
            .hexpand(true)
            .build();
        let headerbar_bg_color = Self::get_color_button(
            self,
            "headerbar_bg_color",
            &fl!("headerbar-background-color"),
        );
        headerbar_section.add_row(&headerbar_bg_color);

        let headerbar_fg_color = Self::get_color_button(
            self,
            "headerbar_fg_color",
            &fl!("headerbar-foreground-color"),
        );
        headerbar_section.add_row(&headerbar_fg_color);

        let headerbar_border_color = Self::get_color_button(
            self,
            "headerbar_border_color",
            &fl!("headerbar-border-color"),
        );
        headerbar_section.add_row(&headerbar_border_color);

        let headerbar_backdrop_color =
            Self::get_color_button(self, "headerbar_backdrop_color", "headerbar-backdrop-color");
        headerbar_section.add_row(&headerbar_backdrop_color);

        let headerbar_shade_color =
            Self::get_color_button(self, "headerbar_shade_color", &fl!("headerbar-shade-color"));
        headerbar_section.add_row(&headerbar_shade_color);

        let card_section = ExpanderRow::builder()
            .name(&fl!("card-colors"))
            .expanded(false)
            .enable_expansion(true)
            .title(&fl!("card-colors"))
            .hexpand(true)
            .build();
        let card_bg_color =
            Self::get_color_button(self, "card_bg_color", &fl!("card-background-color"));
        card_section.add_row(&card_bg_color);
        let card_fg_color =
            Self::get_color_button(self, "card_fg_color", &fl!("card-foreground-color"));
        card_section.add_row(&card_fg_color);
        let card_shade_color =
            Self::get_color_button(self, "card_shade_color", &fl!("card-shade-color"));
        card_section.add_row(&card_shade_color);

        let popover_section = ExpanderRow::builder()
            .name(&fl!("popover-colors"))
            .expanded(false)
            .enable_expansion(true)
            .title(&fl!("popover-colors"))
            .hexpand(true)
            .build();
        let popover_bg_color =
            Self::get_color_button(self, "popover_bg_color", &fl!("popover-background-color"));
        popover_section.add_row(&popover_bg_color);
        let popover_fg_color =
            Self::get_color_button(self, "popover_fg_color", &fl!("popover-foreground-color"));
        popover_section.add_row(&popover_fg_color);

        let misc_section = ExpanderRow::builder()
            .name(&fl!("miscellaneous-colors"))
            .expanded(false)
            .enable_expansion(true)
            .title(&fl!("miscellaneous-colors"))
            .hexpand(true)
            .build();
        let scrollbar_outline_color = Self::get_color_button(
            self,
            "scrollbar_outline_color",
            &fl!("scrollbar-outline-color"),
        );
        misc_section.add_row(&scrollbar_outline_color);
        let shade_color = Self::get_color_button(self, "shade_color", &fl!("shade-color"));
        misc_section.add_row(&shade_color);

        color_editor.append(&accent_section);
        color_editor.append(&destructive_section);
        color_editor.append(&status_section);
        color_editor.append(&content_section);
        color_editor.append(&window_section);
        color_editor.append(&headerbar_section);
        color_editor.append(&card_section);
        color_editor.append(&popover_section);
        color_editor.append(&misc_section);
    }

    fn get_color_button(&self, id: &str, label: &str) -> Box {
        // TODO add button for clearing color
        let imp = imp::ColorOverridesEditor::from_instance(self);

        let rgba = SRGBA::default().into();
        let color_button = cascade! {
            ColorButton::with_rgba(&rgba);
            ..set_title(label);
            ..set_use_alpha(true);
        };
        if let Some(Ok(c)) = imp.theme.borrow().get_key(id).map(|c| RGBA::parse(&c)) {
            color_button.set_rgba(&c);
        } else {
            color_button.set_rgba(&RGBA::new(0.0, 0.0, 0.0, 0.0));
        }
        let id_clone = id.to_string();
        color_button
        .connect_rgba_notify(glib::clone!(@weak imp.theme as theme, @weak self as self_ => move |color_button| {
            {
                let mut t = theme.borrow_mut();
                t.set_key(&id_clone, Some(hex_from_rgba(color_button.rgba()))).unwrap_or_else(|_| panic!("Failed to set {}", id_clone));
            }
            self_.preview();
        }));
        let clear_button = Button::with_label("Clear");
        clear_button.add_css_class("destructive-action");
        clear_button.set_halign(Align::End);
        let id_clone = id.to_string();
        clear_button.connect_clicked(
            glib::clone!(@weak color_button, @weak imp.theme as theme => move |_| {
                {
                    let mut t = theme.borrow_mut();
                    t.set_key(&id_clone, None).unwrap_or_else(|_| panic!("Failed to set {id_clone}"));
                    drop(t);
                    color_button.set_rgba(&RGBA::new(0.0, 0.0, 0.0, 0.0));
                }
            }),
        );
        view! {
            color_box = Box {
                set_orientation: Orientation::Horizontal,
                set_spacing: 4,
                set_margin_top: 4,
                set_margin_bottom: 4,
                set_margin_start: 4,
                set_margin_end: 4,
                set_hexpand: true,

                append: &color_button,

                append: accent_color_label = &Label {
                    set_text: label,
                },
                append = &Box {
                    set_orientation: Orientation::Horizontal,
                    set_hexpand: true,
                    set_halign: Align::End,
                    append: &clear_button,
                },
            }
        };
        color_box
    }

    fn connect_control_buttons(&self) {
        let imp = imp::ColorOverridesEditor::from_instance(self);
        let theme = &imp.theme;

        imp.save.get().unwrap().connect_clicked(
            glib::clone!(@weak theme, @weak self as self_ => move |_| {
                if !theme.borrow().name.is_empty() {
                    // TODO toast if fails
                    let _ = theme.borrow().save();
                    if let Err(err) = Config::load().and_then(|c| match c.active_name() {
                        Some(n) if !n.is_empty() => c.apply_gtk4(),
                        _ => Ok(()),
                    }) {
                        if let Some(window) = self_.root().and_then(|root| {
                            root.downcast::<Window>().ok()
                        }) {
                            glib::MainContext::default().spawn_local(Self::dialog(window, format!("Warning to apply custom colors. {}", err)));
                        };
                    }
                } else {
                    // todo replace with toast
                    let window = self_.root().map(|root| {
                        if let Ok(w) = root.downcast::<Window>() {
                            Some(w)
                        } else {
                            None
                        }
                    }).unwrap_or_default();
                    if let Some(window) = window {
                        glib::MainContext::default().spawn_local(Self::dialog(window, "Please enter a name"));
                    }
                }
            }),
        );
    }

    fn preview(&self) {
        let imp = self.imp();
        let theme = self.imp().theme.borrow();
        let preview_css = &mut theme.as_gtk_css();
        preview_css.push_str(&imp.theme.borrow().as_gtk_css());
        imp.css_provider
            .get()
            .unwrap()
            .load_from_data(preview_css.as_bytes());
    }

    async fn dialog<T: Display>(window: Window, msg: T) {
        let msg_dialog = MessageDialog::builder()
            .transient_for(&window)
            .modal(true)
            .buttons(gtk4::ButtonsType::Close)
            .text(&format!("{}", msg))
            .build();
        cascade! {
            msg_dialog.message_area();
            ..set_margin_top(8);
            ..set_margin_bottom(8);
            ..set_margin_start(8);
            ..set_margin_end(8);
        };
        let _ = msg_dialog.run_future().await;
        msg_dialog.close();
    }
}
