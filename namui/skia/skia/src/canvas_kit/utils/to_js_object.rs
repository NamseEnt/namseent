use canvas_kit_wasm_bindgen::*;

pub(crate) trait ToJsObject {
    fn to_js_object(&self) -> js_sys::Object;
}

impl ToJsObject for crate::ImageInfo {
    fn to_js_object(&self) -> js_sys::Object {
        let obj = js_sys::Object::new();

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

        let canvas_kit_color_type: &CanvasKitColorType = self.color_type.into();
        js_sys::Reflect::set(
            &obj,
            &wasm_bindgen::JsValue::from("colorType"),
            &wasm_bindgen::JsValue::from(canvas_kit_color_type),
        )
        .expect("Failed to set colorType");

        let canvas_kit_alpha_type: &CanvasKitAlphaType = self.alpha_type.into();
        js_sys::Reflect::set(
            &obj,
            &wasm_bindgen::JsValue::from("alphaType"),
            &wasm_bindgen::JsValue::from(canvas_kit_alpha_type),
        )
        .expect("Failed to set alphaType");

        obj
    }
}
