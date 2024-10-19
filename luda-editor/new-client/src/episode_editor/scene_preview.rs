use super::{psd_sprite_util::render_psd_sprite, scene_sprite_editor::SIZE_TOOL_DRAGGING_ATOM};
use luda_rpc::Scene;
use namui::*;
use namui_prebuilt::*;
use std::ops::Deref;

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
        let (size_tool_dragging, _) = ctx.init_atom(&SIZE_TOOL_DRAGGING_ATOM, || None);

        let size_tool_dragging = size_tool_dragging.deref().as_ref().and_then(|dragging| {
            if dragging.scene_id != scene.id {
                return None;
            }
            Some(dragging)
        });

        for (sprite_index, scene_sprite) in scene.scene_sprites.iter().enumerate() {
            if let Some(size_tool_dragging) = size_tool_dragging {
                if size_tool_dragging.sprite_index == sprite_index {
                    let mut scene_sprite = scene_sprite.clone();
                    scene_sprite.circumcircle.radius = size_tool_dragging.radius;
                    render_psd_sprite(ctx, &scene_sprite, screen_wh);
                    continue;
                }
            }
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
