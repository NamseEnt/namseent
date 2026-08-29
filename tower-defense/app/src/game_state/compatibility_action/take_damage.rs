use crate::Damage;
use crate::game_state::camera::ShakeIntensity;
use crate::game_state::*;
use rand::Rng;

const DAMAGE_SOUND_DELAY_MIN_MS: i64 = 10;
const DAMAGE_SOUND_DELAY_MAX_MS: i64 = 50;

pub(super) fn shake_camera(game_state: &mut GameState, damage: Damage) {
    let intensity = match damage.as_f32() {
        d if d < 10.0 => ShakeIntensity::Light,
        d if d < 25.0 => ShakeIntensity::Medium,
        _ => ShakeIntensity::Heavy,
    };
    game_state.on_player_damaged(intensity);
}

/// Returns the actual damage after shield absorption.
#[cfg(test)]
pub(super) fn apply_shield_and_damage(game_state: &mut GameState, damage: Damage) -> Damage {
    let mut raw = game_state.raw_core.state().clone();
    let actual_damage = raw.apply_player_damage_raw(damage.raw());
    game_state
        .restore_raw_core_projection(raw)
        .expect("raw damage must be restorable in headed adapter");
    Damage::from_raw(actual_damage)
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
        game_state.push_presentation_event(PresentationEvent::PlaySoundCueDelayed {
            cue: SoundCue::Pickaxe,
            position: None,
            volume: SoundVolume::High,
            delay_ms: accumulated_delay_ms,
        });
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

pub(crate) fn apply_presentation_effects(
    game_state: &mut GameState,
    damage: Damage,
    actual_damage: Damage,
) {
    shake_camera(game_state, damage);
    play_damage_sounds(game_state, damage);
    record_history_event(game_state, damage, actual_damage);
}

#[cfg(test)]
mod tests {
    use super::apply_shield_and_damage;
    use crate::game_state::create_game_state_with_seed;
    use crate::{Damage, Health, Shield};

    #[test]
    fn damage_is_capped_by_remaining_hp() {
        let mut game_state = create_game_state_with_seed(0xDAAA_000E);
        game_state
            .raw_core
            .edit_snapshot(|parts| parts.hp_raw = Health::from_integer(10).raw())
            .expect("test HP edit must preserve a valid snapshot");

        let actual_damage = apply_shield_and_damage(&mut game_state, Damage::from_integer(100));

        assert_eq!(actual_damage, Damage::from_integer(10));
        assert_eq!(game_state.hp, Health::ZERO);
        assert_eq!(
            Health::from_raw(game_state.metrics.total_player_damage_raw),
            Health::from_integer(10)
        );
        assert_eq!(
            game_state
                .metrics
                .stage_damage
                .iter()
                .map(|(stage, damage)| (*stage, Health::from_raw(*damage)))
                .collect::<Vec<_>>(),
            vec![(game_state.stage, Health::from_integer(10))]
        );
    }

    #[test]
    fn shield_absorption_and_hp_damage_are_recorded_separately() {
        let mut game_state = create_game_state_with_seed(0xDAAA_000E);
        game_state
            .raw_core
            .edit_snapshot(|parts| {
                parts.hp_raw = Health::from_integer(100).raw();
                parts.shield_raw = Shield::from_integer(25).raw();
            })
            .expect("test shield edit must preserve a valid snapshot");

        let actual_damage = apply_shield_and_damage(&mut game_state, Damage::from_integer(50));

        assert_eq!(actual_damage, Damage::from_integer(25));
        assert_eq!(game_state.hp, Health::from_integer(75));
        assert_eq!(game_state.shield_amount(), Shield::ZERO);
        assert_eq!(
            Health::from_raw(game_state.metrics.total_player_damage_raw),
            Health::from_integer(25)
        );
    }

    #[test]
    fn shield_only_damage_records_zero_authoritative_damage() {
        let mut game_state = create_game_state_with_seed(0xDAAA_000E);
        game_state
            .raw_core
            .edit_snapshot(|parts| {
                parts.hp_raw = Health::from_integer(100).raw();
                parts.shield_raw = Shield::from_integer(25).raw();
            })
            .expect("test shield edit must preserve a valid snapshot");

        let actual_damage = apply_shield_and_damage(&mut game_state, Damage::from_integer(10));

        assert_eq!(actual_damage, Damage::ZERO);
        assert_eq!(game_state.hp, Health::from_integer(100));
        assert_eq!(game_state.shield_amount(), Shield::from_integer(15));
        assert_eq!(
            game_state.metrics.total_player_damage_raw,
            Health::ZERO.raw()
        );
        assert!(game_state.metrics.stage_damage.is_empty());
    }
}
