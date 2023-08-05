use namui::prelude::*;
use std::sync::Arc;

pub enum TableCell<'a> {
    Empty,
    Some {
        unit: Unit,
        render: Box<dyn FnOnce(Direction, Wh<Px>) -> Arc<dyn 'a + Component> + 'a>,
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

pub fn fixed<'a, C: Component + 'a>(
    pixel: Px,
    cell_render_closure: impl 'a + FnOnce(Wh<Px>) -> C,
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Vertical,
    Horizontal,
}
#[namui::component]
pub struct Table {
    items: Vec<(Rect<Px>, Arc<dyn Component>, bool)>,
}

impl Component for Table {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let rect_need_clip_tuples = self
            .items
            .iter()
            .map(|(rect, _, need_clip)| (*rect, *need_clip))
            .collect::<Vec<_>>();

        for (_, item, _) in &self.items {
            ctx.add(item.as_ref());
        }
        ctx.done_with_rendering_tree(move |children| {
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
                                    PathBuilder::new().add_rect(Rect::Xywh {
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
        })
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
            TableCell::Some {
                unit,
                render,
                need_clip,
            } => {
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

        let ratio_sum = units.iter().fold(0.0, |sum, unit| match unit {
            Unit::Ratio(ratio) => sum + ratio,
            _ => sum,
        });

        let pixel_size_or_ratio_list = units
            .into_iter()
            .map(|unit| {
                let (pixel_size, ratio) = match unit {
                    Unit::Ratio(ratio) => (None, Some(ratio)),
                    Unit::Fixed(pixel_size) => (Some(pixel_size), None),
                    Unit::Calculative(calculative_fn) => (Some(calculative_fn(wh)), None),
                    Unit::Responsive(responsive_fn) => (Some(responsive_fn(direction)), None),
                };
                (pixel_size, ratio)
            })
            .collect::<Vec<_>>();

        let non_ratio_pixel_size_sum: Px = pixel_size_or_ratio_list
            .iter()
            .filter_map(|(pixel_size, _ratio)| *pixel_size)
            .sum();

        let pixel_sizes = pixel_size_or_ratio_list.iter().map(|(pixel_size, ratio)| {
            if let Some(pixel_size) = pixel_size {
                *pixel_size
            } else {
                (direction_pixel_size - non_ratio_pixel_size_sum) * ratio.unwrap() / ratio_sum
            }
        });

        let mut advanced_pixel_size = px(0.0);

        let mut items = Vec::new();

        for pixel_size in pixel_sizes {
            let (render_fn, need_clip) = for_renders.pop_front().unwrap();
            let xywh = match direction {
                Direction::Vertical => Rect::Xywh {
                    x: px(0.0),
                    y: advanced_pixel_size,
                    width: wh.width,
                    height: pixel_size,
                },
                Direction::Horizontal => Rect::Xywh {
                    x: advanced_pixel_size,
                    y: px(0.0),
                    width: pixel_size,
                    height: wh.height,
                },
            };

            // let component = render_fn(direction, xywh.wh());
            // items.push((xywh, component, need_clip));
            // advanced_pixel_size += pixel_size;
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
        Some(bounding_box) => TableCell::Some {
            unit: Unit::Responsive(Box::new(move |direction| match direction {
                Direction::Vertical => bounding_box.y() + bounding_box.height(),
                Direction::Horizontal => bounding_box.x() + bounding_box.width(),
            })),
            render: Box::new(move |direction, wh| {
                let x = match direction {
                    Direction::Vertical => match align {
                        FitAlign::LeftTop => 0.px(),
                        FitAlign::CenterMiddle => (wh.width - bounding_box.width()) / 2.0,
                        FitAlign::RightBottom => wh.width - bounding_box.width(),
                    },
                    Direction::Horizontal => 0.px(),
                };
                let y = match direction {
                    Direction::Vertical => 0.px(),
                    Direction::Horizontal => match align {
                        FitAlign::LeftTop => 0.px(),
                        FitAlign::CenterMiddle => (wh.height - bounding_box.height()) / 2.0,
                        FitAlign::RightBottom => wh.height - bounding_box.height(),
                    },
                };
                Arc::new(translate(x, y, rendering_tree))
            }),
            need_clip: true,
        },
        None => ratio(0, |_| rendering_tree),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn closure_should_give_right_wh() {
        let button_render_called = AtomicBool::new(false);
        let label_render_called = AtomicBool::new(false);
        let body_render_called = AtomicBool::new(false);
        let body_inner_render_called = AtomicBool::new(false);

        let button = calculative(
            |parent_wh| parent_wh.height,
            |wh| {
                button_render_called.store(true, std::sync::atomic::Ordering::Relaxed);
                assert_eq!(px(20.0), wh.width);
                assert_eq!(px(20.0), wh.height);
                RenderingTree::Empty
            },
        );

        let label = ratio(1, |wh| {
            label_render_called.store(true, std::sync::atomic::Ordering::Relaxed);
            assert_eq!(px(280.0), wh.width);
            assert_eq!(px(20.0), wh.height);
            RenderingTree::Empty
        });

        let header = fixed(px(20.0), horizontal([button, label]));

        let body = ratio(1.0, |wh| {
            body_render_called.store(true, std::sync::atomic::Ordering::Relaxed);
            assert_eq!(px(300.0), wh.width);
            assert_eq!(px(480.0), wh.height);
            vertical([
                ratio(
                    1,
                    padding(5.px(), |wh| {
                        body_inner_render_called.store(true, std::sync::atomic::Ordering::Relaxed);
                        assert_eq!(px(290.0), wh.width);
                        assert_eq!(px(470.0), wh.height);
                        RenderingTree::Empty
                    }),
                ),
                // Note: RenderingTree is not testable yet, So you cannot test fit well now.
                fit(FitAlign::LeftTop, RenderingTree::Empty),
            ])(wh)
        });

        vertical([header, body])(Wh {
            width: px(300.0),
            height: px(500.0),
        });

        assert_eq!(
            true,
            button_render_called.load(std::sync::atomic::Ordering::Relaxed)
        );
        assert_eq!(
            true,
            label_render_called.load(std::sync::atomic::Ordering::Relaxed)
        );
        assert_eq!(
            true,
            body_render_called.load(std::sync::atomic::Ordering::Relaxed)
        );
        assert_eq!(
            true,
            body_inner_render_called.load(std::sync::atomic::Ordering::Relaxed)
        );
    }
}
