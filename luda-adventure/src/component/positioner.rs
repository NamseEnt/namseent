use crate::app::game::Tile;
use namui::prelude::*;

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

    pub fn xy_with_interpolation(&self, interpolation_progress: f32) -> Xy<Tile> {
        if interpolation_progress >= 1.0 {
            self.xy
        } else {
            self.previous_xy + (self.xy - self.previous_xy) * interpolation_progress
        }
    }

    pub fn save_current_xy(&mut self) {
        self.previous_xy = self.xy;
    }
}
