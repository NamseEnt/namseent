use crate::app::game::{
    new_wall, types::game_object::player_character::player_character::new_player, Collider, Mover,
    Position, TileExt, Velocity,
};
use float_cmp::assert_approx_eq;
use namui::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test;

#[test]
#[wasm_bindgen_test]
fn move_to_wall_then_move_along_wall_finally_stop_at_corner() {
    // Character going upper right
    // Step 0: Init
    // ══════════════════╗
    //                   ║
    //    ▲              ║
    //
    // *******************
    //
    // Step 1: Move to wall
    // ══════════════════╗
    //     ▲             ║
    //    /              ║
    //
    // *******************
    //
    // Step 2: Move along wall
    // ══════════════════╗
    //          -▲       ║
    //                   ║
    //
    // *******************
    //
    // Step 3: Stay corner forever
    // ══════════════════╗
    //                  ▲║
    //                   ║

    // Step 0: Init
    let mut app = crate::ecs::App::new();

    for wall in mock_walls() {
        app.add_entity(wall);
    }

    let mut character = new_player(Position {
        x: 2.tile(),
        y: 4.tile(),
    });

    character
        .get_component_mut::<&mut Mover>()
        .unwrap()
        .set_velocity(
            0.sec(),
            Velocity {
                x: Per::new(1.tile(), 1.sec()),
                y: Per::new((-1).tile(), 1.sec()),
            },
            f32::INFINITY.sec(),
        );
    let character_id = character.id();

    let (mover, collider) = character
        .get_component_mut::<(&mut Mover, &Collider)>()
        .unwrap();

    let collision_box_list_without_character = app
        .query_entities::<(&Collider, &Mover)>()
        .iter()
        .filter_map(|(entity, (collider, mover))| {
            if entity.id() == character_id {
                None
            } else {
                let position = mover.get_position(0.sec());
                Some(collider.get_collision_box(position))
            }
        })
        .collect::<Vec<_>>();

    while mover.get_predicted_movement_end_time() < 20.sec() {
        mover.predict_movement(&collider, &collision_box_list_without_character);
    }

    // Step 1: Move to wall
    let position_at_step_1 = mover.get_position(2.sec());
    assert_approx_eq!(f32, position_at_step_1.x.as_f32(), 4.0);
    assert_approx_eq!(f32, position_at_step_1.y.as_f32(), 2.0);

    // Step 2: Move along wall
    let position_at_step_2 = mover.get_position(4.sec());
    assert_approx_eq!(f32, position_at_step_2.x.as_f32(), 6.0);
    assert_approx_eq!(f32, position_at_step_2.y.as_f32(), 2.0);

    // Step 3: Stay corner forever
    let position_at_step_3 = mover.get_position(20.sec());
    assert_approx_eq!(f32, position_at_step_3.x.as_f32(), 7.0);
    assert_approx_eq!(f32, position_at_step_3.y.as_f32(), 2.0);
}

fn mock_walls() -> Vec<crate::ecs::Entity> {
    let mut walls = Vec::new();
    for x in 0..10 {
        match x {
            9 => {
                for y in 0..10 {
                    walls.push(mock_wall(x, y))
                }
            }
            _ => walls.push(mock_wall(x, 0)),
        }
    }
    walls
}

fn mock_wall(x: i32, y: i32) -> crate::ecs::Entity {
    new_wall(
        Position {
            x: x.tile(),
            y: y.tile(),
        },
        0.sec(),
    )
}
