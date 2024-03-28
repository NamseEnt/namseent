use crate::app::game::Tile;
use namui::*;

#[ecs_macro::component]
#[derive(Debug)]
pub struct Mover {
    pub movement: Movement,
}

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug)]
pub enum Movement {
    Fixed,
    Moving(Velocity),
}

pub type Velocity = Xy<Per<Tile, Time>>;

impl Mover {
    pub fn new() -> Self {
        Self {
            movement: Movement::Fixed,
        }
    }
}
