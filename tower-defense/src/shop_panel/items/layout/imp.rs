use namui::*;

impl<'a> Component for super::ShopItemLayout<'a> {
    fn render(self, ctx: &RenderCtx) {
        super::body::render_body(ctx, self.params);
    }
}
