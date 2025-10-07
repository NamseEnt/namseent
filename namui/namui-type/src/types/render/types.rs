use crate::*;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct FontMetrics {
    /// suggested space above the baseline. < 0
    pub ascent: Px,
    /// suggested space below the baseline. > 0
    pub descent: Px,
    /// suggested spacing between descent of previous line and ascent of next line.
    pub leading: Px,
}

impl FontMetrics {
    pub fn height(&self) -> Px {
        -self.ascent + self.descent
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Default, Hash, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Color {
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const TRANSPARENT: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    pub const RED: Color = Color {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const GREEN: Color = Color {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const BLUE: Color = Color {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };
    pub fn from_f01(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color {
            r: (r * 255.0) as u8,
            g: (g * 255.0) as u8,
            b: (b * 255.0) as u8,
            a: (a * 255.0) as u8,
        }
    }
    pub const fn from_u8(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
    pub fn grayscale_f01(value: f32) -> Color {
        Color::from_f01(value, value, value, 1.0)
    }
    pub fn grayscale_alpha_f01(value: f32, alpha: f32) -> Color {
        Color::from_f01(value, value, value, alpha)
    }
    pub const fn grayscale_u8(value: u8) -> Color {
        Color::from_u8(value, value, value, 255)
    }
    pub fn from_string_for_random_color(value: &str, is_random_alpha: bool) -> Self {
        let mut hasher = DefaultHasher::default();
        value.hash(&mut hasher);
        let hash = hasher.finish();
        Self::from_u8(
            ((hash >> 24) & 0xff) as u8,
            ((hash >> 16) & 0xff) as u8,
            ((hash >> 8) & 0xff) as u8,
            if is_random_alpha {
                (hash & 0xff) as u8
            } else {
                255
            },
        )
    }
    pub fn brighter(&self, value: f32) -> Self {
        let Hsl01 {
            hue,
            saturation,
            lightness,
            alpha,
        } = self.as_hsl01();

        Self::from_hsl01(Hsl01 {
            hue,
            saturation: num::clamp(saturation - value, 0.0, 1.0),
            lightness: num::clamp(lightness + value, 0.0, 1.0),
            alpha,
        })
    }

    fn as_hsl01(&self) -> Hsl01 {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let hue = if delta == 0.0 {
            0.0
        } else {
            60.0 * match max {
                value if value == r => (g - b) / delta,
                value if value == g => (b - r) / delta + 2.0,
                value if value == b => (r - g) / delta + 4.0,
                _ => unreachable!(),
            }
        };

        let lightness = (max + min) / 2.0;

        let saturation = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * lightness - 1.0).abs())
        };

        Hsl01 {
            hue,
            saturation,
            lightness,
            alpha: self.a as f32 / 255.0,
        }
    }

    fn from_hsl01(hsl: Hsl01) -> Self {
        let Hsl01 {
            hue,
            saturation,
            lightness,
            alpha,
        } = hsl;

        let hue = hue % 360.0;
        let hue_stage = hue / 60.0;
        let primary_chroma = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
        let secondary_chroma = primary_chroma * (1.0 - (hue_stage % 2.0).abs());
        let (base_r, base_g, base_b) = match hue_stage {
            x if x < 1.0 => (primary_chroma, secondary_chroma, 0.0),
            x if x < 2.0 => (secondary_chroma, primary_chroma, 0.0),
            x if x < 3.0 => (0.0, primary_chroma, secondary_chroma),
            x if x < 4.0 => (0.0, secondary_chroma, primary_chroma),
            x if x < 5.0 => (secondary_chroma, 0.0, primary_chroma),
            x if x < 6.0 => (primary_chroma, 0.0, secondary_chroma),
            _ => (0.0, 0.0, 0.0),
        };
        let lightness_factor = lightness - primary_chroma / 2.0;
        Color::from_f01(
            base_r + lightness_factor,
            base_g + lightness_factor,
            base_b + lightness_factor,
            alpha,
        )
    }

    pub const fn with_alpha(mut self, alpha: u8) -> Self {
        self.a = alpha;
        self
    }
}

impl From<Color> for skia_safe::Color4f {
    fn from(color: Color) -> Self {
        skia_safe::Color4f::from_bytes_rgba(u32::from_le_bytes([
            color.r, color.g, color.b, color.a,
        ]))
    }
}

impl From<Color> for skia_safe::Color {
    fn from(color: Color) -> Self {
        skia_safe::Color::from_argb(color.a, color.r, color.g, color.b)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Hsl01 {
    hue: f32,
    saturation: f32,
    lightness: f32,
    alpha: f32,
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum PaintStyle {
    Fill,
    Stroke,
}

impl From<PaintStyle> for skia_safe::PaintStyle {
    fn from(paint_style: PaintStyle) -> Self {
        match paint_style {
            PaintStyle::Fill => skia_safe::PaintStyle::Fill,
            PaintStyle::Stroke => skia_safe::PaintStyle::Stroke,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum StrokeCap {
    Butt,
    Round,
    Square,
}

impl From<StrokeCap> for skia_safe::PaintCap {
    fn from(stroke_cap: StrokeCap) -> Self {
        match stroke_cap {
            StrokeCap::Butt => skia_safe::PaintCap::Butt,
            StrokeCap::Round => skia_safe::PaintCap::Round,
            StrokeCap::Square => skia_safe::PaintCap::Square,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum StrokeJoin {
    Bevel,
    Miter,
    Round,
}

impl From<StrokeJoin> for skia_safe::PaintJoin {
    fn from(stroke_join: StrokeJoin) -> Self {
        match stroke_join {
            StrokeJoin::Bevel => skia_safe::PaintJoin::Bevel,
            StrokeJoin::Miter => skia_safe::PaintJoin::Miter,
            StrokeJoin::Round => skia_safe::PaintJoin::Round,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub struct StrokeOptions {
    pub width: Option<Px>,
    pub miter_limit: Option<Px>,
    ///
    /// if > 1, increase precision, else if (0 < resScale < 1) reduce precision to
    /// favor speed and size
    ///
    pub precision: Option<OrderedFloat>,
    pub join: Option<StrokeJoin>,
    pub cap: Option<StrokeCap>,
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum ClipOp {
    Intersect,
    Difference,
}

impl From<ClipOp> for skia_safe::ClipOp {
    fn from(clip_op: ClipOp) -> Self {
        match clip_op {
            ClipOp::Intersect => skia_safe::ClipOp::Intersect,
            ClipOp::Difference => skia_safe::ClipOp::Difference,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Hash)]
pub enum AlphaType {
    Opaque,
    Premul,
    Unpremul,
    // Unknown, // not support by canvaskit
}

impl From<skia_safe::AlphaType> for AlphaType {
    fn from(val: skia_safe::AlphaType) -> Self {
        match val {
            skia_safe::AlphaType::Opaque => AlphaType::Opaque,
            skia_safe::AlphaType::Premul => AlphaType::Premul,
            skia_safe::AlphaType::Unpremul => AlphaType::Unpremul,
            skia_safe::AlphaType::Unknown => {
                unimplemented!("canvaskit doesn't support AlphaType::Unknown")
            }
        }
    }
}

impl From<AlphaType> for skia_safe::AlphaType {
    fn from(val: AlphaType) -> Self {
        match val {
            AlphaType::Opaque => skia_safe::AlphaType::Opaque,
            AlphaType::Premul => skia_safe::AlphaType::Premul,
            AlphaType::Unpremul => skia_safe::AlphaType::Unpremul,
            // AlphaType::Unknown => skia_safe::AlphaType::Unknown,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ColorType {
    Alpha8,
    Rgb565,
    Rgba8888,
    Bgra8888,
    Rgba1010102,
    Rgb101010x,
    Gray8,
    RgbaF16,
    RgbaF32,
    // not support by canvaskit
    // ARGB4444,
    // RGB888x,
    // BGRA1010102,
    // BGR101010x,
    // BGR101010xXR,
    // RGBA10x6,
    // RGBAF16Norm,
    // R8G8UNorm,
    // A16Float,
    // R16G16Float,
    // A16UNorm,
    // R16G16UNorm,
    // R16G16B16A16UNorm,
    // SRGBA8888,
    // R8UNorm,
    // Unknown,
}

impl ColorType {
    pub fn word(&self) -> usize {
        match self {
            ColorType::Alpha8 => 1,
            ColorType::Rgb565 => 2,
            ColorType::Rgba8888 => 4,
            ColorType::Bgra8888 => 4,
            ColorType::Rgba1010102 => 4,
            ColorType::Rgb101010x => 4,
            ColorType::Gray8 => 1,
            ColorType::RgbaF16 => 8,
            ColorType::RgbaF32 => 16,
            // ColorType::ARGB4444 => 2,
            // ColorType::RGB888x => 4,
            // ColorType::BGRA1010102 => 4,
            // ColorType::BGR101010x => 4,
            // ColorType::BGR101010xXR => 4,
            // ColorType::RGBA10x6 => 8,
            // ColorType::RGBAF16Norm => 8,
            // ColorType::R8G8UNorm => 2,
            // ColorType::A16Float => 2,
            // ColorType::R16G16Float => 4,
            // ColorType::A16UNorm => 2,
            // ColorType::R16G16UNorm => 4,
            // ColorType::R16G16B16A16UNorm => 8,
            // ColorType::SRGBA8888 => 4,
            // ColorType::R8UNorm => 1,
            // ColorType::Unknown => unreachable!(),
        }
    }
}

impl From<skia_safe::ColorType> for ColorType {
    fn from(val: skia_safe::ColorType) -> Self {
        match val {
            skia_safe::ColorType::Alpha8 => ColorType::Alpha8,
            skia_safe::ColorType::RGB565 => ColorType::Rgb565,
            skia_safe::ColorType::RGBA8888 => ColorType::Rgba8888,
            skia_safe::ColorType::BGRA8888 => ColorType::Bgra8888,
            skia_safe::ColorType::RGBA1010102 => ColorType::Rgba1010102,
            skia_safe::ColorType::RGB101010x => ColorType::Rgb101010x,
            skia_safe::ColorType::Gray8 => ColorType::Gray8,
            skia_safe::ColorType::RGBAF16 => ColorType::RgbaF16,
            skia_safe::ColorType::RGBAF32 => ColorType::RgbaF32,
            _ => unimplemented!(),
            // skia_safe::ColorType::ARGB4444 => ColorType::ARGB4444,
            // skia_safe::ColorType::RGB888x => ColorType::RGB888x,
            // skia_safe::ColorType::BGRA1010102 => ColorType::BGRA1010102,
            // skia_safe::ColorType::BGR101010x => ColorType::BGR101010x,
            // skia_safe::ColorType::BGR101010xXR => ColorType::BGR101010xXR,
            // skia_safe::ColorType::RGBA10x6 => ColorType::RGBA10x6,
            // skia_safe::ColorType::RGBAF16Norm => ColorType::RGBAF16Norm,
            // skia_safe::ColorType::R8G8UNorm => ColorType::R8G8UNorm,
            // skia_safe::ColorType::A16Float => ColorType::A16Float,
            // skia_safe::ColorType::R16G16Float => ColorType::R16G16Float,
            // skia_safe::ColorType::A16UNorm => ColorType::A16UNorm,
            // skia_safe::ColorType::R16G16UNorm => ColorType::R16G16UNorm,
            // skia_safe::ColorType::R16G16B16A16UNorm => ColorType::R16G16B16A16UNorm,
            // skia_safe::ColorType::SRGBA8888 => ColorType::SRGBA8888,
            // skia_safe::ColorType::R8UNorm => ColorType::R8UNorm,
            // skia_safe::ColorType::Unknown => ColorType::Unknown,
        }
    }
}

impl From<ColorType> for skia_safe::ColorType {
    fn from(val: ColorType) -> Self {
        match val {
            ColorType::Alpha8 => skia_safe::ColorType::Alpha8,
            ColorType::Rgb565 => skia_safe::ColorType::RGB565,
            ColorType::Rgba8888 => skia_safe::ColorType::RGBA8888,
            ColorType::Bgra8888 => skia_safe::ColorType::BGRA8888,
            ColorType::Rgba1010102 => skia_safe::ColorType::RGBA1010102,
            ColorType::Rgb101010x => skia_safe::ColorType::RGB101010x,
            ColorType::Gray8 => skia_safe::ColorType::Gray8,
            ColorType::RgbaF16 => skia_safe::ColorType::RGBAF16,
            ColorType::RgbaF32 => skia_safe::ColorType::RGBAF32,
            // ColorType::ARGB4444 => skia_safe::ColorType::ARGB4444,
            // ColorType::RGB888x => skia_safe::ColorType::RGB888x,
            // ColorType::BGRA1010102 => skia_safe::ColorType::BGRA1010102,
            // ColorType::BGR101010x => skia_safe::ColorType::BGR101010x,
            // ColorType::BGR101010xXR => skia_safe::ColorType::BGR101010xXR,
            // ColorType::RGBA10x6 => skia_safe::ColorType::RGBA10x6,
            // ColorType::RGBAF16Norm => skia_safe::ColorType::RGBAF16Norm,
            // ColorType::R8G8UNorm => skia_safe::ColorType::R8G8UNorm,
            // ColorType::A16Float => skia_safe::ColorType::A16Float,
            // ColorType::R16G16Float => skia_safe::ColorType::R16G16Float,
            // ColorType::A16UNorm => skia_safe::ColorType::A16UNorm,
            // ColorType::R16G16UNorm => skia_safe::ColorType::R16G16UNorm,
            // ColorType::R16G16B16A16UNorm => skia_safe::ColorType::R16G16B16A16UNorm,
            // ColorType::SRGBA8888 => skia_safe::ColorType::SRGBA8888,
            // ColorType::R8UNorm => skia_safe::ColorType::R8UNorm,
            // ColorType::Unknown => skia_safe::ColorType::Unknown,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FilterMode {
    Linear,
    Nearest,
}

impl From<FilterMode> for skia_safe::FilterMode {
    fn from(filter_mode: FilterMode) -> Self {
        match filter_mode {
            FilterMode::Linear => skia_safe::FilterMode::Linear,
            FilterMode::Nearest => skia_safe::FilterMode::Nearest,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MipmapMode {
    None,
    Nearest,
    Linear,
}

impl From<MipmapMode> for skia_safe::MipmapMode {
    fn from(mipmap_mode: MipmapMode) -> Self {
        match mipmap_mode {
            MipmapMode::None => skia_safe::MipmapMode::None,
            MipmapMode::Nearest => skia_safe::MipmapMode::Nearest,
            MipmapMode::Linear => skia_safe::MipmapMode::Linear,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum BlendMode {
    Clear,
    Src,
    Dst,
    SrcOver,
    DstOver,
    SrcIn,
    DstIn,
    SrcOut,
    DstOut,
    SrcATop,
    DstATop,
    Xor,
    Plus,
    Modulate,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Multiply,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

impl From<BlendMode> for skia_safe::BlendMode {
    fn from(blend_mode: BlendMode) -> Self {
        match blend_mode {
            BlendMode::Clear => skia_safe::BlendMode::Clear,
            BlendMode::Src => skia_safe::BlendMode::Src,
            BlendMode::Dst => skia_safe::BlendMode::Dst,
            BlendMode::SrcOver => skia_safe::BlendMode::SrcOver,
            BlendMode::DstOver => skia_safe::BlendMode::DstOver,
            BlendMode::SrcIn => skia_safe::BlendMode::SrcIn,
            BlendMode::DstIn => skia_safe::BlendMode::DstIn,
            BlendMode::SrcOut => skia_safe::BlendMode::SrcOut,
            BlendMode::DstOut => skia_safe::BlendMode::DstOut,
            BlendMode::SrcATop => skia_safe::BlendMode::SrcATop,
            BlendMode::DstATop => skia_safe::BlendMode::DstATop,
            BlendMode::Xor => skia_safe::BlendMode::Xor,
            BlendMode::Plus => skia_safe::BlendMode::Plus,
            BlendMode::Modulate => skia_safe::BlendMode::Modulate,
            BlendMode::Screen => skia_safe::BlendMode::Screen,
            BlendMode::Overlay => skia_safe::BlendMode::Overlay,
            BlendMode::Darken => skia_safe::BlendMode::Darken,
            BlendMode::Lighten => skia_safe::BlendMode::Lighten,
            BlendMode::ColorDodge => skia_safe::BlendMode::ColorDodge,
            BlendMode::ColorBurn => skia_safe::BlendMode::ColorBurn,
            BlendMode::HardLight => skia_safe::BlendMode::HardLight,
            BlendMode::SoftLight => skia_safe::BlendMode::SoftLight,
            BlendMode::Difference => skia_safe::BlendMode::Difference,
            BlendMode::Exclusion => skia_safe::BlendMode::Exclusion,
            BlendMode::Multiply => skia_safe::BlendMode::Multiply,
            BlendMode::Hue => skia_safe::BlendMode::Hue,
            BlendMode::Saturation => skia_safe::BlendMode::Saturation,
            BlendMode::Color => skia_safe::BlendMode::Color,
            BlendMode::Luminosity => skia_safe::BlendMode::Luminosity,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
/// Explain: https://developer.android.com/reference/android/graphics/Shader.TileMode#summary
pub enum TileMode {
    /// Replicate the edge color if the shader draws outside of its original bounds
    Clamp,
    /// Render the shader's image pixels only within its original bounds.
    Decal,
    /// Repeat the shader's image horizontally and vertically, alternating mirror images so that adjacent images always seam
    Mirror,
    /// Repeat the shader's image horizontally and vertically.
    Repeat,
}

impl From<TileMode> for skia_safe::TileMode {
    fn from(tile_mode: TileMode) -> Self {
        match tile_mode {
            TileMode::Clamp => skia_safe::TileMode::Clamp,
            TileMode::Decal => skia_safe::TileMode::Decal,
            TileMode::Mirror => skia_safe::TileMode::Mirror,
            TileMode::Repeat => skia_safe::TileMode::Repeat,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ColorSpace {
    Srgb,
    DisplayP3,
    AdobeRgb,
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum TextBaseline {
    Top,
    Middle,
    Bottom,
}

/// Example: https://developer.mozilla.org/ko/docs/Web/CSS/object-fit
#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum ImageFit {
    /// The replaced content is sized to fill the element's content box.
    /// The entire object will completely fill the box.
    /// If the object's aspect ratio does not match the aspect ratio of its box,
    /// then the object will be stretched to fit.
    Fill,
    /// The replaced content is scaled to maintain its aspect ratio while fitting within the element's content box.
    /// The entire object is made to fill the box, while preserving its aspect ratio, so the object will be letterboxed
    /// if its aspect ratio does not match the aspect ratio of the box.
    Contain,
    /// The replaced content is sized to maintain its aspect ratio while filling the element's entire content box.
    /// If the object's aspect ratio does not match the aspect ratio of its box, then the object will be clipped to fit.
    Cover,
    /// The content is sized as if `none` or `contain` were specified, whichever would result in a smaller concrete object size.
    ScaleDown,
    /// The replaced content is not resized.
    None,
}

pub type GlyphId = skia_safe::GlyphId;
pub type GlyphIds = Vec<GlyphId>;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}
