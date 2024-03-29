pub mod hooks;

use namui::*;

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
    cell_render_closure: impl FnOnce(Wh<Px>) -> RenderingTree + 'a,
) -> TableCell<'a> {
    TableCell {
        unit: Unit::Ratio(ratio.into_f32()),
        render: Box::new(|_direction, wh| cell_render_closure(wh)),
        need_clip: true,
    }
}

pub fn ratio_no_clip<'a>(
    ratio: impl F32OrI32,
    cell_render_closure: impl FnOnce(Wh<Px>) -> RenderingTree + 'a,
) -> TableCell<'a> {
    TableCell {
        unit: Unit::Ratio(ratio.into_f32()),
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

#[derive(Copy, Clone)]
enum Direction {
    Vertical,
    Horizontal,
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
            let rendering_tree = namui::translate(
                xywh.x(),
                xywh.y(),
                if need_clip {
                    RenderingTree::Special(SpecialRenderingNode::Clip(ClipNode {
                        path: Path::new().add_rect(Rect::Xywh {
                            x: px(0.0),
                            y: px(0.0),
                            width: xywh.width(),
                            height: xywh.height(),
                        }),
                        clip_op: ClipOp::Intersect,
                        rendering_tree: Box::new(render_fn(direction, xywh.wh())),
                    }))
                } else {
                    render_fn(direction, xywh.wh())
                },
            );

            rendering_tree_list.push(rendering_tree);
            advanced_pixel_size += pixel_size;
        }
        render(rendering_tree_list)
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
    match namui::bounding_box(&rendering_tree) {
        Some(bounding_box) => TableCell {
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
                translate(x, y, rendering_tree)
            }),
            need_clip: true,
        },
        None => ratio(0, |_| rendering_tree),
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::sync::atomic::AtomicBool;

//     #[test]
//     fn closure_should_give_right_wh() {
//         let button_render_called = AtomicBool::new(false);
//         let label_render_called = AtomicBool::new(false);
//         let body_render_called = AtomicBool::new(false);
//         let body_inner_render_called = AtomicBool::new(false);

//         let button = calculative(
//             |parent_wh| parent_wh.height,
//             |wh| {
//                 button_render_called.store(true, std::sync::atomic::Ordering::Relaxed);
//                 assert_eq!(px(20.0), wh.width);
//                 assert_eq!(px(20.0), wh.height);
//                 RenderingTree::Empty
//             },
//         );

//         let label = ratio(1, |wh| {
//             label_render_called.store(true, std::sync::atomic::Ordering::Relaxed);
//             assert_eq!(px(280.0), wh.width);
//             assert_eq!(px(20.0), wh.height);
//             RenderingTree::Empty
//         });

//         let header = fixed(px(20.0), horizontal([button, label]));

//         let body = ratio(1.0, |wh| {
//             body_render_called.store(true, std::sync::atomic::Ordering::Relaxed);
//             assert_eq!(px(300.0), wh.width);
//             assert_eq!(px(480.0), wh.height);
//             vertical([
//                 ratio(
//                     1,
//                     padding(5.px(), |wh| {
//                         body_inner_render_called.store(true, std::sync::atomic::Ordering::Relaxed);
//                         assert_eq!(px(290.0), wh.width);
//                         assert_eq!(px(470.0), wh.height);
//                         RenderingTree::Empty
//                     }),
//                 ),
//                 // Note: RenderingTree is not testable yet, So you cannot test fit well now.
//                 fit(FitAlign::LeftTop, RenderingTree::Empty),
//             ])(wh)
//         });

//         vertical([header, body])(Wh {
//             width: px(300.0),
//             height: px(500.0),
//         });

//         assert!(button_render_called.load(std::sync::atomic::Ordering::Relaxed));
//         assert!(label_render_called.load(std::sync::atomic::Ordering::Relaxed));
//         assert!(body_render_called.load(std::sync::atomic::Ordering::Relaxed));
//         assert!(body_inner_render_called.load(std::sync::atomic::Ordering::Relaxed));
//     }
// }
