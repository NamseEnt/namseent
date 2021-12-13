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
