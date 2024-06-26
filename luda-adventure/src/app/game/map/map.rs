use super::try_create_new_polygon::try_create_new_polygon;
use super::MapObject;
use crate::app::game::{new_floor, new_wall, types::TileExt, Tile};
use crate::component::*;
use namui::{Wh, Xy};
use serde::{Deserialize, Serialize};

/// Mock map. Spec and concept may change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    wh: Wh<usize>,
    wall: Vec<String>,
    objects: Vec<MapObject>,
}

impl Map {
    pub fn new(wh: Wh<usize>, wall: Vec<String>, objects: Vec<MapObject>) -> Self {
        Self { wh, wall, objects }
    }

    pub fn create_entities(&self, app: &mut crate::ecs::App) {
        self.create_floor_entities(app);
        self.create_wall_entities(app);
        self.create_map_object_entities(app);
    }

    fn create_floor_entities(&self, app: &mut crate::ecs::App) {
        let positions = (0..self.wh.width)
            .flat_map(|x| {
                (0..self.wh.height).map(move |y| Xy::new((x as f32).tile(), (y as f32).tile()))
            })
            .collect();
        new_floor(app, positions);
    }

    fn create_wall_entities(&self, app: &mut crate::ecs::App) {
        self.create_wall_visual_entities(app);
        self.create_wall_collision_entities(app);
    }

    fn create_wall_visual_entities(&self, app: &mut crate::ecs::App) {
        self.wall.iter().enumerate().for_each(|(y, row)| {
            let positions = row
                .chars()
                .enumerate()
                .filter_map(|(x, wall)| match wall {
                    '1' => Some(Xy::new(Tile::from(x as f32), Tile::from(y as f32))),
                    _ => None,
                })
                .collect::<Vec<_>>();
            if let true = positions.len() > 0 {
                new_wall(app, positions);
            };
        });
    }

    fn create_wall_collision_entities(&self, app: &mut crate::ecs::App) {
        let mut visit_map = vec![vec![false; self.wh.width]; self.wh.height];
        for y in 0..self.wh.height {
            for x in 0..self.wh.width {
                if let Some(polygon) =
                    try_create_new_polygon(&self.wall, &mut visit_map, Xy::new(x, y))
                {
                    app.new_entity()
                        .add_component(Positioner::new())
                        .add_component(Collider::from_polygon(polygon));
                }
            }
        }
    }

    fn create_map_object_entities(&self, app: &mut crate::ecs::App) {
        for object in &self.objects {
            object.create_entity(app);
        }
    }
}
