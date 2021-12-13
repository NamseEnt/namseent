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

#[derive(Serialize)]
pub enum ClipOp {
    Intersect,
    Difference,
}
