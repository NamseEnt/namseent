use crate::scroll_view::{self};
use namui::prelude::*;
use std::sync::Arc;

#[namui::component]
pub struct ListView<C: Component> {
    pub xy: Xy<Px>,
    pub height: Px,
    pub scroll_bar_width: Px,
    pub item_wh: Wh<Px>,
    pub items: Vec<C>,
}

impl<C: Component> Component for ListView<C> {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let &Self {
            xy,
            height,
            scroll_bar_width,
            item_wh,
            ref items,
        } = self;
        let (scroll_y, set_scroll_y) = ctx.use_state(|| 0.px());

        ctx.add(scroll_view::ScrollView {
            xy,
            scroll_bar_width,
            height,
            content: Arc::new(ListViewInner {
                height,
                item_wh,
                items: items.clone(),
                scroll_y: *scroll_y,
            }),
            scroll_y: *scroll_y,
            set_scroll_y,
        });

        ctx.done()
    }
}

#[namui::component]
struct ListViewInner<'a, C: Component> {
    height: Px,
    item_wh: Wh<Px>,
    items: &'a Vec<C>,
    scroll_y: Px,
}

impl<C: Component> Component for ListViewInner<'_, C> {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let &Self {
            height,
            item_wh,
            ref items,
            scroll_y,
        } = self;

        let item_len = items.len();

        if item_len == 0 {
            return ctx.done();
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

        for visible_item in visible_items.into_iter() {
            ctx.add(visible_item)
        }

        ctx.done_with_rendering_tree(move |children| {
            let max_scroll_y = item_wh.height * item_len - height;

            let scroll_y = scroll_y.min(max_scroll_y);

            let visible_item_start_index = (scroll_y / item_wh.height).floor() as usize;

            let visible_rendering_tree =
                namui::render(children.into_iter().enumerate().map(|(index, child)| {
                    translate(
                        px(0.0),
                        item_wh.height * (index + visible_item_start_index),
                        child,
                    )
                }));

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

            namui::render![transparent_pillar, visible_rendering_tree]
        })
    }
}

// /// ListView is a vertical list view with fixed height items.
// #[derive(Debug, Clone)]
// pub struct ListView {
//     scroll_view: scroll_view::ScrollView,
//     requested_scroll_index: Arc<Mutex<Option<usize>>>,
// }

// impl PartialEq for ListView {
//     fn eq(&self, other: &Self) -> bool {
//         self.scroll_view == other.scroll_view
//             && self
//                 .requested_scroll_index
//                 .lock()
//                 .unwrap()
//                 .eq(&other.requested_scroll_index.lock().unwrap())
//     }
// }

// pub struct Props<TItem, TIterator, TItems, TItemRender>
// where
//     TIterator: ExactSizeIterator<Item = TItem>,
//     TItems: IntoIterator<Item = TItem, IntoIter = TIterator>,
//     TItemRender: Fn(Wh<Px>, TItem) -> RenderingTree,
// {
//     pub xy: Xy<Px>,
//     pub height: Px,
//     pub scroll_bar_width: Px,
//     pub item_wh: Wh<Px>,
//     pub items: TItems,
//     pub item_render: TItemRender,
// }

// pub enum Event {}

// impl ListView {
//     pub fn new() -> Self {
//         Self {
//             scroll_view: scroll_view::ScrollView::new(),
//             requested_scroll_index: Arc::new(Mutex::new(None)),
//         }
//     }
//     pub fn update(&mut self, event: &namui::Event) {
//         self.scroll_view.update(event);
//     }
//     /// This will scroll on next rendering stage.
//     pub fn scroll_to(&mut self, index: usize) {
//         *self.requested_scroll_index.lock().unwrap() = Some(index);
//     }
//     pub fn render<TItem, TIterator, TItems, TItemRender>(
//         &self,
//         props: Props<TItem, TIterator, TItems, TItemRender>,
//     ) -> RenderingTree
//     where
//         TIterator: ExactSizeIterator<Item = TItem>,
//         TItems: IntoIterator<Item = TItem, IntoIter = TIterator>,
//         TItemRender: Fn(Wh<Px>, TItem) -> RenderingTree,
//     {
//         let items_iter = props.items.into_iter();
//         let item_len = items_iter.len();

//         if item_len == 0 {
//             return RenderingTree::Empty;
//         }

//         let max_scroll_y = props.item_wh.height * item_len - props.height;
//         let scroll_y = {
//             let mut index_guard = self.requested_scroll_index.lock().unwrap();
//             (if let Some(index) = index_guard.as_ref() {
//                 let scroll_y = props.item_wh.height * (*index);
//                 namui::event::send(scroll_view::Event::Scrolled(
//                     self.scroll_view.id.clone(),
//                     scroll_y,
//                 ));
//                 *index_guard = None;
//                 scroll_y
//             } else {
//                 self.scroll_view.scroll_y
//             })
//             .min(max_scroll_y)
//         };

//         let visible_item_start_index = (scroll_y / props.item_wh.height).floor() as usize;
//         let visible_item_end_index =
//             ((scroll_y + props.height) / props.item_wh.height).ceil() as usize;
//         let visible_item_count = visible_item_end_index - visible_item_start_index + 1;

//         let visible_items = items_iter
//             .enumerate()
//             .skip(visible_item_start_index)
//             .take(visible_item_count);

//         let rendered_items = visible_items.map(|(index, item)| {
//             translate(
//                 px(0.0),
//                 props.item_wh.height * index,
//                 (props.item_render)(props.item_wh, item),
//             )
//         });

//         let content_height = props.item_wh.height * item_len;

//         let transparent_pillar = rect(RectParam {
//             rect: Rect::Xywh {
//                 x: px(0.0),
//                 y: px(0.0),
//                 width: props.item_wh.width,
//                 height: content_height,
//             },
//             style: RectStyle {
//                 fill: Some(RectFill {
//                     color: Color::TRANSPARENT,
//                 }),
//                 ..Default::default()
//             },
//         });

//         let content = namui::render![transparent_pillar, namui::render(rendered_items),];

//         self.scroll_view.render(&scroll_view::Props {
//             xy: props.xy,
//             height: props.height,
//             scroll_bar_width: props.scroll_bar_width,
//             content,
//         })
//     }
// }

// #[allow(dead_code)]
// fn test_props_passing() {
//     let list_view = ListView::new();
//     let items = [1, 2, 3, 4, 5];
//     let _props_with_enumerate_items = list_view.render(Props {
//         xy: Xy {
//             x: px(0.0),
//             y: px(0.0),
//         },
//         height: px(100.0),
//         scroll_bar_width: px(10.0),
//         item_wh: Wh::new(px(100.0), px(100.0)),
//         items: items.iter().enumerate(),
//         item_render: |_wh, (_index, _item)| namui::render![],
//     });
//     let _props_with_slice_iter = list_view.render(Props {
//         xy: Xy {
//             x: px(0.0),
//             y: px(0.0),
//         },
//         height: px(100.0),
//         scroll_bar_width: px(10.0),
//         item_wh: Wh::new(px(100.0), px(100.0)),
//         items: items.iter(),
//         item_render: |_wh, _item| namui::render![],
//     });
//     let _props_with_reference_of_slice = list_view.render(Props {
//         xy: Xy {
//             x: px(0.0),
//             y: px(0.0),
//         },
//         height: px(100.0),
//         scroll_bar_width: px(10.0),
//         item_wh: Wh::new(px(100.0), px(100.0)),
//         items: &items,
//         item_render: |_wh, _item| namui::render![],
//     });
// }
