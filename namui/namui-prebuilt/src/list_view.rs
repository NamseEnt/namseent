use crate::scroll_view::{self};
use namui::*;

pub struct AutoListView<Items, Key, C>
where
    C: Component,
    Key: Into<AddKey>,
    Items: ExactSizeIterator<Item = (Key, C)>,
{
    pub height: Px,
    pub scroll_bar_width: Px,
    pub item_wh: Wh<Px>,
    pub items: Items,
}

impl<Items, Key, C> Component for AutoListView<Items, Key, C>
where
    C: Component,
    Key: Into<AddKey>,
    Items: ExactSizeIterator<Item = (Key, C)>,
{
    fn render(self, ctx: &RenderCtx) {
        let Self {
            height,
            scroll_bar_width,
            item_wh,
            items,
        } = self;
        let (scroll_y, set_scroll_y) = ctx.state(|| 0.px());

        ctx.add(ListView {
            scroll_y: *scroll_y,
            set_scroll_y,
            height,
            scroll_bar_width,
            item_wh,
            items,
        });
    }
}

pub struct AutoListViewWithCtx<Key, Item, Items, RenderFunc>
where
    Key: Into<AddKey>,
    RenderFunc: Fn(Item, ComposeCtx),
    Items: ExactSizeIterator<Item = (Key, Item)>,
{
    pub height: Px,
    pub scroll_bar_width: Px,
    pub item_wh: Wh<Px>,
    pub items: Items,
    pub item_render: RenderFunc,
}

impl<Key, Item, Items, RenderFunc> Component for AutoListViewWithCtx<Key, Item, Items, RenderFunc>
where
    Key: Into<AddKey>,
    RenderFunc: Fn(Item, ComposeCtx),
    Items: ExactSizeIterator<Item = (Key, Item)>,
{
    fn render(self, ctx: &RenderCtx) {
        let Self {
            height,
            scroll_bar_width,
            item_wh,
            items,
            item_render,
        } = self;
        let (scroll_y, set_scroll_y) = ctx.state(|| 0.px());

        ctx.add(ListViewWithCtx {
            height,
            scroll_bar_width,
            item_wh,
            items,
            scroll_y: *scroll_y,
            set_scroll_y,
            item_render,
        });
    }
}

pub struct ListView<Items, Key, C>
where
    C: Component,
    Key: Into<AddKey>,
    Items: ExactSizeIterator<Item = (Key, C)>,
{
    pub height: Px,
    pub scroll_bar_width: Px,
    pub item_wh: Wh<Px>,
    pub items: Items,
    pub scroll_y: Px,
    pub set_scroll_y: SetState<Px>,
}

impl<Items, Key, C> Component for ListView<Items, Key, C>
where
    C: Component,
    Key: Into<AddKey>,
    Items: ExactSizeIterator<Item = (Key, C)>,
{
    fn render(self, ctx: &RenderCtx) {
        let Self {
            height,
            scroll_bar_width,
            item_wh,
            items,
            scroll_y,
            set_scroll_y,
        } = self;

        ctx.add(ListViewWithCtx {
            height,
            scroll_bar_width,
            item_wh,
            items,
            scroll_y,
            set_scroll_y,
            item_render: |component, ctx| {
                ctx.add(component);
            },
        });
    }
}

pub struct ListViewWithCtx<Key, Item, Items, RenderFunc>
where
    Key: Into<AddKey>,
    RenderFunc: Fn(Item, ComposeCtx),
    Items: ExactSizeIterator<Item = (Key, Item)>,
{
    pub height: Px,
    pub scroll_bar_width: Px,
    pub item_wh: Wh<Px>,
    pub items: Items,
    pub scroll_y: Px,
    pub set_scroll_y: SetState<Px>,
    pub item_render: RenderFunc,
}

impl<Key, Item, Items, RenderFunc> Component for ListViewWithCtx<Key, Item, Items, RenderFunc>
where
    Key: Into<AddKey>,
    RenderFunc: Fn(Item, ComposeCtx),
    Items: ExactSizeIterator<Item = (Key, Item)>,
{
    fn render(self, ctx: &RenderCtx) {
        let Self {
            height,
            scroll_bar_width,
            item_wh,
            items,
            scroll_y,
            set_scroll_y,
            item_render,
        } = self;

        ctx.add(scroll_view::ScrollView {
            wh: Wh::new(item_wh.width, height),
            scroll_bar_width,
            content: ListViewInnerWithCtx {
                height,
                item_wh,
                items,
                scroll_y,
                item_render,
            },
            scroll_y,
            set_scroll_y,
        });
    }
}

struct ListViewInnerWithCtx<Key, Item, Items, RenderFunc>
where
    Key: Into<AddKey>,
    RenderFunc: Fn(Item, ComposeCtx),
    Items: ExactSizeIterator<Item = (Key, Item)>,
{
    height: Px,
    item_wh: Wh<Px>,
    items: Items,
    scroll_y: Px,
    item_render: RenderFunc,
}

impl<Key, Item, Items, RenderFunc> Component for ListViewInnerWithCtx<Key, Item, Items, RenderFunc>
where
    Key: Into<AddKey>,
    RenderFunc: Fn(Item, ComposeCtx),
    Items: ExactSizeIterator<Item = (Key, Item)>,
{
    fn render(self, ctx: &RenderCtx) {
        let Self {
            height,
            item_wh,
            items,
            scroll_y,
            item_render,
        } = self;

        let item_len = items.len();

        if item_len == 0 {
            return;
        }
        let max_scroll_y = item_wh.height * item_len - height;

        let scroll_y = scroll_y.min(max_scroll_y);

        let visible_item_start_index = (scroll_y / item_wh.height).floor() as usize;
        let visible_item_end_index = ((scroll_y + height) / item_wh.height).ceil() as usize;
        let visible_item_count = visible_item_end_index - visible_item_start_index + 1;

        let visible_items = items
            .into_iter()
            .skip(visible_item_start_index)
            .take(visible_item_count);

        let content_height = item_wh.height * item_len;

        let transparent_pillar = rect(RectParam {
            rect: Rect::Xywh {
                x: px(0.0),
                y: px(0.0),
                width: item_wh.width,
                height: content_height,
            },
            style: RectStyle {
                fill: Some(RectFill {
                    color: Color::TRANSPARENT,
                }),
                ..Default::default()
            },
        });

        ctx.add(transparent_pillar);

        let max_scroll_y = item_wh.height * item_len - height;

        let scroll_y = scroll_y.min(max_scroll_y);

        let visible_item_start_index = (scroll_y / item_wh.height).floor() as usize;

        ctx.compose(|ctx| {
            for (index, (key, item)) in visible_items.into_iter().enumerate() {
                let absolute_index = index + visible_item_start_index;
                ctx.compose_with_key(key, |ctx| {
                    let ctx = ctx.translate((0.px(), item_wh.height * absolute_index));
                    item_render(item, ctx);
                });
            }
        });
    }
}
