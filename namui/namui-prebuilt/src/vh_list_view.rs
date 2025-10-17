use crate::scroll_view::ScrollView;
use namui::*;
use std::fmt::Debug;

type ItemRenderFn<'a, TItem> = Box<dyn 'a + Fn(Wh<Px>, TItem, ComposeCtx)>;

/// Auto Variable Height List View
pub struct AutoVHListView<'a, TItem, TIterator, TItems>
where
    TIterator: Iterator<Item = TItem>,
    TItems: IntoIterator<Item = TItem, IntoIter = TIterator> + Debug,
{
    pub wh: Wh<Px>,
    pub scroll_bar_width: Px,
    pub items: TItems,
    pub item_height: Box<dyn 'a + Fn(&TItem) -> Px>,
    pub item_render: ItemRenderFn<'a, TItem>,
}
impl<TItem, TIterator, TItems> Component for AutoVHListView<'_, TItem, TIterator, TItems>
where
    TItem: Debug,
    TIterator: Iterator<Item = TItem>,
    TItems: IntoIterator<Item = TItem, IntoIter = TIterator> + Debug,
{
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            scroll_bar_width,
            items,
            item_height,
            item_render,
        } = self;

        let (scroll_y, set_scroll_y) = ctx.state(|| 0.px());

        ctx.add(VHListView {
            wh,
            scroll_bar_width,
            items,
            item_height,
            item_render,
            scroll_y: *scroll_y,
            set_scroll_y,
        });
    }
}

/// Variable Height List View
pub struct VHListView<'a, TItem, TIterator, TItems>
where
    TIterator: Iterator<Item = TItem>,
    TItems: IntoIterator<Item = TItem, IntoIter = TIterator> + Debug,
{
    pub wh: Wh<Px>,
    pub scroll_bar_width: Px,
    pub items: TItems,
    pub item_height: Box<dyn 'a + Fn(&TItem) -> Px>,
    pub item_render: ItemRenderFn<'a, TItem>,
    pub scroll_y: Px,
    pub set_scroll_y: SetState<Px>,
}
impl<TItem, TIterator, TItems> Component for VHListView<'_, TItem, TIterator, TItems>
where
    TItem: Debug,
    TIterator: Iterator<Item = TItem>,
    TItems: IntoIterator<Item = TItem, IntoIter = TIterator> + Debug,
{
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            scroll_bar_width,
            items,
            item_height,
            item_render,
            scroll_y,
            set_scroll_y,
        } = self;

        let items_iter = items.into_iter();
        let items = items_iter.collect::<Vec<_>>();

        ctx.compose(|ctx| {
            if items.is_empty() {
                return;
            }

            let content = Content {
                wh,
                items,
                item_height,
                item_render,
                scroll_y,
            };

            ctx.add(ScrollView {
                wh,
                scroll_bar_width,
                content,
                scroll_y,
                set_scroll_y,
            });
        });
    }
}

struct Content<'a, TItem, TIterator, TItems>
where
    TIterator: Iterator<Item = TItem>,
    TItems: IntoIterator<Item = TItem, IntoIter = TIterator> + Debug,
{
    pub wh: Wh<Px>,
    pub items: TItems,
    pub item_height: Box<dyn 'a + Fn(&TItem) -> Px>,
    pub item_render: ItemRenderFn<'a, TItem>,
    pub scroll_y: Px,
}
impl<TItem, TIterator, TItems> Component for Content<'_, TItem, TIterator, TItems>
where
    TIterator: Iterator<Item = TItem>,
    TItems: IntoIterator<Item = TItem, IntoIter = TIterator> + Debug,
{
    fn render(self, ctx: &RenderCtx) {
        let Self {
            items,
            wh,
            item_height,
            item_render,
            scroll_y,
        } = self;

        ctx.compose(|ctx| {
            let items_iter = items.into_iter();
            let items = items_iter.collect::<Vec<_>>();

            if items.is_empty() {
                return;
            }

            let total_item_height = items.iter().map(&item_height).sum();
            let max_scroll_y = total_item_height - wh.height;
            let scroll_y = scroll_y.clamp(0.px(), 0.px().max(max_scroll_y));

            let mut bottom = 0.px();
            for item in items {
                let top = bottom;
                let item_height = (item_height)(&item);
                bottom = top + item_height;

                if bottom < scroll_y {
                    continue;
                }
                if top > scroll_y + wh.height {
                    break;
                }
                let wh = Wh::new(wh.width, item_height);

                (item_render)(wh, item, ctx.translate((0.px(), top)));
            }

            ctx.add(rect(RectParam {
                rect: Rect::Xywh {
                    x: px(0.0),
                    y: px(0.0),
                    width: wh.width,
                    height: total_item_height,
                },
                style: RectStyle {
                    fill: Some(RectFill {
                        color: Color::TRANSPARENT,
                    }),
                    ..Default::default()
                },
            }));
        });
    }
}
