use super::render_psd_sprite::render_psd_sprite;
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
            table::vertical([
                table::fixed(24.px(), |wh, ctx| {
                    ctx.add(typography::center_text(
                        wh,
                        "Scene Preview",
                        Color::WHITE,
                        16.int_px(),
                    ));
                }),
                table::ratio(1, |wh, ctx| {
                    ctx.add(ScenePreviewScreen {
                        scene,
                        screen_wh: wh,
                    });
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

struct ScenePreviewScreen<'a> {
    scene: &'a Scene,
    screen_wh: Wh<Px>,
}
impl Component for ScenePreviewScreen<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { scene, screen_wh } = self;

        for scene_sprite in &scene.scene_sprites {
            render_psd_sprite(ctx, scene_sprite, screen_wh);
        }

        ctx.add(simple_rect(
            screen_wh,
            Color::TRANSPARENT,
            1.px(),
            Color::BLACK,
        ));
    }
}
