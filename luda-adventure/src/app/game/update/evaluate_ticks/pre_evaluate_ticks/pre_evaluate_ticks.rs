use crate::app::game::Game;

impl Game {
    pub fn pre_evaluate_ticks(&mut self) {
        self.save_positioner_xy();
    }
}
