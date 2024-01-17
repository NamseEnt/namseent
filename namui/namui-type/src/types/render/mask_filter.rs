use crate::*;

#[type_derives(Copy)]
pub enum MaskFilter {
    Blur { blur: Blur },
}

#[type_derives(Copy)]
pub enum Blur {
    /// Fuzzy inside and outside
    Normal { sigma: f32 },
    /// Solid inside, fuzzy outside
    Solid { sigma: f32 },
    /// Nothing inside, fuzzy outside
    Outer { sigma: f32 },
    /// Fuzzy inside, nothing outside
    Inner { sigma: f32 },
}

/// https://android.googlesource.com/platform/frameworks/base/+/41fceb4/libs/hwui/utils/Blur.cpp
/// This constant approximates the scaling done in the software path's
/// "high quality" mode, in SkBlurMask::Blur() (1 / sqrt(3)).
const BLUR_SIGMA_SCALE: f32 = 0.57735;

impl Blur {
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
