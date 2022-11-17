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

pub fn new_player(app: &mut crate::ecs::App, xy: Xy<Tile>) -> &mut crate::ecs::Entity {
    app.new_entity_with_id(PLAYER_CHARACTER)
        .add_component(Positioner::new_with_xy(xy))
        .add_component(Collider::from_circle(Xy::zero(), tile(1.5)))
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
                let position =
                    positioner.xy_with_interpolation(rendering_context.interpolation_progress);
                translate(
                    (rendering_context.px_per_tile * (position.x + VISUAL_OFFSET_X)).floor(),
                    (rendering_context.px_per_tile * (position.y + VISUAL_OFFSET_Y)).floor(),
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
                        namui_prebuilt::typography::center_text_full_height(
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
        .add_component(Mover::new())
}
