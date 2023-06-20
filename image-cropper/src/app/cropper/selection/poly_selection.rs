use super::SelectionTrait;
use crate::app::cropper::selection::SelectionEvent;
use namui::prelude::*;

#[derive(Clone)]
pub struct PolySelection {
    pub point_list: Vec<Xy<Px>>,
    id: namui::Uuid,
    creation_state: PolySelectionCreationState,
}
impl PolySelection {
    pub fn new(point_list: Vec<Xy<Px>>, creation_state: PolySelectionCreationState) -> Self {
        let id = uuid();
        Self {
            point_list,
            id,
            creation_state,
        }
    }

    fn render_selection_create_button(&self, scale: f32) -> namui::RenderingTree {
        match self.creation_state {
            PolySelectionCreationState::Creating => match self.point_list.first() {
                Some(first_point) => {
                    const BUTTON_SIZE: Px = px(10.0);
                    rect(RectParam {
                        rect: Rect::Xywh {
                            x: first_point.x * scale - BUTTON_SIZE / 2.0,
                            y: first_point.y * scale - BUTTON_SIZE / 2.0,
                            width: BUTTON_SIZE,
                            height: BUTTON_SIZE,
                        },
                        style: RectStyle {
                            stroke: Some(RectStroke {
                                color: Color::grayscale_f01(0.5),
                                width: px(1.0),
                                border_position: namui::BorderPosition::Inside,
                            }),
                            fill: Some(RectFill {
                                color: Color::WHITE,
                            }),
                            round: None,
                        },
                    })
                    .attach_event(|builder| {
                        builder.on_mouse_down_in(move |_| {
                            namui::event::send(SelectionEvent::PolySelectionCreateButtonClicked)
                        });
                    })
                    .with_mouse_cursor(namui::MouseCursor::Pointer)
                }
                None => namui::RenderingTree::Empty,
            },
            PolySelectionCreationState::Created => namui::RenderingTree::Empty,
        }
    }
}
impl SelectionTrait for PolySelection {
    fn render(&self, scale: f32) -> namui::RenderingTree {
        match self.point_list.first() {
            Some(first_point) => {
                let path = self
                    .point_list
                    .iter()
                    .skip(1)
                    .fold(
                        PathBuilder::new().move_to(first_point.x, first_point.y),
                        |path_builder, point| path_builder.line_to(point.x, point.y),
                    )
                    .scale(scale, scale);
                let path = match self.creation_state {
                    PolySelectionCreationState::Creating => path,
                    PolySelectionCreationState::Created => path.close(),
                };
                let paint = PaintBuilder::new()
                    .set_style(PaintStyle::Stroke)
                    .set_color(Color::grayscale_f01(0.5))
                    .set_stroke_width(px(1.0));
                render([
                    namui::path(path, paint).attach_event(|builder| {
                        let id = self.id.clone();
                        builder.on_mouse_down_in(move |event: MouseEvent| {
                            if event.pressing_buttons.contains(&namui::MouseButton::Right) {
                                namui::event::send(SelectionEvent::SelectionRightClicked {
                                    target_id: id.clone(),
                                })
                            }
                        });
                    }),
                    self.render_selection_create_button(scale),
                ])
            }
            _ => namui::RenderingTree::Empty,
        }
    }

    fn get_polygon(&self) -> Vec<namui::Xy<Px>> {
        self.point_list.clone()
    }

    fn get_id(&self) -> Uuid {
        self.id
    }
}

#[derive(Clone)]
pub enum PolySelectionCreationState {
    Creating,
    Created,
}
