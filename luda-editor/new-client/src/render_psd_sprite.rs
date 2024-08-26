use super::*;
use luda_rpc::SceneSprite;
use psd_sprite::PsdSprite;
use psd_sprite_render::RenderPsdSprite;
use std::{collections::HashMap, error::Error};

static PSD_SPRITE_LOAD_STATE_ATOM: Atom<HashMap<String, PsdSpriteLoadState>> =
    Atom::uninitialized();

pub fn render_psd_sprite(ctx: &RenderCtx, scene_sprite: &SceneSprite, screen_wh: Wh<Px>) {
    let (psd_sprite_load_state_map, set_psd_sprite_load_state_map) =
        ctx.init_atom(&PSD_SPRITE_LOAD_STATE_ATOM, HashMap::new);

    let Some(sprite_id) = &scene_sprite.sprite_id else {
        return;
    };

    let Some(load_state) = psd_sprite_load_state_map.get(sprite_id) else {
        let sprite_id = sprite_id.clone();
        set_psd_sprite_load_state_map.mutate({
            let sprite_id = sprite_id.clone();
            move |map| {
                map.insert(sprite_id, PsdSpriteLoadState::Loading);
            }
        });
        ctx.spawn(async move {
            // TODO: Load PSD sprite from the server.
            let psd_bytes = namui::file::bundle::read("test.psd").await.unwrap();
            let psd_sprite = PsdSprite::from_psd_bytes(&psd_bytes);

            match psd_sprite {
                Ok(psd_sprite) => {
                    set_psd_sprite_load_state_map.mutate(move |map| {
                        map.insert(sprite_id, PsdSpriteLoadState::Loaded(psd_sprite));
                    });
                }
                Err(err) => {
                    set_psd_sprite_load_state_map.mutate(move |map| {
                        map.insert(sprite_id, PsdSpriteLoadState::Error(err.into()));
                    });
                }
            }
        });
        return;
    };

    let PsdSpriteLoadState::Loaded(psd_sprite) = load_state else {
        return;
    };

    psd_sprite.render(ctx, scene_sprite, screen_wh);
}

enum PsdSpriteLoadState {
    Loading,
    Loaded(PsdSprite),
    #[allow(unused)]
    Error(Box<dyn Error + Send + Sync>),
}
