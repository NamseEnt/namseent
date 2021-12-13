use super::*;
use crate::namui::{self, Xy};
pub use base::*;
use serde::Serialize;
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen::JsValue;

#[derive(Serialize)]
pub struct Path {
    id: String,
    #[serde(skip)]
    pub canvas_kit_path: CanvasKitPath,
}

static PATH_ID: AtomicUsize = AtomicUsize::new(0);

impl Path {
    pub fn new() -> Self {
        let canvas_kit_path = CanvasKitPath::new();
        Path::from_canvas_kit_path(canvas_kit_path)
    }
    fn from_canvas_kit_path(canvas_kit_path: CanvasKitPath) -> Self {
        let id = format!("path-{}", PATH_ID.fetch_add(1, Ordering::Relaxed));
        namui::log(format!("Path::from_canvas_kit_path: id={}", id));
        Path {
            id,
            canvas_kit_path,
        }
    }
    pub fn add_rect(
        self,
        LtrbRect {
            left,
            top,
            right,
            bottom,
        }: LtrbRect,
    ) -> Self {
        let array = js_sys::Float32Array::new_with_length(4);
        array.set_index(0, left as f32);
        array.set_index(1, top as f32);
        array.set_index(2, right as f32);
        array.set_index(3, bottom as f32);

        self.canvas_kit_path.addRect(array, None);

        self
    }
    pub fn add_rrect(
        self,
        LtrbRect {
            left,
            top,
            right,
            bottom,
        }: LtrbRect,
        rx: f32,
        ry: f32,
    ) -> Self {
        let rect = js_sys::Float32Array::new_with_length(4);
        rect.set_index(0, left as f32);
        rect.set_index(1, top as f32);
        rect.set_index(2, right as f32);
        rect.set_index(3, bottom as f32);
        let rrect = canvas_kit().RRectXY(rect, rx, ry);
        self.canvas_kit_path.addRRect(rrect, None);

        self
    }
    pub fn contains(&self, xy: &Xy<f32>) -> bool {
        self.canvas_kit_path.contains(xy.x, xy.y)
    }
    pub fn stroke(&self, options: Option<StrokeOptions>) -> Result<(), ()> {
        let js_option = match options {
            Some(options) => {
                let js_options = js_sys::Object::new();
                if let Some(width) = options.width {
                    js_sys::Reflect::set(&js_options, &"width".into(), &width.into());
                }
                if let Some(miter_limit) = options.miter_limit {
                    js_sys::Reflect::set(&js_options, &"miterLimit".into(), &miter_limit.into());
                }
                if let Some(precision) = options.precision {
                    js_sys::Reflect::set(&js_options, &"precision".into(), &precision.into());
                }
                if let Some(join) = options.join {
                    let canvas_kit_stroke_join = match join {
                        StrokeJoin::Bevel => canvas_kit().StrokeJoin().Bevel(),
                        StrokeJoin::Miter => canvas_kit().StrokeJoin().Miter(),
                        StrokeJoin::Round => canvas_kit().StrokeJoin().Round(),
                    };
                    js_sys::Reflect::set(&js_options, &"join".into(), &canvas_kit_stroke_join);
                }
                if let Some(cap) = options.cap {
                    let canvas_kit_stroke_cap = match cap {
                        StrokeCap::Butt => canvas_kit().StrokeCap().Butt(),
                        StrokeCap::Round => canvas_kit().StrokeCap().Round(),
                        StrokeCap::Square => canvas_kit().StrokeCap().Square(),
                    };
                    js_sys::Reflect::set(&js_options, &"cap".into(), &canvas_kit_stroke_cap);
                }
                js_options.into()
            }
            None => JsValue::undefined(),
        };
        let result = self.canvas_kit_path.stroke(js_option);
        if result == JsValue::undefined() {
            Err(())
        } else {
            Ok(())
        }
    }
}

impl Drop for Path {
    fn drop(&mut self) {
        namui::log(format!("Dropping Path {}", self.id));
        self.canvas_kit_path.delete();
    }
}

impl std::fmt::Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Clone for Path {
    fn clone(&self) -> Self {
        namui::log(format!("Cloning Path {}", self.id));
        let canvas_kit_path = self.canvas_kit_path.copy();
        Path::from_canvas_kit_path(canvas_kit_path)
    }
}
