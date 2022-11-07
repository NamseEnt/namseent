use super::CollisionInfo;
use crate::app::game::Tile;
use geo::{
    coord, polygon, Contains, Coordinate, CoordsIter, EuclideanDistance, Line, LinesIter, Polygon,
    Translate,
};
use namui::prelude::*;

pub struct RigidBody {
    pub polygon: Polygon,
}

impl RigidBody {
    pub fn new(polygon: Polygon) -> Self {
        Self { polygon }
    }
    pub fn from_rect(rect: Rect<Tile>) -> Self {
        Self::new(polygon![
            (x: rect.left().as_f32() as f64, y: rect.top().as_f32() as f64),
            (x: rect.left().as_f32() as f64, y: rect.bottom().as_f32() as f64),
            (x: rect.right().as_f32() as f64, y: rect.bottom().as_f32() as f64),
            (x: rect.right().as_f32() as f64, y: rect.top().as_f32() as f64),
        ])
    }

    pub fn translate(&self, xy: Xy<Tile>) -> Self {
        Self {
            polygon: self
                .polygon
                .translate(xy.x.as_f32() as f64, xy.y.as_f32() as f64),
        }
    }

    pub fn collide(&self, other: &Self) -> CollisionInfo {
        let penetrating_points = self
            .polygon
            .coords_iter()
            .filter(|point| other.polygon.contains(point))
            .collect::<Vec<_>>();
        let other_collider_lines = other.polygon.lines_iter().collect::<Vec<_>>();
        match minimum_distance_line_pair(penetrating_points, other_collider_lines) {
            Some((distance, line)) => {
                let penetration_depth = Tile::from(distance as f32);
                let collision_normal = normal_vector(line);
                CollisionInfo::Collided {
                    penetration_depth,
                    collision_normal,
                }
            }
            None => CollisionInfo::NotCollided,
        }
    }
}
fn normal_vector(line: Line) -> Xy<Tile> {
    let vector = line.delta();
    let length = vector.euclidean_distance(&coord! {x: 0., y: 0.});
    let normalized_vector = vector / length;
    let normal_vector = Xy {
        x: Tile::from(-normalized_vector.y as f32),
        y: Tile::from(normalized_vector.x as f32),
    };
    normal_vector
}

fn minimum_distance_line_pair(points: Vec<Coordinate>, lines: Vec<Line>) -> Option<(f64, Line)> {
    let mut minimum_distance_line_pair = None;
    for point in points {
        for line in lines.iter() {
            let distance = point.euclidean_distance(line);
            replace_minimum_distance_line_pair_if_less(
                &mut minimum_distance_line_pair,
                (distance, *line),
            );
        }
    }
    minimum_distance_line_pair
}
fn replace_minimum_distance_line_pair_if_less(
    minimum_distance_line_pair: &mut Option<(f64, Line)>,
    new: (f64, Line),
) {
    match minimum_distance_line_pair {
        Some(old) if old.0 < new.0 => (),
        _ => *minimum_distance_line_pair = Some(new),
    }
}
