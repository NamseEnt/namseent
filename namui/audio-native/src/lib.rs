use kira::{
    AudioManager, AudioManagerSettings, DefaultBackend, Tween,
    sound::static_sound::{StaticSoundData, StaticSoundHandle},
    track::{SpatialTrackBuilder, SpatialTrackHandle},
    listener::ListenerHandle,
    Decibels,
};
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::{LazyLock, Mutex};

/// Identity quaternion (no rotation): w=1, x=0, y=0, z=0.
fn identity_quat() -> mint::Quaternion<f32> {
    mint::Quaternion {
        s: 1.0,
        v: mint::Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    }
}

static MANAGER: LazyLock<Mutex<AudioManager<DefaultBackend>>> = LazyLock::new(|| {
    let manager = AudioManager::new(AudioManagerSettings::default())
        .expect("Failed to create audio manager");
    Mutex::new(manager)
});

static SOUND_DATA: LazyLock<Mutex<HashMap<usize, StaticSoundData>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

struct PlaybackEntry {
    sound_handle: StaticSoundHandle,
    spatial_track: Option<SpatialTrackHandle>,
}

static PLAYBACKS: LazyLock<Mutex<HashMap<usize, PlaybackEntry>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static LISTENER: LazyLock<Mutex<Option<ListenerHandle>>> =
    LazyLock::new(|| Mutex::new(None));

fn linear_to_decibels(volume: f32) -> Decibels {
    if volume <= 0.0 {
        Decibels::SILENCE
    } else {
        Decibels(20.0 * volume.log10())
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _register_audio(audio_id: usize, buffer_ptr: *const u8, buffer_len: usize) {
    let bytes = unsafe { std::slice::from_raw_parts(buffer_ptr, buffer_len) };
    let sound_data = StaticSoundData::from_cursor(Cursor::new(bytes.to_vec()))
        .unwrap_or_else(|e| panic!("Failed to decode audio {audio_id}: {e}"));
    SOUND_DATA.lock().unwrap().insert(audio_id, sound_data);
}

#[unsafe(no_mangle)]
pub extern "C" fn _audio_play(audio_id: usize, playback_id: usize, repeat: bool) {
    let data_map = SOUND_DATA.lock().unwrap();
    let Some(data) = data_map.get(&audio_id) else {
        eprintln!("audio_play: unknown audio_id {audio_id}");
        return;
    };
    let mut sound_data = data.clone();
    if repeat {
        sound_data = sound_data.loop_region(..);
    }

    let mut manager = MANAGER.lock().unwrap();
    match manager.play(sound_data) {
        Ok(handle) => {
            PLAYBACKS.lock().unwrap().insert(
                playback_id,
                PlaybackEntry {
                    sound_handle: handle,
                    spatial_track: None,
                },
            );
        }
        Err(e) => {
            eprintln!("audio_play: failed to play audio {audio_id}: {e}");
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _audio_play_spatial(audio_id: usize, playback_id: usize, repeat: bool) {
    let data_map = SOUND_DATA.lock().unwrap();
    let Some(data) = data_map.get(&audio_id) else {
        eprintln!("audio_play_spatial: unknown audio_id {audio_id}");
        return;
    };
    let mut sound_data = data.clone();
    if repeat {
        sound_data = sound_data.loop_region(..);
    }

    let mut manager = MANAGER.lock().unwrap();

    // Ensure listener exists
    let mut listener_guard = LISTENER.lock().unwrap();
    if listener_guard.is_none() {
        match manager.add_listener(
            mint::Vector3 {
                x: 0.0f32,
                y: 0.0,
                z: 0.0,
            },
            identity_quat(),
        ) {
            Ok(handle) => {
                *listener_guard = Some(handle);
            }
            Err(e) => {
                eprintln!("audio_play_spatial: failed to create listener: {e}");
                return;
            }
        }
    }
    let listener = listener_guard.as_ref().unwrap();

    // Create a spatial sub-track for this playback
    let spatial_track = manager.add_spatial_sub_track(
        listener,
        mint::Vector3 {
            x: 0.0f32,
            y: 0.0,
            z: 0.0,
        },
        SpatialTrackBuilder::new(),
    );

    match spatial_track {
        Ok(mut track) => match track.play(sound_data) {
            Ok(handle) => {
                PLAYBACKS.lock().unwrap().insert(
                    playback_id,
                    PlaybackEntry {
                        sound_handle: handle,
                        spatial_track: Some(track),
                    },
                );
            }
            Err(e) => {
                eprintln!("audio_play_spatial: failed to play audio {audio_id}: {e}");
            }
        },
        Err(e) => {
            eprintln!("audio_play_spatial: failed to create spatial track: {e}");
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _audio_playback_drop(playback_id: usize) {
    let mut playbacks = PLAYBACKS.lock().unwrap();
    if let Some(mut entry) = playbacks.remove(&playback_id) {
        entry.sound_handle.stop(Tween::default());
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _audio_playback_set_volume(playback_id: usize, volume: f32) {
    let mut playbacks = PLAYBACKS.lock().unwrap();
    if let Some(entry) = playbacks.get_mut(&playback_id) {
        entry
            .sound_handle
            .set_volume(linear_to_decibels(volume), Tween::default());
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _audio_playback_set_position(playback_id: usize, x: f32, y: f32, z: f32) {
    let mut playbacks = PLAYBACKS.lock().unwrap();
    if let Some(entry) = playbacks.get_mut(&playback_id) {
        if let Some(ref mut track) = entry.spatial_track {
            track.set_position([x, y, z], Tween::default());
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _audio_set_listener_position(x: f32, y: f32, z: f32) {
    let mut listener_guard = LISTENER.lock().unwrap();
    if let Some(ref mut listener) = *listener_guard {
        listener.set_position([x, y, z], Tween::default());
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _audio_set_volume(volume: f32) {
    let mut manager = MANAGER.lock().unwrap();
    manager
        .main_track()
        .set_volume(linear_to_decibels(volume), Tween::default());
}
