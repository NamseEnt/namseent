use super::*;

impl Draw for PathDrawCommand {
    fn draw<Skia: SkSkia>(self, ctx: &mut DrawContext<'_, Skia>) {
        ctx.canvas().draw_path(&self.path, &self.paint);
    }
}
