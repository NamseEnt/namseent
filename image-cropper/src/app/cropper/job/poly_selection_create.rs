use super::JobExecution;
use crate::app::cropper::selection::{PolySelection, PolySelectionCreationState, Selection};
use namui::prelude::*;

pub struct PolySelectionCreate {
    last_mouse_hover_position: Xy<Px>,
    point_list: Vec<Xy<Px>>,
    create_state: PolySelectionCreationState,
}
impl PolySelectionCreate {
    pub fn new(initial_point: Xy<Px>) -> Self {
        Self {
            last_mouse_hover_position: initial_point.clone(),
            point_list: vec![initial_point],
            create_state: PolySelectionCreationState::Creating,
        }
    }
    pub fn update_position(&mut self, position: Xy<Px>) {
        self.last_mouse_hover_position = position;
    }
    pub fn add_point(&mut self, position: Xy<Px>) {
        self.point_list.push(position);
    }
    pub fn done(&mut self) {
        self.create_state = PolySelectionCreationState::Created;
    }
    pub fn is_done(&self) -> bool {
        match self.create_state {
            PolySelectionCreationState::Creating => false,
            PolySelectionCreationState::Created => true,
        }
    }
}
impl JobExecution for PolySelectionCreate {
    fn execute(
        &self,
        mut selection_list: Vec<crate::app::cropper::selection::Selection>,
    ) -> Vec<crate::app::cropper::selection::Selection> {
        let mut point_list = self.point_list.clone();
        if let PolySelectionCreationState::Creating = self.create_state {
            point_list.push(self.last_mouse_hover_position);
        }
        let poly_selection = PolySelection::new(point_list, self.create_state.clone());
        selection_list.push(Selection::PolySelection(poly_selection));
        selection_list
    }
}
