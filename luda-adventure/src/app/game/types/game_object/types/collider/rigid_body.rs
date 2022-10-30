use super::{CollisionInfo, Polygon};
use crate::app::game::Tile;
use namui::prelude::*;

pub struct RigidBody {
    combined_polygon: Vec<Polygon>,
}

impl RigidBody {
    pub fn from_rect(rect: Rect<Tile>) -> Self {
        Self {
            combined_polygon: vec![Polygon::from_rect(rect)],
        }
    }

    pub fn translate(&self, xy: Xy<Tile>) -> Self {
        Self {
            combined_polygon: self
                .combined_polygon
                .iter()
                .map(|polygon| polygon.translate(xy))
                .collect(),
        }
    }

    pub fn collide(&self, other: &Self) -> CollisionInfo {
        let mut collision_info = CollisionInfo::NotCollided;
        for polygon_of_self in &self.combined_polygon {
            for polygon_of_other in &other.combined_polygon {
                let new_collision_info = polygon_of_self.collide(&polygon_of_other);
                if let CollisionInfo::Collided {
                    penetration_depth, ..
                } = new_collision_info
                {
                    match collision_info {
                        CollisionInfo::Collided {
                            penetration_depth: previous_penetration_depth,
                            ..
                        } if previous_penetration_depth < penetration_depth => (),
                        _ => collision_info = new_collision_info,
                    }
                }
            }
        }
        collision_info
    }
}
