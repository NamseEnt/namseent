use super::*;
use luda_rpc::SceneSprite;
use psd_sprite::{decode_psd_sprite, PsdSprite};
use psd_sprite_render::RenderPsdSprite;
use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, RwLock},
};

lazy_static! {
    static ref PSD_SPRITE_LOAD_STATE: PsdSpriteStorage = PsdSpriteStorage::new();
}

pub fn render_psd_sprite(ctx: &RenderCtx, scene_sprite: &SceneSprite, screen_wh: Wh<Px>) {
    let Some(sprite_id) = &scene_sprite.sprite_id else {
        return;
    };

    let Some(load_state) = PSD_SPRITE_LOAD_STATE.try_get(sprite_id) else {
        let sprite_id = sprite_id.clone();
        PSD_SPRITE_LOAD_STATE.set(sprite_id.clone(), PsdSpriteLoadState::Loading);
        ctx.spawn(async move {
            namui::log!("Loading PSD sprite: {}", sprite_id);
            // TODO: Load PSD sprite from the server and cache.
            let psd_bytes = namui::file::bundle::read("test.psd").await.unwrap();
            let psd_sprite = PsdSprite::from_psd_bytes(&psd_bytes);
            let (encoded_psd_sprite, _parts_sprite) =
                psd_sprite::encode_psd_sprite(&psd_bytes, "test.psd").unwrap();
            let now = std::time::Instant::now();
            let (psd_sprite, loaded_images) =
                psd_sprite::decode_psd_sprite(futures_util::stream::iter(vec![Ok(
                    bytes::Bytes::copy_from_slice(&encoded_psd_sprite),
                )]))
                .await
                .unwrap();

            let load_state = psd_sprite.map_or_else(
                |err| PsdSpriteLoadState::Error(err.into()),
                |psd_sprite| PsdSpriteLoadState::Loaded {
                    psd_sprite: Arc::new(psd_sprite),
                    loaded_images: Arc::new(loaded_images),
                },
            );
            PSD_SPRITE_LOAD_STATE.set(sprite_id.clone(), load_state);
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

    psd_sprite.render(ctx, scene_sprite, loaded_images, screen_wh);
}

enum PsdSpriteLoadState {
    Loading,
    Loaded {
        psd_sprite: Arc<PsdSprite>,
        loaded_images: HashMap<SpriteImageId, SpriteLoadedImage>,
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
