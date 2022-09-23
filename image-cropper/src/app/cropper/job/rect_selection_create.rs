use super::JobExecution;
use crate::app::cropper::selection::{RectSelection, Selection};
use namui::prelude::*;

pub struct RectSelectionCreate {
    initial_position: Xy<Px>,
    last_position: Xy<Px>,
}
impl RectSelectionCreate {
    pub fn new(initial_position: Xy<Px>) -> Self {
        Self {
            initial_position: initial_position.clone(),
            last_position: initial_position.clone(),
        }
    }
    pub fn update_position(&mut self, position: Xy<Px>) {
        self.last_position = position;
    }
}
impl JobExecution for RectSelectionCreate {
    fn execute(
        &self,
        mut selection_list: Vec<crate::app::cropper::selection::Selection>,
    ) -> Vec<crate::app::cropper::selection::Selection> {
        let left_top_point = Xy {
            x: self.initial_position.x.min(self.last_position.x),
            y: self.initial_position.y.min(self.last_position.y),
        };
        let right_bottom_point = Xy {
            x: self.initial_position.x.max(self.last_position.x),
            y: self.initial_position.y.max(self.last_position.y),
        };
        let selection_xywh = Rect::Xywh {
            x: left_top_point.x,
            y: left_top_point.y,
            width: right_bottom_point.x - left_top_point.x,
            height: right_bottom_point.y - left_top_point.y,
        };
        selection_list.push(Selection::RectSelection(RectSelection::new(selection_xywh)));
        selection_list
    }
}
