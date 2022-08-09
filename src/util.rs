// SPDX-License-Identifier: MPL-2.0-only

use gtk4::gdk::RGBA;
use hex::encode;
// use kmeans_colors::{get_kmeans_hamerly, Kmeans, Sort};
use palette::{rgb::Srgba, Pixel};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct SRGBA(pub Srgba);

pub fn hex_from_rgba(rgba: RGBA) -> String {
    let c = SRGBA::from(rgba);
    let hex = encode::<[u8; 4]>(Srgba::into_raw(c.0.into_format()));
    format!("#{hex}")
}

impl SRGBA {
    pub fn into_inner(self) -> Srgba {
        self.0
    }
}
impl From<Srgba> for SRGBA {
    fn from(c: Srgba) -> Self {
        Self(c)
    }
}
impl From<RGBA> for SRGBA {
    fn from(rgba: RGBA) -> Self {
        Self(Srgba::new(
            rgba.red(),
            rgba.green(),
            rgba.blue(),
            rgba.alpha(),
        ))
    }
}

impl Into<RGBA> for SRGBA {
    fn into(self) -> RGBA {
        RGBA::new(self.0.red, self.0.green, self.0.blue, self.0.alpha)
    }
}

impl Into<Srgba> for SRGBA {
    fn into(self) -> Srgba {
        self.0
    }
}
