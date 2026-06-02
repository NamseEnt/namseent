use crate::*;

impl Draw for &PathDrawCommand {
    fn draw(self, skia: &mut NativeSkia) {
        let canvas = skia.surface().canvas();

        let is_stroke = self.paint.paint_style == Some(PaintStyle::Stroke);
        if !is_stroke || self.paint.stroke_width <= 0.px() {
            canvas.draw_path(&self.path, &self.paint);
            return;
        }

        match self.paint.stroke_position {
            StrokePosition::Middle => {
                canvas.draw_path(&self.path, &self.paint);
            }
            StrokePosition::Outside => {
                let outside_stroke_paint = self
                    .paint
                    .clone()
                    .set_stroke_width(self.paint.stroke_width * 2.0)
                    .set_stroke_position(StrokePosition::Middle);

                canvas.save();
                canvas.clip_path(&self.path, ClipOp::Difference, true);
                canvas.draw_path(&self.path, &outside_stroke_paint);
                canvas.restore();
            }
            StrokePosition::Inside => {
                let inside_stroke_paint = self
                    .paint
                    .clone()
                    .set_stroke_width(self.paint.stroke_width * 2.0)
                    .set_stroke_position(StrokePosition::Middle);

                canvas.save();
                canvas.clip_path(&self.path, ClipOp::Intersect, true);
                canvas.draw_path(&self.path, &inside_stroke_paint);
                canvas.restore();
            }
        }
    }
}
