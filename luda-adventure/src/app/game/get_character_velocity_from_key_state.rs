use super::{TileExt, Velocity};
use namui::prelude::*;

pub fn get_character_velocity_from_key_state() -> Velocity {
    let mut direction = Xy::<f32>::zero();
    if namui::keyboard::any_code_press([Code::ArrowDown]) {
        direction.y += 1.0;
    }
    if namui::keyboard::any_code_press([Code::ArrowUp]) {
        direction.y -= 1.0;
    }
    if namui::keyboard::any_code_press([Code::ArrowRight]) {
        direction.x += 1.0;
    }
    if namui::keyboard::any_code_press([Code::ArrowLeft]) {
        direction.x -= 1.0;
    }
    let direction_length = direction.length();
    let normalized_direction = match direction_length == 0.0 {
        true => direction,
        false => direction / direction_length,
    };
    Xy {
        x: Per::new(10.tile() * normalized_direction.x, 1.sec()),
        y: Per::new(10.tile() * normalized_direction.y, 1.sec()),
    }
}
