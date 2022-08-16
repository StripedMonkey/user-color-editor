// SPDX-License-Identifier: MPL-2.0-only

pub mod colors;
pub mod config;
#[cfg(feature = "gtk4")]
pub mod notify;

pub const NAME: &'static str = "com.system76.UserColorEditor";
pub const THEME_DIR: &'static str = "color-overrides";
