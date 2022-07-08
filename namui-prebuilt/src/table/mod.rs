use namui::prelude::*;

pub struct TableCell<'a> {
    unit: Unit,
    render: Box<dyn FnOnce(Wh<Px>) -> RenderingTree + 'a>,
    need_clip: bool,
}

pub enum Unit {
    Ratio(f32),
    Fixed(Px),
    Calculative(Box<dyn FnOnce(Wh<Px>) -> Px>),
}

pub fn ratio<'a>(
    ratio: f32,
    cell_render_closure: impl FnOnce(Wh<Px>) -> RenderingTree + 'a,
) -> TableCell<'a> {
    TableCell {
        unit: Unit::Ratio(ratio),
        render: Box::new(cell_render_closure),
        need_clip: true,
    }
}

pub fn ratio_no_clip<'a>(
    ratio: f32,
    cell_render_closure: impl FnOnce(Wh<Px>) -> RenderingTree + 'a,
) -> TableCell<'a> {
    TableCell {
        unit: Unit::Ratio(ratio),
        render: Box::new(cell_render_closure),
        need_clip: false,
    }
}

pub fn fixed<'a>(
    pixel: Px,
    cell_render_closure: impl FnOnce(Wh<Px>) -> RenderingTree + 'a,
) -> TableCell<'a> {
    TableCell {
        unit: Unit::Fixed(pixel),
        render: Box::new(cell_render_closure),
        need_clip: true,
    }
}

pub fn fixed_no_clip<'a>(
    pixel: Px,
    cell_render_closure: impl FnOnce(Wh<Px>) -> RenderingTree + 'a,
) -> TableCell<'a> {
    TableCell {
        unit: Unit::Fixed(pixel),
        render: Box::new(cell_render_closure),
        need_clip: false,
    }
}

pub fn calculative<'a>(
    from_parent_wh: impl FnOnce(Wh<Px>) -> Px + 'static,
    cell_render_closure: impl FnOnce(Wh<Px>) -> RenderingTree + 'a,
) -> TableCell<'a> {
    TableCell {
        unit: Unit::Calculative(Box::new(from_parent_wh)),
        render: Box::new(cell_render_closure),
        need_clip: true,
    }
}

pub fn calculative_no_clip<'a>(
    from_parent_wh: impl FnOnce(Wh<Px>) -> Px + 'static,
    cell_render_closure: impl FnOnce(Wh<Px>) -> RenderingTree + 'a,
) -> TableCell<'a> {
    TableCell {
        unit: Unit::Calculative(Box::new(from_parent_wh)),
        render: Box::new(cell_render_closure),
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
                    namui::clip(
                        PathBuilder::new().add_rect(Rect::Xywh {
                            x: px(0.0),
                            y: px(0.0),
                            width: xywh.width(),
                            height: xywh.height(),
                        }),
                        ClipOp::Intersect,
                        render_fn(xywh.wh()),
                    )
                } else {
                    render_fn(xywh.wh())
                },
            );

            rendering_tree_list.push(rendering_tree);
            advanced_pixel_size += pixel_size;
        }
        RenderingTree::Children(rendering_tree_list)
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

        let button = calculative(
            |parent_wh| parent_wh.height,
            |wh| {
                button_render_called.store(true, std::sync::atomic::Ordering::Relaxed);
                assert_eq!(px(20.0), wh.width);
                assert_eq!(px(20.0), wh.height);
                RenderingTree::Empty
            },
        );

        let label = ratio(1.0, |wh| {
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
            RenderingTree::Empty
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
    }
}
