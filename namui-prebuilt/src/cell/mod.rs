use namui::{RenderingTree, Wh};
pub mod container;
pub mod split;

pub trait AbsoluteSized {
    fn get_wh(&self) -> Wh<f32>;
    fn render(&self) -> RenderingTree;
}

pub trait RelativeSized {
    fn render(&self, wh: Wh<f32>) -> RenderingTree;
}

pub struct AbsoluteSizedCell(RenderingTree);
impl AbsoluteSized for AbsoluteSizedCell {
    fn get_wh(&self) -> Wh<f32> {
        self.0.get_bounding_box().map_or(
            Wh {
                width: 0.0,
                height: 0.0,
            },
            |xywh| xywh.wh(),
        )
    }
    fn render(&self) -> RenderingTree {
        self.0.clone()
    }
}
impl From<RenderingTree> for AbsoluteSizedCell {
    fn from(tree: RenderingTree) -> Self {
        AbsoluteSizedCell(tree)
    }
}

pub struct RelativeSizedCell<F>(F)
where
    F: Fn(Wh<f32>) -> RenderingTree;

impl<F> RelativeSized for RelativeSizedCell<F>
where
    F: Fn(Wh<f32>) -> RenderingTree,
{
    fn render(&self, wh: Wh<f32>) -> RenderingTree {
        (self.0)(wh)
    }
}

fn test1() -> RenderingTree {
    let a: AbsoluteSizedCell = namui::rect(namui::RectParam {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 20.0,
        style: namui::RectStyle {
            ..Default::default()
        },
        ..Default::default()
    })
    .into();

    let b = RelativeSizedCell(|wh| {
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
    });

    let c = RelativeSizedCell(|wh| {
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
    });

    let d = split::AR(split::Direction::Horizontal, &a, &b);

    let e = container::AbsoluteSizedContainer::RR(
        Wh {
            width: 1920.0,
            height: 1080.0,
        },
        split::RR(split::Direction::Horizontal, 2.0, &d, 1.0, &c),
    );

    e.render()
}

fn test2() -> RenderingTree {
    let a: AbsoluteSizedCell = namui::rect(namui::RectParam {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 20.0,
        style: namui::RectStyle {
            ..Default::default()
        },
        ..Default::default()
    })
    .into();

    let b: AbsoluteSizedCell = namui::rect(namui::RectParam {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 20.0,
        style: namui::RectStyle {
            ..Default::default()
        },
        ..Default::default()
    })
    .into();

    let c = RelativeSizedCell(|wh| {
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
    });

    let d = split::AA(split::Direction::Horizontal, &a, &b);

    let e = container::ChildrenFit::AR(split::Direction::Horizontal, 2.0, &d, 1.0, &c);

    e.render()
}
