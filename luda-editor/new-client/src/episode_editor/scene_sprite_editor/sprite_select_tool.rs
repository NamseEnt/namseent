use crate::*;
use list_view::AutoListView;
use luda_rpc::*;

pub struct SpriteSelectTool<'a> {
    pub wh: Wh<Px>,
    pub sprite_docs: &'a [SpriteDoc],
}

impl Component for SpriteSelectTool<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, sprite_docs } = self;

        let (selected_sprite_index, set_selected_sprite_index) =
            ctx.state::<Option<usize>>(|| None);
        let (selected_part_name, set_selected_part_name) = ctx.state::<Option<String>>(|| None);

        ctx.compose(|ctx| {
            table::horizontal([
                table::ratio(1, |wh, ctx| {
                    let sprite_column = Column {
                        wh,
                        items: sprite_docs.into_iter().enumerate().map(|(index, sprite)| {
                            let preview = |wh: Wh<Px>, ctx: &ComposeCtx| todo!();
                            let on_select = || todo!();
                            (
                                sprite.id.as_str(),
                                preview,
                                sprite.sprite.name().to_string(),
                                on_select,
                            )
                        }),
                    };
                    ctx.add(sprite_column);
                }),
                table::ratio(1, |wh, ctx| {
                    let Some(selected_sprite_index) = selected_sprite_index.clone_inner() else {
                        return;
                    };
                    let Some(sprite_doc) = sprite_docs.get(selected_sprite_index) else {
                        return;
                    };
                    let Sprite::Parts { sprite } = &sprite_doc.sprite else {
                        return;
                    };
                    let part_column = Column {
                        wh,
                        items: sprite
                            .parts
                            .iter()
                            .enumerate()
                            .map(|(index, (name, part))| {
                                let preview = |wh: Wh<Px>, ctx: &ComposeCtx| todo!();
                                let on_select = || todo!();
                                (index, preview, name.to_string(), on_select)
                            }),
                    };
                    ctx.add(part_column);
                }),
                table::ratio(1, |wh, ctx| {
                    let Some(selected_sprite_index) = selected_sprite_index.clone_inner() else {
                        return;
                    };
                    let Some(selected_part_name) = selected_part_name.as_ref() else {
                        return;
                    };
                    let Some(sprite_doc) = sprite_docs.get(selected_sprite_index) else {
                        return;
                    };
                    let Sprite::Parts { sprite } = &sprite_doc.sprite else {
                        return;
                    };
                    let Some(part) = sprite.parts.get(selected_part_name) else {
                        return;
                    };
                    let part_option_column =
                        Column {
                            wh,
                            items: part.part_options.iter().enumerate().map(
                                |(index, part_option)| {
                                    let preview = |wh: Wh<Px>, ctx: &ComposeCtx| todo!();
                                    let on_select = || todo!();
                                    (index, preview, part_option.name.to_string(), on_select)
                                },
                            ),
                        };
                    ctx.add(part_option_column);
                }),
            ])(wh, ctx)
        });
    }
}

struct Column<Key, Items, Preview, OnSelect>
where
    Key: Into<AddKey>,
    Preview: Fn(Wh<Px>, &ComposeCtx),
    OnSelect: Fn(),
    Items: ExactSizeIterator<Item = (Key, Preview, String, OnSelect)>,
{
    wh: Wh<Px>,
    items: Items,
}

impl<Key, Items, Preview, OnSelect> Component for Column<Key, Items, Preview, OnSelect>
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
