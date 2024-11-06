use super::*;
use luda_rpc::*;
use psd_sprite_util::render_psd_sprite;
use router::Route;

pub struct Player<'a> {
    pub scenes: &'a [Scene],
}

impl Component for Player<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { scenes } = self;

        let wh = namui::screen::size().map(|x| x.into_px());
        let (scene_index, set_scene_index) = ctx.state(|| 0);
        let scene = scenes.get(*scene_index);

        let exit = || {
            router::route(Route::Home {
                initial_selection: home::Selection::Nothing,
            });
        };
        let next = || {
            let next_scene_index = *scene_index + 1;
            if next_scene_index >= scenes.len() {
                exit();
                return;
            }
            set_scene_index.set(next_scene_index);
        };

        ctx.add(SceneScreen {
            scene,
            screen_wh: wh,
        });

        ctx.on_raw_event(|event| match event {
            RawEvent::KeyDown { event } => match event.code {
                Code::ArrowRight => {
                    next();
                }
                Code::Escape => {
                    exit();
                }
                _ => {}
            },
            RawEvent::MouseDown { .. } => {
                next();
            }
            _ => {}
        });
    }
}

struct SceneScreen<'a> {
    scene: Option<&'a Scene>,
    screen_wh: Wh<Px>,
}
impl Component for SceneScreen<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { scene, screen_wh } = self;

        if let Some(scene) = scene {
            for scene_sprite in scene.scene_sprites.iter() {
                render_psd_sprite(ctx, scene_sprite, screen_wh);
            }
        }

        ctx.add(simple_rect(
            screen_wh,
            Color::TRANSPARENT,
            1.px(),
            Color::BLACK,
        ));
    }
}
