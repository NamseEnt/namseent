use crate::{
    engine::{self, XywhRect},
    render,
};
mod main;
pub use main::main;
mod timeline;
pub use timeline::*;

struct Editor {
    timeline: Timeline,
    // clip_editor: ClipEditor,
    playback_time: chrono::Duration,
}

impl engine::Entity for Editor {
    type Props = ();
    fn update(&mut self, event: &dyn std::any::Any) {}
    fn render(&self, props: &Self::Props) -> engine::RenderingTree {
        let a = vec![1, 2, 3];
        // let selected_clip = self.timeline.selected_clip;
        render![
            // self.clip_editor
            //     .render(&ClipEditorProps { selected_clip }),
            self.timeline.render(&TimelineProps {
                playback_time: self.playback_time,
            }),
        ]
    }
}

pub struct Clip {}

struct ClipEditor {}

struct ClipEditorProps {
    selected_clip: Option<Clip>,
}

impl engine::Entity for ClipEditor {
    type Props = ClipEditorProps;
    fn update(&mut self, event: &dyn std::any::Any) {}
    fn render(&self, props: &Self::Props) -> engine::RenderingTree {
        todo!()
    }
}
impl Editor {
    fn new(screen_wh: engine::Wh<f32>) -> Self {
        Self {
            timeline: Timeline::new(Editor::calculate_timeline_xywh(screen_wh)),
            playback_time: chrono::Duration::zero(),
        }
    }
    fn resize(&mut self, wh: engine::Wh<f32>) {
        self.timeline.resize(Editor::calculate_timeline_xywh(wh));
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
