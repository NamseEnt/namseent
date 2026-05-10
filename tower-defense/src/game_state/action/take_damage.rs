use crate::game_state::*;
use crate::game_state::camera::ShakeIntensity;
use namui::Duration;
use rand::Rng;

const DAMAGE_SOUND_DELAY_MIN_MS: i64 = 10;
const DAMAGE_SOUND_DELAY_MAX_MS: i64 = 50;

pub(crate) fn take_damage(game_state: &mut GameState, damage: f32) {
    let mut actual_damage = damage;

    let intensity = match actual_damage {
        d if d < 10.0 => ShakeIntensity::Light,
        d if d < 25.0 => ShakeIntensity::Medium,
        _ => ShakeIntensity::Heavy,
    };
    game_state.camera.shake(intensity);
    game_state.on_player_damaged(intensity);

    if game_state.shield > 0.0 {
        let absorbed = damage.min(game_state.shield);
        actual_damage -= absorbed;
        game_state.shield -= absorbed;
    }

    game_state.hp -= actual_damage;
    if let GameFlow::Defense(defense_flow) = &mut game_state.flow {
        defense_flow.took_damage = true;
    }

    if damage > 0.0 {
        let repeat_count = match damage {
            d if d < 10.0 => 1,
            d if d < 25.0 => 2,
            d if d < 50.0 => 3,
            _ => 4,
        };

        let mut rng = rand::thread_rng();
        let mut accumulated_delay_ms = 0i64;

        for index in 0..repeat_count {
            game_state.effect_events.push(GameEffectEvent::PlaySoundDelayed(
                sound::EmitSoundParams::one_shot(
                    sound::random_pickaxe(),
                    sound::SoundGroup::Sfx,
                    sound::VolumePreset::High,
                    sound::SpatialMode::NonSpatial,
                ),
                Duration::from_millis(accumulated_delay_ms),
            ));

            if index + 1 < repeat_count {
                accumulated_delay_ms += rng
                    .gen_range(DAMAGE_SOUND_DELAY_MIN_MS..=DAMAGE_SOUND_DELAY_MAX_MS);
            }
        }

        game_state.record_event(
            crate::game_state::play_history::HistoryEventType::DamageTaken {
                amount: actual_damage,
            },
        );
    }

    if game_state.hp <= 0.0 {
        game_state.effect_events.push(GameEffectEvent::PlaySound(
            sound::EmitSoundParams::one_shot(
                sound::random_fail(),
                sound::SoundGroup::Sfx,
                sound::VolumePreset::High,
                sound::SpatialMode::NonSpatial,
            ),
        ));
        game_state.goto_result();
    }
}
