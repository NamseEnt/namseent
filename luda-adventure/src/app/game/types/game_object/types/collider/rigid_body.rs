use super::CollisionInfo;
use crate::app::game::Tile;
use geo::{
    coord, polygon, ClosestPoint, Contains, Coordinate, CoordsIter, EuclideanDistance, Line,
    LinesIter, Polygon, Translate,
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
        match closest_point_line_pair(penetrating_points, other_collider_lines) {
            Some((point, line)) => {
                let counter_penetration_vector = match line.closest_point(&point.into()) {
                    geo::Closest::SinglePoint(closest_point) => {
                        normalized_vector(point, closest_point.into())
                    }
                    _ => normal_vector(line),
                };
                let penetration_depth = Tile::from(point.euclidean_distance(&line) as f32);
                CollisionInfo::Collided {
                    penetration_depth,
                    counter_penetration_vector,
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
fn normalized_vector(from: Coordinate, to: Coordinate) -> Xy<Tile> {
    let vector = to - from;
    let length = vector.euclidean_distance(&coord! {x: 0., y: 0.});
    let normalized_vector = vector / length;
    Xy::new(
        Tile::from(normalized_vector.x as f32),
        Tile::from(normalized_vector.y as f32),
    )
}
fn closest_point_line_pair(
    points: Vec<Coordinate>,
    lines: Vec<Line>,
) -> Option<(Coordinate, Line)> {
    let mut closest_point_line_pair = None;
    let mut closest_distance = None;
    for point in points {
        for line in lines.iter() {
            let distance = point.euclidean_distance(line);
            if is_new_distance_closest(closest_distance, distance) {
                closest_distance = Some(distance);
                closest_point_line_pair = Some((point, *line));
            }
        }
    }
    closest_point_line_pair
}
fn is_new_distance_closest(closest_distance: Option<f64>, new_distance: f64) -> bool {
    match closest_distance {
        Some(old_distance) if old_distance < new_distance => false,
        _ => true,
    }
}
