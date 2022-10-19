use crate::Px;
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub type GlyphIds = Box<[u16]>;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct FontMetrics {
    /// suggested space above the baseline. < 0
    pub ascent: Px,
    /// suggested space below the baseline. > 0
    pub descent: Px,
    /// suggested spacing between descent of previous line and ascent of next line.
    pub leading: Px,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Default, Hash, Eq, PartialEq)]
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
    pub fn from_u8(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
    pub fn grayscale_f01(value: f32) -> Color {
        Color::from_f01(value, value, value, 1.0)
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
                ((hash >> 0) & 0xff) as u8
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
        } = self.into_hsl01();

        Self::from_hsl01(Hsl01 {
            hue,
            saturation: num::clamp(saturation - value, 0.0, 1.0),
            lightness: num::clamp(lightness - value, 0.0, 1.0),
            alpha,
        })
    }

    fn into_hsl01(&self) -> Hsl01 {
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
}

struct Hsl01 {
    hue: f32,
    saturation: f32,
    lightness: f32,
    alpha: f32,
}

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Hash)]
pub enum PaintStyle {
    Fill,
    Stroke,
}

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Hash)]
pub enum StrokeCap {
    Butt,
    Round,
    Square,
}

#[derive(Debug, Serialize, Clone, Copy, PartialEq)]
pub enum StrokeJoin {
    Bevel,
    Miter,
    Round,
}
#[derive(Debug, Serialize, Clone, Copy, PartialEq)]
pub struct StrokeOptions {
    pub width: Option<Px>,
    pub miter_limit: Option<Px>,
    ///
    /// if > 1, increase precision, else if (0 < resScale < 1) reduce precision to
    /// favor speed and size
    ///
    pub precision: Option<f32>,
    pub join: Option<StrokeJoin>,
    pub cap: Option<StrokeCap>,
}

#[derive(Serialize, Clone, Debug)]
pub enum ClipOp {
    Intersect,
    Difference,
}
#[derive(Debug, Serialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AlphaType {
    Opaque,
    Premul,
    Unpremul,
}
#[derive(Debug, Serialize, Clone, Copy, Hash, PartialEq, Eq)]
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
#[derive(Debug, Serialize, Clone, Copy, PartialEq)]
pub struct PartialImageInfo {
    pub alpha_type: AlphaType,
    pub color_type: ColorType,
    pub height: Px,
    pub width: Px,
}
impl PartialImageInfo {
    pub fn into_js_object(&self) -> js_sys::Object {
        let mut obj = js_sys::Object::new();

        js_sys::Reflect::set(
            &obj,
            &wasm_bindgen::JsValue::from("width"),
            &wasm_bindgen::JsValue::from(self.width.as_f32()),
        )
        .expect("Failed to set width");
        js_sys::Reflect::set(
            &obj,
            &wasm_bindgen::JsValue::from("height"),
            &wasm_bindgen::JsValue::from(self.height.as_f32()),
        )
        .expect("Failed to set height");
        js_sys::Reflect::set(
            &obj,
            &wasm_bindgen::JsValue::from("colorType"),
            &wasm_bindgen::JsValue::from(self.color_type.into_canvas_kit()),
        )
        .expect("Failed to set colorType");
        js_sys::Reflect::set(
            &obj,
            &wasm_bindgen::JsValue::from("alphaType"),
            &wasm_bindgen::JsValue::from(self.alpha_type.into_canvas_kit()),
        )
        .expect("Failed to set alphaType");

        obj
    }
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
#[derive(Debug, Serialize, Clone, Copy, Hash, PartialEq, Eq)]
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
pub enum TileMode {
    Clamp,
    Decal,
    Mirror,
    Repeat,
}
