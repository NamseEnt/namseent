use crate::app::game::Tile;
use geo::{coord, Coordinate, EuclideanDistance};
use namui::prelude::*;

pub fn normalized_vector(from: Coordinate, to: Coordinate) -> Xy<Tile> {
    let vector = to - from;
    let length = vector.euclidean_distance(&coord! {x: 0., y: 0.});
    let normalized_vector = vector / length;
    Xy::new(
        Tile::from(normalized_vector.x as f32),
        Tile::from(normalized_vector.y as f32),
    )
}
