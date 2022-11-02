use super::{CollisionInfo, Edge};
use crate::app::game::{dot, Tile};
use namui::prelude::*;

/// Polygon should always be counterclockwise convex.
pub struct Polygon {
    edges: Vec<Edge>,
}

impl Polygon {
    pub fn from_rect(rect: Rect<Tile>) -> Self {
        let top_left = Xy {
            x: rect.left(),
            y: rect.top(),
        };
        let bottom_left = Xy {
            x: rect.left(),
            y: rect.bottom(),
        };
        let bottom_right = Xy {
            x: rect.right(),
            y: rect.bottom(),
        };
        let top_right = Xy {
            x: rect.right(),
            y: rect.top(),
        };
        Self {
            edges: vec![
                Edge::new_outside(top_left, bottom_left),
                Edge::new_outside(bottom_left, bottom_right),
                Edge::new_outside(bottom_right, top_right),
                Edge::new_outside(top_right, top_left),
            ],
        }
    }

    pub fn translate(&self, xy: Xy<Tile>) -> Self {
        Self {
            edges: self.edges.iter().map(|edge| edge.translate(xy)).collect(),
        }
    }

    pub fn collide(&self, other: &Self) -> CollisionInfo {
        let mut collision_info = CollisionInfo::NotCollided;
        let points_of_self = self.outside_points();
        let points_of_other = other.outside_points();
        for edge in self.outside_edges() {
            let normal_vector = edge.normal_vector();
            if let (Some(projected_max_of_self), Some(projected_min_of_other)) = (
                points_of_self
                    .iter()
                    .map(|point| dot(point, &normal_vector))
                    .max(),
                points_of_other
                    .iter()
                    .map(|point| dot(point, &normal_vector))
                    .min(),
            ) {
                let penetration_depth = projected_max_of_self - projected_min_of_other;
                if penetration_depth.is_sign_negative() {
                    return CollisionInfo::NotCollided;
                }
                match collision_info {
                    CollisionInfo::Collided {
                        penetration_depth: previous_penetration_depth,
                        ..
                    } if penetration_depth > previous_penetration_depth => (),
                    _ => {
                        collision_info = CollisionInfo::Collided {
                            penetration_depth,
                            collision_normal: normal_vector,
                        }
                    }
                }
            } else {
                return CollisionInfo::NotCollided;
            }
        }
        collision_info
    }

    fn outside_points(&self) -> Vec<Xy<Tile>> {
        if let Some(mut previous_edge) = self.edges.last().copied() {
            return self
                .edges
                .iter()
                .filter_map(|current_edge| {
                    let point = match (previous_edge, current_edge) {
                        (Edge::Inside { .. }, Edge::Inside { .. }) => None,
                        _ => Some(current_edge.start_point()),
                    };
                    previous_edge = *current_edge;
                    point
                })
                .collect();
        }
        Vec::new()
    }

    fn outside_edges(&self) -> Vec<Edge> {
        self.edges
            .iter()
            .filter_map(|edge| match edge {
                Edge::Outside { .. } => Some(*edge),
                Edge::Inside { .. } => None,
            })
            .collect()
    }
}
