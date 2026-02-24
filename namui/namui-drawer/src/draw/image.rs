use crate::*;

impl Draw for &ImageDrawCommand {
    fn draw(self, skia: &mut NativeSkia) {
        if self.sprites.is_empty() {
            return;
        }

        let xforms: Vec<RSXform> = self.sprites.iter().map(|s| s.xform).collect();
        let tex_rects: Vec<Rect<Px>> = self.sprites.iter().map(|s| s.src_rect).collect();

        let has_non_white_color = self
            .sprites
            .iter()
            .any(|s| s.color.is_some_and(|c| c != Color::WHITE));
        let colors: Option<Vec<Color>> = if has_non_white_color {
            Some(
                self.sprites
                    .iter()
                    .map(|s| s.color.unwrap_or(Color::WHITE))
                    .collect(),
            )
        } else {
            None
        };

        skia.surface().canvas().draw_atlas(
            &self.image,
            &xforms,
            &tex_rects,
            colors.as_deref(),
            self.sprite_colors_blend_mode,
            &self.paint,
        );
    }
}
