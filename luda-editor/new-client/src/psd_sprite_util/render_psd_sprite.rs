use super::*;
use luda_rpc::SceneSprite;
use namui::*;
use psd_sprite_render::RenderPsdSprite;

pub fn render_psd_sprite(ctx: &RenderCtx, scene_sprite: &SceneSprite, screen_wh: Wh<Px>) {
    let Some(sprite_id) = &scene_sprite.sprite_id else {
        return;
    };

    let load_state = get_or_load_psd_sprite(*sprite_id);

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
