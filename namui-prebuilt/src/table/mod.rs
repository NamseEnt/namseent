use namui::prelude::*;

pub trait CellRender<Props> {
    fn render(&self, wh: Wh<f32>, props: Props) -> RenderingTree;
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
) -> (Unit, impl FnOnce(Wh<f32>) -> RenderingTree + 'a) {
    (Unit::Ratio(ratio), |wh: Wh<f32>| {
        cell_render_trait.render(wh, props)
    })
}

pub fn ratio_closure<'a>(
    ratio: f32,
    cell_render_closure: impl FnOnce(Wh<f32>) -> RenderingTree + 'a,
) -> (Unit, impl FnOnce(Wh<f32>) -> RenderingTree + 'a) {
    (Unit::Ratio(ratio), cell_render_closure)
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
) -> (Unit, impl FnOnce(Wh<f32>) -> RenderingTree + 'a) {
    (Unit::Fixed(pixel), |wh: Wh<f32>| {
        cell_render_trait.render(wh, props)
    })
}

pub fn fixed_closure<'a>(
    pixel: f32,
    cell_render_closure: impl FnOnce(Wh<f32>) -> RenderingTree + 'a,
) -> (Unit, impl FnOnce(Wh<f32>) -> RenderingTree + 'a) {
    (Unit::Fixed(pixel), cell_render_closure)
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
) -> (Unit, impl FnOnce(Wh<f32>) -> RenderingTree + 'a) {
    (
        Unit::Calculative(Box::new(from_parent_wh)),
        |wh: Wh<f32>| cell_render_trait.render(wh, props),
    )
}

pub fn calculative_closure<'a>(
    from_parent_wh: impl FnOnce(Wh<f32>) -> f32 + 'static,
    cell_render_closure: impl FnOnce(Wh<f32>) -> RenderingTree + 'a,
) -> (Unit, impl FnOnce(Wh<f32>) -> RenderingTree + 'a) {
    (
        Unit::Calculative(Box::new(from_parent_wh)),
        cell_render_closure,
    )
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
macro_rules! horizontal {
    // item = (Unit, impl FnOnce(Wh<f32>) + 'a)
    ($($item: expr),* $(,)?) => {
        |wh: Wh<f32>| {
            let mut unit_vec = Vec::new();
            let mut render_fn_queue: std::collections::VecDeque<Box<dyn FnOnce(Wh<f32>) -> RenderingTree>>
                = std::collections::VecDeque::new();
            let mut rendering_tree_list: Vec<RenderingTree> = Vec::new();

            $(
                let (unit, render_fn) = $item;
                unit_vec.push(unit);
                let render_fn = Box::new(render_fn);
                render_fn_queue.push_back(render_fn);
            )*

            let ratio_sum = unit_vec.iter().fold(0.0, |sum, unit| match unit {
                $crate::table::Unit::Ratio(ratio) => sum + ratio,
                _ => sum,
            });

            let width_or_ratio_list = unit_vec
                .into_iter()
                .map(|unit| {
                    let (width, ratio) = match unit {
                        $crate::table::Unit::Ratio(ratio) => (None, Some(ratio)),
                        $crate::table::Unit::Fixed(pixel_size) => (Some(pixel_size), None),
                        $crate::table::Unit::Calculative(calculative_fn) => (Some(calculative_fn(wh)), None),
                    };
                    (width, ratio)
                })
                .collect::<Vec<_>>();

            let non_ratio_width_sum = width_or_ratio_list
                .iter()
                .filter_map(|(width, _ratio)| *width)
                .fold(0.0, |sum, width| sum + width);

            let widths = width_or_ratio_list.iter().map(|(width, ratio)| {
                if let Some(width) = width {
                    *width
                } else {
                    (wh.width - non_ratio_width_sum) * ratio.unwrap() / ratio_sum
                }
            });

            let mut x = 0.0;

            for width in widths {
                let render_fn = render_fn_queue.pop_front().unwrap();
                let rendering_tree = namui::translate(
                    x,
                    0.0,
                    render_fn(Wh {
                        width,
                        height: wh.height,
                    }),
                );

                rendering_tree_list.push(rendering_tree);
                x += width;
            }
            RenderingTree::Children(rendering_tree_list)
        }
    };
}

#[macro_export]
macro_rules! vertical {
    // item = (Unit, impl FnOnce(Wh<f32>) + 'a)
    ($($item: expr),* $(,)?) => {
        |wh: Wh<f32>| {
            let mut unit_vec = Vec::new();
            let mut render_fn_queue: std::collections::VecDeque<Box<dyn FnOnce(Wh<f32>) -> RenderingTree>>
                = std::collections::VecDeque::new();
            let mut rendering_tree_list: Vec<RenderingTree> = Vec::new();

            $(
                let (unit, render_fn) = $item;
                unit_vec.push(unit);
                let render_fn = Box::new(render_fn);
                render_fn_queue.push_back(render_fn);
            )*

            let ratio_sum = unit_vec.iter().fold(0.0, |sum, unit| match unit {
                $crate::table::Unit::Ratio(ratio) => sum + ratio,
                _ => sum,
            });

            let height_or_ratio_list = unit_vec
                .into_iter()
                .map(|unit| {
                    let (height, ratio) = match unit {
                        $crate::table::Unit::Ratio(ratio) => (None, Some(ratio)),
                        $crate::table::Unit::Fixed(pixel_size) => (Some(pixel_size), None),
                        $crate::table::Unit::Calculative(calculative_fn) => (Some(calculative_fn(wh)), None),
                    };
                    (height, ratio)
                })
                .collect::<Vec<_>>();

            let non_ratio_height_sum = height_or_ratio_list
                .iter()
                .filter_map(|(height, _ratio)| *height)
                .fold(0.0, |sum, height| sum + height);

            let heights = height_or_ratio_list.iter().map(|(height, ratio)| {
                if let Some(height) = height {
                    *height
                } else {
                    (wh.height - non_ratio_height_sum) * ratio.unwrap() / ratio_sum
                }
            });

            let mut y = 0.0;

            for height in heights {
                let render_fn = render_fn_queue.pop_front().unwrap();
                let rendering_tree = namui::translate(
                    0.0,
                    y,
                    render_fn(Wh {
                        width: wh.width,
                        height,
                    }),
                );

                rendering_tree_list.push(rendering_tree);
                y += height;
            }
            RenderingTree::Children(rendering_tree_list)
        }
    };
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
