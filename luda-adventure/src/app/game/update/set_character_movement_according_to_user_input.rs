use crate::app::game::{Game, Movement, Mover, PlayerCharacter, TileExt};
use namui::prelude::*;
use std::collections::{hash_map::RandomState, HashSet};

impl Game {
    pub fn set_character_movement_according_to_user_input(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match event {
                NamuiEvent::KeyDown(event) | NamuiEvent::KeyUp(event) => {
                    let movement_direction =
                        get_movement_direction_from_pressing_codes(&event.pressing_codes);

                    if let Some((_entity, (player_character, mover))) = self
                        .ecs_app
                        .query_entities_mut::<(&mut PlayerCharacter, &mut Mover)>()
                        .first_mut()
                    {
                        player_character.update_heading(movement_direction);
                        mover.movement = Movement::Moving(Xy {
                            x: Per::new(10.tile() * movement_direction.x, 1.sec()),
                            y: Per::new(10.tile() * movement_direction.y, 1.sec()),
                        });
                    }
                }
                _ => (),
            }
        }
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
