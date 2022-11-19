use crate::app::game::*;
use crate::component::*;
use namui::prelude::*;

const VISUAL_WIDTH: Tile = tile(1.0);
const VISUAL_HEIGHT: Tile = tile(1.0);
const VISUAL_OFFSET_X: Tile = tile(-0.5);
const VISUAL_OFFSET_Y: Tile = tile(-0.5);

pub fn new_wall(app: &mut crate::ecs::App, positions: Vec<Xy<Tile>>) -> &mut crate::ecs::Entity {
    new_wall_with_id(app, Uuid::new_v4(), positions)
}

pub fn new_wall_with_id(
    app: &mut crate::ecs::App,
    id: Uuid,
    positions: Vec<Xy<Tile>>,
) -> &mut crate::ecs::Entity {
    let entity = app.new_entity_with_id(id);
    append_components(entity, positions)
}

fn append_components(
    entity: &mut crate::ecs::Entity,
    positions: Vec<Xy<Tile>>,
) -> &mut crate::ecs::Entity {
    let min_x = positions
        .iter()
        .map(|p| p.x)
        .reduce(|a, b| a.min(b))
        .unwrap();
    let min_y = positions
        .iter()
        .map(|p| p.y)
        .reduce(|a, b| a.min(b))
        .unwrap();
    let max_x = positions
        .iter()
        .map(|p| p.x)
        .reduce(|a, b| a.max(b))
        .unwrap();
    let max_y = positions
        .iter()
        .map(|p| p.y)
        .reduce(|a, b| a.max(b))
        .unwrap();
    let center_xy = Xy {
        x: (min_x + max_x) * 0.5,
        y: (min_y + max_y) * 0.5,
    };
    let width = max_x - min_x;
    let height = max_y - min_y;

    entity
        .add_component(Positioner::new_with_xy(center_xy))
        .add_component(Renderer::new(
            0,
            Rect::Xywh {
                x: width * -0.5,
                y: height * -0.5,
                width,
                height,
            },
            RenderType::Wall {
                positions,
                visual_offset_rect: Rect::Xywh {
                    x: VISUAL_OFFSET_X,
                    y: VISUAL_OFFSET_Y,
                    width: VISUAL_WIDTH,
                    height: VISUAL_HEIGHT,
                },
            },
        ))
}

pub struct Wall {
    id: Uuid,
    position: Xy<Tile>,
}
impl Wall {
    pub fn new(position: Xy<Tile>) -> Wall {
        Wall {
            id: Uuid::new_v4(),
            position,
        }
    }

    pub fn new_with_id(position: Xy<Tile>, id: Uuid) -> Wall {
        Wall { id, position }
    }
}
