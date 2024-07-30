use luda_rpc::Scene;
use namui::*;
use namui_prebuilt::*;

pub struct ScenePreview<'a> {
    pub wh: Wh<Px>,
    pub scene: &'a Scene,
}

impl Component for ScenePreview<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, scene } = self;

        ctx.compose(|ctx| {
            table::horizontal([
                table::ratio(1, |wh, ctx| {
                    ctx.add(typography::center_text(
                        wh,
                        "Scene Preview",
                        Color::WHITE,
                        16.int_px(),
                    ));
                }),
                table::ratio(1, |wh, ctx| {
                    ctx.add(typography::center_text(
                        wh,
                        scene.id.as_str(),
                        Color::WHITE,
                        16.int_px(),
                    ));
                }),
            ])(wh, ctx);
        });
    }
}

struct TextEditor<'a> {
    wh: Wh<Px>,
    scene: &'a Scene,
}

impl Component for TextEditor<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, scene } = self;
    }
}
