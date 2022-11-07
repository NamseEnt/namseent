use super::*;
use crate::app::game::*;
use namui::prelude::*;
use namui_prebuilt::simple_rect;

const VISUAL_WIDTH: Tile = tile(1.0);
const VISUAL_HEIGHT: Tile = tile(1.0);
const VISUAL_OFFSET_X: Tile = tile(-0.5);
const VISUAL_OFFSET_Y: Tile = tile(-0.5);

pub fn new_wall(positions: Vec<Xy<Tile>>) -> crate::ecs::Entity {
    let entity = crate::ecs::Entity::new();
    append_components(entity, positions)
}

pub fn new_wall_with_id(id: Uuid, positions: Vec<Xy<Tile>>) -> crate::ecs::Entity {
    let entity = crate::ecs::Entity::with_id(id);
    append_components(entity, positions)
}

fn append_components(entity: crate::ecs::Entity, positions: Vec<Xy<Tile>>) -> crate::ecs::Entity {
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
            move |_entity, _game_context, rendering_context| {
                render(
                    positions
                        .iter()
                        .filter(|position| {
                            rendering_context
                                .screen_rect
                                .intersect(Rect::from_xy_wh(
                                    *position + Xy::new(VISUAL_OFFSET_X, VISUAL_OFFSET_Y),
                                    Wh::new(VISUAL_WIDTH, VISUAL_HEIGHT),
                                ))
                                .is_some()
                        })
                        .map(|position| {
                            translate(
                                rendering_context.px_per_tile * (position.x + VISUAL_OFFSET_X),
                                rendering_context.px_per_tile * (position.y + VISUAL_OFFSET_Y),
                                simple_rect(
                                    Wh {
                                        width: rendering_context.px_per_tile * VISUAL_WIDTH,
                                        height: rendering_context.px_per_tile * VISUAL_HEIGHT,
                                    },
                                    Color::TRANSPARENT,
                                    0.px(),
                                    Color::from_f01(0.9, 0.3, 0.3, 1.0),
                                ),
                            )
                        }),
                )
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
