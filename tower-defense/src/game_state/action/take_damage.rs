use crate::game_state::camera::ShakeIntensity;
use crate::game_state::*;
use crate::{Damage, Health, Shield};
use namui::Duration;
use rand::Rng;

const DAMAGE_SOUND_DELAY_MIN_MS: i64 = 10;
const DAMAGE_SOUND_DELAY_MAX_MS: i64 = 50;

pub(super) fn shake_camera(game_state: &mut GameState, damage: Damage) {
    let intensity = match damage.as_f32() {
        d if d < 10.0 => ShakeIntensity::Light,
        d if d < 25.0 => ShakeIntensity::Medium,
        _ => ShakeIntensity::Heavy,
    };
    game_state.camera.shake(intensity);
    game_state.on_player_damaged(intensity);
}

/// Returns the actual damage after shield absorption.
pub(super) fn apply_shield_and_damage(game_state: &mut GameState, damage: Damage) -> Damage {
    let hp_before = game_state.hp;
    let mut damage_after_shield = damage;
    if !game_state.shield.is_zero() {
        let absorbed = Shield::from_raw(damage.raw().min(game_state.shield.raw()));
        damage_after_shield = damage.saturating_sub(Damage::from_raw(absorbed.raw()));
        game_state.shield = game_state.shield.saturating_sub(absorbed);
    }
    game_state.hp = game_state
        .hp
        .saturating_sub(Health::from_raw(damage_after_shield.raw()));
    let actual_damage = Damage::from_raw(hp_before.raw().saturating_sub(game_state.hp.raw()));
    if !actual_damage.is_zero() {
        game_state.metrics.total_player_damage = game_state
            .metrics
            .total_player_damage
            .saturating_add(Health::from_raw(actual_damage.raw()));
        if let Some((_, stage_damage)) = game_state
            .metrics
            .stage_damage
            .iter_mut()
            .find(|(stage, _)| *stage == game_state.stage)
        {
            *stage_damage = stage_damage.saturating_add(Health::from_raw(actual_damage.raw()));
        } else {
            game_state
                .metrics
                .stage_damage
                .push((game_state.stage, Health::from_raw(actual_damage.raw())));
        }
    }
    if let GameFlow::Defense(defense_flow) = &mut game_state.flow {
        defense_flow.took_damage = true;
    }
    actual_damage
}

pub(super) fn play_damage_sounds(game_state: &mut GameState, damage: Damage) {
    let damage = damage.as_f32();
    if damage <= 0.0 {
        return;
    }
    let repeat_count = match damage {
        d if d < 10.0 => 1,
        d if d < 25.0 => 2,
        d if d < 50.0 => 3,
        _ => 4,
    };
    let mut rng = rand::thread_rng();
    let mut accumulated_delay_ms = 0i64;
    for index in 0..repeat_count {
        game_state
            .effect_events
            .push(GameEffectEvent::PlaySoundDelayed(
                sound::EmitSoundParams::one_shot(
                    sound::random_pickaxe(),
                    sound::SoundGroup::Sfx,
                    sound::VolumePreset::High,
                    sound::SpatialMode::NonSpatial,
                ),
                Duration::from_millis(accumulated_delay_ms),
            ));
        if index + 1 < repeat_count {
            accumulated_delay_ms +=
                rng.gen_range(DAMAGE_SOUND_DELAY_MIN_MS..=DAMAGE_SOUND_DELAY_MAX_MS);
        }
    }
}

/// `damage` is the original damage (for the `> 0` guard); `actual_damage` is post-shield.
pub(super) fn record_history_event(
    game_state: &mut GameState,
    damage: Damage,
    actual_damage: Damage,
) {
    if damage.is_zero() {
        return;
    }
    game_state.record_event(
        crate::game_state::play_history::HistoryEventType::DamageTaken {
            amount: actual_damage.as_f32(),
        },
    );
}

pub(super) fn check_game_over(game_state: &mut GameState) {
    if game_state.hp.is_zero() {
        game_state.effect_events.push(GameEffectEvent::PlaySound(
            sound::EmitSoundParams::one_shot(
                sound::random_fail(),
                sound::SoundGroup::Sfx,
                sound::VolumePreset::High,
                sound::SpatialMode::NonSpatial,
            ),
        ));
        game_state.action(crate::game_state::GameStateAction::GameOver);
    }
}

#[cfg(test)]
mod tests {
    use super::apply_shield_and_damage;
    use crate::game_state::create_game_state_with_seed;
    use crate::{Damage, Health, Shield};

    #[test]
    fn damage_is_capped_by_remaining_hp() {
        let mut game_state = create_game_state_with_seed(0xDAAA_000E);
        game_state.hp = Health::from_integer(10);

        let actual_damage = apply_shield_and_damage(&mut game_state, Damage::from_integer(100));

        assert_eq!(actual_damage, Damage::from_integer(10));
        assert_eq!(game_state.hp, Health::ZERO);
        assert_eq!(
            game_state.metrics.total_player_damage,
            Health::from_integer(10)
        );
        assert_eq!(
            game_state.metrics.stage_damage,
            vec![(game_state.stage, Health::from_integer(10))]
        );
    }

    #[test]
    fn shield_absorption_and_hp_damage_are_recorded_separately() {
        let mut game_state = create_game_state_with_seed(0xDAAA_000E);
        game_state.hp = Health::from_integer(100);
        game_state.shield = Shield::from_integer(25);

        let actual_damage = apply_shield_and_damage(&mut game_state, Damage::from_integer(50));

        assert_eq!(actual_damage, Damage::from_integer(25));
        assert_eq!(game_state.hp, Health::from_integer(75));
        assert_eq!(game_state.shield, Shield::ZERO);
        assert_eq!(
            game_state.metrics.total_player_damage,
            Health::from_integer(25)
        );
    }

    #[test]
    fn shield_only_damage_records_zero_authoritative_damage() {
        let mut game_state = create_game_state_with_seed(0xDAAA_000E);
        game_state.hp = Health::from_integer(100);
        game_state.shield = Shield::from_integer(25);

        let actual_damage = apply_shield_and_damage(&mut game_state, Damage::from_integer(10));

        assert_eq!(actual_damage, Damage::ZERO);
        assert_eq!(game_state.hp, Health::from_integer(100));
        assert_eq!(game_state.shield, Shield::from_integer(15));
        assert_eq!(game_state.metrics.total_player_damage, Health::ZERO);
        assert!(game_state.metrics.stage_damage.is_empty());
    }
}
