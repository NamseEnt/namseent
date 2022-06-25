use super::{SelectionEvent, SelectionTrait};
use crate::app::cropper::job::RectSelectionResizeDirection;
use namui::{
    nanoid, render, BorderPosition, Color, MouseCursor, PaintBuilder, PaintStyle, PathBuilder,
    RectParam, RectStroke, RectStyle, RenderingTree, Xy, XywhRect,
};

#[derive(Clone)]
pub struct RectSelection {
    pub xywh: XywhRect<f32>,
    id: String,
}
impl RectSelection {
    pub fn new(xywh: XywhRect<f32>) -> Self {
        let id = nanoid();
        Self { xywh, id }
    }
}
impl SelectionTrait for RectSelection {
    fn render(&self, scale: f32) -> namui::RenderingTree {
        let scaled_xywh = XywhRect {
            x: self.xywh.x * scale,
            y: self.xywh.y * scale,
            width: self.xywh.width * scale,
            height: self.xywh.height * scale,
        };

        render([
            namui::rect(RectParam {
                x: scaled_xywh.x,
                y: scaled_xywh.y,
                width: scaled_xywh.width,
                height: scaled_xywh.height,
                style: RectStyle {
                    stroke: Some(RectStroke {
                        color: Color::grayscale_f01(0.5),
                        width: 1.0,
                        border_position: BorderPosition::Inside,
                    }),
                    ..Default::default()
                },
            })
            .attach_event(|builder| {
                let id = self.id.clone();
                builder.on_mouse_down(move |event| {
                    if event.pressing_buttons.contains(&namui::MouseButton::Right) {
                        namui::event::send(SelectionEvent::SelectionRightClicked {
                            target_id: id.clone(),
                        })
                    }
                });
            }),
            render_handles(&scaled_xywh, self.id.clone()),
        ])
    }

    fn get_polygon(&self) -> Vec<namui::Xy<f32>> {
        vec![
            Xy {
                x: self.xywh.x,
                y: self.xywh.y,
            },
            Xy {
                x: self.xywh.x + self.xywh.width,
                y: self.xywh.y,
            },
            Xy {
                x: self.xywh.x + self.xywh.width,
                y: self.xywh.y + self.xywh.height,
            },
            Xy {
                x: self.xywh.x,
                y: self.xywh.y + self.xywh.height,
            },
        ]
    }

    fn get_id(&self) -> &String {
        &self.id
    }
}

#[derive(Clone)]
struct RectSelectionResizeHandle {
    pub handle_direction: RectSelectionResizeDirection,
    pub polygon_xy: Vec<Xy<f32>>,
    pub cursor: MouseCursor,
}
fn render_handles(xywh: &XywhRect<f32>, selection_id: String) -> RenderingTree {
    render(get_handles(xywh).iter().map(|handle| {
        let selection_id = selection_id.clone();
        let direction = handle.handle_direction.clone();
        let path = PathBuilder::new().add_poly(&handle.polygon_xy, true);

        let stroke_paint = PaintBuilder::new()
            .set_style(PaintStyle::Stroke)
            .set_stroke_width(1.0)
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
            let selection_id = selection_id.clone();
            let direction = direction.clone();
            builder.on_mouse_down(move |_| {
                let selection_id = selection_id.clone();
                let direction = direction.clone();
                namui::event::send(SelectionEvent::RectSelectionResizeHandleClicked {
                    selection_id,
                    direction,
                })
            });
        })
    }))
}

fn get_handles(xywh: &XywhRect<f32>) -> Vec<RectSelectionResizeHandle> {
    let center = xywh.center();
    const HANDLE_SIZE: f32 = 24.0;
    const HANDLE_THICKNESS: f32 = 6.0;
    let left = xywh.x;
    let top = xywh.y;
    let right = xywh.x + xywh.width;
    let bottom = xywh.y + xywh.height;
    vec![
        RectSelectionResizeHandle {
            handle_direction: RectSelectionResizeDirection::Top,
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
        RectSelectionResizeHandle {
            handle_direction: RectSelectionResizeDirection::Bottom,
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
        RectSelectionResizeHandle {
            handle_direction: RectSelectionResizeDirection::Left,
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
        RectSelectionResizeHandle {
            handle_direction: RectSelectionResizeDirection::Right,
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
        RectSelectionResizeHandle {
            handle_direction: RectSelectionResizeDirection::TopLeft,
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
        RectSelectionResizeHandle {
            handle_direction: RectSelectionResizeDirection::TopRight,
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
        RectSelectionResizeHandle {
            handle_direction: RectSelectionResizeDirection::BottomLeft,
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
        RectSelectionResizeHandle {
            handle_direction: RectSelectionResizeDirection::BottomRight,
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
