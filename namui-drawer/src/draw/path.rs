use super::*;

impl Draw for PathDrawCommand {
    fn draw(self, ctx: &DrawContext) {
        ctx.canvas().draw_path(&self.path, &self.paint);
    }
}
