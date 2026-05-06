#[test]
fn popcorn_effect_decrements_over_waves_and_expires() {
    use super::support;
    use crate::game_state::GameFlow;
    use crate::game_state::flow::DefenseFlow;
    use crate::game_state::tower::{Tower, TowerTemplate};

    let mut game_state = support::create_mock_game_state();
    game_state.upgrade(crate::game_state::upgrade::PopcornUpgrade::into_upgrade(
        5.0, 5, 5,
    ));
    game_state.handle_upgrade_trigger(
        crate::game_state::upgrade::UpgradeTriggerEvent::StageStart { stage: 1 },
    );

    game_state.flow = GameFlow::Defense(DefenseFlow::new(&game_state));
    let tower_template = TowerTemplate::new(
        crate::game_state::tower::TowerKind::High,
        crate::card::Suit::Hearts,
        crate::card::Rank::Two,
    );
    let tower = Tower::new(
        &tower_template,
        crate::MapCoord::new(0, 0),
        game_state.now(),
    );
    game_state.place_tower(tower);

    let expected_multipliers = [5.0, 4.0, 3.0, 2.0, 1.0, 1.0];
    for expected_multiplier in expected_multipliers {
        let tower = game_state
            .towers
            .iter()
            .next()
            .expect("expected tower still present");
        support::assert_tower_cached_damage_mul(tower, expected_multiplier);

        if expected_multiplier > 1.0 {
            game_state.flow = GameFlow::Defense(DefenseFlow::new(&game_state));
            crate::game_state::tick::defense_end::check_defense_end(&mut game_state);
        }
    }
}
