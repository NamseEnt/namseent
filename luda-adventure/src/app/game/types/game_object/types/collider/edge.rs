use crate::app::game::{Tile, TileExt};
use namui::prelude::*;

#[derive(Clone, Copy)]
pub enum Edge {
    Outside {
        start_point: Xy<Tile>,
        end_point: Xy<Tile>,
        normal_vector: Xy<Tile>,
    },
    Inside {
        start_point: Xy<Tile>,
        end_point: Xy<Tile>,
        normal_vector: Xy<Tile>,
    },
}

impl Edge {
    pub fn new_outside(start_point: Xy<Tile>, end_point: Xy<Tile>) -> Self {
        let normal_vector = calculate_normal_vector(start_point, end_point);
        Self::Outside {
            start_point,
            end_point,
            normal_vector,
        }
    }

    pub fn translate(&self, xy: Xy<Tile>) -> Self {
        match self {
            Edge::Outside {
                start_point,
                end_point,
                normal_vector,
            } => Edge::Outside {
                start_point: start_point + xy,
                end_point: end_point + xy,
                normal_vector: *normal_vector,
            },
            Edge::Inside {
                start_point,
                end_point,
                normal_vector,
            } => Edge::Inside {
                start_point: start_point + xy,
                end_point: end_point + xy,
                normal_vector: *normal_vector,
            },
        }
    }

    pub fn start_point(&self) -> Xy<Tile> {
        match self {
            Edge::Outside { start_point, .. } => *start_point,
            Edge::Inside { start_point, .. } => *start_point,
        }
    }

    pub fn end_point(&self) -> Xy<Tile> {
        match self {
            Edge::Outside { end_point, .. } => *end_point,
            Edge::Inside { end_point, .. } => *end_point,
        }
    }

    pub fn normal_vector(&self) -> Xy<Tile> {
        match self {
            Edge::Outside { normal_vector, .. } => *normal_vector,
            Edge::Inside { normal_vector, .. } => *normal_vector,
        }
    }
}

fn calculate_normal_vector(start_point: Xy<Tile>, end_point: Xy<Tile>) -> Xy<Tile> {
    let delta_xy = end_point - start_point;
    let orthogonal_vector = Xy {
        x: -delta_xy.y,
        y: delta_xy.x,
    };
    let length = orthogonal_vector.length();
    if length <= 0.tile() {
        orthogonal_vector
    } else {
        orthogonal_vector / length.as_f32()
    }
}
