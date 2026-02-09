use crate::*;

impl Draw for &ImageDrawCommand {
    fn draw(self, skia: &mut NativeSkia) {
        if self.sprites.is_empty() {
            return;
        }

        let xforms: Vec<RSXform> = self.sprites.iter().map(|s| s.xform).collect();
        let tex_rects: Vec<Rect<Px>> = self.sprites.iter().map(|s| s.src_rect).collect();

        skia.surface()
            .canvas()
            .draw_atlas(&self.image, &xforms, &tex_rects, &self.paint);
    }
}
