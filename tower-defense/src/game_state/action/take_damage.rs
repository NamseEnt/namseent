use crate::game_state::camera::ShakeIntensity;
use crate::game_state::*;
use namui::Duration;
use rand::Rng;

const DAMAGE_SOUND_DELAY_MIN_MS: i64 = 10;
const DAMAGE_SOUND_DELAY_MAX_MS: i64 = 50;

pub(super) fn shake_camera(game_state: &mut GameState, damage: f32) {
    let intensity = match damage {
        d if d < 10.0 => ShakeIntensity::Light,
        d if d < 25.0 => ShakeIntensity::Medium,
        _ => ShakeIntensity::Heavy,
    };
    game_state.camera.shake(intensity);
    game_state.on_player_damaged(intensity);
}

/// Returns the actual damage after shield absorption.
pub(super) fn apply_shield_and_damage(game_state: &mut GameState, damage: f32) -> f32 {
    let mut actual_damage = damage;
    if game_state.shield > 0.0 {
        let absorbed = damage.min(game_state.shield);
        actual_damage -= absorbed;
        game_state.shield -= absorbed;
    }
    game_state.hp -= actual_damage;
    if let GameFlow::Defense(defense_flow) = &mut game_state.flow {
        defense_flow.took_damage = true;
    }
    actual_damage
}

pub(super) fn play_damage_sounds(game_state: &mut GameState, damage: f32) {
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
pub(super) fn record_history_event(game_state: &mut GameState, damage: f32, actual_damage: f32) {
    if damage <= 0.0 {
        return;
    }
    game_state.record_event(
        crate::game_state::play_history::HistoryEventType::DamageTaken {
            amount: actual_damage,
        },
    );
}

pub(super) fn check_game_over(game_state: &mut GameState) {
    if game_state.hp <= 0.0 {
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
