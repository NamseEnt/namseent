use crate::app::game::Game;

impl Game {
    pub fn evaluate_ticks(&mut self) {
        while let Some(delta_time) = self.state.tick.try_consume_one_tick() {
            self.move_character(delta_time);
            self.resolve_collision_about_character();
        }
    }
}
