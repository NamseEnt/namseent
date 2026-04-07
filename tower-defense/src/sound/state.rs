use namui::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{LazyLock, Mutex};

use super::event::{EmitSoundParams, SoundEvent, SoundId, SpatialMode};
use super::volume::{SoundGroup, VolumeSettings, clamp01};

static SOUND_STATE_ATOM: Atom<SoundState> = Atom::uninitialized();
static NEXT_SOUND_ID: AtomicU64 = AtomicU64::new(1);
static SOUND_EVENTS: LazyLock<Mutex<Vec<SoundEvent>>> = LazyLock::new(|| Mutex::new(Vec::new()));

#[derive(Clone, Debug, State, Default)]
pub struct SoundState {
    pub volume_settings: VolumeSettings,
}

pub fn active_sounds() -> Vec<SoundEvent> {
    SOUND_EVENTS.lock().unwrap().clone()
}

pub fn init_sound_state<'a>(ctx: &'a RenderCtx) -> Sig<'a, SoundState> {
    ctx.init_atom(&SOUND_STATE_ATOM, SoundState::default).0
}

pub fn use_sound_state<'a>(ctx: &'a RenderCtx) -> Sig<'a, SoundState> {
    ctx.atom(&SOUND_STATE_ATOM).0
}

pub fn emit_sound(params: EmitSoundParams) -> SoundId {
    if crate::is_headless() {
        return 0;
    }
    emit_sound_after(params, Duration::ZERO)
}

pub fn emit_sound_after(params: EmitSoundParams, delay: Duration) -> SoundId {
    if crate::is_headless() {
        return 0;
    }
    let sound_id = NEXT_SOUND_ID.fetch_add(1, Ordering::Relaxed);
    let now = Instant::now();
    let play_at = now + delay;
    SOUND_EVENTS.lock().unwrap().push(SoundEvent {
        id: sound_id,
        asset: params.asset,
        group: params.group,
        volume_preset: params.volume_preset,
        spatial: params.spatial,
        repeat: params.repeat,
        play_at,
        created_at: now,
        max_duration: params.max_duration.or_else(|| {
            if params.repeat {
                None
            } else {
                Some(Duration::from_secs(2))
            }
        }),
    });

    sound_id
}

pub fn stop_sound(sound_id: SoundId) {
    SOUND_EVENTS
        .lock()
        .unwrap()
        .retain(|sound| sound.id != sound_id);
}

pub fn update_sound_position(sound_id: SoundId, position: crate::MapCoordF32) {
    let mut events = SOUND_EVENTS.lock().unwrap();
    let Some(sound) = events.iter_mut().find(|sound| sound.id == sound_id) else {
        return;
    };

    sound.spatial = SpatialMode::Spatial { position };
}

pub fn cleanup_expired_sounds(now: Instant) {
    SOUND_EVENTS
        .lock()
        .unwrap()
        .retain(|sound| !sound.is_expired(now));
}

pub fn set_master_volume(volume: f32) {
    SOUND_STATE_ATOM.mutate(move |sound_state| {
        sound_state.volume_settings.master = clamp01(volume);
    });
}

pub fn set_group_volume(group: SoundGroup, volume: f32) {
    SOUND_STATE_ATOM.mutate(move |sound_state| {
        let volume = clamp01(volume);
        match group {
            SoundGroup::Sfx => sound_state.volume_settings.sfx = volume,
            SoundGroup::Ui => sound_state.volume_settings.ui = volume,
            SoundGroup::Ambient => sound_state.volume_settings.ambient = volume,
            SoundGroup::Music => sound_state.volume_settings.music = volume,
        }
    });
}

pub fn adjust_master_volume(delta: f32) {
    SOUND_STATE_ATOM.mutate(move |sound_state| {
        sound_state.volume_settings.master = clamp01(sound_state.volume_settings.master + delta);
    });
}

pub fn adjust_group_volume(group: SoundGroup, delta: f32) {
    SOUND_STATE_ATOM.mutate(move |sound_state| match group {
        SoundGroup::Sfx => {
            sound_state.volume_settings.sfx = clamp01(sound_state.volume_settings.sfx + delta)
        }
        SoundGroup::Ui => {
            sound_state.volume_settings.ui = clamp01(sound_state.volume_settings.ui + delta)
        }
        SoundGroup::Ambient => {
            sound_state.volume_settings.ambient =
                clamp01(sound_state.volume_settings.ambient + delta)
        }
        SoundGroup::Music => {
            sound_state.volume_settings.music = clamp01(sound_state.volume_settings.music + delta)
        }
    });
}
