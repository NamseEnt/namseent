use crate::*;
use list_view::AutoListView;
use luda_rpc::*;

pub struct SceneSpriteList<'a> {
    pub wh: Wh<Px>,
    pub scene_sprites: &'a [SceneSprite],
    pub remove_scene_sprite: &'a dyn Fn(usize),
    pub add_new_scene_sprite: &'a dyn Fn(),
    /// true for up, false for down
    pub move_scene_sprite_up_down: &'a dyn Fn(usize, bool),
    pub select_scene_sprite: &'a dyn Fn(usize),
    pub selected_scene_sprite_index: Option<usize>,
}

impl Component for SceneSpriteList<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            scene_sprites,
            remove_scene_sprite,
            add_new_scene_sprite,
            move_scene_sprite_up_down,
            select_scene_sprite,
            selected_scene_sprite_index,
        } = self;
        ctx.compose(|ctx| {
            table::vertical([
                table::fixed(
                    128.px(),
                    table::horizontal([
                        table::fixed(64.px(), |wh, ctx| {
                            ctx.add(add_new_button);
                        }),
                        table::fixed(64.px(), |wh, ctx| {
                            ctx.add(move_up_button);
                        }),
                        table::fixed(64.px(), |wh, ctx| {
                            ctx.add(move_down_button);
                        }),
                        table::ratio(1, |_, _| {}),
                        table::fixed(64.px(), |wh, ctx| {
                            ctx.add(remove_button);
                        }),
                    ]),
                ),
                table::ratio(1, |wh, ctx| {
                    let item_wh = Wh::new(wh.width, 128.px());
                    ctx.add(AutoListView {
                        scroll_bar_width: 16.px(),
                        height: wh.height,
                        item_wh,
                        items: scene_sprites
                            .into_iter()
                            .enumerate()
                            .map(|(index, scene_sprite)| (index, SceneSpriteCell { wh: item_wh })),
                    });
                }),
            ])(wh, ctx)
        });
    }
}

struct SceneSpriteCell<'a> {
    wh: Wh<Px>,
}
impl Component for SceneSpriteCell<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;
        ctx.compose(|ctx| {
            table::horizontal([
                table::fixed(128.px(), |wh, ctx| {
                    ctx.add(scene_sprite_preview);
                }),
                table::ratio(1, |wh, ctx| {
                    ctx.add(scene_sprite_name);
                }),
            ])(wh, ctx)
        });
    }
}
