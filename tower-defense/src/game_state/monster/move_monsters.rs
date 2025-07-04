use crate::game_state::GameState;
use crate::game_state::monster::MonsterStatusEffectKind;
use crate::game_state::user_status_effect::UserStatusEffectKind;
use namui::Duration;

pub fn move_monsters(game_state: &mut GameState, dt: Duration) {
    for monster in &mut game_state.monsters {
        let is_immune_to_slow = monster.status_effects.iter().any(|status_effect| {
            matches!(status_effect.kind, MonsterStatusEffectKind::ImmuneToSlow)
        });
        let mut dt = dt;
        let mut speed_multiplier = 1.0f32;
        for status_effect in &monster.status_effects {
            match status_effect.kind {
                MonsterStatusEffectKind::SpeedMul { mul } => {
                    if is_immune_to_slow && mul < 1.0 {
                        continue;
                    }
                    speed_multiplier *= mul;
                }
                MonsterStatusEffectKind::Invincible | MonsterStatusEffectKind::ImmuneToSlow => {}
            }
        }
        dt *= speed_multiplier;
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
    if game_state.shield > 0.0 {
        let min = damage.min(game_state.shield);
        damage -= min;
        game_state.shield -= min;
    }
    game_state.hp -= damage;
    if game_state.hp <= 0.0 {
        game_state.goto_result();
    }
}
