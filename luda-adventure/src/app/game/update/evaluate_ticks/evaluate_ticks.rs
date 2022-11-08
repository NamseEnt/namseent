use crate::app::game::Game;

impl Game {
    pub fn evaluate_ticks(&mut self) {
        let need_evaluate = self.state.tick.need_to_evaluate_more_than_one_tick();
        if !need_evaluate {
            return;
        }

        self.pre_evaluate_ticks();

        while self.state.tick.need_to_evaluate_more_than_one_tick() {
            self.move_character();
            self.resolve_collision_about_character();
            self.state.tick.consume_one_tick()
        }
    }
}
