use super::{split::*, *};
pub enum ChildrenFit<'a, 'b> {
    AA(AA<'a, 'b>),
    AR(
        Direction,
        f32,
        &'a dyn AbsoluteSized,
        f32,
        &'b dyn RelativeSized,
    ),
    RA(
        Direction,
        f32,
        &'a dyn RelativeSized,
        f32,
        &'b dyn AbsoluteSized,
    ),
}

impl<'a, 'b> AbsoluteSized for ChildrenFit<'a, 'b> {
    fn get_wh(&self) -> Wh<f32> {
        match self {
            ChildrenFit::AA(aa) => aa.get_wh(),
            ChildrenFit::AR(direction, a_ratio, a, b_ratio, _b) => {
                let b_ratio = b_ratio / (a_ratio + b_ratio);
                let a_wh = a.get_wh();
                match direction {
                    Direction::Horizontal => Wh {
                        width: (a_wh.width) * (1.0 * b_ratio),
                        height: a_wh.height,
                    },
                    Direction::Vertical => Wh {
                        width: a_wh.width,
                        height: (a_wh.height) * (1.0 * b_ratio),
                    },
                }
            }
            ChildrenFit::RA(direction, a_ratio, _a, b_ratio, b) => {
                let a_ratio = a_ratio / (a_ratio + b_ratio);
                let b_wh = b.get_wh();
                match direction {
                    Direction::Horizontal => Wh {
                        width: (b_wh.width) * (1.0 * a_ratio),
                        height: b_wh.height,
                    },
                    Direction::Vertical => Wh {
                        width: b_wh.width,
                        height: (b_wh.height) * (1.0 * a_ratio),
                    },
                }
            }
        }
    }

    fn render(&self) -> RenderingTree {
        match self {
            ChildrenFit::AA(aa) => aa.render(),
            ChildrenFit::AR(direction, _a_ratio, a, _b_ratio, b) => {
                let wh = self.get_wh();
                let a_wh = a.get_wh();
                match direction {
                    Direction::Horizontal => namui::render![
                        a.render(),
                        namui::translate(
                            a_wh.width,
                            0.0,
                            b.render(Wh {
                                width: wh.width - a_wh.width,
                                height: wh.height,
                            })
                        ),
                    ],
                    Direction::Vertical => namui::render![
                        a.render(),
                        namui::translate(
                            0.0,
                            a_wh.height,
                            b.render(Wh {
                                width: wh.width,
                                height: wh.height - a_wh.height,
                            })
                        ),
                    ],
                }
            }
            ChildrenFit::RA(direction, _a_ratio, a, _b_ratio, b) => {
                let wh = self.get_wh();
                let b_wh = b.get_wh();
                match direction {
                    Direction::Horizontal => {
                        let a_width = wh.width - b_wh.width;
                        namui::render![
                            a.render(Wh {
                                width: a_width,
                                height: wh.height,
                            }),
                            namui::translate(a_width, 0.0, b.render()),
                        ]
                    }
                    Direction::Vertical => {
                        let a_height = wh.height - b_wh.height;
                        namui::render![
                            a.render(Wh {
                                width: wh.width,
                                height: a_height,
                            }),
                            namui::translate(0.0, a_height, b.render()),
                        ]
                    }
                }
            }
        }
    }
}

pub enum AbsoluteSizedContainer<'a, 'b> {
    AR(Wh<f32>, AR<'a, 'b>),
    RA(Wh<f32>, RA<'a, 'b>),
    RR(Wh<f32>, RR<'a, 'b>),
}

impl<'a, 'b> AbsoluteSized for AbsoluteSizedContainer<'a, 'b> {
    fn get_wh(&self) -> Wh<f32> {
        match self {
            AbsoluteSizedContainer::AR(wh, _) => *wh,
            AbsoluteSizedContainer::RA(wh, _) => *wh,
            AbsoluteSizedContainer::RR(wh, _) => *wh,
        }
    }

    fn render(&self) -> RenderingTree {
        match self {
            AbsoluteSizedContainer::AR(wh, AR(direction, a, b)) => {
                let a_wh = a.get_wh();
                namui::render![
                    a.render(),
                    namui::translate(
                        match direction {
                            Direction::Horizontal => a_wh.width,
                            Direction::Vertical => 0.0,
                        },
                        match direction {
                            Direction::Horizontal => 0.0,
                            Direction::Vertical => a_wh.height,
                        },
                        b.render(match direction {
                            Direction::Horizontal => Wh {
                                width: wh.width - a_wh.width,
                                height: wh.height,
                            },
                            Direction::Vertical => Wh {
                                width: wh.width,
                                height: wh.height - a_wh.height,
                            },
                        })
                    ),
                ]
            }
            AbsoluteSizedContainer::RA(wh, RA(direction, a, b)) => {
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
                namui::render![
                    a.render(a_wh),
                    namui::translate(
                        match direction {
                            Direction::Horizontal => a_wh.width,
                            Direction::Vertical => 0.0,
                        },
                        match direction {
                            Direction::Horizontal => 0.0,
                            Direction::Vertical => a_wh.height,
                        },
                        b.render()
                    ),
                ]
            }
            AbsoluteSizedContainer::RR(wh, RR(direction, a_ratio, a, b_ratio, b)) => {
                let a_ratio = a_ratio / (a_ratio + b_ratio);

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
                        width: wh.width - a_wh.width,
                        height: wh.height,
                    },
                    Direction::Vertical => Wh {
                        width: wh.width,
                        height: wh.height - a_wh.height,
                    },
                };

                namui::render![
                    a.render(a_wh),
                    namui::translate(
                        match direction {
                            Direction::Horizontal => a_wh.width,
                            Direction::Vertical => 0.0,
                        },
                        match direction {
                            Direction::Horizontal => 0.0,
                            Direction::Vertical => a_wh.height,
                        },
                        b.render(b_wh)
                    ),
                ]
            }
        }
    }
}
