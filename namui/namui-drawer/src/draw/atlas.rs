use crate::*;

impl Draw for &AtlasDrawCommand {
    fn draw(self, skia: &mut NativeSkia) {
        if self.sprites.is_empty() {
            return;
        }

        // Prepare xforms and tex_rects for the draw_atlas call
        let xforms: Vec<RSXform> = self.sprites.iter().map(|s| s.xform).collect();
        let tex_rects: Vec<Rect<Px>> = self.sprites.iter().map(|s| s.tex).collect();

        skia.surface()
            .canvas()
            .draw_atlas(&self.atlas, &xforms, &tex_rects, &self.paint);
    }
}
