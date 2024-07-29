use super::speaker_selector::SpeakerSelector;
use luda_rpc::Scene;
use namui::*;
use namui_prebuilt::*;

pub struct SceneEditor<'a> {
    pub wh: Wh<Px>,
    pub scene: Option<&'a Scene>,
    pub project_id: &'a String,
    pub episode_id: &'a String,
}

impl Component for SceneEditor<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            scene,
            project_id,
            episode_id,
        } = self;

        let Some(scene) = scene else { return };

        ctx.compose(|ctx| {
            table::vertical([
                table::ratio(1, |wh, ctx| {
                    ctx.add(ScenePreview { wh, scene });
                }),
                table::fixed(160.px(), |wh, ctx| {
                    ctx.add(SpeakerSelector {
                        wh,
                        scene,
                        project_id,
                        episode_id,
                    });
                }),
                table::fixed(320.px(), |wh, ctx| {
                    ctx.add(TextEditor { wh, scene });
                }),
            ])(wh, ctx);
        });
    }
}

struct ScenePreview<'a> {
    wh: Wh<Px>,
    scene: &'a Scene,
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
