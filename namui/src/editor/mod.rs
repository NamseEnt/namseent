use crate::{
    engine::{self, XywhRect},
    render,
};
mod main;
pub use main::main;
mod timeline;
pub use timeline::Timeline;

struct Editor {
    timeline: Timeline,
    // clip_editor: ClipEditor,
}

impl engine::Entity for Editor {
    type RenderingContext = ();
    fn update(&mut self, event: &dyn std::any::Any) {}
    fn render(&self, context: &Self::RenderingContext) -> engine::RenderingTree {
        // let selected_clip = self.timeline.selected_clip;
        render![
            // self.clip_editor
            //     .render(&ClipEditorRenderingContext { selected_clip }),
            self.timeline.render(&())
        ]
    }
}

pub struct Clip {}

struct ClipEditor {}

struct ClipEditorRenderingContext {
    selected_clip: Option<Clip>,
}

impl engine::Entity for ClipEditor {
    type RenderingContext = ClipEditorRenderingContext;
    fn update(&mut self, event: &dyn std::any::Any) {}
    fn render(&self, context: &Self::RenderingContext) -> engine::RenderingTree {
        todo!()
    }
}
impl Editor {
    fn new(screen_wh: engine::Wh<f32>) -> Self {
        Self {
            timeline: Timeline {
                xywh: Editor::calculate_timeline_xywh(screen_wh),
                selected_clip: None,
            },
        }
    }
    fn resize(&mut self, wh: engine::Wh<f32>) {
        self.timeline.xywh = Editor::calculate_timeline_xywh(wh);
    }
    fn calculate_timeline_xywh(wh: engine::Wh<f32>) -> XywhRect<f32> {
        XywhRect {
            x: 0.0,
            y: wh.height - 200.0,
            width: wh.width,
            height: 200.0,
        }
    }
}
