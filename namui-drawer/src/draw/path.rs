use super::*;

impl Draw for PathDrawCommand {
    fn draw(self, ctx: &DrawContext) {
        ctx.canvas().draw_path(&self.path, &self.paint);
    }
}
//     pub fn get_bounding_box(&self) -> Option<Rect<Px>> {
//         let path = self.path.build();
//         let paint = self.paint.build();

//         let mut stroke_path = self.path.clone();
//         let stroke_result = stroke_path.stroke(StrokeOptions {
//             cap: Some(paint.get_stroke_cap()),
//             join: Some(paint.get_stroke_join()),
//             width: Some(paint.get_stroke_width()),
//             miter_limit: Some(paint.get_stroke_miter()),
//             precision: None,
//         });

//         let path = match stroke_result {
//             Ok(()) => stroke_path.build(),
//             Err(()) => path,
//         };

//         path.get_bounding_box()
//     }
//     pub fn is_xy_in(&self, xy: Xy<Px>) -> bool {
//         let path = self.path.build();
//         if path.contains(xy) {
//             return true;
//         }

//         let paint = self.paint.build();

//         let mut stroke_path = self.path.clone();
//         let stroke_result = stroke_path.stroke(StrokeOptions {
//             cap: Some(paint.get_stroke_cap()),
//             join: Some(paint.get_stroke_join()),
//             width: Some(paint.get_stroke_width()),
//             miter_limit: Some(paint.get_stroke_miter()),
//             precision: None,
//         });

//         let path = match stroke_result {
//             Ok(()) => stroke_path.build(),
//             Err(()) => path,
//         };

//         path.contains(xy)
//     }
// }
