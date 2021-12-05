use super::*;

#[wasm_bindgen]
extern "C" {
    pub type CanvasKitPaintStyleEnumValues;
    pub type CanvasKitPaintStyle;

    #[wasm_bindgen(method, getter)]
    pub fn Fill(this: &CanvasKitPaintStyleEnumValues) -> CanvasKitPaintStyle;

    #[wasm_bindgen(method, getter)]
    pub fn Stroke(this: &CanvasKitPaintStyleEnumValues) -> CanvasKitPaintStyle;
}
