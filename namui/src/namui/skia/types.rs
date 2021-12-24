use serde::{Deserialize, Serialize};

pub type GlyphIds = [u16];

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Default)]
#[repr(C)]
pub struct LtrbRect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FontMetrics {
    /// suggested space above the baseline. < 0
    pub ascent: f32,
    /// suggested space below the baseline. > 0
    pub descent: f32,
    /// suggested spacing between descent of previous line and ascent of next line.
    pub leading: f32,
    /// smallest rect containing all glyphs (relative to 0,0)
    pub bounds: Option<LtrbRect>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Default)]
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
    pub fn gary_scale_f01(value: f32) -> Color {
        Color::from_f01(value, value, value, 1.0)
    }
}

pub enum PaintStyle {
    Fill,
    Stroke,
}

pub enum StrokeCap {
    Butt,
    Round,
    Square,
}

pub enum StrokeJoin {
    Bevel,
    Miter,
    Round,
}
pub struct StrokeOptions {
    pub width: Option<f32>,
    pub miter_limit: Option<f32>,
    pub precision: Option<f32>,
    pub join: Option<StrokeJoin>,
    pub cap: Option<StrokeCap>,
}

#[derive(Serialize, Clone)]
pub enum ClipOp {
    Intersect,
    Difference,
}

pub enum AlphaType {
    Opaque,
    Premul,
    Unpremul,
}
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
}
pub struct PartialImageInfo {
    pub alphaType: AlphaType,
    pub colorType: ColorType,
    pub height: f32,
    pub width: f32,
}
pub enum FilterMode {
    Linear,
    Nearest,
}
pub enum MipmapMode {
    None,
    Nearest,
    Linear,
}
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
