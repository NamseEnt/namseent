use namui::*;
use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, RwLock},
};

lazy_static! {
    static ref AUDIO_STORAGE: AudioStorage = AudioStorage::new();
}

pub fn get_or_load_audio(audio_id: u128) -> Arc<AudioLoadState> {
    let Some(load_state) = AUDIO_STORAGE.get(audio_id) else {
        let loading = AUDIO_STORAGE.set(audio_id.clone(), AudioLoadState::Loading);
        spawn(async move {
            let audio_id = audio_id.clone();

            let request = match network::http::Request::get(audio_url(audio_id)).body(()) {
                Ok(response) => response,
                Err(error) => {
                    AUDIO_STORAGE.set(audio_id, AudioLoadState::Error(error.into()));
                    return;
                }
            };
            let bytes = match request.send().await {
                Ok(response) => response.bytes(),
                Err(error) => {
                    AUDIO_STORAGE.set(audio_id, AudioLoadState::Error(error.into()));
                    return;
                }
            };
            let bytes = match bytes.await {
                Ok(bytes) => bytes,
                Err(error) => {
                    AUDIO_STORAGE.set(audio_id, AudioLoadState::Error(error.into()));
                    return;
                }
            };
            let load_state = match Audio::from_ogg_opus_bytes(bytes) {
                Ok(audio) => AudioLoadState::Loaded { audio },
                Err(error) => AudioLoadState::Error(error.into()),
            };

            AUDIO_STORAGE.set(audio_id, load_state);
        });
        return loading;
    };
    load_state
}

pub enum AudioLoadState {
    Loading,
    Loaded {
        audio: Audio,
    },
    #[allow(unused)]
    Error(Box<dyn Error + Send + Sync>),
}
struct AudioStorage {
    storage: RwLock<HashMap<u128, Arc<AudioLoadState>>>,
}
impl AudioStorage {
    fn new() -> Self {
        Self {
            storage: RwLock::new(HashMap::new()),
        }
    }
    fn get(&self, sprite_id: u128) -> Option<Arc<AudioLoadState>> {
        self.storage.read().unwrap().get(&sprite_id).cloned()
    }
    fn set(&self, sprite_id: u128, load_state: AudioLoadState) -> Arc<AudioLoadState> {
        let load_state = Arc::new(load_state);
        self.storage
            .write()
            .unwrap()
            .insert(sprite_id, load_state.clone());
        load_state
    }
}

fn audio_url(audio_id: u128) -> String {
    const PREFIX: &str = "http://localhost:4566/visual-novel-asset/audio/after-transcode";
    format!("{PREFIX}/{audio_id}")
}
