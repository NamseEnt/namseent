use crate::app::game::*;
use crate::component::*;
use namui::*;

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
    let sprites = positions
        .into_iter()
        .map(|position| Sprite {
            visual_rect: Rect::Xywh {
                x: position.x + VISUAL_OFFSET_X,
                y: position.y + VISUAL_OFFSET_Y,
                width: VISUAL_WIDTH,
                height: VISUAL_HEIGHT,
            },
            image_url: Url::parse("image/wall.png").unwrap(),
        })
        .collect();
    let mut sprite_batch = SpriteBatch::new(sprites);
    let center_xy = sprite_batch.visual_rect.center();
    sprite_batch.translate(Xy {
        x: -center_xy.x,
        y: -center_xy.y,
    });

    entity
        .add_component(Positioner::new_with_xy(center_xy))
        .add_component(Renderer::new(0, RenderType::SpriteBatch(sprite_batch)))
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
