use crate::*;
use list_view::AutoListView;
use luda_rpc::*;
use psd_sprite_util::{get_or_load_psd_sprite, PsdSpriteLoadState};
use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

pub struct SpriteSelectTool<'a> {
    pub wh: Wh<Px>,
    pub asset_docs: Sig<'a, HashMap<u128, AssetDoc>>,
    pub select_sprite: &'a dyn Fn(u128),
    pub select_part_option: &'a dyn Fn(&str, &str, bool),
}

impl Component for SpriteSelectTool<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            asset_docs,
            select_sprite,
            select_part_option,
        } = self;

        let (selected_sprite_id, set_selected_sprite_id) = ctx.state::<Option<u128>>(|| None);
        let (selected_part_name, set_selected_part_name) = ctx.state::<Option<String>>(|| None);
        let (selected_tags, set_selected_tags) =
            ctx.state::<HashSet<AssetSystemTag>>(Default::default);
        let parts = ctx.memo(|| {
            let Some(selected_sprite_id) = selected_sprite_id.deref() else {
                return Default::default();
            };
            let psd_load_state = get_or_load_psd_sprite(*selected_sprite_id);
            let PsdSpriteLoadState::Loaded { psd_sprite, .. } = psd_load_state.as_ref() else {
                return Default::default();
            };
            psd_sprite.parts()
        });

        let tag_filtered_asset_docs = ctx.memo(|| {
            asset_docs
                .iter()
                .filter(|(_id, asset_tag)| {
                    if !matches!(asset_tag.asset_kind, AssetKind::Sprite) {
                        return false;
                    }
                    asset_tag.tags.iter().any(|tag| match tag {
                        AssetTag::System { tag } => selected_tags.contains(tag),
                        AssetTag::Custom { .. } => false,
                    })
                })
                .map(|(id, sprite)| (id.clone(), sprite.clone()))
                .collect::<HashMap<u128, AssetDoc>>()
        });

        let tag_toggle_button = |tag: AssetSystemTag| {
            let is_on = selected_tags.contains(&tag);
            let text = match tag {
                AssetSystemTag::SpriteCharacter => "인물",
                AssetSystemTag::SpriteObject => "사물",
                AssetSystemTag::SpriteBackground => "배경",
                _ => unreachable!(),
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

        ctx.compose(|ctx| {
            table::vertical([
                table::fixed(
                    64.px(),
                    table::horizontal([
                        table::fixed(64.px(), |_, _| {}),
                        tag_toggle_button(AssetSystemTag::SpriteCharacter),
                        table::fixed(16.px(), |_, _| {}),
                        tag_toggle_button(AssetSystemTag::SpriteObject),
                        table::fixed(16.px(), |_, _| {}),
                        tag_toggle_button(AssetSystemTag::SpriteBackground),
                        table::fixed(64.px(), |_, _| {}),
                    ]),
                ),
                table::ratio(
                    1,
                    table::horizontal([
                        table::ratio(1, |wh, ctx| {
                            let sprite_column = Column {
                                wh,
                                items: tag_filtered_asset_docs.iter().map(|(id, sprite)| {
                                    let on_select = || {
                                        set_selected_sprite_id.set(Some(*id));
                                        select_sprite(*id);
                                    };
                                    (sprite.id, sprite.name.to_string(), on_select)
                                }),
                            };
                            ctx.add(sprite_column);
                        }),
                        table::ratio(1, |wh, ctx| {
                            let part_column = Column {
                                wh,
                                items: parts.iter().enumerate().map(|(index, (name, _part))| {
                                    let on_select = || {
                                        set_selected_part_name.set(Some(name.clone()));
                                    };
                                    (index, name.to_string(), on_select)
                                }),
                            };
                            ctx.add(part_column);
                        }),
                        table::ratio(1, |wh, ctx| {
                            let Some(selected_part_name) = selected_part_name.as_ref() else {
                                return;
                            };
                            let Some(part) = parts.get(selected_part_name) else {
                                return;
                            };
                            let part_option_column = Column {
                                wh,
                                items: part.options.iter().enumerate().map(
                                    |(index, option_name)| {
                                        let on_select = move || {
                                            select_part_option(
                                                selected_part_name,
                                                option_name,
                                                part.is_single_select,
                                            );
                                        };
                                        (index, option_name.to_string(), on_select)
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

        let item_wh = Wh::new(wh.width, 48.px());

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
                        y: item_wh.height / 2.0,
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
                    ctx.add(simple_button(item_wh, "", move |_| {
                        on_select();
                    }));
                })
            }),
        });
    }
}
