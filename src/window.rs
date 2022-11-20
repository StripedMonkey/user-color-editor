use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use gtk4::{gio, glib};

use crate::application::ExampleApplication;
use crate::components::ColorOverridesEditor;
use crate::config::{APP_ID, PROFILE};

mod imp {

    use gtk4::gio::SettingsSchemaSource;

    use super::*;

    pub struct UserColorEditorWindow {
        pub settings: Option<gio::Settings>,
    }

    impl Default for UserColorEditorWindow {
        fn default() -> Self {
            Self {
                settings: SettingsSchemaSource::default()
                    .and_then(|s| s.lookup(APP_ID, true))
                    .map(|_| gio::Settings::new(APP_ID)),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for UserColorEditorWindow {
        const NAME: &'static str = "UserColorEditorWindow";
        type Type = super::UserColorEditorWindow;
        type ParentType = gtk4::ApplicationWindow;

        fn class_init(_: &mut Self::Class) {}

        // You must call `Widget`'s `init_template()` within `instance_init()`.
        fn instance_init(_: &glib::subclass::InitializingObject<Self>) {}
    }

    impl ObjectImpl for UserColorEditorWindow {
        fn constructed(&self) {
            self.parent_constructed();
            // Devel Profile

            if PROFILE == "Devel" {
                self.obj().add_css_class("devel");
            }
        }
    }

    impl WidgetImpl for UserColorEditorWindow {}

    impl WindowImpl for UserColorEditorWindow {
        // Save window state on delete event
        fn close_request(&self) -> gtk4::Inhibit {
            if let Err(err) = self.obj().save_window_size() {
                log::warn!("Failed to save window state, {}", &err);
            }

            // Pass close request on to the parent
            self.parent_close_request()
        }
    }

    impl ApplicationWindowImpl for UserColorEditorWindow {}
}

glib::wrapper! {
    pub struct UserColorEditorWindow(ObjectSubclass<imp::UserColorEditorWindow>)
        @extends gtk4::Widget, gtk4::Window, gtk4::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup, gtk4::Root;
}

impl UserColorEditorWindow {
    pub fn new(app: &ExampleApplication) -> Self {
        let self_ = glib::Object::new::<Self>(&[("application", app)]);
        self_.set_child(Some(&ColorOverridesEditor::new()));
        self_.set_hide_on_close(true);

        self_
    }

    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let imp = self.imp();

        let (width, height) = self.default_size();
        if let Some(settings) = imp.settings.as_ref() {
            settings.set_int("window-width", width)?;
            settings.set_int("window-height", height)?;
        }

        Ok(())
    }

    pub fn load_window_size(&self) {
        let imp = self.imp();
        if let Some(settings) = imp.settings.as_ref() {
            let width = settings.int("window-width");
            let height = settings.int("window-height");

            self.set_default_size(width, height);
        } else {
            self.set_default_size(500, 800);
        }
    }
}
