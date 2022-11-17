use crate::app::game::Game;
use crate::component::*;

impl Game {
    pub fn save_positioner_xy(&mut self) {
        for (_entity, positioner) in self.ecs_app.query_entities_mut::<&mut Positioner>() {
            positioner.save_current_xy();
        }
    }
}
