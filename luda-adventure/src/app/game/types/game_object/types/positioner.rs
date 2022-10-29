use crate::app::game::Tile;
use namui::prelude::*;

#[derive(ecs_macro::Component, Debug)]
pub struct Positioner {
    current_xy: Xy<Tile>,
    previous_xy: Xy<Tile>,
}

impl Positioner {
    pub fn new() -> Self {
        Self::new_with_xy(Xy::zero())
    }
    pub fn new_with_xy(xy: Xy<Tile>) -> Self {
        Self {
            current_xy: xy,
            previous_xy: xy,
        }
    }

    pub fn xy(&self) -> Xy<Tile> {
        self.current_xy
    }
    pub fn xy_with_interpolation(&self, interpolation_progress: f32) -> Xy<Tile> {
        if interpolation_progress >= 1.0 {
            self.current_xy
        } else {
            self.previous_xy + (self.current_xy - self.previous_xy) * interpolation_progress
        }
    }

    pub fn set_xy(&mut self, xy: Xy<Tile>) {
        self.current_xy = xy;
    }
}
