use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

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

#[derive(Debug, Deserialize, Serialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub enum PaintStyle {
    Fill,
    Stroke,
}
