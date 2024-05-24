use crate::*;

#[type_derives(Copy)]
pub enum MaskFilter {
    Blur { blur_style: BlurStyle, sigma: f32 },
}

#[type_derives(Copy)]
pub enum BlurStyle {
    /// Fuzzy inside and outside
    Normal,
    /// Solid inside, fuzzy outside
    Solid,
    /// Nothing inside, fuzzy outside
    Outer,
    /// Fuzzy inside, nothing outside
    Inner,
}

#[cfg(feature = "skia")]
impl From<BlurStyle> for skia_safe::BlurStyle {
    fn from(blur_style: BlurStyle) -> Self {
        match blur_style {
            BlurStyle::Normal => skia_safe::BlurStyle::Normal,
            BlurStyle::Solid => skia_safe::BlurStyle::Solid,
            BlurStyle::Outer => skia_safe::BlurStyle::Outer,
            BlurStyle::Inner => skia_safe::BlurStyle::Inner,
        }
    }
}

/// https://android.googlesource.com/platform/frameworks/base/+/41fceb4/libs/hwui/utils/Blur.cpp
/// This constant approximates the scaling done in the software path's
/// "high quality" mode, in SkBlurMask::Blur() (1 / sqrt(3)).

pub mod blur_sigma {
    const BLUR_SIGMA_SCALE: f32 = 0.57735;

    pub fn from_radius(radius: f32) -> f32 {
        if radius <= 0.0 {
            return 0.0;
        }
        radius * BLUR_SIGMA_SCALE + 0.5
    }

    pub fn to_radius(sigma: f32) -> f32 {
        if sigma <= 0.5 {
            return 0.0;
        }
        (sigma - 0.5) / BLUR_SIGMA_SCALE
    }
}
