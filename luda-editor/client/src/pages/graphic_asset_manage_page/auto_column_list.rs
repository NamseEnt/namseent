use crate::color;
use namui::prelude::*;
use namui_prebuilt::{scroll_view, simple_rect, table::hooks::*, typography};
use std::fmt::Debug;

const THUMBNAIL_WH: Wh<Px> = Wh {
    width: px(144.0),
    height: px(144.0),
};
const VERTICAL_MARGIN: Px = px(8.0);
const MINIMUM_SIDE_MARGIN: Px = px(8.0);
const NAME_MIN_WIDTH: Px = px(128.0);
const NAME_HEIGHT: Px = px(48.0);

#[component]
pub(super) struct AutoColumnList<'a, T>
where
    T: Debug,
{
    pub wh: Wh<Px>,
    pub items: Sig<'a, Vec<T>>,
    pub name_specifier: &'a dyn Fn(&T) -> String,
    pub thumbnail_renderer: &'a dyn Fn(&T, Wh<Px>, &mut ComposeCtx),
}

impl<T> Component for AutoColumnList<'_, T>
where
    T: Debug,
{
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            items,
            name_specifier,
            thumbnail_renderer,
        } = self;

        let wh = ctx.track_eq(&wh);
        let item_wh = ctx.memo(|| Wh {
            width: (THUMBNAIL_WH.width + MINIMUM_SIDE_MARGIN * 2.0).max(NAME_MIN_WIDTH),
            height: THUMBNAIL_WH.height + NAME_HEIGHT + VERTICAL_MARGIN * 3.0,
        });
        let max_items_per_row = ctx.memo(|| (wh.width / item_wh.width).floor() as usize);
        let side_margin = ctx.memo(|| {
            (wh.width - item_wh.width * *max_items_per_row) / (*max_items_per_row as f32 * 2.0)
        });

        let image_rows = items.chunks(*max_items_per_row).map(|items_in_row| {
            fixed(item_wh.height, {
                horizontal(items_in_row.iter().map(|item| {
                    fixed(item_wh.width + *side_margin * 2.0, {
                        horizontal_padding(*side_margin + MINIMUM_SIDE_MARGIN, move |wh, ctx| {
                            vertical_padding(VERTICAL_MARGIN, move |wh, ctx| {
                                vertical([
                                    fixed(THUMBNAIL_WH.height, |wh, ctx| {
                                        thumbnail_renderer(item, wh, ctx);
                                    }),
                                    ratio(1, |_, _| {}),
                                    fixed(NAME_HEIGHT, |wh, ctx| {
                                        ctx.add(Name {
                                            wh,
                                            name: name_specifier(item),
                                        });
                                    }),
                                ])(wh, ctx)
                            })(wh, ctx)
                        })
                    })
                }))
            })
        });

        ctx.component(scroll_view::AutoScrollViewWithCtx {
            scroll_bar_width: 4.px(),
            wh: *wh,
            content: |ctx| vertical(image_rows)(*wh, ctx),
        });

        ctx.component(simple_rect(
            *wh,
            color::STROKE_NORMAL,
            1.px(),
            color::BACKGROUND,
        ));

        ctx.done()
    }
}

#[component]
struct Name {
    wh: Wh<Px>,
    name: String,
}
impl Component for Name {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh, name } = self;

        ctx.component(text(TextParam {
            text: name,
            x: wh.width / 2.0,
            y: 0.0.px(),
            align: TextAlign::Center,
            baseline: TextBaseline::Top,
            font: Font {
                name: "NotoSansKR-Regular".to_string(),
                size: typography::body::FONT_SIZE,
            },
            style: TextStyle {
                border: None,
                drop_shadow: None,
                color: color::STROKE_NORMAL,
                background: None,
                line_height_percent: 125.percent(),
                underline: None,
            },
            max_width: Some(wh.width),
        }));

        ctx.done()
    }
}
