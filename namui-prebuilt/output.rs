#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod button {
    use crate::{simple_rect, typography::center_text_full_height};
    use namui::prelude::*;
    fn attach_text_button_event(
        button: RenderingTree,
        mouse_buttons: impl IntoIterator<Item = MouseButton>,
        on_mouse_up_in: impl Fn(MouseEvent),
    ) -> RenderingTree {
        let mouse_buttons = mouse_buttons.into_iter().collect::<Vec<_>>();
        button
            .attach_event(|builder| {
                builder
                    .on_mouse_up_in(move |event: MouseEvent| {
                        let Some(button) = event.button else { return;
                    };
                        if mouse_buttons.contains(&button) {
                            on_mouse_up_in(event);
                        }
                    });
            })
    }
    pub fn text_button(
        rect: Rect<Px>,
        text: &str,
        text_color: Color,
        stroke_color: Color,
        stroke_width: Px,
        fill_color: Color,
        mouse_buttons: impl IntoIterator<Item = MouseButton>,
        on_mouse_up_in: impl Fn(MouseEvent),
    ) -> namui::RenderingTree {
        attach_text_button_event(
            translate(
                rect.x(),
                rect.y(),
                render([
                    simple_rect(rect.wh(), stroke_color, stroke_width, fill_color),
                    center_text_full_height(rect.wh(), text, text_color),
                ]),
            ),
            mouse_buttons,
            on_mouse_up_in,
        )
    }
    pub fn text_button_fit(
        height: Px,
        text: &str,
        text_color: Color,
        stroke_color: Color,
        stroke_width: Px,
        fill_color: Color,
        side_padding: Px,
        mouse_buttons: impl IntoIterator<Item = MouseButton>,
        on_mouse_up_in: impl Fn(MouseEvent) + 'static,
    ) -> namui::RenderingTree {
        let mouse_buttons = mouse_buttons.into_iter().collect::<Vec<_>>();
        let center_text = center_text_full_height(
            Wh::new(0.px(), height),
            text,
            text_color,
        );
        let width = match center_text.get_bounding_box() {
            Some(bounding_box) => bounding_box.width(),
            None => return RenderingTree::Empty,
        };
        attach_text_button_event(
            render([
                simple_rect(
                    Wh::new(width + side_padding * 2, height),
                    stroke_color,
                    stroke_width,
                    fill_color,
                ),
                translate(width / 2 + side_padding, 0.px(), center_text),
            ]),
            mouse_buttons,
            on_mouse_up_in,
        )
    }
    pub fn body_text_button(
        rect: Rect<Px>,
        text: &str,
        text_color: Color,
        stroke_color: Color,
        stroke_width: Px,
        fill_color: Color,
        text_align: TextAlign,
        mouse_buttons: impl IntoIterator<Item = MouseButton>,
        on_mouse_up_in: impl Fn(MouseEvent) + 'static,
    ) -> namui::RenderingTree {
        attach_text_button_event(
            translate(
                rect.x(),
                rect.y(),
                render([
                    simple_rect(rect.wh(), stroke_color, stroke_width, fill_color),
                    match text_align {
                        TextAlign::Left => {
                            crate::typography::body::left(
                                rect.wh().height,
                                text,
                                text_color,
                            )
                        }
                        TextAlign::Center => {
                            crate::typography::body::center(rect.wh(), text, text_color)
                        }
                        TextAlign::Right => {
                            crate::typography::body::right(rect.wh(), text, text_color)
                        }
                    },
                ]),
            ),
            mouse_buttons,
            on_mouse_up_in,
        )
    }
}
pub mod dropdown {}
mod event_trap {
    use namui::prelude::*;
    pub fn event_trap(content: RenderingTree) -> RenderingTree {
        content
            .attach_event(move |builder| {
                builder
                    .on_mouse_move_in(|event: MouseEvent| event.stop_propagation())
                    .on_mouse_move_out(|event: MouseEvent| event.stop_propagation())
                    .on_mouse_down_in(|event: MouseEvent| event.stop_propagation())
                    .on_mouse_down_out(|event: MouseEvent| event.stop_propagation())
                    .on_mouse_up_in(|event: MouseEvent| event.stop_propagation())
                    .on_mouse_up_out(|event: MouseEvent| event.stop_propagation())
                    .on_wheel(|event: WheelEvent| event.stop_propagation());
            })
    }
    pub fn event_trap_mouse(content: RenderingTree) -> RenderingTree {
        content
            .attach_event(|builder| {
                builder
                    .on_mouse_down_in(|event: MouseEvent| {
                        event.stop_propagation();
                    })
                    .on_mouse_move_in(|event: MouseEvent| {
                        event.stop_propagation();
                    })
                    .on_mouse_up_in(|event: MouseEvent| {
                        event.stop_propagation();
                    })
                    .on_wheel(|event: WheelEvent| {
                        event.stop_propagation();
                    });
            })
    }
}
pub mod list_view {
    use crate::scroll_view::{self, ScrollView};
    use namui::prelude::*;
    pub struct ListViewProps<'a> {
        pub xy: Xy<Px>,
        pub height: Px,
        pub scroll_bar_width: Px,
        pub item_wh: Wh<Px>,
        pub items: Vec<&'a dyn Component>,
    }
    pub struct UseListViewReturn<'a> {
        pub list_view: ScrollView<'a>,
        pub set_scroll_y: SetState<Px>,
    }
    pub fn use_list_view<'a>(
        ctx: &'a RenderCtx,
        props: ListViewProps<'a>,
    ) -> UseListViewReturn<'a> {
        let (scroll_y, set_scroll_y) = ctx.use_state(|| 0.px());
        let list_view = scroll_view::ScrollView {
            xy: props.xy,
            scroll_bar_width: props.scroll_bar_width,
            height: props.height,
            content: Box::new(ListViewInner {
                height: props.height,
                item_wh: props.item_wh,
                items: &props.items,
                scroll_y: *scroll_y,
            }),
            scroll_y: *scroll_y,
            set_scroll_y,
        };
        UseListViewReturn {
            list_view,
            set_scroll_y,
        }
    }
    pub struct ListView<C: Component> {
        pub xy: Xy<Px>,
        pub height: Px,
        pub scroll_bar_width: Px,
        pub item_wh: Wh<Px>,
        pub items: Vec<C>,
    }
    impl<C: Component> std::fmt::Debug for ListView<C> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ListView").finish()
        }
    }
    impl<C: Component> namui::StaticType for ListView<C> {
        fn static_type_id(&self) -> namui::StaticTypeId {
            namui::StaticTypeId::Single(std::any::TypeId::of::<ListView<C>>())
        }
    }
    impl<C: Component> Component for ListView<C> {
        fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
            let &Self { xy, height, scroll_bar_width, item_wh, ref items } = self;
            let (scroll_y, set_scroll_y) = ctx.use_state(|| 0.px());
            ctx.use_children(|ctx| {
                ctx.add(scroll_view::ScrollView {
                    xy,
                    scroll_bar_width,
                    height,
                    content: Box::new(ListViewInner {
                        height,
                        item_wh,
                        items: items.clone(),
                        scroll_y: *scroll_y,
                    }),
                    scroll_y: *scroll_y,
                    set_scroll_y,
                });
                ctx.done()
            })
        }
    }
    struct ListViewInner<'a, C: Component> {
        height: Px,
        item_wh: Wh<Px>,
        items: &'a Vec<C>,
        scroll_y: Px,
    }
    impl<C: Component> std::fmt::Debug for ListViewInner<'_, C> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ListViewInner").finish()
        }
    }
    impl<C: Component> namui::StaticType for ListViewInner<'_, C> {
        fn static_type_id(&self) -> namui::StaticTypeId {
            namui::StaticTypeId::Single(
                std::any::TypeId::of::<ListViewInner<'static, C>>(),
            )
        }
    }
    impl<C: Component> Component for ListViewInner<'_, C> {
        fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
            let &Self { height, item_wh, ref items, scroll_y } = self;
            let item_len = items.len();
            if item_len == 0 {
                return ctx.use_no_children();
            }
            let max_scroll_y = item_wh.height * item_len - height;
            let scroll_y = scroll_y.min(max_scroll_y);
            let visible_item_start_index = (scroll_y / item_wh.height).floor() as usize;
            let visible_item_end_index = ((scroll_y + height) / item_wh.height).ceil()
                as usize;
            let visible_item_count = visible_item_end_index - visible_item_start_index
                + 1;
            let visible_items = items
                .into_iter()
                .skip(visible_item_start_index)
                .take(visible_item_count);
            ctx.use_children_with_rendering_tree(
                |ctx| {
                    for visible_item in visible_items.into_iter() {
                        ctx.add(*visible_item)
                    }
                    ctx.done()
                },
                move |children| {
                    let max_scroll_y = item_wh.height * item_len - height;
                    let scroll_y = scroll_y.min(max_scroll_y);
                    let visible_item_start_index = (scroll_y / item_wh.height).floor()
                        as usize;
                    let visible_rendering_tree = namui::render(
                        children
                            .into_iter()
                            .enumerate()
                            .map(|(index, child)| {
                                translate(
                                    px(0.0),
                                    item_wh.height * (index + visible_item_start_index),
                                    child,
                                )
                            }),
                    );
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
                    {
                        let mut temp_vec = Vec::new();
                        let rendering_tree = ::namui::RenderingTree::from(
                            transparent_pillar,
                        );
                        temp_vec.push(rendering_tree);
                        let rendering_tree = ::namui::RenderingTree::from(
                            visible_rendering_tree,
                        );
                        temp_vec.push(rendering_tree);
                        if temp_vec.len() == 1 {
                            temp_vec.swap_remove(0)
                        } else {
                            ::namui::RenderingTree::Children(temp_vec)
                        }
                    }
                },
            )
        }
    }
}
pub mod scroll_view {
    use namui::prelude::*;
    use std::{any::TypeId, fmt::Debug};
    pub struct ScrollView<'a> {
        pub xy: Xy<Px>,
        pub scroll_bar_width: Px,
        pub height: Px,
        pub content: Box<dyn Component + 'a>,
        pub scroll_y: Px,
        pub set_scroll_y: SetState<Px>,
    }
    impl StaticType for ScrollView<'_> {
        fn static_type_id(&self) -> StaticTypeId {
            StaticTypeId::Single(TypeId::of::<ScrollView<'static>>())
        }
    }
    impl Debug for ScrollView<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ScrollView")
                .field("xy", &self.xy)
                .field("scroll_bar_width", &self.scroll_bar_width)
                .field("height", &self.height)
                .field("content", &self.content)
                .field("scroll_y", &self.scroll_y)
                .field("set_scroll_y", &self.set_scroll_y)
                .finish()
        }
    }
    impl Component for ScrollView<'_> {
        fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
            let &Self {
                xy,
                scroll_bar_width,
                height,
                ref content,
                scroll_y,
                set_scroll_y,
            } = self;
            ctx.use_children_with_rendering_tree(
                |ctx| {
                    ctx.add(content.as_ref());
                    ctx.done()
                },
                move |children| {
                    let content = namui::render(children);
                    let content_bounding_box = content.get_bounding_box();
                    if content_bounding_box.is_none() {
                        return RenderingTree::Empty;
                    }
                    let content_bounding_box = content_bounding_box.unwrap();
                    let scroll_y = namui::math::num::clamp(
                        scroll_y,
                        px(0.0),
                        px(0.0).max(content_bounding_box.height() - height),
                    );
                    let inner = namui::clip(
                        namui::PathBuilder::new()
                            .add_rect(Rect::Xywh {
                                x: content_bounding_box.x(),
                                y: content_bounding_box.y(),
                                width: content_bounding_box.width(),
                                height,
                            }),
                        namui::ClipOp::Intersect,
                        namui::translate(px(0.0), -scroll_y.floor(), content.clone()),
                    );
                    let scroll_bar_handle_height = height
                        * (height / content_bounding_box.height());
                    let scroll_bar_y = (height - scroll_bar_handle_height)
                        * (scroll_y / (content_bounding_box.height() - height));
                    let scroll_bar = match content_bounding_box.height() > height {
                        true => {
                            rect(RectParam {
                                rect: Rect::Xywh {
                                    x: content_bounding_box.width() - scroll_bar_width,
                                    y: scroll_bar_y,
                                    width: scroll_bar_width,
                                    height: scroll_bar_handle_height,
                                },
                                style: RectStyle {
                                    fill: Some(RectFill {
                                        color: Color::grayscale_f01(0.5),
                                    }),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                        }
                        false => RenderingTree::Empty,
                    };
                    let whole_rect = rect(RectParam {
                            rect: Rect::Xywh {
                                x: px(0.0),
                                y: px(0.0),
                                width: content_bounding_box.width(),
                                height,
                            },
                            style: RectStyle {
                                fill: Some(RectFill {
                                    color: Color::TRANSPARENT,
                                }),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .attach_event(move |builder| {
                            let height = height;
                            builder
                                .on_wheel(move |event: WheelEvent| {
                                    let next_scroll_y = namui::math::num::clamp(
                                        scroll_y + px(event.delta_xy.y),
                                        px(0.0),
                                        (px(0.0)).max(content_bounding_box.height() - height),
                                    );
                                    set_scroll_y.set(next_scroll_y);
                                    event.stop_propagation();
                                });
                        });
                    translate(xy.x, xy.y, namui::render([whole_rect, inner, scroll_bar]))
                },
            )
        }
    }
}
pub mod sheet {}
mod simple_rect {
    use namui::prelude::*;
    pub fn simple_rect(
        wh: Wh<Px>,
        stroke_color: Color,
        stroke_width: Px,
        fill_color: Color,
    ) -> RenderingTree {
        namui::rect(RectParam {
            rect: Rect::Xywh {
                x: px(0.0),
                y: px(0.0),
                width: wh.width,
                height: wh.height,
            },
            style: RectStyle {
                stroke: Some(RectStroke {
                    color: stroke_color,
                    width: stroke_width,
                    border_position: BorderPosition::Inside,
                }),
                fill: Some(RectFill { color: fill_color }),
                ..Default::default()
            },
            ..Default::default()
        })
    }
    pub fn transparent_rect(wh: Wh<Px>) -> RenderingTree {
        simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT)
    }
}
pub mod table {
    pub mod hooks {
        use namui::prelude::*;
        use std::sync::Arc;
        pub enum TableCell<'a> {
            Empty,
            Some {
                unit: Unit,
                render: Box<dyn FnOnce(Direction, Wh<Px>) -> Arc<dyn Component> + 'a>,
                need_clip: bool,
            },
        }
        pub enum Unit {
            Ratio(f32),
            Fixed(Px),
            Calculative(Box<dyn FnOnce(Wh<Px>) -> Px>),
            Responsive(Box<dyn FnOnce(Direction) -> Px>),
        }
        pub trait F32OrI32 {
            fn as_f32(self) -> f32;
        }
        impl F32OrI32 for i32 {
            fn as_f32(self) -> f32 {
                self as f32
            }
        }
        impl F32OrI32 for f32 {
            fn as_f32(self) -> f32 {
                self
            }
        }
        pub fn ratio<'a, C: Component + 'static>(
            ratio: impl F32OrI32,
            cell_render_closure: impl FnOnce(Wh<Px>) -> C + 'a,
        ) -> TableCell<'a> {
            TableCell::Some {
                unit: Unit::Ratio(ratio.as_f32()),
                render: Box::new(|_direction, wh| Arc::new(cell_render_closure(wh))),
                need_clip: true,
            }
        }
        pub fn ratio_no_clip<'a, C: Component + 'static>(
            ratio: impl F32OrI32,
            cell_render_closure: impl FnOnce(Wh<Px>) -> C + 'a,
        ) -> TableCell<'a> {
            TableCell::Some {
                unit: Unit::Ratio(ratio.as_f32()),
                render: Box::new(|_direction, wh| Arc::new(cell_render_closure(wh))),
                need_clip: false,
            }
        }
        pub fn fixed<'a, C: Component + 'static>(
            pixel: Px,
            cell_render_closure: impl FnOnce(Wh<Px>) -> C + 'a,
        ) -> TableCell<'a> {
            TableCell::Some {
                unit: Unit::Fixed(pixel),
                render: Box::new(|_direction, wh| Arc::new(cell_render_closure(wh))),
                need_clip: true,
            }
        }
        pub fn fixed_no_clip<'a, C: Component + 'static>(
            pixel: Px,
            cell_render_closure: impl FnOnce(Wh<Px>) -> C + 'a,
        ) -> TableCell<'a> {
            TableCell::Some {
                unit: Unit::Fixed(pixel),
                render: Box::new(|_direction, wh| Arc::new(cell_render_closure(wh))),
                need_clip: false,
            }
        }
        pub fn calculative<'a, C: Component + 'static>(
            from_parent_wh: impl FnOnce(Wh<Px>) -> Px + 'static,
            cell_render_closure: impl FnOnce(Wh<Px>) -> C + 'a,
        ) -> TableCell<'a> {
            TableCell::Some {
                unit: Unit::Calculative(Box::new(from_parent_wh)),
                render: Box::new(|_direction, wh| Arc::new(cell_render_closure(wh))),
                need_clip: true,
            }
        }
        pub fn calculative_no_clip<'a, C: Component + 'static>(
            from_parent_wh: impl FnOnce(Wh<Px>) -> Px + 'static,
            cell_render_closure: impl FnOnce(Wh<Px>) -> C + 'a,
        ) -> TableCell<'a> {
            TableCell::Some {
                unit: Unit::Calculative(Box::new(from_parent_wh)),
                render: Box::new(|_direction, wh| Arc::new(cell_render_closure(wh))),
                need_clip: false,
            }
        }
        pub fn empty<'a>() -> TableCell<'a> {
            TableCell::Empty
        }
        pub fn vertical<'a>(
            items: impl IntoIterator<Item = TableCell<'a>> + 'a,
        ) -> impl FnOnce(Wh<Px>) -> Table + 'a {
            slice_internal(Direction::Vertical, items)
        }
        pub fn horizontal<'a>(
            items: impl IntoIterator<Item = TableCell<'a>> + 'a,
        ) -> impl FnOnce(Wh<Px>) -> Table + 'a {
            slice_internal(Direction::Horizontal, items)
        }
        pub enum Direction {
            Vertical,
            Horizontal,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Direction {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        Direction::Vertical => "Vertical",
                        Direction::Horizontal => "Horizontal",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Direction {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Direction {
            #[inline]
            fn eq(&self, other: &Direction) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Direction {
            #[inline]
            fn clone(&self) -> Direction {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Direction {}
        pub struct Table {
            items: Vec<(Rect<Px>, Arc<dyn Component>, bool)>,
        }
        impl std::fmt::Debug for Table {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("Table").finish()
            }
        }
        impl namui::StaticType for Table {
            fn static_type_id(&self) -> namui::StaticTypeId {
                namui::StaticTypeId::Single(std::any::TypeId::of::<Table>())
            }
        }
        impl Component for Table {
            fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
                let rect_need_clip_tuples = self
                    .items
                    .iter()
                    .map(|(rect, _, need_clip)| (*rect, *need_clip))
                    .collect::<Vec<_>>();
                ctx.use_children_with_rendering_tree(
                    |ctx| {
                        for (_, item, _) in &self.items {
                            ctx.add(item.as_ref());
                        }
                        ctx.done()
                    },
                    move |children| {
                        namui::render(
                            children
                                .into_iter()
                                .zip(rect_need_clip_tuples.clone().into_iter())
                                .map(|(child, (rect, need_clip))| {
                                    namui::translate(
                                        rect.x(),
                                        rect.y(),
                                        if need_clip {
                                            namui::clip(
                                                PathBuilder::new()
                                                    .add_rect(Rect::Xywh {
                                                        x: px(0.0),
                                                        y: px(0.0),
                                                        width: rect.width(),
                                                        height: rect.height(),
                                                    }),
                                                ClipOp::Intersect,
                                                child,
                                            )
                                        } else {
                                            child
                                        },
                                    )
                                }),
                        )
                    },
                )
            }
        }
        fn slice_internal<'a>(
            direction: Direction,
            items: impl IntoIterator<Item = TableCell<'a>> + 'a,
        ) -> impl FnOnce(Wh<Px>) -> Table + 'a {
            let mut units = Vec::new();
            let mut for_renders = std::collections::VecDeque::new();
            for item in items {
                match item {
                    TableCell::Empty => {}
                    TableCell::Some { unit, render, need_clip } => {
                        units.push(unit);
                        for_renders.push_back((render, need_clip));
                    }
                }
            }
            move |wh: Wh<Px>| {
                let direction_pixel_size = match direction {
                    Direction::Vertical => wh.height,
                    Direction::Horizontal => wh.width,
                };
                let ratio_sum = units
                    .iter()
                    .fold(
                        0.0,
                        |sum, unit| match unit {
                            Unit::Ratio(ratio) => sum + ratio,
                            _ => sum,
                        },
                    );
                let pixel_size_or_ratio_list = units
                    .into_iter()
                    .map(|unit| {
                        let (pixel_size, ratio) = match unit {
                            Unit::Ratio(ratio) => (None, Some(ratio)),
                            Unit::Fixed(pixel_size) => (Some(pixel_size), None),
                            Unit::Calculative(calculative_fn) => {
                                (Some(calculative_fn(wh)), None)
                            }
                            Unit::Responsive(responsive_fn) => {
                                (Some(responsive_fn(direction)), None)
                            }
                        };
                        (pixel_size, ratio)
                    })
                    .collect::<Vec<_>>();
                let non_ratio_pixel_size_sum: Px = pixel_size_or_ratio_list
                    .iter()
                    .filter_map(|(pixel_size, _ratio)| *pixel_size)
                    .sum();
                let pixel_sizes = pixel_size_or_ratio_list
                    .iter()
                    .map(|(pixel_size, ratio)| {
                        if let Some(pixel_size) = pixel_size {
                            *pixel_size
                        } else {
                            (direction_pixel_size - non_ratio_pixel_size_sum)
                                * ratio.unwrap() / ratio_sum
                        }
                    });
                let mut advanced_pixel_size = px(0.0);
                let mut items = Vec::new();
                for pixel_size in pixel_sizes {
                    let (render_fn, need_clip) = for_renders.pop_front().unwrap();
                    let xywh = match direction {
                        Direction::Vertical => {
                            Rect::Xywh {
                                x: px(0.0),
                                y: advanced_pixel_size,
                                width: wh.width,
                                height: pixel_size,
                            }
                        }
                        Direction::Horizontal => {
                            Rect::Xywh {
                                x: advanced_pixel_size,
                                y: px(0.0),
                                width: pixel_size,
                                height: wh.height,
                            }
                        }
                    };
                    let component = render_fn(direction, xywh.wh());
                    items.push((xywh, component, need_clip));
                    advanced_pixel_size += pixel_size;
                }
                Table { items }
            }
        }
        pub fn padding<'a, C: Component + 'static>(
            padding: Px,
            cell_render_closure: impl FnOnce(Wh<Px>) -> C + 'a,
        ) -> impl FnOnce(Wh<Px>) -> Table + 'a {
            horizontal_padding(padding, vertical_padding(padding, cell_render_closure))
        }
        pub fn padding_no_clip<'a, C: Component + 'static>(
            padding: Px,
            cell_render_closure: impl FnOnce(Wh<Px>) -> C + 'a,
        ) -> impl FnOnce(Wh<Px>) -> Table + 'a {
            horizontal_padding_no_clip(
                padding,
                vertical_padding_no_clip(padding, cell_render_closure),
            )
        }
        pub fn horizontal_padding<'a, C: Component + 'static>(
            padding: Px,
            cell_render_closure: impl FnOnce(Wh<Px>) -> C + 'a,
        ) -> impl FnOnce(Wh<Px>) -> Table + 'a {
            horizontal([
                fixed(padding, |_| RenderingTree::Empty),
                ratio(1, cell_render_closure),
                fixed(padding, |_| RenderingTree::Empty),
            ])
        }
        pub fn vertical_padding<'a, C: Component + 'static>(
            padding: Px,
            cell_render_closure: impl FnOnce(Wh<Px>) -> C + 'a,
        ) -> impl FnOnce(Wh<Px>) -> Table + 'a {
            vertical([
                fixed(padding, |_| RenderingTree::Empty),
                ratio(1, cell_render_closure),
                fixed(padding, |_| RenderingTree::Empty),
            ])
        }
        pub fn horizontal_padding_no_clip<'a, C: Component + 'static>(
            padding: Px,
            cell_render_closure: impl FnOnce(Wh<Px>) -> C + 'a,
        ) -> impl FnOnce(Wh<Px>) -> Table + 'a {
            horizontal([
                fixed(padding, |_| RenderingTree::Empty),
                ratio_no_clip(1, cell_render_closure),
                fixed(padding, |_| RenderingTree::Empty),
            ])
        }
        pub fn vertical_padding_no_clip<'a, C: Component + 'static>(
            padding: Px,
            cell_render_closure: impl FnOnce(Wh<Px>) -> C + 'a,
        ) -> impl FnOnce(Wh<Px>) -> Table + 'a {
            vertical([
                fixed(padding, |_| RenderingTree::Empty),
                ratio_no_clip(1, cell_render_closure),
                fixed(padding, |_| RenderingTree::Empty),
            ])
        }
        pub enum FitAlign {
            LeftTop,
            CenterMiddle,
            RightBottom,
        }
        pub fn fit<'a>(align: FitAlign, rendering_tree: RenderingTree) -> TableCell<'a> {
            match rendering_tree.get_bounding_box() {
                Some(bounding_box) => {
                    TableCell::Some {
                        unit: Unit::Responsive(
                            Box::new(move |direction| match direction {
                                Direction::Vertical => {
                                    bounding_box.y() + bounding_box.height()
                                }
                                Direction::Horizontal => {
                                    bounding_box.x() + bounding_box.width()
                                }
                            }),
                        ),
                        render: Box::new(move |direction, wh| {
                            let x = match direction {
                                Direction::Vertical => {
                                    match align {
                                        FitAlign::LeftTop => 0.px(),
                                        FitAlign::CenterMiddle => {
                                            (wh.width - bounding_box.width()) / 2.0
                                        }
                                        FitAlign::RightBottom => wh.width - bounding_box.width(),
                                    }
                                }
                                Direction::Horizontal => 0.px(),
                            };
                            let y = match direction {
                                Direction::Vertical => 0.px(),
                                Direction::Horizontal => {
                                    match align {
                                        FitAlign::LeftTop => 0.px(),
                                        FitAlign::CenterMiddle => {
                                            (wh.height - bounding_box.height()) / 2.0
                                        }
                                        FitAlign::RightBottom => wh.height - bounding_box.height(),
                                    }
                                }
                            };
                            Arc::new(translate(x, y, rendering_tree))
                        }),
                        need_clip: true,
                    }
                }
                None => ratio(0, |_| rendering_tree),
            }
        }
    }
    use namui::prelude::*;
    pub struct TableCell<'a> {
        unit: Unit,
        render: Box<dyn FnOnce(Direction, Wh<Px>) -> RenderingTree + 'a>,
        need_clip: bool,
    }
    enum Unit {
        Ratio(f32),
        Fixed(Px),
        Calculative(Box<dyn FnOnce(Wh<Px>) -> Px>),
        Responsive(Box<dyn FnOnce(Direction) -> Px>),
    }
    pub trait F32OrI32 {
        fn as_f32(self) -> f32;
    }
    impl F32OrI32 for i32 {
        fn as_f32(self) -> f32 {
            self as f32
        }
    }
    impl F32OrI32 for f32 {
        fn as_f32(self) -> f32 {
            self
        }
    }
    pub fn ratio<'a>(
        ratio: impl F32OrI32,
        cell_render_closure: impl FnOnce(Wh<Px>) -> RenderingTree + 'a,
    ) -> TableCell<'a> {
        TableCell {
            unit: Unit::Ratio(ratio.as_f32()),
            render: Box::new(|_direction, wh| cell_render_closure(wh)),
            need_clip: true,
        }
    }
    pub fn ratio_no_clip<'a>(
        ratio: impl F32OrI32,
        cell_render_closure: impl FnOnce(Wh<Px>) -> RenderingTree + 'a,
    ) -> TableCell<'a> {
        TableCell {
            unit: Unit::Ratio(ratio.as_f32()),
            render: Box::new(|_direction, wh| cell_render_closure(wh)),
            need_clip: false,
        }
    }
    pub fn fixed<'a>(
        pixel: Px,
        cell_render_closure: impl FnOnce(Wh<Px>) -> RenderingTree + 'a,
    ) -> TableCell<'a> {
        TableCell {
            unit: Unit::Fixed(pixel),
            render: Box::new(|_direction, wh| cell_render_closure(wh)),
            need_clip: true,
        }
    }
    pub fn fixed_no_clip<'a>(
        pixel: Px,
        cell_render_closure: impl FnOnce(Wh<Px>) -> RenderingTree + 'a,
    ) -> TableCell<'a> {
        TableCell {
            unit: Unit::Fixed(pixel),
            render: Box::new(|_direction, wh| cell_render_closure(wh)),
            need_clip: false,
        }
    }
    pub fn calculative<'a>(
        from_parent_wh: impl FnOnce(Wh<Px>) -> Px + 'static,
        cell_render_closure: impl FnOnce(Wh<Px>) -> RenderingTree + 'a,
    ) -> TableCell<'a> {
        TableCell {
            unit: Unit::Calculative(Box::new(from_parent_wh)),
            render: Box::new(|_direction, wh| cell_render_closure(wh)),
            need_clip: true,
        }
    }
    pub fn calculative_no_clip<'a>(
        from_parent_wh: impl FnOnce(Wh<Px>) -> Px + 'static,
        cell_render_closure: impl FnOnce(Wh<Px>) -> RenderingTree + 'a,
    ) -> TableCell<'a> {
        TableCell {
            unit: Unit::Calculative(Box::new(from_parent_wh)),
            render: Box::new(|_direction, wh| cell_render_closure(wh)),
            need_clip: false,
        }
    }
    pub fn vertical<'a>(
        items: impl IntoIterator<Item = TableCell<'a>> + 'a,
    ) -> impl FnOnce(Wh<Px>) -> RenderingTree + 'a {
        slice_internal(Direction::Vertical, items)
    }
    pub fn horizontal<'a>(
        items: impl IntoIterator<Item = TableCell<'a>> + 'a,
    ) -> impl FnOnce(Wh<Px>) -> RenderingTree + 'a {
        slice_internal(Direction::Horizontal, items)
    }
    enum Direction {
        Vertical,
        Horizontal,
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Direction {}
    #[automatically_derived]
    impl ::core::clone::Clone for Direction {
        #[inline]
        fn clone(&self) -> Direction {
            *self
        }
    }
    fn slice_internal<'a>(
        direction: Direction,
        items: impl IntoIterator<Item = TableCell<'a>> + 'a,
    ) -> impl FnOnce(Wh<Px>) -> RenderingTree + 'a {
        let mut units = Vec::new();
        let mut for_renders = std::collections::VecDeque::new();
        for item in items {
            units.push(item.unit);
            for_renders.push_back((item.render, item.need_clip));
        }
        move |wh: Wh<Px>| {
            let direction_pixel_size = match direction {
                Direction::Vertical => wh.height,
                Direction::Horizontal => wh.width,
            };
            let mut rendering_tree_list: Vec<RenderingTree> = Vec::new();
            let ratio_sum = units
                .iter()
                .fold(
                    0.0,
                    |sum, unit| match unit {
                        Unit::Ratio(ratio) => sum + ratio,
                        _ => sum,
                    },
                );
            let pixel_size_or_ratio_list = units
                .into_iter()
                .map(|unit| {
                    let (pixel_size, ratio) = match unit {
                        Unit::Ratio(ratio) => (None, Some(ratio)),
                        Unit::Fixed(pixel_size) => (Some(pixel_size), None),
                        Unit::Calculative(calculative_fn) => {
                            (Some(calculative_fn(wh)), None)
                        }
                        Unit::Responsive(responsive_fn) => {
                            (Some(responsive_fn(direction)), None)
                        }
                    };
                    (pixel_size, ratio)
                })
                .collect::<Vec<_>>();
            let non_ratio_pixel_size_sum: Px = pixel_size_or_ratio_list
                .iter()
                .filter_map(|(pixel_size, _ratio)| *pixel_size)
                .sum();
            let pixel_sizes = pixel_size_or_ratio_list
                .iter()
                .map(|(pixel_size, ratio)| {
                    if let Some(pixel_size) = pixel_size {
                        *pixel_size
                    } else {
                        (direction_pixel_size - non_ratio_pixel_size_sum)
                            * ratio.unwrap() / ratio_sum
                    }
                });
            let mut advanced_pixel_size = px(0.0);
            for pixel_size in pixel_sizes {
                let (render_fn, need_clip) = for_renders.pop_front().unwrap();
                let xywh = match direction {
                    Direction::Vertical => {
                        Rect::Xywh {
                            x: px(0.0),
                            y: advanced_pixel_size,
                            width: wh.width,
                            height: pixel_size,
                        }
                    }
                    Direction::Horizontal => {
                        Rect::Xywh {
                            x: advanced_pixel_size,
                            y: px(0.0),
                            width: pixel_size,
                            height: wh.height,
                        }
                    }
                };
                let rendering_tree = namui::translate(
                    xywh.x(),
                    xywh.y(),
                    if need_clip {
                        namui::clip(
                            PathBuilder::new()
                                .add_rect(Rect::Xywh {
                                    x: px(0.0),
                                    y: px(0.0),
                                    width: xywh.width(),
                                    height: xywh.height(),
                                }),
                            ClipOp::Intersect,
                            render_fn(direction, xywh.wh()),
                        )
                    } else {
                        render_fn(direction, xywh.wh())
                    },
                );
                rendering_tree_list.push(rendering_tree);
                advanced_pixel_size += pixel_size;
            }
            RenderingTree::Children(rendering_tree_list)
        }
    }
    pub fn padding<'a>(
        padding: Px,
        cell_render_closure: impl FnOnce(Wh<Px>) -> RenderingTree + 'a,
    ) -> impl FnOnce(Wh<Px>) -> RenderingTree + 'a {
        horizontal_padding(padding, vertical_padding(padding, cell_render_closure))
    }
    pub fn padding_no_clip<'a>(
        padding: Px,
        cell_render_closure: impl FnOnce(Wh<Px>) -> RenderingTree + 'a,
    ) -> impl FnOnce(Wh<Px>) -> RenderingTree + 'a {
        horizontal_padding_no_clip(
            padding,
            vertical_padding_no_clip(padding, cell_render_closure),
        )
    }
    pub fn horizontal_padding<'a>(
        padding: Px,
        cell_render_closure: impl FnOnce(Wh<Px>) -> RenderingTree + 'a,
    ) -> impl FnOnce(Wh<Px>) -> RenderingTree + 'a {
        horizontal([
            fixed(padding, |_| RenderingTree::Empty),
            ratio(1, cell_render_closure),
            fixed(padding, |_| RenderingTree::Empty),
        ])
    }
    pub fn vertical_padding<'a>(
        padding: Px,
        cell_render_closure: impl FnOnce(Wh<Px>) -> RenderingTree + 'a,
    ) -> impl FnOnce(Wh<Px>) -> RenderingTree + 'a {
        vertical([
            fixed(padding, |_| RenderingTree::Empty),
            ratio(1, cell_render_closure),
            fixed(padding, |_| RenderingTree::Empty),
        ])
    }
    pub fn horizontal_padding_no_clip<'a>(
        padding: Px,
        cell_render_closure: impl FnOnce(Wh<Px>) -> RenderingTree + 'a,
    ) -> impl FnOnce(Wh<Px>) -> RenderingTree + 'a {
        horizontal([
            fixed(padding, |_| RenderingTree::Empty),
            ratio_no_clip(1, cell_render_closure),
            fixed(padding, |_| RenderingTree::Empty),
        ])
    }
    pub fn vertical_padding_no_clip<'a>(
        padding: Px,
        cell_render_closure: impl FnOnce(Wh<Px>) -> RenderingTree + 'a,
    ) -> impl FnOnce(Wh<Px>) -> RenderingTree + 'a {
        vertical([
            fixed(padding, |_| RenderingTree::Empty),
            ratio_no_clip(1, cell_render_closure),
            fixed(padding, |_| RenderingTree::Empty),
        ])
    }
    pub enum FitAlign {
        LeftTop,
        CenterMiddle,
        RightBottom,
    }
    pub fn fit<'a>(align: FitAlign, rendering_tree: RenderingTree) -> TableCell<'a> {
        match rendering_tree.get_bounding_box() {
            Some(bounding_box) => {
                TableCell {
                    unit: Unit::Responsive(
                        Box::new(move |direction| match direction {
                            Direction::Vertical => {
                                bounding_box.y() + bounding_box.height()
                            }
                            Direction::Horizontal => {
                                bounding_box.x() + bounding_box.width()
                            }
                        }),
                    ),
                    render: Box::new(move |direction, wh| {
                        let x = match direction {
                            Direction::Vertical => {
                                match align {
                                    FitAlign::LeftTop => 0.px(),
                                    FitAlign::CenterMiddle => {
                                        (wh.width - bounding_box.width()) / 2.0
                                    }
                                    FitAlign::RightBottom => wh.width - bounding_box.width(),
                                }
                            }
                            Direction::Horizontal => 0.px(),
                        };
                        let y = match direction {
                            Direction::Vertical => 0.px(),
                            Direction::Horizontal => {
                                match align {
                                    FitAlign::LeftTop => 0.px(),
                                    FitAlign::CenterMiddle => {
                                        (wh.height - bounding_box.height()) / 2.0
                                    }
                                    FitAlign::RightBottom => wh.height - bounding_box.height(),
                                }
                            }
                        };
                        translate(x, y, rendering_tree)
                    }),
                    need_clip: true,
                }
            }
            None => ratio(0, |_| rendering_tree),
        }
    }
}
pub mod typography {
    pub mod body {
        use super::*;
        pub const FONT_SIZE: IntPx = int_px(12);
        pub fn left(height: Px, text: impl AsRef<str>, color: Color) -> RenderingTree {
            namui::text(TextParam {
                text: String::from(text.as_ref()),
                x: 0.px(),
                y: height / 2.0,
                align: TextAlign::Left,
                baseline: TextBaseline::Middle,
                font_type: FontType {
                    font_weight: FontWeight::REGULAR,
                    language: Language::Ko,
                    serif: false,
                    size: FONT_SIZE,
                },
                style: TextStyle {
                    color,
                    ..Default::default()
                },
                max_width: None,
            })
        }
        pub fn left_top(text: impl AsRef<str>, color: Color) -> RenderingTree {
            namui::text(TextParam {
                text: String::from(text.as_ref()),
                x: 0.px(),
                y: 0.px(),
                align: TextAlign::Left,
                baseline: TextBaseline::Top,
                font_type: FontType {
                    font_weight: FontWeight::REGULAR,
                    language: Language::Ko,
                    serif: false,
                    size: FONT_SIZE,
                },
                style: TextStyle {
                    color,
                    ..Default::default()
                },
                max_width: None,
            })
        }
        pub fn center(wh: Wh<Px>, text: impl AsRef<str>, color: Color) -> RenderingTree {
            namui::text(TextParam {
                text: String::from(text.as_ref()),
                x: wh.width / 2.0,
                y: wh.height / 2.0,
                align: TextAlign::Center,
                baseline: TextBaseline::Middle,
                font_type: FontType {
                    font_weight: FontWeight::REGULAR,
                    language: Language::Ko,
                    serif: false,
                    size: FONT_SIZE,
                },
                style: TextStyle {
                    color,
                    ..Default::default()
                },
                max_width: None,
            })
        }
        pub fn center_top(
            width: Px,
            text: impl AsRef<str>,
            color: Color,
        ) -> RenderingTree {
            namui::text(TextParam {
                text: String::from(text.as_ref()),
                x: width / 2.0,
                y: 0.px(),
                align: TextAlign::Center,
                baseline: TextBaseline::Top,
                font_type: FontType {
                    font_weight: FontWeight::REGULAR,
                    language: Language::Ko,
                    serif: false,
                    size: FONT_SIZE,
                },
                style: TextStyle {
                    color,
                    ..Default::default()
                },
                max_width: None,
            })
        }
        pub fn right(wh: Wh<Px>, text: impl AsRef<str>, color: Color) -> RenderingTree {
            namui::text(TextParam {
                text: String::from(text.as_ref()),
                x: wh.width,
                y: wh.height / 2.0,
                align: TextAlign::Right,
                baseline: TextBaseline::Middle,
                font_type: FontType {
                    font_weight: FontWeight::REGULAR,
                    language: Language::Ko,
                    serif: false,
                    size: FONT_SIZE,
                },
                style: TextStyle {
                    color,
                    ..Default::default()
                },
                max_width: None,
            })
        }
        pub fn left_bold(
            height: Px,
            text: impl AsRef<str>,
            color: Color,
        ) -> RenderingTree {
            namui::text(TextParam {
                text: String::from(text.as_ref()),
                x: 0.px(),
                y: height / 2.0,
                align: TextAlign::Left,
                baseline: TextBaseline::Middle,
                font_type: FontType {
                    font_weight: FontWeight::BOLD,
                    language: Language::Ko,
                    serif: false,
                    size: FONT_SIZE,
                },
                style: TextStyle {
                    color,
                    ..Default::default()
                },
                max_width: None,
            })
        }
        pub fn left_top_bold(text: impl AsRef<str>, color: Color) -> RenderingTree {
            namui::text(TextParam {
                text: String::from(text.as_ref()),
                x: 0.px(),
                y: 0.px(),
                align: TextAlign::Left,
                baseline: TextBaseline::Middle,
                font_type: FontType {
                    font_weight: FontWeight::BOLD,
                    language: Language::Ko,
                    serif: false,
                    size: FONT_SIZE,
                },
                style: TextStyle {
                    color,
                    ..Default::default()
                },
                max_width: None,
            })
        }
        pub fn center_bold(
            wh: Wh<Px>,
            text: impl AsRef<str>,
            color: Color,
        ) -> RenderingTree {
            namui::text(TextParam {
                text: String::from(text.as_ref()),
                x: wh.width / 2.0,
                y: wh.height / 2.0,
                align: TextAlign::Center,
                baseline: TextBaseline::Middle,
                font_type: FontType {
                    font_weight: FontWeight::BOLD,
                    language: Language::Ko,
                    serif: false,
                    size: FONT_SIZE,
                },
                style: TextStyle {
                    color,
                    ..Default::default()
                },
                max_width: None,
            })
        }
        pub fn right_bold(
            wh: Wh<Px>,
            text: impl AsRef<str>,
            color: Color,
        ) -> RenderingTree {
            namui::text(TextParam {
                text: String::from(text.as_ref()),
                x: wh.width,
                y: wh.height / 2.0,
                align: TextAlign::Right,
                baseline: TextBaseline::Middle,
                font_type: FontType {
                    font_weight: FontWeight::BOLD,
                    language: Language::Ko,
                    serif: false,
                    size: FONT_SIZE,
                },
                style: TextStyle {
                    color,
                    ..Default::default()
                },
                max_width: None,
            })
        }
    }
    pub mod title {
        use super::*;
        pub const FONT_SIZE: IntPx = int_px(20);
        pub fn left(height: Px, text: impl AsRef<str>, color: Color) -> RenderingTree {
            namui::text(TextParam {
                text: String::from(text.as_ref()),
                x: px(0.0),
                y: height / 2.0,
                align: TextAlign::Left,
                baseline: TextBaseline::Middle,
                font_type: FontType {
                    font_weight: FontWeight::BOLD,
                    language: Language::Ko,
                    serif: false,
                    size: FONT_SIZE,
                },
                style: TextStyle {
                    color,
                    ..Default::default()
                },
                max_width: None,
            })
        }
        pub fn left_top(text: impl AsRef<str>, color: Color) -> RenderingTree {
            namui::text(TextParam {
                text: String::from(text.as_ref()),
                x: 0.px(),
                y: 0.px(),
                align: TextAlign::Left,
                baseline: TextBaseline::Top,
                font_type: FontType {
                    font_weight: FontWeight::BOLD,
                    language: Language::Ko,
                    serif: false,
                    size: FONT_SIZE,
                },
                style: TextStyle {
                    color,
                    ..Default::default()
                },
                max_width: None,
            })
        }
        pub fn center(wh: Wh<Px>, text: impl AsRef<str>, color: Color) -> RenderingTree {
            namui::text(TextParam {
                text: String::from(text.as_ref()),
                x: wh.width / 2.0,
                y: wh.height / 2.0,
                align: TextAlign::Center,
                baseline: TextBaseline::Middle,
                font_type: FontType {
                    font_weight: FontWeight::BOLD,
                    language: Language::Ko,
                    serif: false,
                    size: FONT_SIZE,
                },
                style: TextStyle {
                    color,
                    ..Default::default()
                },
                max_width: None,
            })
        }
        pub fn right(wh: Wh<Px>, text: impl AsRef<str>, color: Color) -> RenderingTree {
            namui::text(TextParam {
                text: String::from(text.as_ref()),
                x: wh.width,
                y: wh.height / 2.0,
                align: TextAlign::Right,
                baseline: TextBaseline::Middle,
                font_type: FontType {
                    font_weight: FontWeight::BOLD,
                    language: Language::Ko,
                    serif: false,
                    size: FONT_SIZE,
                },
                style: TextStyle {
                    color,
                    ..Default::default()
                },
                max_width: None,
            })
        }
    }
    use crate::*;
    use namui::prelude::*;
    pub fn center_text(
        wh: Wh<Px>,
        text: impl AsRef<str>,
        color: Color,
        font_size: IntPx,
    ) -> RenderingTree {
        namui::text(TextParam {
            text: String::from(text.as_ref()),
            x: wh.width / 2.0,
            y: wh.height / 2.0,
            align: TextAlign::Center,
            baseline: TextBaseline::Middle,
            font_type: FontType {
                font_weight: FontWeight::REGULAR,
                language: Language::Ko,
                serif: false,
                size: font_size,
            },
            style: TextStyle {
                color,
                ..Default::default()
            },
            max_width: None,
        })
    }
    pub fn center_text_full_height(
        wh: Wh<Px>,
        text: impl AsRef<str>,
        color: Color,
    ) -> RenderingTree {
        namui::text(TextParam {
            text: String::from(text.as_ref()),
            x: wh.width / 2.0,
            y: wh.height / 2.0,
            align: TextAlign::Center,
            baseline: TextBaseline::Middle,
            font_type: FontType {
                font_weight: FontWeight::REGULAR,
                language: Language::Ko,
                serif: false,
                size: adjust_font_size(wh.height),
            },
            style: TextStyle {
                color,
                ..Default::default()
            },
            max_width: None,
        })
    }
    pub fn text_fit(
        height: Px,
        text: impl AsRef<str>,
        color: Color,
        side_padding: Px,
    ) -> namui::RenderingTree {
        let center_text = namui::text(TextParam {
            text: String::from(text.as_ref()),
            x: 0.px(),
            y: height / 2.0,
            align: TextAlign::Center,
            baseline: TextBaseline::Middle,
            font_type: FontType {
                font_weight: FontWeight::REGULAR,
                language: Language::Ko,
                serif: false,
                size: adjust_font_size(height),
            },
            style: TextStyle {
                color,
                ..Default::default()
            },
            max_width: None,
        });
        let width = match center_text.get_bounding_box() {
            Some(bounding_box) => bounding_box.width(),
            None => return RenderingTree::Empty,
        };
        render([
            simple_rect(
                Wh::new(width + side_padding * 2, height),
                Color::TRANSPARENT,
                0.px(),
                Color::TRANSPARENT,
            ),
            translate(width / 2 + side_padding, 0.px(), center_text),
        ])
    }
    pub fn adjust_font_size(height: Px) -> IntPx {
        let mut font_size: Px = height * 0.7;
        font_size -= font_size % 4;
        let result = font_size.into();
        result
    }
}
pub mod vh_list_view {}
pub use event_trap::*;
pub use simple_rect::*;
