#[cfg(test)]
mod tests {
    use crate::Health;
    use crate::game_state::{CompatibilityAction, create_game_state_with_seed};

    #[test]
    fn health_paid_reroll_does_not_consume_missing_die() {
        let mut game_state = create_game_state_with_seed(0xC4D0_7E77);
        game_state.left_dice = 0;
        let health_before = game_state.hp;
        let health_cost = game_state.stage_modifiers.get_reroll_health_cost();

        game_state.apply_compatibility_action(CompatibilityAction::CardReroll);

        assert_eq!(game_state.left_dice, 0);
        assert_eq!(
            game_state.hp,
            health_before.saturating_sub(Health::from_usize(health_cost))
        );
    }
}
