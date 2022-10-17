use crate::app::game::{
    new_player, new_wall,
    update::calculate_next_motion_of_character::{
        calculate_next_movement_of_character, need_to_calculate_next_motion_of_character,
    },
    PlayerCharacter, Positioner, TileExt,
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

    let mut character = new_player(Xy {
        x: 2.tile(),
        y: 4.tile(),
    });
    character
        .get_component_mut::<&mut PlayerCharacter>()
        .unwrap()
        .set_user_input(Xy { x: 1.0, y: -1.0 });
    character
        .get_component_mut::<&mut Positioner>()
        .unwrap()
        .move_from(
            Xy {
                x: 2.tile(),
                y: 4.tile(),
            },
            0.ms(),
        );
    app.add_entity(character);
    while need_to_calculate_next_motion_of_character(&app, 0.ms(), 2.sec()) {
        calculate_next_movement_of_character(&mut app, 2.sec());
    }

    let (_, (_, positioner)) = app
        .query_entities::<(&PlayerCharacter, &Positioner)>()
        .into_iter()
        .next()
        .unwrap();

    // Step 1: Move to wall
    let position_at_step_1 = positioner.xy(0.2.sec());
    assert_approx_eq!(f32, position_at_step_1.x.as_f32(), 4.0);
    assert_approx_eq!(f32, position_at_step_1.y.as_f32(), 2.0);

    // Step 2: Move along wall
    let position_at_step_2 = positioner.xy(0.4.sec());
    assert_approx_eq!(f32, position_at_step_2.x.as_f32(), 6.0);
    assert_approx_eq!(f32, position_at_step_2.y.as_f32(), 2.0);

    // Step 3: Stay corner forever
    let position_at_step_3 = positioner.xy(2.sec());
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
    new_wall(Xy {
        x: x.tile(),
        y: y.tile(),
    })
}
