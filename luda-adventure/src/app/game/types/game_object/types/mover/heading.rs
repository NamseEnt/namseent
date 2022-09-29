use crate::app::game::{TileExt, Velocity};
use namui::prelude::*;

pub enum Heading {
    Left,
    Right,
}

pub fn get_heading_from_velocity(velocity: Velocity) -> Option<Heading> {
    let unit_delta_x = velocity.x * 1.ms();
    let unit_delta_y = velocity.y * 1.ms();

    if unit_delta_x == 0.tile() {
        return None;
    }
    let tangent = unit_delta_y / unit_delta_x;
    if tangent.abs() > 8.0 {
        return None;
    } else if unit_delta_x > 0.0.tile() {
        return Some(Heading::Right);
    } else {
        return Some(Heading::Left);
    }
}
