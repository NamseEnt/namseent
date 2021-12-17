mod main;
pub use main::main;
mod timeline;
use namui::prelude::*;
pub use timeline::*;
mod types;
use self::{
    events::*,
    job::{Job, MoveCameraClipJob},
};
use types::*;
mod events;
mod job;

struct Editor {
    timeline: Timeline,
    // clip_editor: ClipEditor,
    playback_time: chrono::Duration,
}

impl namui::Entity for Editor {
    type Props = ();
    fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<EditorEvent>() {
            match event {
                EditorEvent::CameraClipBodyMouseDownEvent {
                    clip_id,
                    global_mouse_xy,
                    ..
                } => {
                    if self.timeline.job.is_none() {
                        self.timeline.job = Some(Job::MoveCameraClip(MoveCameraClipJob {
                            clip_id: clip_id.clone(),
                            click_anchor_in_global: *global_mouse_xy,
                            last_global_mouse_xy: *global_mouse_xy,
                        }));
                    }
                    self.timeline.selected_clip_id = Some(clip_id.clone());
                }
            }
        } else if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match event {
                NamuiEvent::MouseMove(global_xy) => match self.timeline.job {
                    Some(Job::MoveCameraClip(ref mut job)) => {
                        job.last_global_mouse_xy = *global_xy;
                    }
                    _ => {}
                },
                NamuiEvent::MouseUp(global_xy) => {
                    let job = self.timeline.job.clone();
                    match job {
                        Some(Job::MoveCameraClip(mut job)) => {
                            job.last_global_mouse_xy = *global_xy;
                            job.execute(&mut self.timeline);
                            self.timeline.job = None;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        };
    }
    fn render(&self, props: &Self::Props) -> namui::RenderingTree {
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

struct ClipEditor {}

struct ClipEditorProps {
    selected_clip: Option<Clip>,
}

impl namui::Entity for ClipEditor {
    type Props = ClipEditorProps;
    fn update(&mut self, event: &dyn std::any::Any) {}
    fn render(&self, props: &Self::Props) -> namui::RenderingTree {
        todo!()
    }
}
impl Editor {
    fn new(screen_wh: namui::Wh<f32>) -> Self {
        Self {
            timeline: Timeline::new(
                Editor::calculate_timeline_xywh(screen_wh),
                get_sample_sequence(),
            ),
            playback_time: chrono::Duration::zero(),
        }
    }
    fn resize(&mut self, wh: namui::Wh<f32>) {
        self.timeline.resize(Editor::calculate_timeline_xywh(wh));
    }
    fn calculate_timeline_xywh(wh: namui::Wh<f32>) -> XywhRect<f32> {
        XywhRect {
            x: 0.0,
            y: wh.height - 200.0,
            width: wh.width,
            height: 200.0,
        }
    }
}
