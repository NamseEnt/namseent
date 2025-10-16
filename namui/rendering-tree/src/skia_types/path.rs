use crate::*;
use std::sync::Arc;

pub struct NativePath {
    skia_path: skia_safe::Path,
    path: Path,
}

impl NativePath {
    pub fn get(path: &Path) -> Arc<Self> {
        static CACHE: LruCache<Path, NativePath> = LruCache::new();
        CACHE.get_or_create(path, NativePath::new)
    }
    pub fn new(path: &Path) -> Self {
        let mut skia_path = skia_safe::Path::new();

        apply_command_to_skia_path(&mut skia_path, path);

        NativePath {
            skia_path,
            path: path.clone(),
        }
    }
    fn painted_path(path: &Path, paint: &Paint) -> Arc<Self> {
        let path = path.clone().stroke(StrokeOptions {
            cap: paint.stroke_cap,
            join: paint.stroke_join,
            width: Some(paint.stroke_width),
            miter_limit: Some(paint.stroke_miter),
            precision: None,
        });

        Self::get(&path)
    }
    pub fn contains(&self, paint: Option<&Paint>, xy: Xy<Px>) -> bool {
        if self.skia().contains((xy.x.as_f32(), xy.y.as_f32())) {
            return true;
        }

        let Some(paint) = paint else {
            return false;
        };

        Self::painted_path(&self.path, paint)
            .skia()
            .contains((xy.x.as_f32(), xy.y.as_f32()))
    }
    pub fn bounding_box(&self, paint: Option<&Paint>) -> Option<Rect<Px>> {
        if let Some(paint) = paint {
            Self::painted_path(&self.path, paint).bounding_box(None)
        } else {
            let bounds = self.skia_path.bounds();
            if bounds.left == 0.0
                && bounds.top == 0.0
                && bounds.right == 0.0
                && bounds.bottom == 0.0
            {
                None
            } else {
                Some((*bounds).into())
            }
        }
    }

    pub fn skia(&self) -> &skia_safe::Path {
        &self.skia_path
    }
}

fn apply_command_to_skia_path(skia_path: &mut skia_safe::Path, path: &Path) {
    for command in path.commands() {
        match command {
            &PathCommand::AddRect { rect } => {
                skia_path.add_rect(skia_safe::Rect::from(rect), None);
            }
            &PathCommand::AddRrect { rect, rx, ry } => {
                skia_path.add_rrect(
                    skia_safe::RRect::new_rect_xy(
                        skia_safe::Rect::from(rect),
                        rx.into(),
                        ry.into(),
                    ),
                    None,
                );
            }
            PathCommand::Stroke { stroke_options } => {
                let mut paint = skia_safe::Paint::default();
                paint.set_style(skia_safe::PaintStyle::Stroke);
                paint.set_stroke_cap(
                    stroke_options
                        .cap
                        .map(|c| c.into())
                        .unwrap_or(skia_safe::PaintCap::Butt),
                );
                paint.set_stroke_join(
                    stroke_options
                        .join
                        .map(|j| j.into())
                        .unwrap_or(skia_safe::PaintJoin::Miter),
                );
                paint.set_stroke_width(stroke_options.width.map(|w| w.into()).unwrap_or(1.0));
                paint.set_stroke_miter(stroke_options.miter_limit.map(|m| m.into()).unwrap_or(4.0));

                let precision = *stroke_options.precision.unwrap_or(1.0.into());

                if !skia_safe::path_utils::fill_path_with_paint(
                    &skia_path.clone(),
                    &paint,
                    skia_path,
                    None,
                    skia_safe::Matrix::scale((precision, precision)),
                ) {
                    // Nothing applied because of Paint.
                }
            }
            PathCommand::MoveTo { xy } => {
                skia_path.move_to(*xy);
            }
            PathCommand::LineTo { xy } => {
                skia_path.line_to(*xy);
            }
            PathCommand::CubicTo {
                first_xy,
                second_xy,
                end_xy,
            } => {
                skia_path.cubic_to(*first_xy, *second_xy, *end_xy);
            }
            &PathCommand::ArcTo {
                oval,
                start_angle,
                delta_angle,
            } => {
                skia_path.arc_to(
                    skia_safe::Rect::from(oval),
                    start_angle.as_degrees(),
                    delta_angle.as_degrees(),
                    false,
                );
            }
            &PathCommand::Scale { xy } => {
                skia_path.transform(&skia_safe::Matrix::scale(xy.map(|x| x.as_f32()).into()));
            }
            &PathCommand::Translate { xy } => {
                skia_path.offset(xy);
            }
            &PathCommand::Transform { matrix } => {
                skia_path.transform(&matrix.into());
            }
            &PathCommand::AddOval { rect } => {
                skia_path.add_oval(skia_safe::Rect::from(rect), None);
            }
            &PathCommand::AddArc {
                oval,
                start_angle,
                delta_angle,
            } => {
                skia_path.add_arc(
                    skia_safe::Rect::from(oval),
                    start_angle.as_degrees(),
                    delta_angle.as_degrees(),
                );
            }
            PathCommand::AddPoly { xys, close } => {
                let points = xys.iter().map(|xy| (*xy).into()).collect::<Vec<_>>();
                skia_path.add_poly(points.as_slice(), *close);
            }
            PathCommand::Close => {
                skia_path.close();
            }
        }
    }
}
