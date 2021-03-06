use crate::scroll_view;
use namui::prelude::*;

/// ListView is a vertical list view with fixed height items.
#[derive(Debug)]
pub struct ListView {
    scroll_view: scroll_view::ScrollView,
}

pub struct Props<TItem, TIterator, TItems, TItemRender>
where
    TIterator: ExactSizeIterator<Item = TItem>,
    TItems: IntoIterator<Item = TItem, IntoIter = TIterator>,
    TItemRender: Fn(Wh<Px>, TItem) -> RenderingTree,
{
    pub xy: Xy<Px>,
    pub height: Px,
    pub scroll_bar_width: Px,
    pub item_wh: Wh<Px>,
    pub items: TItems,
    pub item_render: TItemRender,
}

pub enum Event {}

impl ListView {
    pub fn new() -> Self {
        Self {
            scroll_view: scroll_view::ScrollView::new(),
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        self.scroll_view.update(event);
    }
    pub fn render<TItem, TIterator, TItems, TItemRender>(
        &self,
        props: Props<TItem, TIterator, TItems, TItemRender>,
    ) -> RenderingTree
    where
        TIterator: ExactSizeIterator<Item = TItem>,
        TItems: IntoIterator<Item = TItem, IntoIter = TIterator>,
        TItemRender: Fn(Wh<Px>, TItem) -> RenderingTree,
    {
        let items_iter = props.items.into_iter();
        let item_len = items_iter.len();

        if item_len == 0 {
            return RenderingTree::Empty;
        }

        let scroll_y = self.scroll_view.scroll_y;
        let visible_item_start_index = (scroll_y / props.item_wh.height).floor() as usize;
        let visible_item_end_index =
            ((scroll_y + props.height) / props.item_wh.height).ceil() as usize;
        let visible_item_count = visible_item_end_index - visible_item_start_index + 1;

        let visible_items = items_iter
            .enumerate()
            .skip(visible_item_start_index)
            .take(visible_item_count);

        let rendered_items = visible_items.map(|(index, item)| {
            translate(
                px(0.0),
                index * props.item_wh.height,
                (props.item_render)(props.item_wh, item),
            )
        });

        let content_height = item_len * props.item_wh.height;

        let transparent_pillar = rect(RectParam {
            rect: Rect::Xywh {
                x: px(0.0),
                y: px(0.0),
                width: props.item_wh.width,
                height: content_height,
            },
            style: RectStyle {
                fill: Some(RectFill {
                    color: Color::TRANSPARENT,
                }),
                ..Default::default()
            },
        });

        let content = namui::render![transparent_pillar, namui::render(rendered_items),];

        self.scroll_view.render(&scroll_view::Props {
            xy: props.xy,
            height: props.height,
            scroll_bar_width: props.scroll_bar_width,
            content,
        })
    }
}

#[allow(dead_code)]
fn test_props_passing() {
    let list_view = ListView::new();
    let items = [1, 2, 3, 4, 5];
    let _props_with_enumerate_items = list_view.render(Props {
        xy: Xy {
            x: px(0.0),
            y: px(0.0),
        },
        height: px(100.0),
        scroll_bar_width: px(10.0),
        item_wh: Wh::new(px(100.0), px(100.0)),
        items: items.iter().enumerate(),
        item_render: |_wh, (_index, _item)| namui::render![],
    });
    let _props_with_slice_iter = list_view.render(Props {
        xy: Xy {
            x: px(0.0),
            y: px(0.0),
        },
        height: px(100.0),
        scroll_bar_width: px(10.0),
        item_wh: Wh::new(px(100.0), px(100.0)),
        items: items.iter(),
        item_render: |_wh, _item| namui::render![],
    });
    let _props_with_reference_of_slice = list_view.render(Props {
        xy: Xy {
            x: px(0.0),
            y: px(0.0),
        },
        height: px(100.0),
        scroll_bar_width: px(10.0),
        item_wh: Wh::new(px(100.0), px(100.0)),
        items: &items,
        item_render: |_wh, _item| namui::render![],
    });
}
