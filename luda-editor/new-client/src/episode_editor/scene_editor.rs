use luda_rpc::Scene;
use namui::*;
use namui_prebuilt::*;

pub struct SceneEditor<'a> {
    pub wh: Wh<Px>,
    pub scene: Option<&'a Scene>,
}

impl Component for SceneEditor<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, scene } = self;
    }
}

