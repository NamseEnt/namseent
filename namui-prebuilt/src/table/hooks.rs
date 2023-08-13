use namui::prelude::*;

pub enum TableCell<'a> {
    Empty,
    Some {
        unit: Unit<'a>,
        render: Box<dyn 'a + FnOnce(Direction, Wh<Px>, &mut ComposeCtx)>,
        need_clip: bool,
    },
    NeedBoundingBox {
        component_type_name: &'static str,
        func: Box<dyn 'a + FnOnce(&RenderCtx) -> TableCell<'a>>,
    },
}

pub enum Unit<'a> {
    Ratio(f32),
    Fixed(Px),
    Calculative(Box<dyn 'a + FnOnce(Wh<Px>) -> Px>),
    Responsive(Box<dyn 'a + FnOnce(Direction) -> Px>),
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
    cell_render_closure: impl 'a + FnOnce(Wh<Px>, &mut ComposeCtx),
) -> TableCell<'a> {
    TableCell::Some {
        unit: Unit::Ratio(ratio.as_f32()),
        render: Box::new(|_direction, wh, ctx| {
            cell_render_closure(wh, ctx);
        }),
        need_clip: true,
    }
}

pub fn ratio_no_clip<'a>(
    ratio: impl F32OrI32,
    cell_render_closure: impl 'a + FnOnce(Wh<Px>, &mut ComposeCtx),
) -> TableCell<'a> {
    TableCell::Some {
        unit: Unit::Ratio(ratio.as_f32()),
        render: Box::new(|_direction, wh, ctx| {
            cell_render_closure(wh, ctx);
        }),
        need_clip: false,
    }
}

pub fn fixed<'a>(
    pixel: Px,
    cell_render_closure: impl 'a + FnOnce(Wh<Px>, &mut ComposeCtx),
) -> TableCell<'a> {
    TableCell::Some {
        unit: Unit::Fixed(pixel),
        render: Box::new(|_direction, wh, ctx| {
            cell_render_closure(wh, ctx);
        }),
        need_clip: true,
    }
}

pub fn fixed_no_clip<'a>(
    pixel: Px,
    cell_render_closure: impl 'a + FnOnce(Wh<Px>, &mut ComposeCtx),
) -> TableCell<'a> {
    TableCell::Some {
        unit: Unit::Fixed(pixel),
        render: Box::new(|_direction, wh, ctx| {
            cell_render_closure(wh, ctx);
        }),
        need_clip: false,
    }
}

pub fn calculative<'a>(
    from_parent_wh: impl FnOnce(Wh<Px>) -> Px + 'a,
    cell_render_closure: impl 'a + FnOnce(Wh<Px>, &mut ComposeCtx),
) -> TableCell<'a> {
    TableCell::Some {
        unit: Unit::Calculative(Box::new(from_parent_wh)),
        render: Box::new(|_direction, wh, ctx| {
            cell_render_closure(wh, ctx);
        }),
        need_clip: true,
    }
}

pub fn calculative_no_clip<'a>(
    from_parent_wh: impl FnOnce(Wh<Px>) -> Px + 'a,
    cell_render_closure: impl 'a + FnOnce(Wh<Px>, &mut ComposeCtx),
) -> TableCell<'a> {
    TableCell::Some {
        unit: Unit::Calculative(Box::new(from_parent_wh)),
        render: Box::new(|_direction, wh, ctx| {
            cell_render_closure(wh, ctx);
        }),
        need_clip: false,
    }
}

pub fn empty<'a>() -> TableCell<'a> {
    TableCell::Empty
}

pub fn vertical<'a, Item: ToKeyCell<'a>>(
    items: impl 'a + IntoIterator<Item = Item>,
) -> impl 'a + FnOnce(Wh<Px>, &mut ComposeCtx) {
    slice_internal(Direction::Vertical, items)
}

pub fn horizontal<'a, Item: ToKeyCell<'a>>(
    items: impl 'a + IntoIterator<Item = Item>,
) -> impl 'a + FnOnce(Wh<Px>, &mut ComposeCtx) {
    slice_internal(Direction::Horizontal, items)
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Vertical,
    Horizontal,
}

pub trait ToKeyCell<'a> {
    fn to_key_cell(self, index: String) -> (String, TableCell<'a>);
}
impl<'a> ToKeyCell<'a> for TableCell<'a> {
    fn to_key_cell(self, index: String) -> (String, TableCell<'a>) {
        (index, self)
    }
}
impl<'a> ToKeyCell<'a> for (&'a str, TableCell<'a>) {
    fn to_key_cell(self, _index: String) -> (String, TableCell<'a>) {
        (self.0.to_string(), self.1)
    }
}

fn slice_internal<'a, Item: ToKeyCell<'a>>(
    direction: Direction,
    items: impl 'a + IntoIterator<Item = Item>,
) -> impl 'a + FnOnce(Wh<Px>, &mut ComposeCtx) {
    move |wh: Wh<Px>, ctx: &mut ComposeCtx| {
        let mut units = Vec::new();
        let mut for_renders = std::collections::VecDeque::new();

        for (index, item) in items.into_iter().enumerate() {
            let (key, cell) = item.to_key_cell(index.to_string());
            match cell {
                TableCell::Empty => {}
                TableCell::Some {
                    unit,
                    render,
                    need_clip,
                } => {
                    units.push(unit);
                    for_renders.push_back((key, render, need_clip));
                }
                TableCell::NeedBoundingBox {
                    func,
                    component_type_name,
                } => {
                    let mut output = None;
                    ctx.ghost_render(Some(key.clone()), component_type_name, |ctx| {
                        output = Some(func(ctx));
                        ctx.done()
                    });
                    match output.unwrap() {
                        TableCell::Empty => {}
                        TableCell::Some {
                            unit,
                            render,
                            need_clip,
                        } => {
                            units.push(unit);
                            for_renders.push_back((key, render, need_clip));
                        }
                        TableCell::NeedBoundingBox { .. } => unreachable!(),
                    }
                }
            }
        }

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

        for pixel_size in pixel_sizes {
            let (key, render_fn, need_clip) = for_renders.pop_front().unwrap();
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

            ctx.compose_with_key(key, |ctx| {
                let mut ctx = ctx.translate((xywh.x(), xywh.y()));
                if need_clip {
                    ctx = ctx.clip(
                        Path::new().add_rect(Rect::Xywh {
                            x: px(0.0),
                            y: px(0.0),
                            width: xywh.width(),
                            height: xywh.height(),
                        }),
                        ClipOp::Intersect,
                    );
                }
                render_fn(direction, xywh.wh(), &mut ctx);
                ctx.debug();
            });

            advanced_pixel_size += pixel_size;
        }
    }
}

pub fn padding<'a>(
    padding: Px,
    cell_render_closure: impl 'a + FnOnce(Wh<Px>, &mut ComposeCtx),
) -> impl 'a + FnOnce(Wh<Px>, &mut ComposeCtx) {
    horizontal_padding(padding, vertical_padding(padding, cell_render_closure))
}

pub fn padding_no_clip<'a>(
    padding: Px,
    cell_render_closure: impl 'a + FnOnce(Wh<Px>, &mut ComposeCtx),
) -> impl 'a + FnOnce(Wh<Px>, &mut ComposeCtx) {
    horizontal_padding_no_clip(
        padding,
        vertical_padding_no_clip(padding, cell_render_closure),
    )
}

pub fn horizontal_padding<'a>(
    padding: Px,
    cell_render_closure: impl 'a + FnOnce(Wh<Px>, &mut ComposeCtx),
) -> impl 'a + FnOnce(Wh<Px>, &mut ComposeCtx) {
    horizontal([
        ("0", fixed(padding, |_, _| {})),
        ("1", ratio(1, cell_render_closure)),
        ("2", fixed(padding, |_, _| {})),
    ])
}

pub fn vertical_padding<'a>(
    padding: Px,
    cell_render_closure: impl 'a + FnOnce(Wh<Px>, &mut ComposeCtx),
) -> impl 'a + FnOnce(Wh<Px>, &mut ComposeCtx) {
    vertical([
        ("0", fixed(padding, |_, _| {})),
        ("1", ratio(1, cell_render_closure)),
        ("2", fixed(padding, |_, _| {})),
    ])
}

pub fn horizontal_padding_no_clip<'a>(
    padding: Px,
    cell_render_closure: impl 'a + FnOnce(Wh<Px>, &mut ComposeCtx),
) -> impl 'a + FnOnce(Wh<Px>, &mut ComposeCtx) {
    horizontal([
        ("0", fixed(padding, |_, _| {})),
        ("1", ratio_no_clip(1, cell_render_closure)),
        ("2", fixed(padding, |_, _| {})),
    ])
}

pub fn vertical_padding_no_clip<'a>(
    padding: Px,
    cell_render_closure: impl 'a + FnOnce(Wh<Px>, &mut ComposeCtx),
) -> impl 'a + FnOnce(Wh<Px>, &mut ComposeCtx) {
    vertical([
        ("0", fixed(padding, |_, _| {})),
        ("1", ratio_no_clip(1, cell_render_closure)),
        ("2", fixed(padding, |_, _| {})),
    ])
}

pub enum FitAlign {
    LeftTop,
    CenterMiddle,
    RightBottom,
}
pub fn fit<'a>(align: FitAlign, component: impl 'a + Component) -> TableCell<'a> {
    TableCell::NeedBoundingBox {
        component_type_name: component.static_type_name(),
        func: Box::new(|ctx| {
            let (bounding_box, set_bounding_box) = ctx.state::<Option<Rect<Px>>>(|| None);
            namui::log!("bounding_box: {:?}", *bounding_box);

            if let Some(bounding_box) = *bounding_box {
                TableCell::Some {
                    unit: Unit::Responsive(Box::new(move |direction| match direction {
                        Direction::Vertical => bounding_box.y() + bounding_box.height(),
                        Direction::Horizontal => bounding_box.x() + bounding_box.width(),
                    })),
                    render: Box::new(move |direction, wh, ctx| {
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
                        ctx.translate((x, y)).add(component);
                    }),
                    need_clip: true,
                }
            } else {
                let rendering_tree = ctx.ghost_render(component);
                let bounding_box = rendering_tree.bounding_box();
                namui::log!("bounding_box: {bounding_box:#?}, rendering_tree: {rendering_tree:#?}");
                if bounding_box.is_some() {
                    set_bounding_box.set(bounding_box);
                }

                TableCell::Empty
            }
        }),
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
            |wh, ctx| {
                button_render_called.store(true, std::sync::atomic::Ordering::Relaxed);
                assert_eq!(px(20.0), wh.width);
                assert_eq!(px(20.0), wh.height);
            },
        );

        let label = ratio(1, |wh, ctx| {
            label_render_called.store(true, std::sync::atomic::Ordering::Relaxed);
            assert_eq!(px(280.0), wh.width);
            assert_eq!(px(20.0), wh.height);
        });

        let header = fixed(px(20.0), horizontal([("button", button), ("label", label)]));

        let body = ratio(1.0, |wh, ctx| {
            body_render_called.store(true, std::sync::atomic::Ordering::Relaxed);
            assert_eq!(px(300.0), wh.width);
            assert_eq!(px(480.0), wh.height);
            vertical([
                (
                    "0",
                    ratio(
                        1,
                        padding(5.px(), |wh, ctx| {
                            body_inner_render_called
                                .store(true, std::sync::atomic::Ordering::Relaxed);
                            assert_eq!(px(290.0), wh.width);
                            assert_eq!(px(470.0), wh.height);
                        }),
                    ),
                ),
                // Note: RenderingTree is not testable yet, So you cannot test fit well now.
                ("empty", fit(FitAlign::LeftTop, RenderingTree::Empty)),
            ])(wh, ctx)
        });

        let ctx = todo!();

        vertical([header, body])(
            Wh {
                width: px(300.0),
                height: px(500.0),
            },
            ctx,
        );

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
