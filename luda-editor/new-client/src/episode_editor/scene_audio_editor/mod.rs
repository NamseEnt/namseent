mod audio_select_tool;
mod volume_tool;

use luda_rpc::{AssetDoc, Scene, SceneSound};
use namui::*;
use namui_prebuilt::*;
use std::collections::HashMap;

pub struct SceneAudioEditor<'a> {
    pub wh: Wh<Px>,
    pub scene: &'a Scene,
    pub update_scene: &'a dyn Fn(Scene),
    pub asset_docs: Sig<'a, HashMap<u128, AssetDoc>>,
}

impl Component for SceneAudioEditor<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            scene,
            update_scene,
            asset_docs,
        } = self;

        let set_audio = |audio: Option<SceneSound>| {
            let mut scene = scene.clone();
            scene.bgm = audio;
            update_scene(scene);
        };

        ctx.compose(|ctx| {
            table::vertical([
                table::fixed(64.px(), |wh, ctx| {
                    ctx.add(volume_tool::VolumeTool {
                        wh,
                        selected_audio: &scene.bgm,
                        set_audio: &set_audio,
                    });
                }),
                table::ratio(1, |wh, ctx| {
                    ctx.add(audio_select_tool::AudioSelectTool {
                        wh,
                        asset_docs: asset_docs.clone(),
                        selected_audio: &scene.bgm,
                        set_audio: &set_audio,
                    });
                }),
            ])(wh, ctx)
        });
    }
}
