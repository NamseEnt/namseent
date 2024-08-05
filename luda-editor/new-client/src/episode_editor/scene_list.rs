use luda_rpc::Scene;
use namui::*;
use namui_prebuilt::*;

pub struct SceneList<'a> {
    pub wh: Wh<Px>,
    pub scenes: &'a [Scene],
}

impl Component for SceneList<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, scenes } = self;
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
                    ctx.add(list_view::AutoListView {
                        height: wh.height,
                        scroll_bar_width: 20.px(),
                        item_wh: Wh::new(wh.width, 200.px()),
                        items: scenes.iter().enumerate().map(|(index, scene)| {
                            (scene.id.as_str(), SceneListCell { index, scene })
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
}

impl Component for SceneListCell<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { index, scene } = self;
        /*
        썸네일을 어떻게 보여줄 것인가?
        저장하고 보여줄 것인가, 매번 새로 그릴 것인가?
        뭐가 되었든, 처음에는 그려야한다.
        그리고 난 다음에 저장할 것인가? 말 것인가?
        1. 매번 그리는게 부담되는가?
        2. 저장된 것을 불러오는 것은 부담되는가?
        3.
        */

        // for sprite in scene.sprites {
        //     sprite.sprite_id
        // }
        todo!()
    }
}
