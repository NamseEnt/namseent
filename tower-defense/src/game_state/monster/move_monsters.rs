use crate::game_state::user_status_effect::UserStatusEffectKind;
use crate::game_state::{GameState, flow::GameFlow};
use namui::Duration;

pub fn move_monsters(game_state: &mut GameState, dt: Duration) {
    for monster in &mut game_state.monsters {
        let mut dt = dt;
        dt *= monster.get_speed_multiplier();
        monster.move_on_route.move_by(dt);
    }
    let mut damage = 0.0;
    for monster in &mut game_state.monsters {
        if monster.move_on_route.is_finished() {
            // 몬스터가 건물에 도달하기 전에 받은 데미지를 처리량에 반영
            if let GameFlow::Defense(defense_flow) = &mut game_state.flow {
                defense_flow.stage_progress.processed_hp +=
                    (monster.max_hp - monster.hp.max(0.0)).max(0.0);
            }

            damage += monster.get_damage_to_user();

            // normal_monster가 아닌 경우 체력을 유지한 채 시작지점에서 재출발
            if !monster.kind.is_normal_monster() {
                monster.move_on_route.reset();
            }
        }
    }

    // normal_monster만 제거
    game_state.monsters.retain(|monster| {
        !(monster.move_on_route.is_finished() && monster.kind.is_normal_monster())
    });
    for user_status_effect in &game_state.user_status_effects {
        match user_status_effect.kind {
            UserStatusEffectKind::DamageReduction { damage_multiply } => {
                damage *= damage_multiply;
            }
        }
    }

    // Apply contract damage reduction
    damage *= game_state.stage_modifiers.get_damage_reduction_multiplier();
    // Apply contract incoming damage increase
    damage *= game_state.stage_modifiers.get_incoming_damage_multiplier();

    if damage > 0.0 {
        game_state.take_damage(damage);
    }
}
