use super::JobExecution;
use crate::app::cropper::selection::{RectSelection, Selection, SelectionListModify};
use namui::Xy;

#[derive(Clone)]
pub enum RectSelectionResizeDirection {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
    Top,
    Right,
    Bottom,
    Left,
}

pub struct RectSelectionResize {
    target_id: String,
    direction: RectSelectionResizeDirection,
    initial_position: Option<Xy<f32>>,
    last_position: Option<Xy<f32>>,
}
impl RectSelectionResize {
    pub fn new(target_id: impl AsRef<str>, direction: RectSelectionResizeDirection) -> Self {
        Self {
            target_id: String::from(target_id.as_ref()),
            direction,
            initial_position: None,
            last_position: None,
        }
    }

    pub fn update_position(&mut self, position: Xy<f32>) {
        if self.initial_position.is_none() {
            self.initial_position = Some(position);
        }
        self.last_position = Some(position);
    }

    pub fn get_delta_position(&self) -> Xy<f32> {
        if self.last_position.is_none() || self.initial_position.is_none() {
            return Xy { x: 0.0, y: 0.0 };
        }
        self.last_position.unwrap() - self.initial_position.unwrap()
    }
}
impl JobExecution for RectSelectionResize {
    fn execute(
        &self,
        selection_list: Vec<crate::app::cropper::selection::Selection>,
    ) -> Vec<crate::app::cropper::selection::Selection> {
        selection_list.modify_selection(&self.target_id, move |selection| match selection {
            Selection::RectSelection(mut rect_selection) => {
                let delta_position: Xy<f32> = self.get_delta_position();
                match self.direction {
                    RectSelectionResizeDirection::Top
                    | RectSelectionResizeDirection::TopLeft
                    | RectSelectionResizeDirection::TopRight => {
                        resize_height_bottom_fixed(&mut rect_selection, &delta_position)
                    }
                    RectSelectionResizeDirection::Bottom
                    | RectSelectionResizeDirection::BottomLeft
                    | RectSelectionResizeDirection::BottomRight => {
                        resize_height_top_fixed(&mut rect_selection, &delta_position)
                    }
                    _ => {}
                };

                match self.direction {
                    RectSelectionResizeDirection::Left
                    | RectSelectionResizeDirection::TopLeft
                    | RectSelectionResizeDirection::BottomLeft => {
                        resize_width_right_fixed(&mut rect_selection, &delta_position)
                    }
                    RectSelectionResizeDirection::Right
                    | RectSelectionResizeDirection::TopRight
                    | RectSelectionResizeDirection::BottomRight => {
                        resize_width_left_fixed(&mut rect_selection, &delta_position)
                    }
                    _ => {}
                };

                Selection::RectSelection(rect_selection)
            }
        })
    }
}

fn resize_height_top_fixed(selection: &mut RectSelection, delta_position: &Xy<f32>) {
    let delta_y = delta_position.y;
    selection.xywh.height = selection.xywh.height + delta_y;
}

fn resize_height_bottom_fixed(selection: &mut RectSelection, delta_position: &Xy<f32>) {
    let delta_y = delta_position.y;
    selection.xywh.y = selection.xywh.y + delta_y;
    selection.xywh.height = selection.xywh.height - delta_y;
}

fn resize_width_left_fixed(selection: &mut RectSelection, delta_position: &Xy<f32>) {
    let delta_x = delta_position.x;
    selection.xywh.width = selection.xywh.width + delta_x;
}

fn resize_width_right_fixed(selection: &mut RectSelection, delta_position: &Xy<f32>) {
    let delta_x = delta_position.x;
    selection.xywh.x = selection.xywh.x + delta_x;
    selection.xywh.width = selection.xywh.width - delta_x;
}
