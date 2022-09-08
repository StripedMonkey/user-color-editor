use std::hash::Hash;

use crate::{colors::ColorOverrides, config, NAME};
use futures::{channel::mpsc, SinkExt, StreamExt};
use iced::{theme::Palette, Color};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};

// Just a little utility function
#[cfg(feature = "iced")]
pub fn theme<I: 'static + Hash + Copy + Send + Sync>(
    id: I,
) -> iced::Subscription<(I, ThemeUpdate)> {
    use iced::subscription;

    subscription::unfold(id, State::Ready, move |state| load_theme(id, state))
}

#[cfg(feature = "iced")]
async fn load_theme<I: Copy>(id: I, state: State) -> (Option<(I, ThemeUpdate)>, State) {
    match state {
        State::Ready => {
            if let Ok(watcher) = ThemeWatcher::new() {
                let palette = ThemeWatcher::palette();
                (
                    Some((id, ThemeUpdate::Palette(palette))),
                    State::Waiting(watcher),
                )
            } else {
                (Some((id, ThemeUpdate::Errored)), State::Error)
            }
        }
        State::Waiting(mut t) => {
            if let Some(palette) = t.palette_change().await {
                (Some((id, ThemeUpdate::Palette(palette))), State::Waiting(t))
            } else {
                (Some((id, ThemeUpdate::Errored)), State::Error)
            }
        }
        State::Error => iced::futures::future::pending().await,
    }
}

#[cfg(feature = "iced")]
pub enum State {
    Ready,
    Waiting(ThemeWatcher),
    Error,
}

#[cfg(feature = "iced")]
#[derive(Debug, Clone)]
pub enum ThemeUpdate {
    Palette(Palette),
    Errored,
}

#[cfg(feature = "iced")]
pub struct ThemeWatcher {
    rx: mpsc::Receiver<notify::Event>,
    prev_palette: Palette,
}
#[cfg(feature = "iced")]
impl ThemeWatcher {
    pub fn new() -> anyhow::Result<Self> {
        let prev_palette = Self::palette();
        let (mut tx, rx) = mpsc::channel(20);
        let xdg_dirs = xdg::BaseDirectories::with_prefix(NAME)?;

        // Automatically select the best implementation for your platform.
        // You can also access each implementation directly e.g. INotifyWatcher.
        let mut watcher = RecommendedWatcher::new(
            move |res| {
                if let Ok(e) = res {
                    futures::executor::block_on(async {
                        let _ = tx.send(e).await;
                    })
                }
            },
            notify::Config::default(),
        )?;
        for config_dir in xdg_dirs.get_config_dirs() {
            let _ = watcher.watch(&config_dir, RecursiveMode::Recursive);
        }
        for data_dir in xdg_dirs.get_data_dirs() {
            let _ = watcher.watch(&&data_dir.as_ref(), RecursiveMode::Recursive);
        }

        Ok(Self { rx, prev_palette })
    }

    pub fn palette() -> Palette {
        let config = config::Config::load().unwrap_or_default();
        let (mut palette, color_overrides) = config
            .get_active()
            .map(|color_overrides| {
                (
                    match config {
                        config::Config::DarkLight { is_dark, .. } if !is_dark => Palette::LIGHT,
                        _ => Palette::DARK,
                    },
                    color_overrides,
                )
            })
            .unwrap_or_else(|_| match config {
                config::Config::DarkLight {
                    is_high_contrast,
                    is_dark,
                    ..
                } => {
                    let (palette, color_overrides) = if is_dark {
                        (Palette::DARK, ColorOverrides::dark_default())
                    } else {
                        (Palette::LIGHT, ColorOverrides::light_default())
                    };
                    if is_high_contrast {
                        (palette, color_overrides.to_high_contrast())
                    } else {
                        (palette, color_overrides)
                    }
                }
                _ => (Palette::DARK, ColorOverrides::dark_default()),
            });

        if let Some(c) = color_overrides
            .window_bg_color
            .as_ref()
            .and_then(|c| csscolorparser::parse(c).ok())
        {
            palette.background = Color::from_rgba(c.r as f32, c.g as f32, c.b as f32, c.a as f32);
        }
        if let Some(c) = color_overrides
            .window_fg_color
            .as_ref()
            .and_then(|c| csscolorparser::parse(c).ok())
        {
            palette.text = Color::from_rgba(c.r as f32, c.g as f32, c.b as f32, c.a as f32);
        }
        if let Some(c) = color_overrides
            .accent_bg_color
            .as_ref()
            .and_then(|c| csscolorparser::parse(c).ok())
        {
            palette.primary = Color::from_rgba(c.r as f32, c.g as f32, c.b as f32, c.a as f32);
        }
        if let Some(c) = color_overrides
            .success_bg_color
            .as_ref()
            .and_then(|c| csscolorparser::parse(c).ok())
        {
            palette.success = Color::from_rgba(c.r as f32, c.g as f32, c.b as f32, c.a as f32);
        }
        if let Some(c) = color_overrides
            .error_bg_color
            .as_ref()
            .and_then(|c| csscolorparser::parse(c).ok())
        {
            palette.danger = Color::from_rgba(c.r as f32, c.g as f32, c.b as f32, c.a as f32);
        }
        palette
    }

    pub async fn palette_change(&mut self) -> Option<Palette> {
        while let Some(e) = self.rx.next().await {
            match e.kind {
                // TODO only notify for changed data file if it is the active file
                notify::EventKind::Create(_)
                | notify::EventKind::Modify(_)
                | notify::EventKind::Remove(_) => {
                    let new_palette = Self::palette();
                    if self.prev_palette != new_palette {
                        self.prev_palette = new_palette;
                        return Some(new_palette);
                    }
                }
                _ => {}
            }
        }
        None
    }
}
