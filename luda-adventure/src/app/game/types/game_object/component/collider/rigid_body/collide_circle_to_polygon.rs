use super::Circle;
use crate::app::game::{CollisionInfo, Tile};
use geo::{coord, ClosestPoint, Contains, Coordinate, EuclideanDistance, Line, LinesIter, Polygon};
use namui::prelude::*;

pub fn collide_circle_to_polygon(circle: &Circle, polygon: &Polygon) -> CollisionInfo {
    let lines = polygon.lines_iter().collect::<Vec<_>>();
    match closest_point(lines, circle) {
        Some(closest_point) => {
            let distance_between_circle_center_and_closest_point =
                circle.center.euclidean_distance(&closest_point);
            if distance_between_circle_center_and_closest_point >= circle.radius {
                return CollisionInfo::NotCollided;
            }
            let center_is_inside_polygon = polygon.contains(&circle.center);
            let counter_penetration_vector = match center_is_inside_polygon {
                true => normalized_vector(circle.center, closest_point),
                false => normalized_vector(closest_point, circle.center),
            };
            let penetration_depth = match center_is_inside_polygon {
                true => distance_between_circle_center_and_closest_point + circle.radius,
                false => circle.radius - distance_between_circle_center_and_closest_point,
            };
            CollisionInfo::Collided {
                counter_penetration_vector,
                penetration_depth: Tile::from(penetration_depth as f32),
            }
        }
        None => CollisionInfo::NotCollided,
    }
}

fn closest_point(lines: Vec<Line>, circle: &Circle) -> Option<Coordinate<f64>> {
    let mut closest_line = None;
    let mut closest_distance_to_circle_center = None;
    for line in lines {
        let distance = circle.center.euclidean_distance(&line);
        if is_new_closest(closest_distance_to_circle_center, distance) {
            closest_line = Some(line);
            closest_distance_to_circle_center = Some(distance);
        }
    }
    closest_line
        .map(|line| line.closest_point(&circle.center.into()))
        .and_then(|closest| match closest {
            geo::Closest::Intersection(point) | geo::Closest::SinglePoint(point) => {
                Some(point.into())
            }
            geo::Closest::Indeterminate => None,
        })
}
fn is_new_closest(closest_distance_to_circle_center: Option<f64>, new_distance: f64) -> bool {
    match closest_distance_to_circle_center {
        Some(old_distance) if old_distance < new_distance => false,
        _ => true,
    }
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
