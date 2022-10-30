use super::*;
use crate::app::game::*;
use namui::prelude::*;
use namui_prebuilt::simple_rect;

const VISUAL_WIDTH: Tile = tile(1.0);
const VISUAL_HEIGHT: Tile = tile(1.0);
const VISUAL_OFFSET_X: Tile = tile(-0.5);
const VISUAL_OFFSET_Y: Tile = tile(-0.5);

pub fn new_wall(position: Xy<Tile>) -> crate::ecs::Entity {
    let entity = crate::ecs::Entity::new();
    append_components(entity, position)
}

pub fn new_wall_with_id(id: Uuid, position: Xy<Tile>) -> crate::ecs::Entity {
    let entity = crate::ecs::Entity::with_id(id);
    append_components(entity, position)
}

fn append_components(entity: crate::ecs::Entity, xy: Xy<Tile>) -> crate::ecs::Entity {
    entity
        .add_component(Collider::from_rect(namui::Rect::Xywh {
            x: -0.5.tile(),
            y: -0.5.tile(),
            width: 1.tile(),
            height: 1.tile(),
        }))
        .add_component(Positioner::new_with_xy(xy))
        .add_component(Renderer::new(
            0,
            Rect::Xywh {
                x: VISUAL_OFFSET_X,
                y: VISUAL_OFFSET_Y,
                width: VISUAL_WIDTH,
                height: VISUAL_HEIGHT,
            },
            |entity, _game_context, rendering_context| {
                let positioner = entity.get_component::<&Positioner>().unwrap();
                let position = positioner.xy();
                render([translate(
                    rendering_context.px_per_tile * (position.x + VISUAL_OFFSET_X),
                    rendering_context.px_per_tile * (position.y + VISUAL_OFFSET_Y),
                    simple_rect(
                        Wh {
                            width: rendering_context.px_per_tile * VISUAL_WIDTH,
                            height: rendering_context.px_per_tile * VISUAL_HEIGHT,
                        },
                        Color::TRANSPARENT,
                        0.px(),
                        Color::from_f01(1.0, 0.5, 0.5, 0.5),
                    ),
                )])
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
