use namui::prelude::*;
use std::sync::Arc;
pub mod traits;
// TODO: Use GAT for Props when GAT is supported by rust.

#[derive(Clone)]
pub enum Size {
    Fixed(f32),
    FromParentWh(Arc<dyn Fn(Wh<f32>) -> f32>),
}
impl Size {
    pub fn from_parent_wh(f: &'static dyn Fn(Wh<f32>) -> f32) -> Self {
        Size::FromParentWh(Arc::new(f))
    }
    fn get_value(&self, parent_wh: Wh<f32>) -> f32 {
        match self {
            Size::Fixed(value) => *value,
            Size::FromParentWh(f) => f(parent_wh),
        }
    }
}

pub struct Fill<'a, Props> {
    pub render: &'a dyn Fn(Wh<f32>, Props) -> RenderingTree,
}
impl<'a, Props> traits::Fill<Props> for Fill<'a, Props> {
    fn render(&self, wh: Wh<f32>, props: Props) -> RenderingTree {
        (self.render)(wh, props)
    }
}

pub struct Column<'a, Props> {
    pub width: Size,
    pub render: &'a dyn Fn(Wh<f32>, Props) -> RenderingTree,
}
impl<'a, Props> traits::Column<Props> for Column<'a, Props> {
    fn get_width(&self, parent_wh: Wh<f32>) -> f32 {
        self.width.get_value(parent_wh)
    }

    fn render(&self, wh: Wh<f32>, props: Props) -> RenderingTree {
        (self.render)(wh, props)
    }
}

pub struct Row<'a, Props> {
    pub height: Size,
    pub render: &'a dyn Fn(Wh<f32>, Props) -> RenderingTree,
}
impl<'a, Props> traits::Row<Props> for Row<'a, Props> {
    fn get_height(&self, parent_wh: Wh<f32>) -> f32 {
        self.height.get_value(parent_wh)
    }

    fn render(&self, wh: Wh<f32>, props: Props) -> RenderingTree {
        (self.render)(wh, props)
    }
}

pub enum Slice<'a, PropsA, PropsB> {
    Left(&'a dyn traits::Column<PropsA>, &'a dyn traits::Fill<PropsB>),
    Right(&'a dyn traits::Fill<PropsA>, &'a dyn traits::Column<PropsB>),
    Top(&'a dyn traits::Row<PropsA>, &'a dyn traits::Fill<PropsB>),
    Bottom(&'a dyn traits::Fill<PropsA>, &'a dyn traits::Row<PropsB>),
}

impl<'a, PropsA, PropsB> traits::RectSlice<(PropsA, PropsB)> for Slice<'a, PropsA, PropsB> {
    fn render(&self, wh: Wh<f32>, (props_a, props_b): (PropsA, PropsB)) -> RenderingTree {
        let a_wh = match self {
            Slice::Left(a, _) => Wh {
                width: a.get_width(wh),
                height: wh.height,
            },
            Slice::Right(_, b) => Wh {
                width: wh.width - b.get_width(wh),
                height: wh.height,
            },
            Slice::Top(a, _) => Wh {
                width: wh.width,
                height: a.get_height(wh),
            },
            Slice::Bottom(_, b) => Wh {
                width: wh.width,
                height: wh.height - b.get_height(wh),
            },
        };

        let b_xywh = match self {
            Slice::Left(_, _) | Slice::Right(_, _) => XywhRect {
                x: a_wh.width,
                y: 0.0,
                width: wh.width - a_wh.width,
                height: wh.height,
            },
            Slice::Top(_, _) | Slice::Bottom(_, _) => XywhRect {
                x: 0.0,
                y: a_wh.height,
                width: wh.width,
                height: wh.height - a_wh.height,
            },
        };

        macro_rules! render_ab {
            ($a: ident, $b: ident, $props_a: ident, $props_b: ident, $a_wh: ident, $b_xywh: ident) => {
                namui::render![
                    $a.render($a_wh, $props_a),
                    namui::translate($b_xywh.x, $b_xywh.y, $b.render($b_xywh.wh(), $props_b))
                ]
            };
        }

        match self {
            Slice::Left(a, b) => render_ab!(a, b, props_a, props_b, a_wh, b_xywh),
            Slice::Right(a, b) => render_ab!(a, b, props_a, props_b, a_wh, b_xywh),
            Slice::Top(a, b) => render_ab!(a, b, props_a, props_b, a_wh, b_xywh),
            Slice::Bottom(a, b) => render_ab!(a, b, props_a, props_b, a_wh, b_xywh),
        }
    }
}

#[cfg(test)]
mod tests {
    use self::traits::RectSlice;
    use super::*;
    use std::sync::atomic::AtomicBool;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn layout_should_give_right_wh() {
        let button_render_called = AtomicBool::new(false);
        let label_render_called = AtomicBool::new(false);
        let body_render_called = AtomicBool::new(false);

        let button = Column {
            width: Size::from_parent_wh(&|parent_wh| parent_wh.height),
            render: &|wh, _: ()| {
                button_render_called.store(true, std::sync::atomic::Ordering::Relaxed);
                assert_eq!(20.0, wh.width);
                assert_eq!(20.0, wh.height);
                RenderingTree::Empty
            },
        };
        let label = Fill {
            render: &|wh, _: ()| {
                label_render_called.store(true, std::sync::atomic::Ordering::Relaxed);
                assert_eq!(280.0, wh.width);
                assert_eq!(20.0, wh.height);
                RenderingTree::Empty
            },
        };

        let header = Row {
            height: Size::Fixed(20.0),
            render: &|wh, props| Slice::Left(&button, &label).render(wh, props),
        };

        let body = Fill {
            render: &|wh, _: ()| {
                body_render_called.store(true, std::sync::atomic::Ordering::Relaxed);
                assert_eq!(300.0, wh.width);
                assert_eq!(480.0, wh.height);
                RenderingTree::Empty
            },
        };

        Slice::Top(&header, &body).render(
            Wh {
                width: 300.0,
                height: 500.0,
            },
            (((), ()), ()),
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
    }

    #[test]
    #[wasm_bindgen_test]
    fn layout_with_traits_should_give_right_wh() {
        let label_render_called = AtomicBool::new(false);

        struct Button {
            render_called: AtomicBool,
        }
        impl<'a> traits::Column<()> for Button {
            fn get_width(&self, parent_wh: Wh<f32>) -> f32 {
                parent_wh.height
            }

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

        let label = Fill {
            render: &|wh, _: ()| {
                label_render_called.store(true, std::sync::atomic::Ordering::Relaxed);
                assert_eq!(280.0, wh.width);
                assert_eq!(20.0, wh.height);
                RenderingTree::Empty
            },
        };

        let header = Row {
            height: Size::Fixed(20.0),
            render: &|wh, props| Slice::Left(&button, &label).render(wh, props),
        };

        struct Body {
            render_called: AtomicBool,
        }
        impl<'a> traits::Fill<()> for Body {
            fn render(&self, wh: Wh<f32>, _props: ()) -> RenderingTree {
                self.render_called
                    .store(true, std::sync::atomic::Ordering::Relaxed);
                assert_eq!(300.0, wh.width);
                assert_eq!(480.0, wh.height);
                RenderingTree::Empty
            }
        }

        let body = Body {
            render_called: AtomicBool::new(false),
        };

        Slice::Top(&header, &body).render(
            Wh {
                width: 300.0,
                height: 500.0,
            },
            (((), ()), ()),
        );

        assert_eq!(
            true,
            button
                .render_called
                .load(std::sync::atomic::Ordering::Relaxed)
        );
        assert_eq!(
            true,
            body.render_called
                .load(std::sync::atomic::Ordering::Relaxed)
        );
    }
}
