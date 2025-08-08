#[cfg(test)]
mod tests;

use namui::*;
use std::collections::HashMap;

pub enum TableCell<'a> {
    Empty,
    Some {
        unit: Unit<'a>,
        render: TableCellRenderFn<'a>,
        need_clip: bool,
    },
    Fit {
        align: FitAlign,
        render: TableCellRenderFn<'a>,
    },
}
type TableCellRenderFn<'a> = Box<dyn 'a + FnOnce(Direction, Wh<Px>, ComposeCtx)>;

pub enum Unit<'a> {
    Ratio(f32),
    Fixed(Px),
    Empty,
    Calculative(Box<dyn 'a + FnOnce(Wh<Px>) -> Px>),
}

pub trait F32OrI32 {
    fn into_f32(self) -> f32;
}

impl F32OrI32 for i32 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl F32OrI32 for f32 {
    fn into_f32(self) -> f32 {
        self
    }
}

pub fn ratio<'a>(
    ratio: impl F32OrI32,
    cell_render_closure: impl 'a + FnOnce(Wh<Px>, ComposeCtx),
) -> TableCell<'a> {
    TableCell::Some {
        unit: Unit::Ratio(ratio.into_f32()),
        render: Box::new(|_direction, wh, ctx| {
            cell_render_closure(wh, ctx);
        }),
        need_clip: true,
    }
}

pub fn ratio_no_clip<'a>(
    ratio: impl F32OrI32,
    cell_render_closure: impl 'a + FnOnce(Wh<Px>, ComposeCtx),
) -> TableCell<'a> {
    TableCell::Some {
        unit: Unit::Ratio(ratio.into_f32()),
        render: Box::new(|_direction, wh, ctx| {
            cell_render_closure(wh, ctx);
        }),
        need_clip: false,
    }
}

pub fn fixed<'a>(
    pixel: Px,
    cell_render_closure: impl 'a + FnOnce(Wh<Px>, ComposeCtx),
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
    cell_render_closure: impl 'a + FnOnce(Wh<Px>, ComposeCtx),
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
    cell_render_closure: impl 'a + FnOnce(Wh<Px>, ComposeCtx),
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
    cell_render_closure: impl 'a + FnOnce(Wh<Px>, ComposeCtx),
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
) -> impl 'a + FnOnce(Wh<Px>, ComposeCtx) {
    slice_internal(Direction::Vertical, items)
}

pub fn horizontal<'a, Item: ToKeyCell<'a>>(
    items: impl 'a + IntoIterator<Item = Item>,
) -> impl 'a + FnOnce(Wh<Px>, ComposeCtx) {
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

struct InternalSlice<'a> {
    wh: Wh<Px>,
    items: Vec<(String, TableCell<'a>)>,
    direction: Direction,
}

impl Component for InternalSlice<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            items,
            direction,
            ..
        } = self;

        let (fit_bounding_box_map, set_bounding_box_map) =
            ctx.state(HashMap::<usize, Option<Rect<Px>>>::new);

        let mut intermediates: Vec<Intermediate> = vec![];
        let mut units: Vec<Unit> = vec![];

        type RenderFn<'a> = Box<dyn 'a + FnOnce(Direction, Wh<Px>, ComposeCtx)>;

        struct Intermediate<'a> {
            key: String,
            render: RenderFn<'a>,
            need_clip: bool,
            table_cell_type: TableCellType,
        }

        #[derive(Clone, Copy)]
        enum TableCellType {
            Some,
            Fit { align: FitAlign },
        }

        for (index, (key, cell)) in items.into_iter().enumerate() {
            match cell {
                TableCell::Empty => {}
                TableCell::Some {
                    unit,
                    render,
                    need_clip,
                } => {
                    intermediates.push(Intermediate {
                        key,
                        render,
                        need_clip,
                        table_cell_type: TableCellType::Some,
                    });
                    units.push(unit);
                }
                TableCell::Fit { align, render } => {
                    let unit = if let Some(Some(bounding_box)) = fit_bounding_box_map.get(&index) {
                        Unit::Fixed(match direction {
                            Direction::Vertical => bounding_box.y() + bounding_box.height(),
                            Direction::Horizontal => bounding_box.x() + bounding_box.width(),
                        })
                    } else {
                        Unit::Empty
                    };
                    units.push(unit);

                    intermediates.push(Intermediate {
                        key,
                        render,
                        need_clip: false,
                        table_cell_type: TableCellType::Fit { align },
                    });
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
                    Unit::Empty => (None, None),
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
            } else if let Some(ratio) = ratio {
                (direction_pixel_size - non_ratio_pixel_size_sum) * *ratio / ratio_sum
            } else {
                0.px()
            }
        });

        let mut advanced_pixel_size = px(0.0);

        for (
            index,
            (
                pixel_size,
                Intermediate {
                    key,
                    render,
                    need_clip,
                    table_cell_type,
                },
            ),
        ) in pixel_sizes
            .into_iter()
            .zip(intermediates.into_iter())
            .enumerate()
        {
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

            ctx.compose_with_key(key, |mut ctx| {
                ctx = ctx.translate((xywh.x(), xywh.y()));

                if let TableCellType::Fit { align } = table_cell_type {
                    let bounding_box = fit_bounding_box_map.get(&index);
                    if let Some(Some(bounding_box)) = bounding_box {
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
                        ctx = ctx.translate((x, y));
                    }
                }
                ctx.compose(|ctx| {
                    let rendering_tree = ctx.ghost_compose(0_usize, |mut ctx| {
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
                        render(direction, xywh.wh(), ctx);
                    });

                    if let TableCellType::Fit { .. } = table_cell_type {
                        let is_first_draw = fit_bounding_box_map.get(&index).is_none();
                        let bounding_box = namui::bounding_box(&rendering_tree);
                        set_bounding_box_map.mutate({
                            move |bounding_box_map| {
                                bounding_box_map.insert(index, bounding_box);
                            }
                        });

                        if !is_first_draw {
                            ctx.add(rendering_tree);
                        }
                    } else {
                        ctx.add(rendering_tree);
                    }
                });
            });

            advanced_pixel_size += pixel_size;
        }
    }
}

fn slice_internal<'a, Item: ToKeyCell<'a>>(
    direction: Direction,
    items: impl 'a + IntoIterator<Item = Item>,
) -> impl 'a + FnOnce(Wh<Px>, ComposeCtx) {
    move |wh: Wh<Px>, ctx: ComposeCtx| {
        ctx.add(InternalSlice {
            wh,
            items: items
                .into_iter()
                .enumerate()
                .map(|(index, item)| item.to_key_cell(index.to_string()))
                .collect(),
            direction,
        });
    }
}

pub fn padding<'a>(
    padding: Px,
    cell_render_closure: impl 'a + FnOnce(Wh<Px>, ComposeCtx),
) -> impl 'a + FnOnce(Wh<Px>, ComposeCtx) {
    horizontal_padding(padding, vertical_padding(padding, cell_render_closure))
}

pub fn padding_no_clip<'a>(
    padding: Px,
    cell_render_closure: impl 'a + FnOnce(Wh<Px>, ComposeCtx),
) -> impl 'a + FnOnce(Wh<Px>, ComposeCtx) {
    horizontal_padding_no_clip(
        padding,
        vertical_padding_no_clip(padding, cell_render_closure),
    )
}

pub fn horizontal_padding<'a>(
    padding: Px,
    cell_render_closure: impl 'a + FnOnce(Wh<Px>, ComposeCtx),
) -> impl 'a + FnOnce(Wh<Px>, ComposeCtx) {
    horizontal([
        ("0", fixed(padding, |_, _| {})),
        ("1", ratio(1, cell_render_closure)),
        ("2", fixed(padding, |_, _| {})),
    ])
}

pub fn vertical_padding<'a>(
    padding: Px,
    cell_render_closure: impl 'a + FnOnce(Wh<Px>, ComposeCtx),
) -> impl 'a + FnOnce(Wh<Px>, ComposeCtx) {
    vertical([
        ("0", fixed(padding, |_, _| {})),
        ("1", ratio(1, cell_render_closure)),
        ("2", fixed(padding, |_, _| {})),
    ])
}

pub fn horizontal_padding_no_clip<'a>(
    padding: Px,
    cell_render_closure: impl 'a + FnOnce(Wh<Px>, ComposeCtx),
) -> impl 'a + FnOnce(Wh<Px>, ComposeCtx) {
    horizontal([
        ("0", fixed(padding, |_, _| {})),
        ("1", ratio_no_clip(1, cell_render_closure)),
        ("2", fixed(padding, |_, _| {})),
    ])
}

pub fn vertical_padding_no_clip<'a>(
    padding: Px,
    cell_render_closure: impl 'a + FnOnce(Wh<Px>, ComposeCtx),
) -> impl 'a + FnOnce(Wh<Px>, ComposeCtx) {
    vertical([
        ("0", fixed(padding, |_, _| {})),
        ("1", ratio_no_clip(1, cell_render_closure)),
        ("2", fixed(padding, |_, _| {})),
    ])
}

#[derive(Debug, Clone, Copy)]
pub enum FitAlign {
    LeftTop,
    CenterMiddle,
    RightBottom,
}

pub fn fit<'a>(
    align: FitAlign,
    cell_render_closure: impl 'a + FnOnce(ComposeCtx),
) -> TableCell<'a> {
    TableCell::Fit {
        align,
        render: Box::new(|_direction, _wh, ctx| {
            cell_render_closure(ctx);
        }),
    }
}
