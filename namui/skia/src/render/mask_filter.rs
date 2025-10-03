use std::hash::Hash;

#[derive(Debug, Clone, Copy, bincode::Encode, bincode::Decode)]
pub enum MaskFilter {
    Blur { blur_style: BlurStyle, sigma: f32 },
}
impl Hash for MaskFilter {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            MaskFilter::Blur { blur_style, sigma } => {
                blur_style.hash(state);
                sigma.to_bits().hash(state);
            }
        }
    }
}
impl PartialEq for MaskFilter {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                MaskFilter::Blur {
                    blur_style: blur_style1,
                    sigma: sigma1,
                },
                MaskFilter::Blur {
                    blur_style: blur_style2,
                    sigma: sigma2,
                },
            ) => blur_style1 == blur_style2 && sigma1.to_bits() == sigma2.to_bits(),
        }
    }
}
impl Eq for MaskFilter {}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq, bincode::Encode, bincode::Decode)]
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
