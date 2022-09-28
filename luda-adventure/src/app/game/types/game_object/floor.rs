use super::Renderer;
use crate::app::game::*;
use namui::prelude::*;
use namui_prebuilt::simple_rect;

const VISUAL_WIDTH: Tile = Tile(1.0);
const VISUAL_HEIGHT: Tile = Tile(1.0);
const VISUAL_OFFSET_X: Tile = Tile(-0.5);
const VISUAL_OFFSET_Y: Tile = Tile(-0.5);

pub struct Floor {
    id: Uuid,
    position: Position,
}
pub fn new_floor(position: Position) -> crate::ecs::Entity {
    crate::ecs::Entity::new()
        .add_component(Mover::new(MovementPlan::stay_forever(position, 0.sec())))
        .add_component(Renderer::new(
            -1,
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
                        Color::from_f01(0.3, 0.9, 0.3, 1.0),
                    ),
                )])
            },
        ))
}
