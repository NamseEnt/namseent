// use crate::scroll_view;
// use namui::prelude::*;
// use std::sync::{Arc, Mutex};

// #[derive(Debug, Clone)]
// pub struct VHListView {
//     scroll_view: scroll_view::ScrollView,
//     requested_scroll_y: Arc<Mutex<Option<Px>>>,
// }

// pub struct Props<TItem, TIterator, TItems, TItemHeight, TItemRender>
// where
//     TIterator: Iterator<Item = TItem>,
//     TItems: IntoIterator<Item = TItem, IntoIter = TIterator>,
//     TItemHeight: Fn(&TItem) -> Px,
//     TItemRender: Fn(Wh<Px>, TItem) -> RenderingTree,
// {
//     pub xy: Xy<Px>,
//     pub wh: Wh<Px>,
//     pub scroll_bar_width: Px,
//     pub items: TItems,
//     pub item_height: TItemHeight,
//     pub item_render: TItemRender,
// }

// impl VHListView {
//     pub fn new() -> Self {
//         Self {
//             scroll_view: scroll_view::ScrollView::new(),
//             requested_scroll_y: Arc::new(Mutex::new(None)),
//         }
//     }
//     /// This will scroll on next rendering stage.
//     pub fn scroll_to(&mut self, scroll_y: Px) {
//         *self.requested_scroll_y.lock().unwrap() = Some(scroll_y);
//     }
//     pub fn render<TItem, TIterator, TItems, TItemHeight, TItemRender>(
//         &self,
//         props: Props<TItem, TIterator, TItems, TItemHeight, TItemRender>,
//     ) -> RenderingTree
//     where
//         TIterator: Iterator<Item = TItem>,
//         TItems: IntoIterator<Item = TItem, IntoIter = TIterator>,
//         TItemHeight: Fn(&TItem) -> Px,
//         TItemRender: Fn(Wh<Px>, TItem) -> RenderingTree,
//     {
//         let items_iter = props.items.into_iter();
//         let items = items_iter.collect::<Vec<_>>();

//         if items.len() == 0 {
//             return RenderingTree::Empty;
//         }

//         let total_item_height = items.iter().map(|item| (props.item_height)(item)).sum();

//         let max_scroll_y = total_item_height - props.wh.height;
//         let scroll_y = {
//             let mut y_guard = self.requested_scroll_y.lock().unwrap();
//             (if let Some(scroll_y) = y_guard.clone() {
//                 namui::event::send(scroll_view::Event::Scrolled(self.scroll_view.id, scroll_y));
//                 *y_guard = None;
//                 scroll_y
//             } else {
//                 self.scroll_view.scroll_y
//             })
//             .min(max_scroll_y)
//         };

//         let rendered_items = {
//             let mut rendered_items = vec![];
//             let mut bottom = 0.px();
//             for item in items {
//                 let top = bottom;
//                 let item_height = (props.item_height)(&item);
//                 bottom = top + item_height;

//                 if bottom < scroll_y {
//                     continue;
//                 }
//                 if top > scroll_y + props.wh.height {
//                     break;
//                 }
//                 let wh = Wh::new(props.wh.width, item_height);
//                 rendered_items.push(translate(0.px(), top, (props.item_render)(wh, item)));
//             }
//             rendered_items
//         };

//         let transparent_pillar = rect(RectParam {
//             rect: Rect::Xywh {
//                 x: px(0.0),
//                 y: px(0.0),
//                 width: props.wh.width,
//                 height: total_item_height,
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
//             height: props.wh.height,
//             scroll_bar_width: props.scroll_bar_width,
//             content,
//         })
//     }
// }

// #[allow(dead_code)]
// fn test_props_passing() {
//     let list_view = VHListView::new();
//     let items = [1, 2, 3, 4, 5];
//     let _props_with_enumerate_items = list_view.render(Props {
//         xy: Xy {
//             x: 0.0.px(),
//             y: 0.0.px(),
//         },
//         wh: Wh {
//             width: 100.0.px(),
//             height: 100.0.px(),
//         },
//         scroll_bar_width: px(10.0),
//         item_height: |_item| 100.px(),
//         items: items.iter().enumerate(),
//         item_render: |_wh, (_index, _item)| namui::render![],
//     });
//     let _props_with_slice_iter = list_view.render(Props {
//         xy: Xy {
//             x: px(0.0),
//             y: px(0.0),
//         },
//         wh: Wh {
//             width: 100.0.px(),
//             height: 100.0.px(),
//         },
//         scroll_bar_width: px(10.0),
//         item_height: |_item| 100.px(),
//         items: items.iter(),
//         item_render: |_wh, _item| namui::render![],
//     });
//     let _props_with_reference_of_slice = list_view.render(Props {
//         xy: Xy {
//             x: px(0.0),
//             y: px(0.0),
//         },
//         wh: Wh {
//             width: 100.0.px(),
//             height: 100.0.px(),
//         },
//         scroll_bar_width: px(10.0),
//         item_height: |_item| 100.px(),
//         items: &items,
//         item_render: |_wh, _item| namui::render![],
//     });
// }
