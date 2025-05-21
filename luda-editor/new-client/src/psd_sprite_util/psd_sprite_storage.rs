use namui::*;
use psd_sprite::{decode_psd_sprite_from_bytes, PsdSprite, SpriteLoadedImages};
use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, RwLock},
};

lazy_static! {
    static ref PSD_SPRITE_STORAGE: PsdSpriteStorage = PsdSpriteStorage::new();
}

pub fn get_or_load_psd_sprite(sprite_id: u128) -> Arc<PsdSpriteLoadState> {
    let Some(load_state) = PSD_SPRITE_STORAGE.get(sprite_id) else {
        let loading = PSD_SPRITE_STORAGE.set(sprite_id, PsdSpriteLoadState::Loading);
        spawn(async move {
            let psd_bytes = namui::file::bundle::read("test.psd").await.unwrap();
            let decode_result = decode_psd_sprite_from_bytes(&psd_bytes).await;

            let load_state = decode_result.map_or_else(
                |err| PsdSpriteLoadState::Error(err.into()),
                |(psd_sprite, loaded_images)| PsdSpriteLoadState::Loaded {
                    psd_sprite: Arc::new(psd_sprite),
                    loaded_images: Arc::new(loaded_images),
                },
            );
            PSD_SPRITE_STORAGE.set(sprite_id, load_state);
        });
        return loading;
    };
    load_state
}

pub enum PsdSpriteLoadState {
    Loading,
    Loaded {
        psd_sprite: Arc<PsdSprite>,
        loaded_images: Arc<SpriteLoadedImages>,
    },
    #[allow(unused)]
    Error(Box<dyn Error + Send + Sync>),
}
struct PsdSpriteStorage {
    storage: RwLock<HashMap<u128, Arc<PsdSpriteLoadState>>>,
}
impl PsdSpriteStorage {
    fn new() -> Self {
        Self {
            storage: RwLock::new(HashMap::new()),
        }
    }
    fn get(&self, sprite_id: u128) -> Option<Arc<PsdSpriteLoadState>> {
        self.storage.read().unwrap().get(&sprite_id).cloned()
    }
    fn set(&self, sprite_id: u128, load_state: PsdSpriteLoadState) -> Arc<PsdSpriteLoadState> {
        let load_state = Arc::new(load_state);
        self.storage
            .write()
            .unwrap()
            .insert(sprite_id, load_state.clone());
        load_state
    }
}
