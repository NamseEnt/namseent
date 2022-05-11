use super::*;
use crate::namui::{self, Xy};
pub use base::*;
use serde::Serialize;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use wasm_bindgen::JsValue;

unsafe impl Sync for CanvasKitPath {}
unsafe impl Send for CanvasKitPath {}
#[derive(Serialize)]
pub(crate) struct Path {
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
    pub fn contains(&self, xy: &Xy<f32>) -> bool {
        self.canvas_kit_path.contains(xy.x, xy.y)
    }
    pub fn get_bounding_box(&self) -> Option<LtrbRect> {
        let bounds = self.canvas_kit_path.getBounds();
        if bounds[0] == 0.0 && bounds[1] == 0.0 && bounds[2] == 0.0 && bounds[3] == 0.0 {
            None
        } else {
            Some(LtrbRect {
                left: bounds[0],
                top: bounds[1],
                right: bounds[2],
                bottom: bounds[3],
            })
        }
    }

    pub(crate) fn add_rect(self, ltrb_rect: &LtrbRect) -> Self {
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
    pub(crate) fn add_rrect(
        self,
        LtrbRect {
            left,
            top,
            right,
            bottom,
        }: &LtrbRect,
        rx: f32,
        ry: f32,
    ) -> Self {
        let rect = js_sys::Float32Array::new_with_length(4);
        rect.set_index(0, *left);
        rect.set_index(1, *top);
        rect.set_index(2, *right);
        rect.set_index(3, *bottom);
        let rrect = canvas_kit().RRectXY(rect, rx, ry);
        self.canvas_kit_path.addRRect(rrect, None);

        self
    }
    pub(crate) fn stroke(&mut self, options: &StrokeOptions) -> Result<(), ()> {
        let js_options = js_sys::Object::new();
        if let Some(width) = options.width {
            js_sys::Reflect::set(&js_options, &"width".into(), &width.into()).unwrap();
        }
        if let Some(miter_limit) = options.miter_limit {
            js_sys::Reflect::set(&js_options, &"miterLimit".into(), &miter_limit.into()).unwrap();
        }
        if let Some(precision) = options.precision {
            js_sys::Reflect::set(&js_options, &"precision".into(), &precision.into()).unwrap();
        }
        if let Some(join) = &options.join {
            js_sys::Reflect::set(&js_options, &"join".into(), &join.into_canvas_kit()).unwrap();
        }
        if let Some(cap) = &options.cap {
            js_sys::Reflect::set(&js_options, &"cap".into(), &cap.into_canvas_kit()).unwrap();
        }
        let result = self.canvas_kit_path.stroke(js_options.into());
        if result == JsValue::undefined() {
            Err(())
        } else {
            Ok(())
        }
    }
    pub(crate) fn move_to(self, x: f32, y: f32) -> Self {
        self.canvas_kit_path.moveTo(x, y);
        self
    }
    pub(crate) fn line_to(self, x: f32, y: f32) -> Self {
        self.canvas_kit_path.lineTo(x, y);
        self
    }
    pub(crate) fn arc_to(self, oval: &LtrbRect, start_radian: f32, delta_radian: f32) -> Self {
        self.canvas_kit_path.arcToOval(
            &[oval.left, oval.top, oval.right, oval.bottom],
            start_radian * 180.0 / std::f32::consts::PI,
            delta_radian * 180.0 / std::f32::consts::PI,
            false,
        );
        self
    }
    pub(crate) fn scale(self, x: f32, y: f32) -> Self {
        self.transform(&[x, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 1.0])
    }
    pub(crate) fn translate(self, x: f32, y: f32) -> Self {
        self.canvas_kit_path.offset(x, y);
        self
    }
    pub(crate) fn transform(self, matrix_3x3: &[f32; 9]) -> Self {
        self.canvas_kit_path.transform(matrix_3x3);
        self
    }
    pub(crate) fn add_oval(self, ltrb_rect: &LtrbRect) -> Self {
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
    pub(crate) fn add_arc(self, oval: &LtrbRect, start_radian: f32, delta_radian: f32) -> Self {
        self.canvas_kit_path.addArc(
            &[oval.left, oval.top, oval.right, oval.bottom],
            start_radian * 180.0 / std::f32::consts::PI,
            delta_radian * 180.0 / std::f32::consts::PI,
        );
        self
    }
    pub(crate) fn add_poly(self, xy_array: &[Xy<f32>], close: bool) -> Self {
        let array = &xy_array
            .iter()
            .flat_map(|xy| vec![xy.x, xy.y])
            .collect::<Vec<f32>>();
        self.canvas_kit_path.addPoly(array, close);
        self
    }
    pub(crate) fn close(self) -> Self {
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
