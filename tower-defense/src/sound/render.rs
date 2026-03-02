use super::{
    SoundGroup, SpatialMode, cleanup_expired_sounds, state::active_sounds, use_sound_state,
};
use crate::game_state::{TILE_PX_SIZE, use_game_state};
use namui::*;

const AUDIO_GROUPS: [SoundGroup; 4] = [
    SoundGroup::Sfx,
    SoundGroup::Ui,
    SoundGroup::Ambient,
    SoundGroup::Music,
];

pub struct SoundRenderer;

impl Component for SoundRenderer {
    fn render(self, ctx: &RenderCtx) {
        let game_state = use_game_state(ctx);
        let sound_state = use_sound_state(ctx);
        let active_sounds = active_sounds();
        let now = Instant::now();

        if active_sounds.iter().any(|sound| sound.is_expired(now)) {
            cleanup_expired_sounds(now);
        }

        let volume_settings = sound_state.volume_settings.clone();

        ctx.add(AudioGroup {
            volume: volume_settings.master,
            z: 0.0,
            children: move |ctx: ComposeCtx| {
                render_spatial_sounds(&ctx, game_state.as_ref(), &active_sounds, &volume_settings);
                render_non_spatial_sounds(&ctx, &active_sounds, &volume_settings);
            },
        });
    }
}

fn render_spatial_sounds(
    ctx: &ComposeCtx,
    game_state: &crate::game_state::GameState,
    active_sounds: &[super::event::SoundEvent],
    volume_settings: &super::volume::VolumeSettings,
) {
    let camera = &game_state.camera;
    let visual_left_top = camera.visual_left_top();
    let final_offset = TILE_PX_SIZE.to_xy() * visual_left_top * -1.0;
    let zoom_level = camera.zoom_level;

    ctx.compose(|ctx| {
        let world_ctx = ctx.scale(Xy::single(zoom_level)).translate(final_offset);

        let screen_wh = screen::size();
        let half_screen_in_tile = Xy::new(
            screen_wh.width.as_i32().as_f32() / (2.0 * zoom_level * TILE_PX_SIZE.width.as_f32()),
            screen_wh.height.as_i32().as_f32() / (2.0 * zoom_level * TILE_PX_SIZE.height.as_f32()),
        );
        let listener_tile_xy = visual_left_top + half_screen_in_tile;
        let listener_px_xy = TILE_PX_SIZE.to_xy() * listener_tile_xy;

        world_ctx.translate(listener_px_xy).add(AudioGroup {
            volume: 1.0,
            z: (1.0 / zoom_level - 1.0) * 500.0,
            children: |ctx: ComposeCtx| {
                ctx.add(AudioListener);
            },
        });

        for group in AUDIO_GROUPS {
            let subgroup_volume = volume_settings.subgroup_volume(group);
            world_ctx.add(AudioGroup {
                volume: subgroup_volume,
                z: 0.0,
                children: |ctx: ComposeCtx| {
                    for sound in active_sounds.iter().filter(|sound| sound.group == group) {
                        let SpatialMode::Spatial { position } = &sound.spatial else {
                            continue;
                        };

                        ctx.compose_with_key(sound.id as u128, |ctx| {
                            ctx.translate(TILE_PX_SIZE.to_xy() * *position)
                                .add(AudioGroup {
                                    volume: sound.volume_preset.as_f32(),
                                    z: 0.0,
                                    children: |ctx: ComposeCtx| {
                                        ctx.add(Audio {
                                            asset: sound.asset,
                                            repeat: sound.repeat,
                                            spatial: true,
                                        });
                                    },
                                });
                        });
                    }
                },
            });
        }
    });
}

fn render_non_spatial_sounds(
    ctx: &ComposeCtx,
    active_sounds: &[super::event::SoundEvent],
    volume_settings: &super::volume::VolumeSettings,
) {
    for group in AUDIO_GROUPS {
        let subgroup_volume = volume_settings.subgroup_volume(group);

        ctx.add(AudioGroup {
            volume: subgroup_volume,
            z: 0.0,
            children: |ctx: ComposeCtx| {
                for sound in active_sounds.iter().filter(|sound| sound.group == group) {
                    let SpatialMode::NonSpatial = sound.spatial else {
                        continue;
                    };

                    ctx.compose_with_key(sound.id as u128, |ctx| {
                        ctx.add(AudioGroup {
                            volume: sound.volume_preset.as_f32(),
                            z: 0.0,
                            children: |ctx: ComposeCtx| {
                                ctx.add(Audio {
                                    asset: sound.asset,
                                    repeat: sound.repeat,
                                    spatial: false,
                                });
                            },
                        });
                    });
                }
            },
        });
    }
}
