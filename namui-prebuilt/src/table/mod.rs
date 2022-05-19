use namui::prelude::*;

pub trait CellRender<Props> {
    fn render(&self, wh: Wh<f32>, props: Props) -> RenderingTree;
}

pub struct TableCell<'a> {
    unit: Unit,
    render: Box<dyn FnOnce(Wh<f32>) -> RenderingTree + 'a>,
}

pub enum Unit {
    Ratio(f32),
    Fixed(f32),
    Calculative(Box<dyn FnOnce(Wh<f32>) -> f32>),
}

pub fn ratio_trait<'a, Props: 'a>(
    ratio: f32,
    cell_render_trait: &'a impl CellRender<Props>,
    props: Props,
) -> TableCell<'a> {
    TableCell {
        unit: Unit::Ratio(ratio),
        render: Box::new(move |wh| cell_render_trait.render(wh, props)),
    }
}

pub fn ratio_closure<'a>(
    ratio: f32,
    cell_render_closure: impl FnOnce(Wh<f32>) -> RenderingTree + 'a,
) -> TableCell<'a> {
    TableCell {
        unit: Unit::Ratio(ratio),
        render: Box::new(cell_render_closure),
    }
}

#[macro_export]
macro_rules! ratio {
    ($ratio: expr, $cell_render_trait: expr, $props: expr) => {
        $crate::table::ratio_trait($ratio, $cell_render_trait, $props)
    };
    ($ratio: expr, $cell_render_closure: expr) => {
        $crate::table::ratio_closure($ratio, $cell_render_closure)
    };
}

pub fn fixed_trait<'a, Props: 'a>(
    pixel: f32,
    cell_render_trait: &'a impl CellRender<Props>,
    props: Props,
) -> TableCell<'a> {
    TableCell {
        unit: Unit::Fixed(pixel),
        render: Box::new(move |wh| cell_render_trait.render(wh, props)),
    }
}

pub fn fixed_closure<'a>(
    pixel: f32,
    cell_render_closure: impl FnOnce(Wh<f32>) -> RenderingTree + 'a,
) -> TableCell<'a> {
    TableCell {
        unit: Unit::Fixed(pixel),
        render: Box::new(cell_render_closure),
    }
}

#[macro_export]
macro_rules! fixed {
    ($pixel: expr, $cell_render_trait: expr, $props: expr) => {
        $crate::table::fixed_trait($pixel, $cell_render_trait, $props)
    };
    ($pixel: expr, $cell_render_closure: expr) => {
        $crate::table::fixed_closure($pixel, $cell_render_closure)
    };
}

pub fn calculative_trait<'a, Props: 'a>(
    from_parent_wh: impl FnOnce(Wh<f32>) -> f32 + 'static,
    cell_render_trait: &'a impl CellRender<Props>,
    props: Props,
) -> TableCell<'a> {
    TableCell {
        unit: Unit::Calculative(Box::new(from_parent_wh)),
        render: Box::new(move |wh| cell_render_trait.render(wh, props)),
    }
}

pub fn calculative_closure<'a>(
    from_parent_wh: impl FnOnce(Wh<f32>) -> f32 + 'static,
    cell_render_closure: impl FnOnce(Wh<f32>) -> RenderingTree + 'a,
) -> TableCell<'a> {
    TableCell {
        unit: Unit::Calculative(Box::new(from_parent_wh)),
        render: Box::new(cell_render_closure),
    }
}

#[macro_export]
macro_rules! calculative {
    ($from_parent_wh: expr, $cell_render_trait: expr, $props: expr) => {
        $crate::table::calculative_trait($from_parent_wh, $cell_render_trait, $props)
    };
    ($from_parent_wh: expr, $cell_render_closure: expr) => {
        $crate::table::calculative_closure($from_parent_wh, $cell_render_closure)
    };
}

#[macro_export]
macro_rules! vertical {
    // item = CellEntity<'a>
    ($($item: expr),* $(,)?) => {{
        let render_fn_boxed_items = [
            // $(($item.0, Box::new($item.1) as Box<dyn FnOnce(Wh<f32>) -> RenderingTree>),)*
            $($item,)*
        ];

        $crate::table::vertical(render_fn_boxed_items.into_iter())
    }};
}

#[macro_export]
macro_rules! horizontal {
    // item = CellEntity<'a>
    ($($item: expr),* $(,)?) => {{
        let render_fn_boxed_items = [
            // $(($item.0, Box::new($item.1) as Box<dyn FnOnce(Wh<f32>) -> RenderingTree>),)*
            $($item,)*
        ];

        $crate::table::horizontal(render_fn_boxed_items.into_iter())
    }};
}

pub fn vertical<'a>(
    items: impl Iterator<Item = TableCell<'a>> + 'a,
) -> impl FnOnce(Wh<f32>) -> RenderingTree + 'a {
    slice_internal(Direction::Vertical, items)
}

pub fn horizontal<'a>(
    items: impl Iterator<Item = TableCell<'a>> + 'a,
) -> impl FnOnce(Wh<f32>) -> RenderingTree + 'a {
    slice_internal(Direction::Horizontal, items)
}

enum Direction {
    Vertical,
    Horizontal,
}
fn slice_internal<'a>(
    direction: Direction,
    items: impl Iterator<Item = TableCell<'a>> + 'a,
) -> impl FnOnce(Wh<f32>) -> RenderingTree + 'a {
    let mut units = Vec::new();
    let mut render_fns = std::collections::VecDeque::new();

    for item in items {
        units.push(item.unit);
        render_fns.push_back(item.render);
    }

    move |wh: Wh<f32>| {
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

        let non_ratio_pixel_size_sum = pixel_size_or_ratio_list
            .iter()
            .filter_map(|(pixel_size, _ratio)| *pixel_size)
            .fold(0.0, |sum, pixel_size| sum + pixel_size);

        let pixel_sizes = pixel_size_or_ratio_list.iter().map(|(pixel_size, ratio)| {
            if let Some(pixel_size) = pixel_size {
                *pixel_size
            } else {
                (direction_pixel_size - non_ratio_pixel_size_sum) * ratio.unwrap() / ratio_sum
            }
        });

        let mut advanced_pixel_size = 0.0;

        for pixel_size in pixel_sizes {
            let render_fn = render_fns.pop_front().unwrap();
            let xywh = match direction {
                Direction::Vertical => XywhRect {
                    x: 0.0,
                    y: advanced_pixel_size,
                    width: wh.width,
                    height: pixel_size,
                },
                Direction::Horizontal => XywhRect {
                    x: advanced_pixel_size,
                    y: 0.0,
                    width: pixel_size,
                    height: wh.height,
                },
            };
            let rendering_tree = namui::translate(xywh.x, xywh.y, render_fn(xywh.wh()));

            rendering_tree_list.push(rendering_tree);
            advanced_pixel_size += pixel_size;
        }
        RenderingTree::Children(rendering_tree_list)
    }
}

#[macro_export]
macro_rules! chains {
    // item = CellEntity<'a>
    ($($item_iterator: expr),* $(,)?) => {{
        let render_fn_boxed_items = [].into_iter()
        $(.chain($item_iterator.into_iter()))*
        ;

        render_fn_boxed_items.into_iter()
    }};
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

        let button = calculative!(|parent_wh| { parent_wh.height }, |wh| {
            button_render_called.store(true, std::sync::atomic::Ordering::Relaxed);
            assert_eq!(20.0, wh.width);
            assert_eq!(20.0, wh.height);
            RenderingTree::Empty
        });

        let label = ratio!(1.0, |wh| {
            label_render_called.store(true, std::sync::atomic::Ordering::Relaxed);
            assert_eq!(280.0, wh.width);
            assert_eq!(20.0, wh.height);
            RenderingTree::Empty
        });

        let header = fixed!(20.0, horizontal![button, label]);

        let body = ratio!(1.0, |wh| {
            body_render_called.store(true, std::sync::atomic::Ordering::Relaxed);
            assert_eq!(300.0, wh.width);
            assert_eq!(480.0, wh.height);
            RenderingTree::Empty
        });

        vertical![header, body](Wh {
            width: 300.0,
            height: 500.0,
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

    #[test]
    #[wasm_bindgen_test]
    fn trait_should_give_right_wh() {
        let body_render_called = AtomicBool::new(false);
        let label_render_called = AtomicBool::new(false);

        struct Button {
            render_called: AtomicBool,
        }
        impl CellRender<()> for Button {
            fn render(&self, wh: Wh<f32>, _props: ()) -> RenderingTree {
                self.render_called
                    .store(true, std::sync::atomic::Ordering::Relaxed);
                assert_eq!(20.0, wh.width);
                assert_eq!(20.0, wh.height);
                RenderingTree::Empty
            }
        }

        let button = Button {
            render_called: AtomicBool::new(false),
        };

        let label = ratio!(1.0, |wh| {
            label_render_called.store(true, std::sync::atomic::Ordering::Relaxed);
            assert_eq!(280.0, wh.width);
            assert_eq!(20.0, wh.height);
            RenderingTree::Empty
        });

        let header = fixed!(
            20.0,
            horizontal![
                calculative!(|parent_wh| { parent_wh.height }, &button, ()),
                label
            ]
        );

        struct Body {}
        impl CellRender<&'_ AtomicBool> for Body {
            fn render(&self, wh: Wh<f32>, props: &AtomicBool) -> RenderingTree {
                props.store(true, std::sync::atomic::Ordering::Relaxed);
                assert_eq!(300.0, wh.width);
                assert_eq!(480.0, wh.height);
                RenderingTree::Empty
            }
        }

        let body = Body {};

        vertical![header, ratio!(1.0, &body, &body_render_called)](Wh {
            width: 300.0,
            height: 500.0,
        });

        assert_eq!(
            true,
            button
                .render_called
                .load(std::sync::atomic::Ordering::Relaxed)
        );
        assert_eq!(
            true,
            body_render_called.load(std::sync::atomic::Ordering::Relaxed)
        );
        assert_eq!(
            true,
            label_render_called.load(std::sync::atomic::Ordering::Relaxed)
        );
    }
}
