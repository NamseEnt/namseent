use super::*;
use namui_skia::*;

impl Draw for &PathDrawCommand {
    fn draw(self, skia: &mut impl SkSkia) {
        skia.surface().canvas().draw_path(&self.path, &self.paint);
    }
}
