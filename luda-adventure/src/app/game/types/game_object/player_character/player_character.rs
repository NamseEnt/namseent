use crate::app::game::{known_id::object::PLAYER_CHARACTER, *};
use namui::prelude::*;
use namui_prebuilt::simple_rect;

const VISUAL_WIDTH: Tile = tile(3.0);
const VISUAL_HEIGHT: Tile = tile(4.0);
const VISUAL_OFFSET_X: Tile = tile(-1.5);
const VISUAL_OFFSET_Y: Tile = tile(-2.5);

#[derive(ecs_macro::Component)]
pub struct PlayerCharacter {
    heading: Heading,
}

impl PlayerCharacter {
    pub fn heading(&self) -> Heading {
        self.heading
    }

    pub fn update_heading(&mut self, movement_direction: Xy<f32>) {
        if movement_direction.x == 0.0 {
            return;
        }
        let tangent = movement_direction.y / movement_direction.x;
        if tangent.abs() > 8.0 {
            return;
        } else if movement_direction.x > 0.0 {
            self.heading = Heading::Right;
        } else {
            self.heading = Heading::Left;
        }
    }
}

pub fn new_player(xy: Xy<Tile>) -> crate::ecs::Entity {
    crate::ecs::Entity::with_id(PLAYER_CHARACTER)
        .add_component(Positioner::new_with_xy(xy))
        .add_component(Collider::new(Rect::Xywh {
            x: tile(-1.5),
            y: tile(-1.5),
            width: tile(3.0),
            height: tile(3.0),
        }))
        .add_component(PlayerCharacter {
            heading: Heading::Left,
        })
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
                let character = entity.get_component::<&PlayerCharacter>().unwrap();
                let position = positioner.xy();
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
                            match character.heading {
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
