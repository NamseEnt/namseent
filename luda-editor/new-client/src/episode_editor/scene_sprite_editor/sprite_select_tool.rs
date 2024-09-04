use crate::*;
use list_view::AutoListView;
use luda_rpc::*;
use std::collections::{HashMap, HashSet};

pub struct SpriteSelectTool<'a> {
    pub wh: Wh<Px>,
    pub sprite_docs: Sig<'a, HashMap<String, SpriteDoc>>,
    /// fn(part_name, part_option_name)
    pub select_part: &'a dyn Fn(&str, &str),
}

impl Component for SpriteSelectTool<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            sprite_docs,
            select_part,
        } = self;

        let (selected_sprite_id, set_selected_sprite_id) = ctx.state::<Option<String>>(|| None);
        let (selected_part_name, set_selected_part_name) = ctx.state::<Option<String>>(|| None);
        let (selected_tags, set_selected_tags) = ctx.state::<HashSet<SystemTag>>(Default::default);

        let tag_filtered_sprite_docs = ctx.memo(|| {
            sprite_docs
                .iter()
                .filter(|(_id, sprite_doc)| {
                    sprite_doc.tags.iter().any(|tag| match tag {
                        SpriteTag::System { tag } => selected_tags.contains(&tag),
                        SpriteTag::Custom { .. } => false,
                    })
                })
                .map(|(id, sprite)| (id.clone(), sprite.clone()))
                .collect::<HashMap<String, SpriteDoc>>()
        });

        let tag_toggle_button = |tag: SystemTag| {
            let is_on = selected_tags.contains(&SystemTag::Character);
            let text = match tag {
                SystemTag::Character => "인물",
                SystemTag::Object => "사물",
                SystemTag::Background => "배경",
            };

            table::ratio(1, move |wh, ctx| {
                ctx.add(simple_toggle_button(wh, text, is_on, |_| {
                    set_selected_tags.mutate(move |selected_tags| {
                        if selected_tags.contains(&tag) {
                            selected_tags.remove(&tag);
                        } else {
                            selected_tags.insert(tag);
                        }
                    });
                }));
            })
        };

        let selected_sprite_doc = selected_sprite_id
            .as_ref()
            .as_ref()
            .and_then(|selected_sprite_id| sprite_docs.get(selected_sprite_id));

        ctx.compose(|ctx| {
            table::vertical([
                table::fixed(
                    64.px(),
                    table::horizontal([
                        table::fixed(64.px(), |_, _| {}),
                        tag_toggle_button(SystemTag::Character),
                        table::fixed(16.px(), |_, _| {}),
                        tag_toggle_button(SystemTag::Object),
                        table::fixed(16.px(), |_, _| {}),
                        tag_toggle_button(SystemTag::Background),
                        table::fixed(64.px(), |_, _| {}),
                    ]),
                ),
                table::ratio(
                    1,
                    table::horizontal([
                        table::ratio(1, |wh, ctx| {
                            let sprite_column = Column {
                                wh,
                                items: tag_filtered_sprite_docs.iter().map(|(id, sprite)| {
                                    let on_select = || {
                                        set_selected_sprite_id.set(Some(id.clone()));
                                    };
                                    (
                                        sprite.id.as_str(),
                                        sprite.sprite.name().to_string(),
                                        on_select,
                                    )
                                }),
                            };
                            ctx.add(sprite_column);
                        }),
                        table::ratio(1, |wh, ctx| {
                            let Some(sprite_doc) = selected_sprite_doc.as_ref() else {
                                return;
                            };
                            let Sprite::Parts { sprite } = &sprite_doc.sprite else {
                                return;
                            };
                            let part_column = Column {
                                wh,
                                items: sprite.parts.iter().enumerate().map(
                                    |(index, (name, _part))| {
                                        let on_select = || {
                                            set_selected_part_name.set(Some(name.clone()));
                                        };
                                        (index, name.to_string(), on_select)
                                    },
                                ),
                            };
                            ctx.add(part_column);
                        }),
                        table::ratio(1, |wh, ctx| {
                            let Some(sprite_doc) = selected_sprite_doc.as_ref() else {
                                return;
                            };
                            let Some(selected_part_name) = selected_part_name.as_ref() else {
                                return;
                            };
                            let Sprite::Parts { sprite } = &sprite_doc.sprite else {
                                return;
                            };
                            let Some(part) = sprite.parts.get(selected_part_name) else {
                                return;
                            };
                            let part_option_column = Column {
                                wh,
                                items: part.part_options.iter().enumerate().map(
                                    |(index, part_option)| {
                                        let on_select = || {
                                            select_part(selected_part_name, &part_option.name);
                                        };
                                        (index, part_option.name.to_string(), on_select)
                                    },
                                ),
                            };
                            ctx.add(part_option_column);
                        }),
                    ]),
                ),
            ])(wh, ctx)
        });
    }
}

struct Column<Key, Items, OnSelect>
where
    Key: Into<AddKey>,
    OnSelect: Fn(),
    Items: ExactSizeIterator<Item = (Key, String, OnSelect)>,
{
    wh: Wh<Px>,
    items: Items,
}

impl<Key, Items, OnSelect> Component for Column<Key, Items, OnSelect>
where
    Key: Into<AddKey>,
    OnSelect: Fn(),
    Items: ExactSizeIterator<Item = (Key, String, OnSelect)>,
{
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, items } = self;

        let item_wh = Wh::new(wh.width, 80.px());

        ctx.add(AutoListView {
            height: wh.height,
            scroll_bar_width: 10.px(),
            item_wh,
            items: items.map(|item| {
                let (key, text, on_select) = item;
                (key, move |ctx: &RenderCtx| {
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
                    ctx.add(simple_button(wh, "", move |_| {
                        on_select();
                    }));
                })
            }),
        });
    }
}
