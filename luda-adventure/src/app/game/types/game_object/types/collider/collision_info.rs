use crate::app::game::Tile;
use namui::prelude::*;
use std::ops::Neg;

#[derive(Clone, Copy)]
pub enum CollisionInfo {
    NotCollided,
    Collided {
        penetration_depth: Tile,
        collision_normal: Xy<Tile>,
    },
}

impl CollisionInfo {
    pub fn reverse_collision_normal(self) -> Self {
        match self {
            CollisionInfo::NotCollided => self,
            CollisionInfo::Collided {
                penetration_depth,
                collision_normal,
            } => CollisionInfo::Collided {
                penetration_depth,
                collision_normal: Xy {
                    x: collision_normal.x.neg(),
                    y: collision_normal.y.neg(),
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
