use crate::game_state::GameState;

#[derive(Clone, Debug)]
pub enum Effect {
    Dummy, // TODO
}

#[allow(dead_code)]
pub fn run_effect(_game_state: &mut GameState, _effect: &Effect) {
    println!("Run effect: {:?}", _effect);
}
