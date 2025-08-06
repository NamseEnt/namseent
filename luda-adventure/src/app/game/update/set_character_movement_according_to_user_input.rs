use crate::app::game::*;
use crate::component::*;
use namui::*;
use std::collections::{HashSet, hash_map::RandomState};

impl Game {
    pub fn set_character_movement_according_to_user_input(&mut self, event: &namui::Event) {
        event
            .is::<game::Event>(|event| match event {
                game::Event::KeyDown {
                    code: _,
                    pressing_codes,
                }
                | game::Event::KeyUp {
                    code: _,
                    pressing_codes,
                } => {
                    let movement_direction =
                        get_movement_direction_from_pressing_codes(pressing_codes);
                    self.set_character_movement_with_movement_direction(movement_direction);
                }
            })
            .is::<save_load::Event>(|event| match event {
                save_load::Event::Loaded => {
                    self.set_character_movement_with_movement_direction(Xy::zero());
                }
            });
    }

    fn set_character_movement_with_movement_direction(&mut self, movement_direction: Xy<f32>) {
        if let Some((_entity, (player_character, mover, renderer))) = self
            .ecs_app
            .query_entities_mut::<(&mut PlayerCharacter, &mut Mover, &mut Renderer)>()
            .first_mut()
        {
            let new_movement = movement(movement_direction);
            let previous_movement = mover.movement;
            let previous_heading = player_character.heading;

            player_character.update_heading(movement_direction);
            mover.movement = new_movement;

            match (previous_movement, new_movement) {
                (Movement::Fixed, Movement::Moving(_)) => {
                    renderer.render_type = RenderType::SpriteAnimation(
                        player_character::walking_sprite_animation(self.state.tick.current_time),
                    );
                }
                (Movement::Moving(_), Movement::Fixed) => {
                    renderer.render_type = RenderType::Sprite(player_character::standing_sprite());
                }
                _ => (),
            }

            match (previous_heading, player_character.heading) {
                (Heading::Left, Heading::Right) => {
                    renderer.x_reverse = false;
                }
                (Heading::Right, Heading::Left) => {
                    renderer.x_reverse = true;
                }
                _ => (),
            }
        }
    }
}

fn movement(movement_direction: Xy<f32>) -> Movement {
    match movement_direction.length() == 0. {
        true => Movement::Fixed,
        false => Movement::Moving(Xy {
            x: Per::new(10.tile() * movement_direction.x, 1.sec()),
            y: Per::new(10.tile() * movement_direction.y, 1.sec()),
        }),
    }
}

fn get_movement_direction_from_pressing_codes(
    pressing_codes: &HashSet<Code, RandomState>,
) -> Xy<f32> {
    let mut direction = Xy::<f32>::zero();
    if pressing_codes.contains(&Code::ArrowDown) {
        direction.y += 1.0;
    }
    if pressing_codes.contains(&Code::ArrowUp) {
        direction.y -= 1.0;
    }
    if pressing_codes.contains(&Code::ArrowRight) {
        direction.x += 1.0;
    }
    if pressing_codes.contains(&Code::ArrowLeft) {
        direction.x -= 1.0;
    }
    let direction_length = direction.length();
    let normalized_direction = match direction_length == 0.0 {
        true => direction,
        false => direction / direction_length,
    };
    normalized_direction
}
