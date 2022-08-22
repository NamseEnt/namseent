use crate::app::editor::EditorEvent::CharacterWysiwygEditorCropperHandleMouseDownEvent;
use namui::prelude::*;

pub struct Cropper {}

impl Cropper {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct CropperProps {
    pub dest_rect: Rect<Px>,
    pub container_size: Wh<Px>,
}

impl Cropper {
    pub fn update(&mut self, _event: &dyn std::any::Any) {}

    pub fn render(&self, props: &CropperProps) -> RenderingTree {
        render([
            rect(RectParam {
                rect: props.dest_rect,
                style: RectStyle {
                    stroke: Some(RectStroke {
                        color: Color::grayscale_f01(0.5),
                        width: px(1.0),
                        border_position: BorderPosition::Inside,
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }),
            render_handles(props.dest_rect, props.container_size),
        ])
    }
}

fn render_handles(dest_rect: Rect<Px>, container_size: Wh<Px>) -> RenderingTree {
    RenderingTree::Children(
        get_handles(dest_rect)
            .iter()
            .map(|handle| {
                let path = PathBuilder::new().add_poly(&handle.polygon_xy, true);

                let stroke_paint = PaintBuilder::new()
                    .set_style(PaintStyle::Stroke)
                    .set_stroke_width(px(1.0))
                    .set_color(Color::WHITE);

                let fill_paint = PaintBuilder::new()
                    .set_style(PaintStyle::Fill)
                    .set_color(Color::BLACK);

                render([
                    namui::path(path.clone(), fill_paint),
                    namui::path(path, stroke_paint),
                ])
                .with_mouse_cursor(handle.cursor.clone())
                .attach_event(move |builder| {
                    let handle = handle.clone();
                    let container_size = container_size.clone();
                    builder.on_mouse_down_in(move |mouse_event| {
                        namui::event::send(CharacterWysiwygEditorCropperHandleMouseDownEvent {
                            handle: handle.clone(),
                            mouse_xy: mouse_event.global_xy,
                            container_size,
                        })
                    });
                })
            })
            .collect(),
    )
}

#[derive(Debug, Clone, Copy)]
pub enum CropperHandleDirection {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug, Clone)]
pub struct CropperHandle {
    pub handle_direction: CropperHandleDirection,
    pub polygon_xy: Vec<Xy<Px>>,
    pub cursor: MouseCursor,
}

fn get_handles(dest_rect: Rect<Px>) -> Vec<CropperHandle> {
    let center = dest_rect.center();
    const HANDLE_SIZE: Px = px(24.0);
    const HANDLE_THICKNESS: Px = px(6.0);
    let left = dest_rect.left();
    let top = dest_rect.top();
    let right = dest_rect.right();
    let bottom = dest_rect.bottom();
    vec![
        CropperHandle {
            handle_direction: CropperHandleDirection::Top,
            polygon_xy: vec![
                Xy {
                    x: center.x - HANDLE_SIZE / 2.0,
                    y: top,
                },
                Xy {
                    x: center.x + HANDLE_SIZE / 2.0,
                    y: top,
                },
                Xy {
                    x: center.x + HANDLE_SIZE / 2.0,
                    y: top + HANDLE_THICKNESS,
                },
                Xy {
                    x: center.x - HANDLE_SIZE / 2.0,
                    y: top + HANDLE_THICKNESS,
                },
            ],
            cursor: MouseCursor::TopBottomResize,
        },
        CropperHandle {
            handle_direction: CropperHandleDirection::Bottom,
            polygon_xy: vec![
                Xy {
                    x: center.x - HANDLE_SIZE / 2.0,
                    y: bottom - HANDLE_THICKNESS,
                },
                Xy {
                    x: center.x + HANDLE_SIZE / 2.0,
                    y: bottom - HANDLE_THICKNESS,
                },
                Xy {
                    x: center.x + HANDLE_SIZE / 2.0,
                    y: bottom,
                },
                Xy {
                    x: center.x - HANDLE_SIZE / 2.0,
                    y: bottom,
                },
            ],
            cursor: MouseCursor::TopBottomResize,
        },
        CropperHandle {
            handle_direction: CropperHandleDirection::Left,
            polygon_xy: vec![
                Xy {
                    x: left,
                    y: center.y - HANDLE_SIZE / 2.0,
                },
                Xy {
                    x: left + HANDLE_THICKNESS,
                    y: center.y - HANDLE_SIZE / 2.0,
                },
                Xy {
                    x: left + HANDLE_THICKNESS,
                    y: center.y + HANDLE_SIZE / 2.0,
                },
                Xy {
                    x: left,
                    y: center.y + HANDLE_SIZE / 2.0,
                },
            ],
            cursor: MouseCursor::LeftRightResize,
        },
        CropperHandle {
            handle_direction: CropperHandleDirection::Right,
            polygon_xy: vec![
                Xy {
                    x: right - HANDLE_THICKNESS,
                    y: center.y - HANDLE_SIZE / 2.0,
                },
                Xy {
                    x: right,
                    y: center.y - HANDLE_SIZE / 2.0,
                },
                Xy {
                    x: right,
                    y: center.y + HANDLE_SIZE / 2.0,
                },
                Xy {
                    x: right - HANDLE_THICKNESS,
                    y: center.y + HANDLE_SIZE / 2.0,
                },
            ],
            cursor: MouseCursor::LeftRightResize,
        },
        CropperHandle {
            handle_direction: CropperHandleDirection::TopLeft,
            polygon_xy: vec![
                Xy { x: left, y: top },
                Xy {
                    x: left + HANDLE_SIZE,
                    y: top,
                },
                Xy {
                    x: left + HANDLE_SIZE,
                    y: top + HANDLE_THICKNESS,
                },
                Xy {
                    x: left + HANDLE_THICKNESS,
                    y: top + HANDLE_THICKNESS,
                },
                Xy {
                    x: left + HANDLE_THICKNESS,
                    y: top + HANDLE_SIZE,
                },
                Xy {
                    x: left,
                    y: top + HANDLE_SIZE,
                },
            ],
            cursor: MouseCursor::LeftTopRightBottomResize,
        },
        CropperHandle {
            handle_direction: CropperHandleDirection::TopRight,
            polygon_xy: vec![
                Xy { x: right, y: top },
                Xy {
                    x: right,
                    y: top + HANDLE_SIZE,
                },
                Xy {
                    x: right - HANDLE_THICKNESS,
                    y: top + HANDLE_SIZE,
                },
                Xy {
                    x: right - HANDLE_THICKNESS,
                    y: top + HANDLE_THICKNESS,
                },
                Xy {
                    x: right - HANDLE_SIZE,
                    y: top + HANDLE_THICKNESS,
                },
                Xy {
                    x: right - HANDLE_SIZE,
                    y: top,
                },
            ],
            cursor: MouseCursor::RightTopLeftBottomResize,
        },
        CropperHandle {
            handle_direction: CropperHandleDirection::BottomLeft,
            polygon_xy: vec![
                Xy { x: left, y: bottom },
                Xy {
                    x: left + HANDLE_SIZE,
                    y: bottom,
                },
                Xy {
                    x: left + HANDLE_SIZE,
                    y: bottom - HANDLE_THICKNESS,
                },
                Xy {
                    x: left + HANDLE_THICKNESS,
                    y: bottom - HANDLE_THICKNESS,
                },
                Xy {
                    x: left + HANDLE_THICKNESS,
                    y: bottom - HANDLE_SIZE,
                },
                Xy {
                    x: left,
                    y: bottom - HANDLE_SIZE,
                },
            ],
            cursor: MouseCursor::RightTopLeftBottomResize,
        },
        CropperHandle {
            handle_direction: CropperHandleDirection::BottomRight,
            polygon_xy: vec![
                Xy {
                    x: right,
                    y: bottom,
                },
                Xy {
                    x: right,
                    y: bottom - HANDLE_SIZE,
                },
                Xy {
                    x: right - HANDLE_THICKNESS,
                    y: bottom - HANDLE_SIZE,
                },
                Xy {
                    x: right - HANDLE_THICKNESS,
                    y: bottom - HANDLE_THICKNESS,
                },
                Xy {
                    x: right - HANDLE_SIZE,
                    y: bottom - HANDLE_THICKNESS,
                },
                Xy {
                    x: right - HANDLE_SIZE,
                    y: bottom,
                },
            ],
            cursor: MouseCursor::LeftTopRightBottomResize,
        },
    ]
}
