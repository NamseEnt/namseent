use super::super::*;
use crate::game_state::{monster::Monster, monster_spawn, tick};

#[test]
fn fang_recovers_hp_when_monster_dies() {
    let mut game_state = super::support::create_mock_game_state();
    game_state.hp = 10.0;

    game_state.upgrade(crate::game_state::upgrade::FangUpgrade::into_upgrade());

    let (template_queue, _) = monster_spawn::monster_template_queue_table(1, &game_state.config);
    let template = template_queue
        .front()
        .expect("expected at least one monster template in stage 1")
        .clone();
    let target = Monster::new(&template, game_state.route.clone(), game_state.now(), 1.0);
    let target_xy = target.center_xy_tile();
    let now = game_state.now();

    game_state.monsters.push(target);
    tick::monster_death::handle_monster_death(&mut game_state, 0, target_xy, now);

    assert!((game_state.hp - 11.0).abs() < f32::EPSILON);
}

#[test]
fn fang_recovery_respects_current_max_hp() {
    let mut game_state = super::support::create_mock_game_state();
    game_state.hp = game_state.max_hp();

    game_state.upgrade(crate::game_state::upgrade::FangUpgrade::into_upgrade());

    let (template_queue, _) = monster_spawn::monster_template_queue_table(1, &game_state.config);
    let template = template_queue
        .front()
        .expect("expected at least one monster template in stage 1")
        .clone();
    let target = Monster::new(&template, game_state.route.clone(), game_state.now(), 1.0);
    let target_xy = target.center_xy_tile();
    let now = game_state.now();

    game_state.monsters.push(target);
    tick::monster_death::handle_monster_death(&mut game_state, 0, target_xy, now);

    assert!((game_state.hp - game_state.max_hp()).abs() < f32::EPSILON);
}
