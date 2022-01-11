use crate::app::editor::EditorEvent::WysiwygEditorCropperHandleMouseDownEvent;
use namui::prelude::*;

pub struct Cropper {}

impl Cropper {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct CropperProps<'a> {
    pub dest_rect: &'a LtrbRect,
    pub container_size: &'a Wh<f32>,
}

impl Cropper {
    pub fn update(&mut self, event: &dyn std::any::Any) {}

    pub fn render(&self, props: &CropperProps) -> RenderingTree {
        render![
            rect(RectParam {
                x: props.dest_rect.left,
                y: props.dest_rect.top,
                width: props.dest_rect.right - props.dest_rect.left,
                height: props.dest_rect.bottom - props.dest_rect.top,
                style: RectStyle {
                    stroke: Some(RectStroke {
                        color: Color::garyscale_f01(0.5),
                        width: 1.0,
                        border_position: BorderPosition::Inside,
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }),
            render_handles(props.dest_rect, props.container_size),
        ]
    }
}

fn render_handles(dest_rect: &LtrbRect, container_size: &Wh<f32>) -> RenderingTree {
    RenderingTree::Children(
        get_handles(&dest_rect)
            .iter()
            .map(|handle| {
                let path = PathBuilder::new().add_poly(&handle.polygon_xy, true);

                let stroke_paint = PaintBuilder::new()
                    .set_style(PaintStyle::Stroke)
                    .set_stroke_width(1.0)
                    .set_color(Color::WHITE);

                let fill_paint = PaintBuilder::new()
                    .set_style(PaintStyle::Fill)
                    .set_color(Color::BLACK);

                render![
                    namui::path(path.clone(), fill_paint),
                    namui::path(path, stroke_paint),
                ]
                .with_mouse_cursor(handle.cursor)
                .attach_event(move |builder| {
                    let handle = handle.clone();
                    let container_size = container_size.clone();
                    builder.on_mouse_down(Box::new(move |mouse_event| {
                        namui::event::send(Box::new(WysiwygEditorCropperHandleMouseDownEvent {
                            handle: handle.clone(),
                            mouse_xy: mouse_event.global_xy,
                            container_size: container_size,
                        }))
                    }))
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
    pub polygon_xy: Vec<Xy<f32>>,
    pub cursor: MouseCursor,
}

fn get_handles(dest_rect: &LtrbRect) -> Vec<CropperHandle> {
    let center = dest_rect.center();
    const HANDLE_SIZE: f32 = 24.0;
    const HANDLE_THICKNESS: f32 = 6.0;
    let left = dest_rect.left;
    let top = dest_rect.top;
    let right = dest_rect.right;
    let bottom = dest_rect.bottom;
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
