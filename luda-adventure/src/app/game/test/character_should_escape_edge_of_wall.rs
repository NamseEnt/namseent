use crate::{
    app::game::{map::Map, *},
    component::*,
    ecs,
};
use namui::*;

#[test]
fn character_should_escape_edge_of_wall() {
    let mut game = Game::new();
    add_character(&mut game.ecs_app);
    add_wall(&mut game.ecs_app);

    // Character should escape edge of wall and keep going above x > 3
    game.state.tick.current_time = 6.sec();
    game.evaluate_ticks();
    let actual_character_x = get_character_x(&game.ecs_app);
    assert!(actual_character_x > 2.tile());
}

/// Move right down sqrt(2) tiles per second.
/// Start from
/// - x: 0 tile
/// - y: 0 tile
fn add_character(ecs_app: &mut ecs::App) {
    let character = new_player(
        ecs_app,
        Xy {
            x: 0.tile(),
            y: 0.tile(),
        },
    );
    let mover = character.get_component_mut::<&mut Mover>().unwrap();
    mover.movement = Movement::Moving(Xy {
        x: Per::new(1.tile(), 1.sec()),
        y: Per::new(1.tile(), 1.sec()),
    });
}

/// Vertical wall at x = 3 with length 4
fn add_wall(ecs_app: &mut ecs::App) {
    Map::new(
        Wh::new(4, 4),
        vec![
            "0001".to_string(),
            "0001".to_string(),
            "0001".to_string(),
            "0001".to_string(),
        ],
        vec![],
    )
    .create_entities(ecs_app);
}

fn get_character_x(ecs_app: &ecs::App) -> Tile {
    let (_, (_, positioner)) = ecs_app
        .query_entities::<(&PlayerCharacter, &Positioner)>()
        .into_iter()
        .next()
        .unwrap();
    positioner.xy.x
}
