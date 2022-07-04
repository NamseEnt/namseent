use super::*;
use crate::*;
pub use base::*;
use serde::Serialize;
use std::sync::atomic::{AtomicUsize, Ordering};
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
    pub fn contains(&self, xy: Xy<Px>) -> bool {
        self.canvas_kit_path.contains(xy.x.as_f32(), xy.y.as_f32())
    }
    pub fn get_bounding_box(&self) -> Option<Rect<Px>> {
        let bounds = self.canvas_kit_path.getBounds();
        if bounds[0] == 0.0 && bounds[1] == 0.0 && bounds[2] == 0.0 && bounds[3] == 0.0 {
            None
        } else {
            Some(Rect::Ltrb {
                left: px(bounds[0]),
                top: px(bounds[1]),
                right: px(bounds[2]),
                bottom: px(bounds[3]),
            })
        }
    }

    pub(crate) fn add_rect(self, rect: Rect<Px>) -> Self {
        let ltrb = rect.as_ltrb();
        self.canvas_kit_path.addRect(
            &[
                ltrb.left.as_f32(),
                ltrb.top.as_f32(),
                ltrb.right.as_f32(),
                ltrb.bottom.as_f32(),
            ],
            None,
        );

        self
    }
    pub(crate) fn add_rrect(self, rect: Rect<Px>, rx: Px, ry: Px) -> Self {
        let ltrb = rect.as_ltrb();

        let js_rect = js_sys::Float32Array::new_with_length(4);
        js_rect.set_index(0, ltrb.left.as_f32());
        js_rect.set_index(1, ltrb.top.as_f32());
        js_rect.set_index(2, ltrb.right.as_f32());
        js_rect.set_index(3, ltrb.bottom.as_f32());
        let rrect = canvas_kit().RRectXY(js_rect, rx.as_f32(), ry.as_f32());
        self.canvas_kit_path.addRRect(rrect, None);

        self
    }
    pub(crate) fn stroke(&mut self, options: StrokeOptions) -> Result<(), ()> {
        let js_options = js_sys::Object::new();
        if let Some(width) = options.width {
            js_sys::Reflect::set(&js_options, &"width".into(), &width.as_f32().into()).unwrap();
        }
        if let Some(miter_limit) = options.miter_limit {
            js_sys::Reflect::set(
                &js_options,
                &"miterLimit".into(),
                &miter_limit.as_f32().into(),
            )
            .unwrap();
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
    pub(crate) fn move_to(self, x: Px, y: Px) -> Self {
        self.canvas_kit_path.moveTo(x.as_f32(), y.as_f32());
        self
    }
    pub(crate) fn line_to(self, x: Px, y: Px) -> Self {
        self.canvas_kit_path.lineTo(x.as_f32(), y.as_f32());
        self
    }
    pub(crate) fn arc_to(self, oval: Rect<Px>, start_angle: Angle, delta_angle: Angle) -> Self {
        let ltrb = oval.as_ltrb();
        self.canvas_kit_path.arcToOval(
            &[
                ltrb.left.as_f32(),
                ltrb.top.as_f32(),
                ltrb.right.as_f32(),
                ltrb.bottom.as_f32(),
            ],
            start_angle.as_radians(),
            delta_angle.as_radians(),
            false,
        );
        self
    }
    pub(crate) fn scale(self, x: f32, y: f32) -> Self {
        self.transform(&[x, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 1.0])
    }
    pub(crate) fn translate(self, x: Px, y: Px) -> Self {
        self.canvas_kit_path.offset(x.as_f32(), y.as_f32());
        self
    }
    pub(crate) fn transform(self, matrix_3x3: &[f32; 9]) -> Self {
        self.canvas_kit_path.transform(matrix_3x3);
        self
    }
    pub(crate) fn add_oval(self, rect: Rect<Px>) -> Self {
        let ltrb = rect.as_ltrb();
        self.canvas_kit_path.addOval(
            &[
                ltrb.left.as_f32(),
                ltrb.top.as_f32(),
                ltrb.right.as_f32(),
                ltrb.bottom.as_f32(),
            ],
            None,
            None,
        );
        self
    }
    pub(crate) fn add_arc(self, oval: Rect<Px>, start_angle: Angle, delta_angle: Angle) -> Self {
        let ltrb = oval.as_ltrb();
        self.canvas_kit_path.addArc(
            &[
                ltrb.left.as_f32(),
                ltrb.top.as_f32(),
                ltrb.right.as_f32(),
                ltrb.bottom.as_f32(),
            ],
            start_angle.as_degrees(),
            delta_angle.as_degrees(),
        );
        self
    }
    pub(crate) fn add_poly(self, xy_array: &[Xy<Px>], close: bool) -> Self {
        let array = &xy_array
            .iter()
            .flat_map(|xy| vec![xy.x.as_f32(), xy.y.as_f32()])
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
