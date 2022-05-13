use super::*;
pub enum Direction {
    Horizontal,
    Vertical,
}

pub struct AA<'a, 'b>(
    pub Direction,
    pub &'a dyn AbsoluteSized,
    pub &'b dyn AbsoluteSized,
);
pub struct RA<'a, 'b>(
    pub Direction,
    pub &'a dyn RelativeSized,
    pub &'b dyn AbsoluteSized,
);
pub struct AR<'a, 'b>(
    pub Direction,
    pub &'a dyn AbsoluteSized,
    pub &'b dyn RelativeSized,
);
pub struct RR<'a, 'b>(
    pub Direction,
    pub f32,
    pub &'a dyn RelativeSized,
    pub f32,
    pub &'b dyn RelativeSized,
);

impl<'a, 'b> AbsoluteSized for AA<'a, 'b> {
    fn get_wh(&self) -> Wh<f32> {
        let AA(direction, a, b) = self;
        let a_wh = a.get_wh();
        let b_wh = b.get_wh();
        match direction {
            Direction::Horizontal => Wh {
                width: a_wh.width + b_wh.width,
                height: a_wh.height.max(b_wh.height),
            },
            Direction::Vertical => Wh {
                width: a_wh.width.max(b_wh.width),
                height: a_wh.height + b_wh.height,
            },
        }
    }
    fn render(&self) -> RenderingTree {
        let AA(direction, a, b) = self;
        let a_wh = a.get_wh();
        match direction {
            Direction::Horizontal => {
                namui::render![a.render(), namui::translate(a_wh.width, 0.0, b.render()),]
            }
            Direction::Vertical => {
                namui::render![a.render(), namui::translate(0.0, a_wh.height, b.render()),]
            }
        }
    }
}
impl<'a, 'b> RelativeSized for RA<'a, 'b> {
    fn render(&self, wh: Wh<f32>) -> RenderingTree {
        let RA(direction, a, b) = self;
        let b_wh = b.get_wh();

        let a_wh = match direction {
            Direction::Horizontal => Wh {
                width: wh.width - b_wh.width,
                height: wh.height,
            },
            Direction::Vertical => Wh {
                width: wh.width,
                height: wh.height - b_wh.height,
            },
        };

        let b_x = match direction {
            Direction::Horizontal => a_wh.width,
            Direction::Vertical => 0.0,
        };
        let b_y = match direction {
            Direction::Horizontal => 0.0,
            Direction::Vertical => a_wh.height,
        };

        namui::render![a.render(a_wh), namui::translate(b_x, b_y, b.render()),]
    }
}
impl<'a, 'b> RelativeSized for AR<'a, 'b> {
    fn render(&self, wh: Wh<f32>) -> RenderingTree {
        let AR(direction, a, b) = self;
        let a_wh = a.get_wh();

        let b_wh = match direction {
            Direction::Horizontal => Wh {
                width: wh.width - a_wh.width,
                height: wh.height,
            },
            Direction::Vertical => Wh {
                width: wh.width,
                height: wh.height - a_wh.height,
            },
        };

        let b_x = match direction {
            Direction::Horizontal => a_wh.width,
            Direction::Vertical => 0.0,
        };
        let b_y = match direction {
            Direction::Horizontal => 0.0,
            Direction::Vertical => a_wh.height,
        };

        namui::render![a.render(), namui::translate(b_x, b_y, b.render(b_wh)),]
    }
}
impl<'a, 'b> RelativeSized for RR<'a, 'b> {
    fn render(&self, wh: Wh<f32>) -> RenderingTree {
        let RR(direction, a_ratio, a, b_ratio, b) = self;
        let a_ratio = a_ratio / (a_ratio + b_ratio);
        let b_ratio = 1.0 - b_ratio;

        let a_wh = match direction {
            Direction::Horizontal => Wh {
                width: wh.width * a_ratio,
                height: wh.height,
            },
            Direction::Vertical => Wh {
                width: wh.width,
                height: wh.height * a_ratio,
            },
        };

        let b_wh = match direction {
            Direction::Horizontal => Wh {
                width: wh.width * b_ratio,
                height: wh.height,
            },
            Direction::Vertical => Wh {
                width: wh.width,
                height: wh.height * b_ratio,
            },
        };

        let b_x = match direction {
            Direction::Horizontal => a_wh.width,
            Direction::Vertical => 0.0,
        };
        let b_y = match direction {
            Direction::Horizontal => 0.0,
            Direction::Vertical => a_wh.height,
        };

        namui::render![a.render(a_wh), namui::translate(b_x, b_y, b.render(b_wh)),]
    }
}
