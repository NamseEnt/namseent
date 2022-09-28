use super::Heading;
use crate::app::game::*;
use namui::prelude::*;
use namui_prebuilt::simple_rect;

const VISUAL_WIDTH: Tile = Tile(3.0);
const VISUAL_HEIGHT: Tile = Tile(4.0);
const VISUAL_OFFSET_X: Tile = Tile(-1.5);
const VISUAL_OFFSET_Y: Tile = Tile(-2.5);

#[derive(ecs_macro::Component)]
pub struct PlayerCharacter {}

pub fn new_player(position: Position) -> crate::ecs::Entity {
    crate::ecs::Entity::new()
        .add_component(Mover::new(MovementPlan::stay_forever(position, 0.sec())))
        .add_component(Collider::new(Rect::Xywh {
            x: Tile(-1.5),
            y: Tile(-1.5),
            width: Tile(3.0),
            height: Tile(3.0),
        }))
        .add_component(PlayerCharacter {})
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
                translate(
                    rendering_context.px_per_tile * (position.x + VISUAL_OFFSET_X),
                    rendering_context.px_per_tile * (position.y + VISUAL_OFFSET_Y),
                    render([
                        simple_rect(
                            Wh {
                                width: rendering_context.px_per_tile * VISUAL_WIDTH,
                                height: rendering_context.px_per_tile * VISUAL_HEIGHT,
                            },
                            Color::TRANSPARENT,
                            0.px(),
                            Color::from_f01(0.5, 0.5, 1.0, 0.5),
                        ),
                        namui_prebuilt::typography::center_text(
                            Wh {
                                width: rendering_context.px_per_tile * VISUAL_WIDTH,
                                height: rendering_context.px_per_tile * VISUAL_HEIGHT,
                            },
                            match mover.heading {
                                Heading::Left => "L",
                                Heading::Right => "R",
                            },
                            Color::WHITE,
                        ),
                    ]),
                )
            },
        ))
}
