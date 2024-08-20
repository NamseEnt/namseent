use crate::*;
use list_view::AutoListView;
use luda_rpc::*;
use std::collections::HashMap;

pub struct SceneSpriteList<'a> {
    pub wh: Wh<Px>,
    pub scene_sprites: &'a [SceneSprite],
    pub sprite_docs: &'a HashMap<String, SpriteDoc>,
    pub remove_scene_sprite: &'a dyn Fn(usize),
    pub add_new_scene_sprite: &'a dyn Fn(),
    /// true for up, false for down
    pub move_scene_sprite_up_down: &'a dyn Fn(usize, bool),
    pub select_scene_sprite: &'a dyn Fn(&str),
    pub selected_scene_sprite_index: Option<usize>,
}

impl Component for SceneSpriteList<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            scene_sprites,
            sprite_docs,
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
                            ctx.add(simple_button(wh, "[+]", |_| add_new_scene_sprite()));
                        }),
                        table::fixed(64.px(), |wh, ctx| {
                            ctx.add(simple_button(wh, "[↑]", |_| {
                                if let Some(selected_scene_sprite_index) =
                                    selected_scene_sprite_index.as_ref()
                                {
                                    move_scene_sprite_up_down(*selected_scene_sprite_index, true)
                                }
                            }));
                        }),
                        table::fixed(64.px(), |wh, ctx| {
                            ctx.add(simple_button(wh, "[↓]", |_| {
                                if let Some(selected_scene_sprite_index) =
                                    selected_scene_sprite_index.as_ref()
                                {
                                    move_scene_sprite_up_down(*selected_scene_sprite_index, false)
                                }
                            }));
                        }),
                        table::ratio(1, |_, _| {
                            // margin
                        }),
                        table::fixed(64.px(), |wh, ctx| {
                            ctx.add(simple_button(wh, "[-]", |_| {
                                if let Some(selected_scene_sprite_index) =
                                    selected_scene_sprite_index.as_ref()
                                {
                                    remove_scene_sprite(*selected_scene_sprite_index)
                                }
                            }));
                        }),
                    ]),
                ),
                table::ratio(1, |wh, ctx| {
                    let item_wh = Wh::new(wh.width, 128.px());

                    ctx.add(AutoListView {
                        scroll_bar_width: 16.px(),
                        height: wh.height,
                        item_wh,
                        items: scene_sprites.into_iter().enumerate().map(
                            |(index, scene_sprite)| {
                                let sprite_name = scene_sprite
                                    .sprite_id
                                    .as_ref()
                                    .and_then(|sprite_id| {
                                        Some(
                                            sprite_docs
                                                .get(sprite_id)
                                                .map(|sprite_doc| sprite_doc.sprite.name())
                                                .unwrap_or("???"),
                                        )
                                    })
                                    .unwrap_or("");
                                (
                                    index,
                                    SceneSpriteCell {
                                        wh: item_wh,
                                        sprite_name,
                                        scene_sprite,
                                        select_scene_sprite,
                                    },
                                )
                            },
                        ),
                    });
                }),
            ])(wh, ctx)
        });
    }
}

struct SceneSpriteCell<'a> {
    wh: Wh<Px>,
    sprite_name: &'a str,
    scene_sprite: &'a SceneSprite,
    select_scene_sprite: &'a dyn Fn(&str),
}
impl Component for SceneSpriteCell<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            sprite_name,
            scene_sprite,
            select_scene_sprite,
        } = self;

        ctx.add(simple_button(wh, "", |_| {
            if let Some(sprite_id) = scene_sprite.sprite_id.as_ref() {
                select_scene_sprite(sprite_id);
            }
        }));

        ctx.compose(|ctx| {
            table::horizontal([
                table::fixed(128.px(), |wh, ctx| todo!("ctx.add(scene_sprite_preview);")),
                table::ratio(1, |wh, ctx| {
                    ctx.add(typography::body::left(wh.height, sprite_name, Color::WHITE));
                }),
            ])(wh, ctx)
        });
    }
}
