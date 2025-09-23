use crate::game_state::GameState;
use crate::game_state::user_status_effect::UserStatusEffectKind;
use namui::Duration;

pub fn move_monsters(game_state: &mut GameState, dt: Duration) {
    for monster in &mut game_state.monsters {
        let mut dt = dt;
        dt *= monster.get_speed_multiplier();
        monster.move_on_route.move_by(dt);
    }
    let mut damage = 0.0;
    game_state.monsters.retain(|monster| {
        if monster.move_on_route.is_finished() {
            damage += monster.get_damage_to_user();
            return false;
        }
        true
    });
    for user_status_effect in &game_state.user_status_effects {
        match user_status_effect.kind {
            UserStatusEffectKind::DamageReduction { damage_multiply } => {
                damage *= damage_multiply;
            }
        }
    }

    // Apply contract damage reduction
    damage *= game_state.contract_state.get_damage_reduction_multiplier();
    // Apply contract incoming damage increase
    damage *= game_state.contract_state.get_incoming_damage_multiplier();

    game_state.take_damage(damage);
}
