use crate::scroll_view::{self};
use namui::prelude::*;

#[namui::component]
pub struct AutoListView<C: Component> {
    pub height: Px,
    pub scroll_bar_width: Px,
    pub item_wh: Wh<Px>,
    pub items: Vec<(String, C)>,
}

impl<C: Component> Component for AutoListView<C> {
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

#[namui::component]
pub struct ListView<'a, C: Component> {
    pub height: Px,
    pub scroll_bar_width: Px,
    pub item_wh: Wh<Px>,
    pub items: Vec<(String, C)>,
    pub scroll_y: Px,
    pub set_scroll_y: SetState<'a, Px>,
}

impl<C: Component> Component for ListView<'_, C> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            height,
            scroll_bar_width,
            item_wh,
            items,
            scroll_y,
            set_scroll_y,
        } = self;

        ctx.add(scroll_view::ScrollView {
            wh: Wh::new(item_wh.width, height),
            scroll_bar_width,
            content: ListViewInner {
                height,
                item_wh,
                items,
                scroll_y,
            },
            scroll_y,
            set_scroll_y,
        });
    }
}

#[namui::component]
struct ListViewInner<C: Component> {
    height: Px,
    item_wh: Wh<Px>,
    items: Vec<(String, C)>,
    scroll_y: Px,
}

impl<C: Component> Component for ListViewInner<C> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            height,
            item_wh,
            items,
            scroll_y,
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
            for (index, (key, visible_item)) in visible_items.into_iter().enumerate() {
                ctx.compose_with_key(key, |ctx| {
                    ctx.translate((0.px(), item_wh.height * (index + visible_item_start_index)))
                        .add(visible_item);
                });
            }
        });
    }
}
