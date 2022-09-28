use super::*;
use crate::app::game::*;
use namui::prelude::*;
use namui_prebuilt::simple_rect;

const COLLISION_WIDTH: Tile = Tile(1.0);
const COLLISION_HEIGHT: Tile = Tile(1.0);
const COLLISION_OFFSET_X: Tile = Tile(-0.5);
const COLLISION_OFFSET_Y: Tile = Tile(-0.5);
const VISUAL_WIDTH: Tile = Tile(1.0);
const VISUAL_HEIGHT: Tile = Tile(1.0);
const VISUAL_OFFSET_X: Tile = Tile(-0.5);
const VISUAL_OFFSET_Y: Tile = Tile(-0.5);

pub fn new_wall(position: Position, current_time: Time) -> crate::ecs::Entity {
    let entity = crate::ecs::Entity::new();
    append_components(entity, position, current_time)
}

pub fn new_wall_with_id(id: Uuid, position: Position, current_time: Time) -> crate::ecs::Entity {
    let entity = crate::ecs::Entity::with_id(id);
    append_components(entity, position, current_time)
}

fn append_components(
    entity: crate::ecs::Entity,
    position: Position,
    current_time: Time,
) -> crate::ecs::Entity {
    entity
        .add_component(Collider::new(namui::Rect::Xywh {
            x: 0.5.tile(),
            y: 0.5.tile(),
            width: 1.tile(),
            height: 1.tile(),
        }))
        .add_component(Mover::new(MovementPlan::stay_forever(
            position,
            current_time,
        )))
        .add_component(Renderer::new(
            0,
            Rect::Xywh {
                x: VISUAL_OFFSET_X,
                y: VISUAL_OFFSET_Y,
                width: VISUAL_WIDTH,
                height: VISUAL_HEIGHT,
            },
            |entity, _game_context, rendering_context| {
                let mover = entity.get_component::<&Mover>().unwrap();
                let position = mover.get_position(rendering_context.current_time);
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
    position: Position,
}
impl Wall {
    pub fn new(position: Position) -> Wall {
        Wall {
            id: Uuid::new_v4(),
            position,
        }
    }

    pub fn new_with_id(position: Position, id: Uuid) -> Wall {
        Wall { id, position }
    }
}
