use super::Renderer;
use crate::app::game::*;
use namui::prelude::*;
use namui_prebuilt::simple_rect;

const VISUAL_WIDTH: Tile = tile(1.0);
const VISUAL_HEIGHT: Tile = tile(1.0);
const VISUAL_OFFSET_X: Tile = tile(-0.5);
const VISUAL_OFFSET_Y: Tile = tile(-0.5);

pub fn new_floor(positions: Vec<Xy<Tile>>) -> crate::ecs::Entity {
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

    crate::ecs::Entity::new()
        .add_component(Positioner::new())
        .add_component(Renderer::new(
            -1,
            Rect::Xywh {
                x: VISUAL_OFFSET_X + min_x,
                y: VISUAL_OFFSET_Y + min_y,
                width: VISUAL_WIDTH + max_x - min_x,
                height: VISUAL_HEIGHT + max_y - min_y,
            },
            move |entity, _game_context, rendering_context| {
                let positioner = entity.get_component::<&Positioner>().unwrap();
                let main_position = positioner.xy;

                render(
                    positions
                        .iter()
                        .map(|position| position + main_position)
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
                                    Color::from_f01(0.3, 0.9, 0.3, 1.0),
                                ),
                            )
                        }),
                )
            },
        ))
}
