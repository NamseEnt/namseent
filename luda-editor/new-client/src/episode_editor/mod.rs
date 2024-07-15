use super::*;

pub struct EpisodeEditor;

impl Component for EpisodeEditor {
    fn render(self, ctx: &RenderCtx) {
        let Self {} = self;

        let screen_list = table::fixed(160.px(), |wh, ctx| {});
        let scene_editor = table::ratio(1, |wh, ctx| {});
        let properties_panel = table::ratio(1, |wh, ctx| {});

        let screen_wh = namui::screen::size().map(|x| x.into_px());
        ctx.compose(|ctx| {
            horizontal([screen_list, scene_editor, properties_panel])(screen_wh, ctx)
        });
    }
}

/*
Partial Version

Full Version

struct FullEpisode {
    name: String,
    scenes: Vec<Scene>,
}
*/
