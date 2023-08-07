use namui::prelude::*;
use rpc::data::Circumscribed;

#[namui::component]
pub struct Resizer<'a> {
    pub rect: Rect<Px>,
    pub dragging_context: Option<ResizerDraggingContext>,
    pub container_size: Wh<Px>,
    pub image_size: Wh<Px>,
    pub graphic_index: Uuid,
    pub on_event: Box<dyn 'a + Fn(Event)>,
}

pub enum Event {
    OnResize {
        circumscribed: Circumscribed<Percent>,
        graphic_index: Uuid,
    },
    OnUpdateDraggingContext {
        context: Option<ResizerDraggingContext>,
    },
}

impl Component for Resizer<'_> {
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
        let Self {
            rect,
            dragging_context,
            container_size,
            image_size,
            graphic_index,
            ref on_event,
        } = self;
        let on_event = on_event.clone();

        const HANDLE_RADIUS: Px = px(5.0);

        ctx.compose(|ctx| {
            HANDLES
                .into_iter()
                .map(|handle| {
                    let handle_xy = handle.xy(rect);
                    let path = namui::PathBuilder::new().add_oval(Rect::Ltrb {
                        left: handle_xy.x - HANDLE_RADIUS,
                        top: handle_xy.y - HANDLE_RADIUS,
                        right: handle_xy.x + HANDLE_RADIUS,
                        bottom: handle_xy.y + HANDLE_RADIUS,
                    });

                    let fill_paint = namui::PaintBuilder::new()
                        .set_style(namui::PaintStyle::Fill)
                        .set_color(Color::WHITE);

                    let stroke_paint = namui::PaintBuilder::new()
                        .set_style(namui::PaintStyle::Stroke)
                        .set_color(Color::grayscale_f01(0.5))
                        .set_stroke_width(px(2.0))
                        .set_anti_alias(true);

                    render([
                        namui::path(path.clone(), fill_paint),
                        namui::path(path, stroke_paint),
                    ])
                    .with_mouse_cursor(handle.cursor())
                    .attach_event(move |event| match dragging_context {
                        Some(context) => {
                            if context.handle != handle {
                                return;
                            }

                            match event {
                                namui::Event::MouseMove { event } => {
                                    on_event(Event::OnUpdateDraggingContext {
                                        context: Some(ResizerDraggingContext {
                                            handle,
                                            start_global_xy: context.start_global_xy,
                                            end_global_xy: event.global_xy,
                                        }),
                                    });
                                }
                                namui::Event::MouseUp { event } => {
                                    let delta_xy = event.global_xy - context.start_global_xy;
                                    on_event(Event::OnUpdateDraggingContext { context: None });
                                    on_event(Event::OnResize {
                                        circumscribed: resize_by_center(
                                            handle,
                                            rect.center(),
                                            delta_xy,
                                            container_size,
                                            image_size,
                                        ),
                                        graphic_index,
                                    });
                                }
                                _ => {}
                            }
                        }
                        None => match event {
                            namui::Event::MouseDown { event } => {
                                if event.is_local_xy_in() {
                                    if event.button == Some(MouseButton::Left) {
                                        event.stop_propagation();
                                        on_event(Event::OnUpdateDraggingContext {
                                            context: Some(ResizerDraggingContext {
                                                handle,
                                                start_global_xy: event.global_xy,
                                                end_global_xy: event.global_xy,
                                            }),
                                        });
                                    }
                                }
                            }
                            _ => {}
                        },
                    })
                })
                .for_each(|handle| {
                    ctx.add(handle);
                });
        });

        ctx.done()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResizerDraggingContext {
    handle: ResizerHandle,
    start_global_xy: Xy<Px>,
    end_global_xy: Xy<Px>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResizerHandle {
    LeftTop,
    RightTop,
    RightBottom,
    LeftBottom,
    Top,
    Right,
    Bottom,
    Left,
}
impl ResizerHandle {
    pub fn xy(&self, rect: Rect<Px>) -> Xy<Px> {
        match self {
            ResizerHandle::LeftTop => Xy {
                x: rect.left(),
                y: rect.top(),
            },
            ResizerHandle::Top => Xy {
                x: rect.center().x,
                y: rect.top(),
            },
            ResizerHandle::RightTop => Xy {
                x: rect.right(),
                y: rect.top(),
            },
            ResizerHandle::Left => Xy {
                x: rect.left(),
                y: rect.center().y,
            },
            ResizerHandle::RightBottom => Xy {
                x: rect.right(),
                y: rect.bottom(),
            },
            ResizerHandle::LeftBottom => Xy {
                x: rect.left(),
                y: rect.bottom(),
            },
            ResizerHandle::Right => Xy {
                x: rect.right(),
                y: rect.center().y,
            },
            ResizerHandle::Bottom => Xy {
                x: rect.center().x,
                y: rect.bottom(),
            },
        }
    }
    // pub fn opposite(&self) -> Self {
    //     match self {
    //         ResizerHandle::LeftTop => ResizerHandle::RightBottom,
    //         ResizerHandle::RightTop => ResizerHandle::LeftBottom,
    //         ResizerHandle::LeftBottom => ResizerHandle::RightTop,
    //         ResizerHandle::RightBottom => ResizerHandle::LeftTop,
    //         ResizerHandle::Top => ResizerHandle::Bottom,
    //         ResizerHandle::Bottom => ResizerHandle::Top,
    //         ResizerHandle::Left => ResizerHandle::Right,
    //         ResizerHandle::Right => ResizerHandle::Left,
    //     }
    // }
    pub fn cursor(&self) -> MouseCursor {
        match self {
            ResizerHandle::LeftTop => MouseCursor::LeftTopRightBottomResize,
            ResizerHandle::Top => MouseCursor::TopBottomResize,
            ResizerHandle::RightTop => MouseCursor::RightTopLeftBottomResize,
            ResizerHandle::Left => MouseCursor::LeftRightResize,
            ResizerHandle::Right => MouseCursor::LeftRightResize,
            ResizerHandle::LeftBottom => MouseCursor::RightTopLeftBottomResize,
            ResizerHandle::Bottom => MouseCursor::TopBottomResize,
            ResizerHandle::RightBottom => MouseCursor::LeftTopRightBottomResize,
        }
    }
}
const HANDLES: [ResizerHandle; 8] = [
    ResizerHandle::LeftTop,
    ResizerHandle::Top,
    ResizerHandle::RightTop,
    ResizerHandle::Right,
    ResizerHandle::RightBottom,
    ResizerHandle::Bottom,
    ResizerHandle::LeftBottom,
    ResizerHandle::Left,
];

/// NOTE: I make resizing by center but not sure it is the best way to resize.
/// You can test resizing by anchor, not center.
/// NOTE2: All Xy coordinates are relative to image left top.
fn resize_by_center(
    handle: ResizerHandle,
    center_xy: Xy<Px>,
    diff_xy: Xy<Px>,
    container_size: Wh<Px>,
    image_size: Wh<Px>,
) -> Circumscribed<Percent> {
    let handle_xy = handle.xy(Rect::from_xy_wh(
        center_xy - image_size.as_xy() / 2.0,
        image_size,
    ));
    let designated_xy = handle_xy + diff_xy;

    let projected_xy_by_x = get_y_in_vector(center_xy, handle_xy, designated_xy.x).map(|y| Xy {
        x: designated_xy.x,
        y,
    });
    let projected_xy_by_y = get_x_in_vector(center_xy, handle_xy, designated_xy.y).map(|x| Xy {
        x,
        y: designated_xy.y,
    });

    let candidates = [projected_xy_by_x, projected_xy_by_y];

    let projected_length = candidates
        .iter()
        .filter_map(|candidate| candidate.map(|xy| (xy - center_xy).length()))
        .max_by(|a, b| a.partial_cmp(&b).unwrap())
        .unwrap();

    let radius = match handle {
        ResizerHandle::LeftTop
        | ResizerHandle::RightTop
        | ResizerHandle::RightBottom
        | ResizerHandle::LeftBottom => projected_length,
        ResizerHandle::Top | ResizerHandle::Bottom => {
            image_size.length() * (projected_length / image_size.height)
        }
        ResizerHandle::Right | ResizerHandle::Left => {
            image_size.length() * (projected_length / image_size.width)
        }
    };

    Circumscribed {
        center_xy: Xy {
            x: (center_xy.x / container_size.width).into(),
            y: (center_xy.y / container_size.height).into(),
        },
        radius: (radius / (container_size.length() / 2)).into(),
    }
}

fn get_y_in_vector(xy1: Xy<Px>, xy2: Xy<Px>, x: Px) -> Option<Px> {
    if xy1.x == xy2.x {
        None
    } else if xy1.y == xy2.y {
        Some(xy1.y)
    } else {
        let Xy { x: x1, y: y1 } = xy1;
        let Xy { x: x2, y: y2 } = xy2;
        let a = (y2 - y1) / (x2 - x1);
        let b = y1 - x1 * a;
        Some(x * a + b)
    }
}
fn get_x_in_vector(xy1: Xy<Px>, xy2: Xy<Px>, y: Px) -> Option<Px> {
    if xy1.x == xy2.x {
        Some(xy1.x)
    } else if xy1.y == xy2.y {
        None
    } else {
        let Xy { x: x1, y: y1 } = xy1;
        let Xy { x: x2, y: y2 } = xy2;
        let a = (y2 - y1) / (x2 - x1);
        let b = y1 - x1 * a;
        Some((y - b) / a)
    }
}
impl ResizerDraggingContext {
    pub fn resize(
        &self,
        center_xy: Xy<Px>,
        image_size: Wh<Px>,
        container_size: Wh<Px>,
    ) -> Circumscribed<Percent> {
        let delta_xy = self.end_global_xy - self.start_global_xy;
        resize_by_center(self.handle, center_xy, delta_xy, container_size, image_size)
    }
}
