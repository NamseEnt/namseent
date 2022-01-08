use super::*;
use crate::namui;
use wasm_bindgen::JsValue;

unsafe impl Sync for CanvasKitSurface {}
unsafe impl Send for CanvasKitSurface {}
pub(crate) struct Surface {
    canvas_kit_surface: CanvasKitSurface,
    canvas: Canvas,
}
impl Surface {
    pub fn new(canvas_kit_surface: CanvasKitSurface) -> Surface {
        let canvas = canvas_kit_surface.getCanvas();
        Surface {
            canvas_kit_surface,
            canvas: Canvas(canvas),
        }
    }
    pub fn flush(&self) {
        self.canvas_kit_surface.flush();
    }
    pub fn canvas(&self) -> &Canvas {
        &self.canvas
    }

    pub fn make_image_from_texture_source(
        &self,
        texture_source: &JsValue,
        width: Option<f64>,
        height: Option<f64>,
    ) -> Option<Image> {
        let partial_image_info = js_sys::Object::new();
        js_sys::Reflect::set(
            &partial_image_info,
            &"alphaType".into(),
            &AlphaType::Unpremul.into_canvas_kit().into(),
        )
        .unwrap();
        js_sys::Reflect::set(
            &partial_image_info,
            &"colorType".into(),
            &ColorType::Rgba8888.into_canvas_kit().into(),
        )
        .unwrap();
        js_sys::Reflect::set(
            &partial_image_info,
            &"colorSpace".into(),
            &ColorSpace::Srgb.into_canvas_kit().into(),
        )
        .unwrap();
        js_sys::Reflect::set(&partial_image_info, &"width".into(), &width.into()).unwrap();
        js_sys::Reflect::set(&partial_image_info, &"height".into(), &height.into()).unwrap();
        self.canvas_kit_surface
            .makeImageFromTextureSource(&texture_source, partial_image_info.into())
            .map(|canvas_kit_image| Image::from(canvas_kit_image))
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        self.canvas_kit_surface.delete();
    }
}
