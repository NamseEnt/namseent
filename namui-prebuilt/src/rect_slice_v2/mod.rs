use namui::prelude::*;

pub trait WhRender<Props> {
    fn render(&self, wh: Wh<f32>, props: Props) -> RenderingTree;
}

pub enum Unit {
    Ratio(f32),
    Fixed(f32),
    Calculative(Box<dyn FnOnce(Wh<f32>) -> f32>),
}

pub fn ratio_trait<'a, Props: 'a>(
    ratio: f32,
    wh_render: &'a impl WhRender<Props>,
    props: Props,
) -> (Unit, impl FnOnce(Wh<f32>) -> RenderingTree + 'a) {
    (Unit::Ratio(ratio), |wh: Wh<f32>| {
        wh_render.render(wh, props)
    })
}

pub fn ratio_closure<'a>(
    ratio: f32,
    wh_render: impl FnOnce(Wh<f32>) -> RenderingTree + 'a,
) -> (Unit, impl FnOnce(Wh<f32>) -> RenderingTree + 'a) {
    (Unit::Ratio(ratio), wh_render)
}

#[macro_export]
macro_rules! ratio {
    ($ratio: expr, $wh_render_trait: expr, $props: expr) => {
        $crate::rect_slice_v2::ratio_trait($ratio, $wh_render_trait, $props)
    };
    ($ratio: expr, $wh_render_closure: expr) => {
        $crate::rect_slice_v2::ratio_closure($ratio, $wh_render_closure)
    };
}

pub fn fixed_trait<'a, Props: 'a>(
    pixel: f32,
    wh_render: &'a impl WhRender<Props>,
    props: Props,
) -> (Unit, impl FnOnce(Wh<f32>) -> RenderingTree + 'a) {
    (Unit::Fixed(pixel), |wh: Wh<f32>| {
        wh_render.render(wh, props)
    })
}

pub fn fixed_closure<'a>(
    pixel: f32,
    wh_render: impl FnOnce(Wh<f32>) -> RenderingTree + 'a,
) -> (Unit, impl FnOnce(Wh<f32>) -> RenderingTree + 'a) {
    (Unit::Fixed(pixel), wh_render)
}

#[macro_export]
macro_rules! fixed {
    ($pixel: expr, $wh_render_trait: expr, $props: expr) => {
        $crate::rect_slice_v2::fixed_trait($pixel, $wh_render_trait, $props)
    };
    ($pixel: expr, $wh_render_closure: expr) => {
        $crate::rect_slice_v2::fixed_closure($pixel, $wh_render_closure)
    };
}

pub fn calculative_trait<'a, Props: 'a>(
    from_parent_wh: impl FnOnce(Wh<f32>) -> f32 + 'static,
    wh_render: &'a impl WhRender<Props>,
    props: Props,
) -> (Unit, impl FnOnce(Wh<f32>) -> RenderingTree + 'a) {
    (
        Unit::Calculative(Box::new(from_parent_wh)),
        |wh: Wh<f32>| wh_render.render(wh, props),
    )
}

pub fn calculative_closure<'a>(
    from_parent_wh: impl FnOnce(Wh<f32>) -> f32 + 'static,
    wh_render: impl FnOnce(Wh<f32>) -> RenderingTree + 'a,
) -> (Unit, impl FnOnce(Wh<f32>) -> RenderingTree + 'a) {
    (Unit::Calculative(Box::new(from_parent_wh)), wh_render)
}

#[macro_export]
macro_rules! calculative {
    ($from_parent_wh: expr, $wh_render_trait: expr, $props: expr) => {
        $crate::rect_slice_v2::calculative_trait($from_parent_wh, $wh_render_trait, $props)
    };
    ($from_parent_wh: expr, $wh_render_closure: expr) => {
        $crate::rect_slice_v2::calculative_closure($from_parent_wh, $wh_render_closure)
    };
}

#[macro_export]
macro_rules! horizontal {
    // item = (Unit, impl FnOnce(Wh<f32>) + 'a)
    ($($item: expr,)*) => {
        |wh: Wh<f32>| {
            let mut unit_vec = Vec::new();
            let mut render_fn_vec: Vec<Box<dyn FnOnce(Wh<f32>) -> RenderingTree>> = Vec::new();
            let mut rendering_tree_list: Vec<RenderingTree> = Vec::new();

            $(
                let (unit, render_fn) = $item;
                unit_vec.push(unit);
                let render_fn = Box::new(render_fn);
                render_fn_vec.push(render_fn);
            )*

            let ratio_sum = unit_vec.iter().fold(0.0, |sum, unit| match unit {
                $crate::rect_slice_v2::Unit::Ratio(ratio) => sum + ratio,
                _ => sum,
            });

            let width_or_ratio_list = unit_vec
                .into_iter()
                .map(|unit| {
                    let (width, ratio) = match unit {
                        $crate::rect_slice_v2::Unit::Ratio(ratio) => (None, Some(ratio)),
                        $crate::rect_slice_v2::Unit::Fixed(pixel_size) => (Some(pixel_size), None),
                        $crate::rect_slice_v2::Unit::Calculative(calculative_fn) => (Some(calculative_fn(wh)), None),
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

            for (index, width) in widths.enumerate() {
                let render_fn = render_fn_vec.swap_remove(index);
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
    ($($item: expr,)*) => {
        |wh: Wh<f32>| {
            let mut unit_vec = Vec::new();
            let mut render_fn_vec: Vec<Box<dyn FnOnce(Wh<f32>) -> RenderingTree>> = Vec::new();
            let mut rendering_tree_list: Vec<RenderingTree> = Vec::new();

            $(
                let (unit, render_fn) = $item;
                unit_vec.push(unit);
                let render_fn = Box::new(render_fn);
                render_fn_vec.push(render_fn);
            )*

            let ratio_sum = unit_vec.iter().fold(0.0, |sum, unit| match unit {
                $crate::rect_slice_v2::Unit::Ratio(ratio) => sum + ratio,
                _ => sum,
            });

            let height_or_ratio_list = unit_vec
                .into_iter()
                .map(|unit| {
                    let (height, ratio) = match unit {
                        $crate::rect_slice_v2::Unit::Ratio(ratio) => (None, Some(ratio)),
                        $crate::rect_slice_v2::Unit::Fixed(pixel_size) => (Some(pixel_size), None),
                        $crate::rect_slice_v2::Unit::Calculative(calculative_fn) => (Some(calculative_fn(wh)), None),
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

            for (index, height) in heights.enumerate() {
                let render_fn = render_fn_vec.swap_remove(index);
                let rendering_tree = namui::translate(
                    y,
                    0.0,
                    render_fn(Wh {
                        width: wh.width,
                        height: height,
                    }),
                );

                rendering_tree_list.push(rendering_tree);
                y += height;
            }
            RenderingTree::Children(rendering_tree_list)
        }
    };
}

struct A {
    a: f32,
}
impl WhRender<()> for A {
    fn render(&self, wh: Wh<f32>, _props: ()) -> RenderingTree {
        RenderingTree::Empty
    }
}

struct B {}
impl WhRender<f32> for B {
    fn render(&self, wh: Wh<f32>, _props: f32) -> RenderingTree {
        RenderingTree::Empty
    }
}

struct AA {
    a: A,
    b: B,
    c: B,
}

impl<'a> AA {
    fn test(&'a self) -> impl FnOnce(Wh<f32>) + 'a {
        let ra = ratio!(1.0, &self.a, ());
        let rb = fixed!(1.0, &self.b, 1.0);
        let rc = calculative!(|wh| wh.width / 1920.0 * 1080.0, &self.c, 1.0);

        |wh: Wh<f32>| {
            ra.1(wh);
        }
    }
}

fn test_aa() {
    let aa = AA {
        a: A { a: 1.0 },
        b: B {},
        c: B {},
    };
    let foo = aa.test();
}

fn test() {
    let a = A { a: 1.0 };
    let b = B {};
    let c = B {};

    let b = horizontal![
        ratio!(1.0, &a, ()),
        fixed!(1.0, &b, 1.0),
        calculative!(|wh| wh.width / 1920.0 * 1080.0, &b, 1.0),
    ];

    let a = ratio!(1.0, b);
}
