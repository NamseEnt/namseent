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

/// https://android.googlesource.com/platform/frameworks/base/+/41fceb4/libs/hwui/utils/Blur.cpp
/// This constant approximates the scaling done in the software path's
/// "high quality" mode, in SkBlurMask::Blur() (1 / sqrt(3)).
const BLUR_SIGMA_SCALE: f32 = 0.57735;

impl BlurStyle {
    pub fn convert_radius_to_sigma(radius: f32) -> f32 {
        if radius <= 0.0 {
            return 0.0;
        }
        radius * BLUR_SIGMA_SCALE + 0.5
    }

    pub fn convert_sigma_to_radius(sigma: f32) -> f32 {
        if sigma <= 0.5 {
            return 0.0;
        }
        (sigma - 0.5) / BLUR_SIGMA_SCALE
    }
}
