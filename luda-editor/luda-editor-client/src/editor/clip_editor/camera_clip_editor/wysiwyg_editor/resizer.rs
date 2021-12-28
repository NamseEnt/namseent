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
            image_size_handles(props.source_rect, props.container_size),
        ]
    }
}

fn image_size_handles(source_rect: &XywhRect<f32>, container_size: &Wh<f32>) -> RenderingTree {
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

                clip(
                    path.clone(),
                    ClipOp::Intersect,
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
                    }),
                )
            })
            .collect::<Vec<RenderingTree>>(),
    )
}

fn get_opposite_handle(handle: &ResizerHandle, source_rect: &XywhRect<f32>) -> ResizerHandle {
    let center_xy = source_rect.center();
    ResizerHandle {
        handle_type: handle.handle_type.opposite(),
        xy: 2.0 * center_xy - handle.xy,
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ResizerHandleType {
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
    pub handle_type: ResizerHandleType,
    pub xy: Xy<f32>,
}
impl ResizerHandle {
    pub fn cursor(&self) -> namui::MouseCursor {
        self.handle_type.cursor()
    }
}
fn get_handles(source_rect: &XywhRect<f32>) -> Vec<ResizerHandle> {
    let top_left = ResizerHandle {
        handle_type: ResizerHandleType::TopLeft,
        xy: Xy {
            x: source_rect.x,
            y: source_rect.y,
        },
    };

    let top = ResizerHandle {
        handle_type: ResizerHandleType::Top,
        xy: Xy {
            x: source_rect.x + source_rect.width / 2.0,
            y: source_rect.y,
        },
    };
    let top_right = ResizerHandle {
        handle_type: ResizerHandleType::TopRight,
        xy: Xy {
            x: source_rect.x + source_rect.width,
            y: source_rect.y,
        },
    };
    let left = ResizerHandle {
        handle_type: ResizerHandleType::Left,
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
impl ResizerHandleType {
    pub(crate) fn opposite(&self) -> Self {
        match self {
            ResizerHandleType::TopLeft => ResizerHandleType::BottomRight,
            ResizerHandleType::TopRight => ResizerHandleType::BottomLeft,
            ResizerHandleType::BottomLeft => ResizerHandleType::TopRight,
            ResizerHandleType::BottomRight => ResizerHandleType::TopLeft,
            ResizerHandleType::Top => ResizerHandleType::Bottom,
            ResizerHandleType::Bottom => ResizerHandleType::Top,
            ResizerHandleType::Left => ResizerHandleType::Right,
            ResizerHandleType::Right => ResizerHandleType::Left,
        }
    }
    pub(crate) fn cursor(&self) -> MouseCursor {
        match self {
            ResizerHandleType::TopLeft => MouseCursor::LeftTopRightBottomResize,
            ResizerHandleType::Top => MouseCursor::TopBottomResize,
            ResizerHandleType::TopRight => MouseCursor::RightTopLeftBottomResize,
            ResizerHandleType::Left => MouseCursor::LeftRightResize,
            ResizerHandleType::Right => MouseCursor::LeftRightResize,
            ResizerHandleType::BottomLeft => MouseCursor::RightTopLeftBottomResize,
            ResizerHandleType::Bottom => MouseCursor::TopBottomResize,
            ResizerHandleType::BottomRight => MouseCursor::LeftTopRightBottomResize,
        }
    }
}
