use crate::app::game::{Game, Positioner};

impl Game {
    pub fn save_positioner_xy(&mut self) {
        for positioner in self.ecs_app.query_component_mut::<Positioner>() {
            positioner.save_current_xy();
        }
    }
}
