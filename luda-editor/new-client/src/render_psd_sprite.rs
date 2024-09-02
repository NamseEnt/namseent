use super::*;
use luda_rpc::SceneSprite;
use psd_sprite::{decode_psd_sprite_from_bytes, PsdSprite, SpriteLoadedImages};
use psd_sprite_render::RenderPsdSprite;
use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, RwLock},
};

lazy_static! {
    static ref PSD_SPRITE_STORAGE: PsdSpriteStorage = PsdSpriteStorage::new();
}

pub fn render_psd_sprite(ctx: &RenderCtx, scene_sprite: &SceneSprite, screen_wh: Wh<Px>) {
    let Some(sprite_id) = &scene_sprite.sprite_id else {
        return;
    };

    let Some(load_state) = PSD_SPRITE_STORAGE.try_get(sprite_id) else {
        let sprite_id = sprite_id.clone();
        PSD_SPRITE_STORAGE.set(sprite_id.clone(), PsdSpriteLoadState::Loading);
        ctx.spawn(async move {
            namui::log!("Loading PSD sprite: {}", sprite_id);
            // TODO: Load PSD sprite from the server and cache.
            let psd_bytes = namui::file::bundle::read("test.psd").await.unwrap();
            let decode_result = decode_psd_sprite_from_bytes(&psd_bytes, "test.psd").await;

            let load_state = decode_result.map_or_else(
                |err| PsdSpriteLoadState::Error(err.into()),
                |(psd_sprite, loaded_images)| PsdSpriteLoadState::Loaded {
                    psd_sprite: Arc::new(psd_sprite),
                    loaded_images: Arc::new(loaded_images),
                },
            );
            PSD_SPRITE_STORAGE.set(sprite_id.clone(), load_state);
        });
        return;
    };

    let PsdSpriteLoadState::Loaded {
        psd_sprite,
        loaded_images,
    } = load_state.as_ref()
    else {
        return;
    };

    ctx.add_with_key(
        format!("{scene_sprite:?}"),
        RenderPsdSprite {
            psd_sprite: psd_sprite.clone(),
            scene_sprite,
            loaded_images: loaded_images.clone(),
            screen_wh,
        },
    );
}

enum PsdSpriteLoadState {
    Loading,
    Loaded {
        psd_sprite: Arc<PsdSprite>,
        loaded_images: Arc<SpriteLoadedImages>,
    },
    #[allow(unused)]
    Error(Box<dyn Error + Send + Sync>),
}

struct PsdSpriteStorage {
    storage: RwLock<HashMap<String, Arc<PsdSpriteLoadState>>>,
}
impl PsdSpriteStorage {
    fn new() -> Self {
        Self {
            storage: RwLock::new(HashMap::new()),
        }
    }
    fn try_get(&self, sprite_id: &str) -> Option<Arc<PsdSpriteLoadState>> {
        self.storage.read().unwrap().get(sprite_id).cloned()
    }
    fn set(&self, sprite_id: String, load_state: PsdSpriteLoadState) {
        self.storage
            .write()
            .unwrap()
            .insert(sprite_id, Arc::new(load_state));
    }
}
