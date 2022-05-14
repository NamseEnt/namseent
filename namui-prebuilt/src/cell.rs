use namui::prelude::*;

trait WhRender {
    fn render(&self, wh: Wh<f32>) -> RenderingTree;
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
pub enum Split<'a> {
    Left(ColumnCell<'a>, Cell<'a>),
    Right(Cell<'a>, ColumnCell<'a>),
    Top(RowCell<'a>, Cell<'a>),
    Bottom(Cell<'a>, RowCell<'a>),
}

impl<'a> Split<'a> {
    fn render(&self, wh: Wh<f32>) -> RenderingTree {
        let a_wh = match self {
            Split::Left(a, _) => Wh {
                width: a.width.get_value(wh),
                height: wh.height,
            },
            Split::Right(_, b) => Wh {
                width: wh.width - b.width.get_value(wh),
                height: wh.height,
            },
            Split::Top(a, _) => Wh {
                width: wh.width,
                height: a.height.get_value(wh),
            },
            Split::Bottom(_, b) => Wh {
                width: wh.width,
                height: wh.height - b.height.get_value(wh),
            },
        };

        let b_xywh = match self {
            Split::Left(_, _) | Split::Right(_, _) => XywhRect {
                x: a_wh.width,
                y: 0.0,
                width: wh.width - a_wh.width,
                height: wh.height,
            },
            Split::Top(_, _) | Split::Bottom(_, _) => XywhRect {
                x: 0.0,
                y: a_wh.height,
                width: wh.width,
                height: wh.height - a_wh.height,
            },
        };

        let render_ab = |a: &dyn WhRender, b: &dyn WhRender| -> RenderingTree {
            namui::render![
                a.render(a_wh),
                namui::translate(
                    b_xywh.x,
                    b_xywh.height,
                    b.render(Wh {
                        width: b_xywh.width,
                        height: b_xywh.height,
                    })
                )
            ]
        };
        match self {
            Split::Left(a, b) => render_ab(a, b),
            Split::Right(a, b) => render_ab(a, b),
            Split::Top(a, b) => render_ab(a, b),
            Split::Bottom(a, b) => render_ab(a, b),
        }
    }
}

#[derive(Clone)]
pub enum Cell<'a> {
    Fill(&'a dyn Fn(Wh<f32>) -> RenderingTree),
    Split(Box<Split<'a>>),
}

impl Cell<'_> {
    fn fill<'a>(render: &'a impl Fn(Wh<f32>) -> RenderingTree) -> Cell<'a> {
        Cell::Fill(render)
    }

    fn split(split: Split<'_>) -> Cell<'_> {
        Cell::Split(Box::new(split))
    }
}

impl WhRender for Cell<'_> {
    fn render(&self, wh: Wh<f32>) -> RenderingTree {
        match self {
            Cell::Fill(render) => render(wh),
            Cell::Split(split) => split.render(wh),
        }
    }
}

#[derive(Clone)]
pub struct RowCell<'a> {
    height: Size<'a>,
    content: Cell<'a>,
}

impl WhRender for RowCell<'_> {
    fn render(&self, wh: Wh<f32>) -> RenderingTree {
        self.content.render(wh)
    }
}

#[derive(Clone)]
pub struct ColumnCell<'a> {
    width: Size<'a>,
    content: Cell<'a>,
}

impl WhRender for ColumnCell<'_> {
    fn render(&self, wh: Wh<f32>) -> RenderingTree {
        self.content.render(wh)
    }
}

#[derive(Clone)]
pub struct Container<'a> {
    wh: Wh<f32>,
    cell: Cell<'a>,
}
impl Container<'_> {
    fn render(&self) -> RenderingTree {
        self.cell.render(self.wh)
    }
}

#[allow(dead_code)]
fn example() -> RenderingTree {
    let button = ColumnCell {
        width: Size::FromParentWh(&|parent_wh| parent_wh.height),
        content: Cell::fill(&|wh| {
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
    let label = Cell::fill(&|wh| {
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

    let header = RowCell {
        height: Size::Fixed(20.0),
        content: Cell::split(Split::Left(button, label)),
    };

    let body = Cell::fill(&|wh| todo!());

    let container = Container {
        wh: Wh {
            width: 500.0,
            height: 500.0,
        },
        cell: Cell::split(Split::Top(header, body)),
    };

    container.render()
}
