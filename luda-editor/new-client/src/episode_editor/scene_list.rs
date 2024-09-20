use super::psd_sprite_util::render_psd_sprite;
use luda_rpc::Scene;
use namui::*;
use namui_prebuilt::*;

pub struct SceneList<'a> {
    pub wh: Wh<Px>,
    pub scenes: &'a [Scene],
    pub select_scene: &'a dyn Fn(&str),
}

impl Component for SceneList<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            scenes,
            select_scene,
        } = self;
        ctx.compose(|ctx| {
            table::vertical([
                table::fixed(40.px(), |wh, ctx| {
                    ctx.add(typography::center_text(
                        wh,
                        "Scene",
                        Color::WHITE,
                        16.int_px(),
                    ));
                }),
                table::ratio(1, |wh, ctx| {
                    let item_wh = Wh::new(wh.width, wh.width / 4 * 3);
                    ctx.add(list_view::AutoListView {
                        height: wh.height,
                        scroll_bar_width: 20.px(),
                        item_wh,
                        items: scenes.iter().enumerate().map(|(index, scene)| {
                            (
                                scene.id.as_str(),
                                SceneListCell {
                                    index,
                                    scene,
                                    wh: item_wh,
                                    select_scene,
                                },
                            )
                        }),
                    });
                }),
            ])(wh, ctx);
        });
    }
}

struct SceneListCell<'a> {
    index: usize,
    scene: &'a Scene,
    wh: Wh<Px>,
    select_scene: &'a dyn Fn(&str),
}

impl Component for SceneListCell<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            index,
            scene,
            wh,
            select_scene,
        } = self;
        /*
        썸네일을 어떻게 보여줄 것인가?
        저장하고 보여줄 것인가, 매번 새로 그릴 것인가?
        뭐가 되었든, 처음에는 그려야한다.
        그리고 난 다음에 저장할 것인가? 말 것인가?
        1. 매번 그리는게 부담되는가?
        2. 저장된 것을 불러오는 것은 부담되는가?
        3.
        */

        ctx.compose(|ctx| {
            ctx.translate(Xy::single(4.px()))
                .add(typography::body::left_top(index.to_string(), Color::WHITE));
        });

        for scene_sprite in &scene.scene_sprites {
            render_psd_sprite(ctx, scene_sprite, wh);
        }

        ctx.add(
            simple_rect(wh, Color::TRANSPARENT, 1.px(), Color::BLACK).attach_event(|event| {
                let Event::MouseDown { event } = event else {
                    return;
                };
                if !event.is_local_xy_in() {
                    return;
                }
                event.stop_propagation();
                select_scene(&scene.id);
            }),
        );
    }
}
