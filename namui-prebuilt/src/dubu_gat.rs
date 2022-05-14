// NOTE: This is unused file for now. I will update it as main when GAT is ready on rust.
use namui::prelude::*;

pub trait Dubu {
    type Props;
    fn render(&self, wh: Wh<f32>, props: Self::Props) -> RenderingTree;
}

#[derive(Clone, Copy)]
pub enum Size<'a> {
    Fixed(f32),
    FromParentWh(&'a dyn Fn(Wh<f32>) -> f32),
}
impl Size<'_> {
    fn get_value(&self, parent_wh: Wh<f32>) -> f32 {
        match self {
            Size::Fixed(value) => *value,
            Size::FromParentWh(f) => f(parent_wh),
        }
    }
}

#[derive(Clone)]
pub enum Slice<'a, PropsA, PropsB> {
    Left(Column<'a, PropsA>, &'a dyn Dubu<Props = PropsB>),
    Right(&'a dyn Dubu<Props = PropsA>, Column<'a, PropsB>),
    Top(Row<'a, PropsA>, &'a dyn Dubu<Props = PropsB>),
    Bottom(&'a dyn Dubu<Props = PropsA>, Row<'a, PropsB>),
}

impl<'a, PropsA, PropsB> Dubu for Slice<'a, PropsA, PropsB> {
    type Props = (PropsA, PropsB);
    fn render(&self, wh: Wh<f32>, (props_a, props_b): Self::Props) -> RenderingTree {
        let a_wh = match self {
            Slice::Left(a, _) => Wh {
                width: a.width.get_value(wh),
                height: wh.height,
            },
            Slice::Right(_, b) => Wh {
                width: wh.width - b.width.get_value(wh),
                height: wh.height,
            },
            Slice::Top(a, _) => Wh {
                width: wh.width,
                height: a.height.get_value(wh),
            },
            Slice::Bottom(_, b) => Wh {
                width: wh.width,
                height: wh.height - b.height.get_value(wh),
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

        let render_ab =
            |a: &dyn Dubu<Props = PropsA>, b: &dyn Dubu<Props = PropsB>| -> RenderingTree {
                namui::render![
                    a.render(a_wh, props_a),
                    namui::translate(
                        b_xywh.x,
                        b_xywh.height,
                        b.render(
                            Wh {
                                width: b_xywh.width,
                                height: b_xywh.height,
                            },
                            props_b
                        )
                    )
                ]
            };
        match self {
            Slice::Left(a, b) => render_ab(a, *b),
            Slice::Right(a, b) => render_ab(*a, b),
            Slice::Top(a, b) => render_ab(a, *b),
            Slice::Bottom(a, b) => render_ab(*a, b),
        }
    }
}

pub struct Fill<'a, Props>(pub &'a dyn Fn(Wh<f32>, Props) -> RenderingTree);
impl<Props> Dubu for Fill<'_, Props> {
    type Props = Props;
    fn render(&self, wh: Wh<f32>, props: Self::Props) -> RenderingTree {
        (self.0)(wh, props)
    }
}

#[derive(Clone)]
pub struct Row<'a, Props> {
    pub height: Size<'a>,
    pub dubu: &'a dyn Dubu<Props = Props>,
}

impl<'a, Props> Dubu for Row<'a, Props> {
    type Props = Props;
    fn render(&self, wh: Wh<f32>, props: Self::Props) -> RenderingTree {
        self.dubu.render(wh, props)
    }
}

#[derive(Clone)]
pub struct Column<'a, Props> {
    pub width: Size<'a>,
    pub dubu: &'a dyn Dubu<Props = Props>,
}

impl<'a, Props> Dubu for Column<'a, Props> {
    type Props = Props;
    fn render(&self, wh: Wh<f32>, props: Self::Props) -> RenderingTree {
        self.dubu.render(wh, props)
    }
}

#[derive(Clone)]
pub struct Container<'a, Props> {
    pub wh: Wh<f32>,
    pub dubu: &'a dyn Dubu<Props = Props>,
}
impl<Props> Container<'_, Props> {
    fn render(&self, props: Props) -> RenderingTree {
        self.dubu.render(self.wh, props)
    }
}

mod test {
    use super::*;

    #[allow(dead_code)]
    fn example() -> RenderingTree {
        let button = Column {
            width: Size::FromParentWh(&|parent_wh| parent_wh.height),
            dubu: &Fill(&|wh, _: ()| {
                namui::rect(namui::RectParam {
                    x: 0.0,
                    y: 0.0,
                    width: wh.width,
                    height: wh.height,
                    style: namui::RectStyle {
                        ..Default::default()
                    },
                    ..Default::default()
                })
            }),
        };
        let label = Fill(&|wh, _: ()| {
            namui::text(TextParam {
                x: wh.width / 2.0,
                y: wh.height / 2.0,
                text: "label".to_string(),
                style: TextStyle {
                    ..Default::default()
                },
                align: TextAlign::Center,
                baseline: TextBaseline::Middle,
                font_type: FontType {
                    font_weight: FontWeight::REGULAR,
                    language: Language::Ko,
                    serif: false,
                    size: (wh.height * 0.8) as i16,
                },
            })
        });

        let header = Row {
            height: Size::Fixed(20.0),
            dubu: &Slice::Left(button, &label),
        };

        let body = Fill(&|_wh, _: ()| todo!());

        let container = Container {
            wh: Wh {
                width: 500.0,
                height: 500.0,
            },
            dubu: &Slice::Top(header, &body),
        };

        container.render((((), ()), ()))
    }
}
