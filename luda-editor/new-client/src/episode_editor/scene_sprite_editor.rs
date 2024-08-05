use crate::*;
use luda_rpc::{Scene, SceneSprite, Sprite};
use namui::*;
use namui_prebuilt::{list_view::AutoListView, *};

pub struct SceneSpriteEditor<'a> {
    pub wh: Wh<Px>,
}

impl Component for SceneSpriteEditor<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;
        ctx.compose(|ctx| {
            table::vertical([
                table::fixed(320.px(), {
                    |wh, ctx| {
                        ctx.add(SceneSpriteList {
                            wh,
                            scene_sprites,
                            remove_scene_sprite,
                            add_new_scene_sprite,
                            move_scene_sprite_up_down,
                            select_scene_sprite,
                            selected_scene_sprite_index,
                        });
                    }
                }),
                table::fixed(320.px(), |wh, ctx| {
                    ctx.add(SpriteSelectTool);
                }),
                table::fixed(320.px(), |wh, ctx| {
                    ctx.add(scene_sprite_position_tool);
                }),
                table::fixed(320.px(), |wh, ctx| {
                    ctx.add(scene_sprite_size_tool);
                }),
            ])(wh, ctx)
        });
    }
}

struct SceneSpriteList<'a> {
    wh: Wh<Px>,
    scene_sprites: &'a [SceneSprite],
    remove_scene_sprite: &'a dyn Fn(usize),
    add_new_scene_sprite: &'a dyn Fn(),
    /// true for up, false for down
    move_scene_sprite_up_down: &'a dyn Fn(usize, bool),
    select_scene_sprite: &'a dyn Fn(usize),
    selected_scene_sprite_index: Option<usize>,
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

struct SpriteSelectTool<'a> {
    wh: Wh<Px>,
    sprites: &'a [Sprite],
}

impl Component for SpriteSelectTool<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, sprites } = self;
    }
}

struct SpriteSelectToolColumn<Key, Items, Preview, OnSelect>
where
    Key: Into<AddKey>,
    Preview: Fn(Wh<Px>, &ComposeCtx),
    OnSelect: Fn(),
    Items: ExactSizeIterator<Item = (Key, Preview, String, OnSelect)>,
{
    wh: Wh<Px>,
    items: Items,
}

impl<Key, Items, Preview, OnSelect> Component
    for SpriteSelectToolColumn<Key, Items, Preview, OnSelect>
where
    Key: Into<AddKey>,
    Preview: Fn(Wh<Px>, &ComposeCtx),
    OnSelect: Fn(),
    Items: ExactSizeIterator<Item = (Key, Preview, String, OnSelect)>,
{
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, items } = self;

        let item_wh = Wh::new(wh.width, 80.px());

        ctx.add(AutoListView {
            height: wh.height,
            scroll_bar_width: 10.px(),
            item_wh,
            items: items.map(|item| {
                let (key, preview, text, on_select) = item;
                (key, move |ctx: &RenderCtx| {
                    ctx.compose(|ctx| {
                        table::horizontal([
                            table::fixed(128.px(), |wh, ctx| {
                                preview(wh, &ctx);
                            }),
                            table::ratio(1, |wh, ctx| {
                                ctx.add(namui::text(TextParam {
                                    text,
                                    x: 0.px(),
                                    y: wh.height / 2.0,
                                    align: TextAlign::Left,
                                    baseline: TextBaseline::Middle,
                                    font: Font {
                                        name: "NotoSansKR-Regular".to_string(),
                                        size: 16.int_px(),
                                    },
                                    style: TextStyle {
                                        color: Color::WHITE,
                                        ..Default::default()
                                    },
                                    max_width: Some(wh.width),
                                }));
                            }),
                        ])(item_wh, ctx)
                    });
                })
            }),
        });
    }
}
