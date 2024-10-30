mod audio_select_tool;

use luda_rpc::{AssetDoc, Scene, SceneSound};
use namui::*;
use namui_prebuilt::*;
use std::collections::HashMap;

pub struct SceneAudioEditor<'a> {
    pub wh: Wh<Px>,
    pub scene: &'a Scene,
    pub update_scene: &'a dyn Fn(Scene),
    pub asset_docs: Sig<'a, HashMap<String, AssetDoc>>,
}

impl Component for SceneAudioEditor<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            scene,
            update_scene,
            asset_docs,
        } = self;

        let select_audio = |audio: Option<SceneSound>| {
            let mut scene = scene.clone();
            scene.bgm = audio;
            update_scene(scene);
        };

        ctx.compose(|ctx| {
            table::vertical([table::ratio(1, |wh, ctx| {
                ctx.add(audio_select_tool::AudioSelectTool {
                    wh,
                    asset_docs: asset_docs.clone(),
                    selected_audio: &scene.bgm,
                    select_audio: &select_audio,
                });
            })])(wh, ctx)
        });
    }
}
