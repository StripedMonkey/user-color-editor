// SPDX-License-Identifier: MPL-2.0-only

pub mod colors;
pub mod config;
#[cfg(feature = "notify")]
pub mod notify;

pub const NAME: &str = "com.system76.UserColorEditor";
pub const THEME_DIR: &str = "color-overrides";
