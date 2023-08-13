use super::*;
use std::sync::Arc;
use wasm_bindgen::JsValue;

pub(crate) struct CkPath {
    pub canvas_kit_path: CanvasKitPath,
}

impl CkPath {
    pub(crate) fn get(path: &Path) -> Arc<Self> {
        static CACHE: SerdeLruCache<Path, CkPath> = SerdeLruCache::new();
        CACHE.get_or_create(path, |path| CkPath::new(path))
    }
    pub fn new(path: &Path) -> Self {
        let canvas_kit_path = CanvasKitPath::new();

        apply_command_to_canvas_kit_path(&canvas_kit_path, path);

        CkPath { canvas_kit_path }
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

    pub(crate) fn canvas_kit(&self) -> &CanvasKitPath {
        &self.canvas_kit_path
    }
}

fn apply_command_to_canvas_kit_path(canvas_kit_path: &CanvasKitPath, path: &Path) {
    for command in path.commands() {
        match command {
            PathCommand::AddRect { rect } => {
                let ltrb = rect.as_ltrb();
                canvas_kit_path.addRect(
                    &[
                        ltrb.left.as_f32(),
                        ltrb.top.as_f32(),
                        ltrb.right.as_f32(),
                        ltrb.bottom.as_f32(),
                    ],
                    None,
                );
            }
            PathCommand::AddRrect { rect, rx, ry } => {
                let ltrb = rect.as_ltrb();

                let js_rect = js_sys::Float32Array::new_with_length(4);
                js_rect.set_index(0, ltrb.left.as_f32());
                js_rect.set_index(1, ltrb.top.as_f32());
                js_rect.set_index(2, ltrb.right.as_f32());
                js_rect.set_index(3, ltrb.bottom.as_f32());
                let rrect = canvas_kit().RRectXY(js_rect, rx.as_f32(), ry.as_f32());
                canvas_kit_path.addRRect(rrect, None);
            }
            PathCommand::Stroke { stroke_options } => {
                let js_options = js_sys::Object::new();
                if let Some(width) = stroke_options.width {
                    js_sys::Reflect::set(&js_options, &"width".into(), &width.as_f32().into())
                        .unwrap();
                }
                if let Some(miter_limit) = stroke_options.miter_limit {
                    js_sys::Reflect::set(
                        &js_options,
                        &"miterLimit".into(),
                        &miter_limit.as_f32().into(),
                    )
                    .unwrap();
                }
                if let Some(precision) = stroke_options.precision {
                    js_sys::Reflect::set(&js_options, &"precision".into(), &precision.into())
                        .unwrap();
                }
                if let Some(join) = stroke_options.join {
                    let canvas_kit_stroke_join: CanvasKitStrokeJoin = join.into();
                    js_sys::Reflect::set(&js_options, &"join".into(), &canvas_kit_stroke_join)
                        .unwrap();
                }
                if let Some(cap) = stroke_options.cap {
                    let canvas_kit_stroke_cap: CanvasKitStrokeCap = cap.into();
                    js_sys::Reflect::set(&js_options, &"cap".into(), &canvas_kit_stroke_cap)
                        .unwrap();
                }
                let result = canvas_kit_path.stroke(js_options.into());
                if result == JsValue::undefined() {
                    panic!("stroke failed");
                }
            }
            PathCommand::MoveTo { xy } => {
                canvas_kit_path.moveTo(xy.x.as_f32(), xy.y.as_f32());
            }
            PathCommand::LineTo { xy } => {
                canvas_kit_path.lineTo(xy.x.as_f32(), xy.y.as_f32());
            }
            PathCommand::ArcTo {
                oval,
                start_angle,
                delta_angle,
            } => {
                let ltrb = oval.as_ltrb();
                canvas_kit_path.arcToOval(
                    &[
                        ltrb.left.as_f32(),
                        ltrb.top.as_f32(),
                        ltrb.right.as_f32(),
                        ltrb.bottom.as_f32(),
                    ],
                    start_angle.as_degrees(),
                    delta_angle.as_degrees(),
                    false,
                );
            }
            PathCommand::Scale { xy } => {
                canvas_kit_path.transform(&Matrix3x3::from_scale(xy.x, xy.y).into_linear_slice());
            }
            PathCommand::Translate { xy } => {
                canvas_kit_path.offset(xy.x.as_f32(), xy.y.as_f32());
            }
            PathCommand::Transform { matrix } => {
                canvas_kit_path.transform(&matrix.into_linear_slice());
            }
            PathCommand::AddOval { rect } => {
                let ltrb = rect.as_ltrb();
                canvas_kit_path.addOval(
                    &[
                        ltrb.left.as_f32(),
                        ltrb.top.as_f32(),
                        ltrb.right.as_f32(),
                        ltrb.bottom.as_f32(),
                    ],
                    None,
                    None,
                );
            }
            PathCommand::AddArc {
                oval,
                start_angle,
                delta_angle,
            } => {
                let ltrb = oval.as_ltrb();
                canvas_kit_path.addArc(
                    &[
                        ltrb.left.as_f32(),
                        ltrb.top.as_f32(),
                        ltrb.right.as_f32(),
                        ltrb.bottom.as_f32(),
                    ],
                    start_angle.as_degrees(),
                    delta_angle.as_degrees(),
                );
            }
            PathCommand::AddPoly { xys, close } => {
                let array = &xys
                    .iter()
                    .flat_map(|xy| vec![xy.x.as_f32(), xy.y.as_f32()])
                    .collect::<Vec<f32>>();
                canvas_kit_path.addPoly(array, *close);
            }
            PathCommand::Close => {
                canvas_kit_path.close();
            }
        }
    }
}

impl Drop for CkPath {
    fn drop(&mut self) {
        self.canvas_kit_path.delete();
    }
}
