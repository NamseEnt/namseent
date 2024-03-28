use crate::app::game::Tile;
use namui::*;

#[ecs_macro::component]
#[derive(Debug)]
pub struct Positioner {
    pub xy: Xy<Tile>,
    previous_xy: Xy<Tile>,
}

impl Positioner {
    pub fn new() -> Self {
        Self::new_with_xy(Xy::zero())
    }
    pub fn new_with_xy(xy: Xy<Tile>) -> Self {
        Self {
            xy,
            previous_xy: xy,
        }
    }

    pub fn save_current_xy(&mut self) {
        self.previous_xy = self.xy;
    }
}
