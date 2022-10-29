use float_cmp::assert_approx_eq;
use namui::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{
    app::game::{
        new_player, new_wall, Game, Movement, Mover, PlayerCharacter, Positioner, Tile, TileExt,
    },
    ecs,
};

#[test]
#[wasm_bindgen_test]
fn character_should_not_move_on_cross_axis_when_collide() {
    let mut game = Game::new();
    add_character(&mut game.ecs_app);
    add_wall(&mut game.ecs_app);

    game.state.tick.set_current_time(2.sec());
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
    let mut character = new_player(Xy {
        x: 2.999.tile(),
        y: 0.tile(),
    });
    let mover = character.get_component_mut::<&mut Mover>().unwrap();
    mover.set_movement(Movement::Moving(Xy {
        x: Per::new(0.tile(), 1.sec()),
        y: Per::new(10.tile(), 1.sec()),
    }));
    ecs_app.add_entity(character);
}

/// Horizontal wall at y = 12
/// Character will stop at y = 10, 1 sec
fn add_wall(ecs_app: &mut ecs::App) {
    let walls = (0..5).map(|x| {
        new_wall(Xy {
            x: x.tile(),
            y: 12.tile(),
        })
    });
    ecs_app.add_entities(walls);
}

fn get_character_x(ecs_app: &ecs::App) -> Tile {
    let (_, (_, positioner)) = ecs_app
        .query_entities::<(&PlayerCharacter, &Positioner)>()
        .into_iter()
        .next()
        .unwrap();
    namui::log!("{:#?}", positioner.xy());
    positioner.xy().x
}
