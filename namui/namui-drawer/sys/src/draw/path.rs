use super::*;
use namui_skia::*;

impl Draw for &PathDrawCommand {
    fn draw(self, ctx: &mut DrawContext) {
        ctx.canvas().draw_path(&self.path, &self.paint);
    }
}
