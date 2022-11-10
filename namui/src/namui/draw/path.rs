use super::*;
use crate::*;

#[derive(Debug, Serialize, Clone)]
pub struct PathDrawCommand {
    pub path_builder: PathBuilder,
    pub paint_builder: PaintBuilder,
}

impl PathDrawCommand {
    pub fn draw(&self) {
        let path = self.path_builder.build();
        let paint = self.paint_builder.build();
        crate::graphics::surface().canvas().draw_path(&path, &paint);
    }
    pub fn get_bounding_box(&self) -> Option<Rect<Px>> {
        let path = self.path_builder.build();
        let paint = self.paint_builder.build();

        let mut stroke_path_builder = self.path_builder.clone();
        let stroke_result = stroke_path_builder.stroke(StrokeOptions {
            cap: Some(paint.get_stroke_cap()),
            join: Some(paint.get_stroke_join()),
            width: Some(paint.get_stroke_width()),
            miter_limit: Some(paint.get_stroke_miter()),
            precision: None,
        });

        let path = match stroke_result {
            Ok(()) => stroke_path_builder.build(),
            Err(()) => path,
        };

        path.get_bounding_box()
    }
    pub fn is_xy_in(&self, xy: Xy<Px>) -> bool {
        let path = self.path_builder.build();
        if path.contains(xy) {
            return true;
        }

        let paint = self.paint_builder.build();

        let mut stroke_path_builder = self.path_builder.clone();
        let stroke_result = stroke_path_builder.stroke(StrokeOptions {
            cap: Some(paint.get_stroke_cap()),
            join: Some(paint.get_stroke_join()),
            width: Some(paint.get_stroke_width()),
            miter_limit: Some(paint.get_stroke_miter()),
            precision: None,
        });

        let path = match stroke_result {
            Ok(()) => stroke_path_builder.build(),
            Err(()) => path,
        };

        path.contains(xy)
    }
}
