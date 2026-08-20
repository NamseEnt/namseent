use crate::Damage;
use crate::game_state::user_status_effect::UserStatusEffectKind;
use crate::game_state::{GameState, flow::GameFlow};
use crate::{RatioProduct, WorldSpeed};

pub fn move_monsters(game_state: &mut GameState) {
    for monster in &mut game_state.monsters {
        let speed = RatioProduct::one()
            .with(monster.get_speed_multiplier())
            .with(game_state.stage_modifiers.get_enemy_speed_multiplier())
            .apply_raw(monster.move_on_route.velocity().raw());
        monster
            .move_on_route
            .move_one_tick(WorldSpeed::from_raw(speed));
    }
}

pub fn resolve_base_damage(game_state: &mut GameState) {
    let mut damage = Damage::ZERO;
    for monster in &mut game_state.monsters {
        if monster.move_on_route.is_finished() {
            if !monster.stage_progress_counted {
                game_state.metrics.total_escaped_hp = game_state
                    .metrics
                    .total_escaped_hp
                    .saturating_add(monster.hp);
                if let GameFlow::Defense(defense_flow) = &mut game_state.flow {
                    defense_flow.stage_progress.processed_hp = defense_flow
                        .stage_progress
                        .processed_hp
                        .saturating_add(monster.hp);
                }
                monster.stage_progress_counted = true;
            }

            damage = damage.saturating_add(monster.get_damage_to_user());

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
    let damage = adjusted_incoming_damage(game_state, damage);

    if !damage.is_zero() {
        game_state.action(crate::game_state::GameStateAction::TakeDamage(damage));
    }
    game_state.monsters.sort_by_key(|monster| monster.id());
}

fn adjusted_incoming_damage(game_state: &GameState, damage: Damage) -> Damage {
    let status_multipliers =
        game_state
            .user_status_effects
            .iter()
            .map(|effect| match effect.kind {
                UserStatusEffectKind::DamageReduction { damage_multiply } => damage_multiply,
            });
    let ratios = crate::RatioProduct::one()
        .with_all(status_multipliers)
        .with_all(
            game_state
                .stage_modifiers
                .damage_reduction_multipliers()
                .iter()
                .copied(),
        )
        .with_all(
            game_state
                .stage_modifiers
                .incoming_damage_multipliers()
                .iter()
                .copied(),
        );
    damage.scaled_by_product(&ratios)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::create_game_state_with_seed;
    use crate::game_state::user_status_effect::UserStatusEffect;
    use crate::{FixedRatio, Health, Shield, SimTick};

    #[test]
    fn escaped_hp_starts_at_zero() {
        let game_state = create_game_state_with_seed(7);
        assert!(game_state.metrics.total_escaped_hp.is_zero());
    }

    #[test]
    fn damage_defense_shield_heal_and_multiplier_stack_golden() {
        let mut game_state = crate::game_state::effect::tests_support::make_test_state();
        let mut config = (*game_state.config).clone();
        config.player.max_hp = Health::from_integer(100);
        game_state.config = std::sync::Arc::new(config);
        game_state.hp = Health::from_integer(100);
        game_state.shield = Shield::from_integer(25);
        game_state
            .stage_modifiers
            .apply_damage_reduction_multiplier(FixedRatio::from_raw(800_000));
        game_state
            .stage_modifiers
            .apply_incoming_damage_multiplier(FixedRatio::from_raw(1_250_000));
        game_state.user_status_effects.push(UserStatusEffect {
            kind: UserStatusEffectKind::DamageReduction {
                damage_multiply: FixedRatio::from_raw(500_000),
            },
            end_at: SimTick::from_ticks(60),
        });

        let adjusted = adjusted_incoming_damage(&game_state, Damage::from_integer(100));
        assert_eq!(adjusted.raw(), 50_000);
        game_state.action(crate::game_state::GameStateAction::TakeDamage(adjusted));
        assert_eq!(game_state.shield, Shield::ZERO);
        assert_eq!(game_state.hp.raw(), 75_000);

        game_state.action(crate::game_state::GameStateAction::Heal(Health::from_raw(
            12_500,
        )));
        assert_eq!(game_state.hp.raw(), 87_500);
    }
}
