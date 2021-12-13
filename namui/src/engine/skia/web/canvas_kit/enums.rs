use lazy_static::lazy_static;
use once_cell::sync::Lazy;

use super::*;

#[wasm_bindgen]
extern "C" {
    pub type EmbindEnumEntity;

    #[wasm_bindgen(method, getter)]
    pub fn value(this: &EmbindEnumEntity) -> f32;
}
#[wasm_bindgen]
extern "C" {
    pub type CanvasKitPaintStyleEnumValues;
    #[wasm_bindgen(extends = EmbindEnumEntity)]
    pub type CanvasKitPaintStyle;

    #[wasm_bindgen(method, getter)]
    pub fn PaintStyle(this: &CanvasKit) -> CanvasKitPaintStyleEnumValues;

    #[wasm_bindgen(method, getter)]
    pub fn Fill(this: &CanvasKitPaintStyleEnumValues) -> CanvasKitPaintStyle;

    #[wasm_bindgen(method, getter)]
    pub fn Stroke(this: &CanvasKitPaintStyleEnumValues) -> CanvasKitPaintStyle;
}
#[wasm_bindgen]
extern "C" {
    pub type CanvasKitStrokeCapEnumValues;
    #[wasm_bindgen(extends = EmbindEnumEntity)]
    pub type CanvasKitStrokeCap;

    #[wasm_bindgen(method, getter)]
    pub fn StrokeCap(this: &CanvasKit) -> CanvasKitStrokeCapEnumValues;

    #[wasm_bindgen(method, getter)]
    pub fn Butt(this: &CanvasKitStrokeCapEnumValues) -> CanvasKitStrokeCap;
    #[wasm_bindgen(method, getter)]
    pub fn Round(this: &CanvasKitStrokeCapEnumValues) -> CanvasKitStrokeCap;
    #[wasm_bindgen(method, getter)]
    pub fn Square(this: &CanvasKitStrokeCapEnumValues) -> CanvasKitStrokeCap;
}
#[wasm_bindgen]
extern "C" {
    pub type CanvasKitStrokeJoinEnumValues;
    #[wasm_bindgen(extends = EmbindEnumEntity)]
    pub type CanvasKitStrokeJoin;

    #[wasm_bindgen(method, getter)]
    pub fn StrokeJoin(this: &CanvasKit) -> CanvasKitStrokeJoinEnumValues;

    #[wasm_bindgen(method, getter)]
    pub fn Bevel(this: &CanvasKitStrokeJoinEnumValues) -> CanvasKitStrokeJoin;
    #[wasm_bindgen(method, getter)]
    pub fn Miter(this: &CanvasKitStrokeJoinEnumValues) -> CanvasKitStrokeJoin;
    #[wasm_bindgen(method, getter)]
    pub fn Round(this: &CanvasKitStrokeJoinEnumValues) -> CanvasKitStrokeJoin;
}
#[wasm_bindgen]
extern "C" {
    pub type CanvasKitClipOpEnumValues;
    #[wasm_bindgen(extends = EmbindEnumEntity)]
    pub type CanvasKitClipOp;

    #[wasm_bindgen(method, getter)]
    pub fn ClipOp(this: &CanvasKit) -> CanvasKitClipOpEnumValues;

    #[wasm_bindgen(method, getter)]
    pub fn Difference(this: &CanvasKitClipOpEnumValues) -> CanvasKitClipOp;
    #[wasm_bindgen(method, getter)]
    pub fn Intersect(this: &CanvasKitClipOpEnumValues) -> CanvasKitClipOp;
}
lazy_static! {
    pub static ref PAINT_STYLE_FILL_VALUE: Lazy<f32> =
        Lazy::new(|| canvas_kit().PaintStyle().Fill().value());
    pub static ref PAINT_STYLE_STROKE_VALUE: Lazy<f32> =
        Lazy::new(|| canvas_kit().PaintStyle().Stroke().value());
    pub static ref STROKE_JOIN_BEVEL_VALUE: Lazy<f32> =
        Lazy::new(|| canvas_kit().StrokeJoin().Bevel().value());
    pub static ref STROKE_JOIN_MITER_VALUE: Lazy<f32> =
        Lazy::new(|| canvas_kit().StrokeJoin().Miter().value());
    pub static ref STROKE_JOIN_ROUND_VALUE: Lazy<f32> =
        Lazy::new(|| canvas_kit().StrokeJoin().Round().value());
    pub static ref STROKE_CAP_BUTT_VALUE: Lazy<f32> =
        Lazy::new(|| canvas_kit().StrokeCap().Butt().value());
    pub static ref STROKE_CAP_ROUND_VALUE: Lazy<f32> =
        Lazy::new(|| canvas_kit().StrokeCap().Round().value());
    pub static ref STROKE_CAP_SQUARE_VALUE: Lazy<f32> =
        Lazy::new(|| canvas_kit().StrokeCap().Square().value());
    pub static ref CLIP_OP_DIFFERENCE_VALUE: Lazy<f32> =
        Lazy::new(|| canvas_kit().ClipOp().Difference().value());
    pub static ref CLIP_OP_INTERSECT_VALUE: Lazy<f32> =
        Lazy::new(|| canvas_kit().ClipOp().Intersect().value());
}
