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
        Path {
            id,
            canvas_kit_path,
        }
    }
    pub fn add_rect(self, ltrb_rect: &LtrbRect) -> Self {
        self.canvas_kit_path.addRect(
            &[
                ltrb_rect.left,
                ltrb_rect.top,
                ltrb_rect.right,
                ltrb_rect.bottom,
            ],
            None,
        );

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
                    js_sys::Reflect::set(&js_options, &"join".into(), &join.into_canvas_kit());
                }
                if let Some(cap) = options.cap {
                    js_sys::Reflect::set(&js_options, &"cap".into(), &cap.into_canvas_kit());
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
    pub fn move_to(self, x: f32, y: f32) -> Self {
        self.canvas_kit_path.moveTo(x, y);
        self
    }
    pub fn line_to(self, x: f32, y: f32) -> Self {
        self.canvas_kit_path.lineTo(x, y);
        self
    }
    pub fn scale(self, x: f32, y: f32) -> Self {
        self.transform(&[x, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, 1.0])
    }
    pub fn translate(self, x: f32, y: f32) -> Self {
        self.canvas_kit_path.offset(x, y);
        self
    }
    pub fn transform(self, matrix_3x3: &[f32; 9]) -> Self {
        self.canvas_kit_path.transform(matrix_3x3);
        self
    }
    pub fn add_oval(self, ltrb_rect: &LtrbRect) -> Self {
        self.canvas_kit_path.addOval(
            &[
                ltrb_rect.left,
                ltrb_rect.top,
                ltrb_rect.right,
                ltrb_rect.bottom,
            ],
            None,
            None,
        );
        self
    }
    pub fn add_poly(self, xy_array: &[Xy<f32>], close: bool) -> Self {
        let array = &xy_array
            .iter()
            .flat_map(|xy| vec![xy.x, xy.y])
            .collect::<Vec<f32>>();
        self.canvas_kit_path.addPoly(array, close);
        self
    }
    pub fn close(self) -> Self {
        self.canvas_kit_path.close();
        self
    }
}

impl Drop for Path {
    fn drop(&mut self) {
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
        let canvas_kit_path = self.canvas_kit_path.copy();
        Path::from_canvas_kit_path(canvas_kit_path)
    }
}
