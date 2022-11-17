use crate::{app::game::*, component::*, ecs};
use float_cmp::assert_approx_eq;
use namui::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test;

#[test]
#[wasm_bindgen_test]
fn character_should_not_move_on_cross_axis_when_collide() {
    let mut game = Game::new();
    add_character(&mut game.ecs_app);
    add_wall(&mut game.ecs_app);

    game.state.tick.current_time = 2.sec();
    game.evaluate_ticks();

    let expected_character_x = 2.999.tile();
    let actual_character_x = get_character_x(&game.ecs_app);

    assert_approx_eq!(Tile, expected_character_x, actual_character_x);
}

/// Move down 10 tiles per second.
/// Start from
/// - x: 2.999 tile
/// - y: 0 tile
fn add_character(ecs_app: &mut ecs::App) {
    let character = new_player(
        ecs_app,
        Xy {
            x: 2.999.tile(),
            y: 0.tile(),
        },
    );
    let mover = character.get_component_mut::<&mut Mover>().unwrap();
    mover.movement = Movement::Moving(Xy {
        x: Per::new(0.tile(), 1.sec()),
        y: Per::new(10.tile(), 1.sec()),
    });
}

/// Horizontal wall at y = 12
/// Character will stop at y = 10, 1 sec
fn add_wall(ecs_app: &mut ecs::App) {
    Map::new(
        Wh::new(12, 12),
        vec![
            "000000000000".to_string(),
            "000000000000".to_string(),
            "000000000000".to_string(),
            "000000000000".to_string(),
            "000000000000".to_string(),
            "000000000000".to_string(),
            "000000000000".to_string(),
            "000000000000".to_string(),
            "000000000000".to_string(),
            "000000000000".to_string(),
            "000000000000".to_string(),
            "111111111111".to_string(),
        ],
    )
    .create_entities(ecs_app)
}

fn get_character_x(ecs_app: &ecs::App) -> Tile {
    let (_, (_, positioner)) = ecs_app
        .query_entities::<(&PlayerCharacter, &Positioner)>()
        .into_iter()
        .next()
        .unwrap();
    positioner.xy.x
}
