use super::{SelectionEvent, SelectionTrait};
use crate::app::cropper::job::RectSelectionResizeDirection;
use namui::prelude::*;

#[derive(Clone)]
pub struct RectSelection {
    pub rect: Rect<Px>,
    id: String,
}
impl RectSelection {
    pub fn new(rect: Rect<Px>) -> Self {
        let id = nanoid();
        Self { rect, id }
    }
}
impl SelectionTrait for RectSelection {
    fn render(&self, scale: f32) -> namui::RenderingTree {
        let scaled_rect = self.rect.scale(scale);

        render([
            namui::rect(RectParam {
                rect: scaled_rect,
                style: RectStyle {
                    stroke: Some(RectStroke {
                        color: Color::grayscale_f01(0.5),
                        width: px(1.0),
                        border_position: BorderPosition::Inside,
                    }),
                    ..Default::default()
                },
            })
            .attach_event(|builder| {
                let id = self.id.clone();
                builder.on_mouse_down_in(move |event| {
                    if event.pressing_buttons.contains(&namui::MouseButton::Right) {
                        namui::event::send(SelectionEvent::SelectionRightClicked {
                            target_id: id.clone(),
                        })
                    }
                });
            }),
            render_handles(scaled_rect, self.id.clone()),
        ])
    }

    fn get_polygon(&self) -> Vec<namui::Xy<Px>> {
        vec![
            Xy {
                x: self.rect.x(),
                y: self.rect.y(),
            },
            Xy {
                x: self.rect.x() + self.rect.width(),
                y: self.rect.y(),
            },
            Xy {
                x: self.rect.x() + self.rect.width(),
                y: self.rect.y() + self.rect.height(),
            },
            Xy {
                x: self.rect.x(),
                y: self.rect.y() + self.rect.height(),
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
    pub polygon_xy: Vec<Xy<Px>>,
    pub cursor: MouseCursor,
}
fn render_handles(rect: Rect<Px>, selection_id: String) -> RenderingTree {
    render(get_handles(rect).iter().map(|handle| {
        let selection_id = selection_id.clone();
        let direction = handle.handle_direction.clone();
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
            let selection_id = selection_id.clone();
            let direction = direction.clone();
            builder.on_mouse_down_in(move |_| {
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

fn get_handles(rect: Rect<Px>) -> Vec<RectSelectionResizeHandle> {
    let center = rect.center();
    const HANDLE_SIZE: Px = px(24.0);
    const HANDLE_THICKNESS: Px = px(6.0);
    let left = rect.left();
    let top = rect.top();
    let right = rect.right();
    let bottom = rect.bottom();
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
