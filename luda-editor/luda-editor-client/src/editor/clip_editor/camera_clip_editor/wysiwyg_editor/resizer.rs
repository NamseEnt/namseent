use crate::editor::types::*;
use crate::editor::EditorEvent::WysiwygEditorResizerHandleMouseDownEvent;
use namui::prelude::*;

pub struct Resizer {}

impl Resizer {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct ResizerProps<'a> {
    pub camera_angle: &'a CameraAngle,
    pub source_rect: &'a XywhRect<f32>,
    pub container_size: &'a Wh<f32>,
}

impl Resizer {
    pub fn update(&mut self, event: &dyn std::any::Any) {}

    pub fn render(&self, props: &ResizerProps) -> RenderingTree {
        render![
            rect(RectParam {
                x: props.source_rect.x,
                y: props.source_rect.y,
                width: props.source_rect.width,
                height: props.source_rect.height,
                style: RectStyle {
                    stroke: Some(RectStroke {
                        color: Color::gary_scale_f01(0.2),
                        width: 1.0,
                        border_position: BorderPosition::Inside,
                    }),
                    ..Default::default()
                },
            }),
            render_resize_handles(props.source_rect, props.container_size),
        ]
    }
}

fn render_resize_handles(source_rect: &XywhRect<f32>, container_size: &Wh<f32>) -> RenderingTree {
    const HANDLE_RADIUS: f32 = 5.0;

    RenderingTree::Children(
        get_handles(&source_rect)
            .iter()
            .map(|handle| {
                let path = namui::Path::new().add_oval(&LtrbRect {
                    left: handle.xy.x - HANDLE_RADIUS,
                    top: handle.xy.y - HANDLE_RADIUS,
                    right: handle.xy.x + HANDLE_RADIUS,
                    bottom: handle.xy.y + HANDLE_RADIUS,
                });

                let fill_paint = namui::Paint::new()
                    .set_style(namui::PaintStyle::Fill)
                    .set_color(Color::WHITE);

                let stroke_paint = namui::Paint::new()
                    .set_style(namui::PaintStyle::Stroke)
                    .set_color(Color::gary_scale_f01(0.5))
                    .set_stroke_width(2.0)
                    .set_anti_alias(true);

                render![
                    namui::path(path.clone(), fill_paint),
                    namui::path(path, stroke_paint),
                ]
                .with_mouse_cursor(handle.cursor())
                .attach_event(move |builder| {
                    let handle = handle.clone();
                    let container_size = container_size.clone();
                    let source_rect = source_rect.clone();
                    builder.on_mouse_down(Box::new(move |mouse_event| {
                        namui::event::send(Box::new(WysiwygEditorResizerHandleMouseDownEvent {
                            handle,
                            center_xy: source_rect.center(),
                            mouse_xy: mouse_event.global_xy,
                            container_size,
                            image_size_ratio: Wh {
                                width: source_rect.width,
                                height: source_rect.height,
                            },
                        }))
                    }))
                })
            })
            .collect::<Vec<RenderingTree>>(),
    )
}

fn get_opposite_handle(handle: &ResizerHandle, source_rect: &XywhRect<f32>) -> ResizerHandle {
    let center_xy = source_rect.center();
    ResizerHandle {
        handle_direction: handle.handle_direction.opposite(),
        xy: 2.0 * center_xy - handle.xy,
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ResizerHandleDirection {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
    Top,
    Right,
    Bottom,
    Left,
}
#[derive(Debug, Clone, Copy)]
pub struct ResizerHandle {
    pub handle_direction: ResizerHandleDirection,
    pub xy: Xy<f32>,
}
impl ResizerHandle {
    pub fn cursor(&self) -> namui::MouseCursor {
        self.handle_direction.cursor()
    }
}
fn get_handles(source_rect: &XywhRect<f32>) -> Vec<ResizerHandle> {
    let top_left = ResizerHandle {
        handle_direction: ResizerHandleDirection::TopLeft,
        xy: Xy {
            x: source_rect.x,
            y: source_rect.y,
        },
    };

    let top = ResizerHandle {
        handle_direction: ResizerHandleDirection::Top,
        xy: Xy {
            x: source_rect.x + source_rect.width / 2.0,
            y: source_rect.y,
        },
    };
    let top_right = ResizerHandle {
        handle_direction: ResizerHandleDirection::TopRight,
        xy: Xy {
            x: source_rect.x + source_rect.width,
            y: source_rect.y,
        },
    };
    let left = ResizerHandle {
        handle_direction: ResizerHandleDirection::Left,
        xy: Xy {
            x: source_rect.x,
            y: source_rect.y + source_rect.height / 2.0,
        },
    };
    vec![
        top_left,
        top,
        top_right,
        left,
        get_opposite_handle(&left, &source_rect),
        get_opposite_handle(&top_right, &source_rect),
        get_opposite_handle(&top, &source_rect),
        get_opposite_handle(&top_left, &source_rect),
    ]
}
impl ResizerHandleDirection {
    pub(crate) fn opposite(&self) -> Self {
        match self {
            ResizerHandleDirection::TopLeft => ResizerHandleDirection::BottomRight,
            ResizerHandleDirection::TopRight => ResizerHandleDirection::BottomLeft,
            ResizerHandleDirection::BottomLeft => ResizerHandleDirection::TopRight,
            ResizerHandleDirection::BottomRight => ResizerHandleDirection::TopLeft,
            ResizerHandleDirection::Top => ResizerHandleDirection::Bottom,
            ResizerHandleDirection::Bottom => ResizerHandleDirection::Top,
            ResizerHandleDirection::Left => ResizerHandleDirection::Right,
            ResizerHandleDirection::Right => ResizerHandleDirection::Left,
        }
    }
    pub(crate) fn cursor(&self) -> MouseCursor {
        match self {
            ResizerHandleDirection::TopLeft => MouseCursor::LeftTopRightBottomResize,
            ResizerHandleDirection::Top => MouseCursor::TopBottomResize,
            ResizerHandleDirection::TopRight => MouseCursor::RightTopLeftBottomResize,
            ResizerHandleDirection::Left => MouseCursor::LeftRightResize,
            ResizerHandleDirection::Right => MouseCursor::LeftRightResize,
            ResizerHandleDirection::BottomLeft => MouseCursor::RightTopLeftBottomResize,
            ResizerHandleDirection::Bottom => MouseCursor::TopBottomResize,
            ResizerHandleDirection::BottomRight => MouseCursor::LeftTopRightBottomResize,
        }
    }
}
