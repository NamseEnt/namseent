use crate::app::game::Tile;
use namui::prelude::*;
use std::ops::Neg;

#[derive(Clone, Copy)]
pub enum CollisionInfo {
    NotCollided,
    Collided {
        penetration_depth: Tile,
        counter_penetration_vector: Xy<Tile>,
    },
}

impl CollisionInfo {
    pub fn reverse_collision_normal(self) -> Self {
        match self {
            CollisionInfo::NotCollided => self,
            CollisionInfo::Collided {
                penetration_depth,
                counter_penetration_vector,
            } => CollisionInfo::Collided {
                penetration_depth,
                counter_penetration_vector: Xy {
                    x: counter_penetration_vector.x.neg(),
                    y: counter_penetration_vector.y.neg(),
                },
            },
        }
    }

    pub fn min_by_penetration_depth(self, other: Self) -> Self {
        match (self, other) {
            (CollisionInfo::NotCollided, CollisionInfo::NotCollided) => self,
            (CollisionInfo::NotCollided, CollisionInfo::Collided { .. }) => other,
            (CollisionInfo::Collided { .. }, CollisionInfo::NotCollided) => self,
            (
                CollisionInfo::Collided {
                    penetration_depth: penetration_depth_of_self,
                    ..
                },
                CollisionInfo::Collided {
                    penetration_depth: penetration_depth_of_other,
                    ..
                },
            ) => {
                if penetration_depth_of_self < penetration_depth_of_other {
                    self
                } else {
                    other
                }
            }
        }
    }
}
