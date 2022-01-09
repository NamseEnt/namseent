mod timeline;
use luda_editor_rpc::Socket;
use namui::prelude::*;
pub use timeline::*;
use wasm_bindgen_futures::spawn_local;
mod types;
use self::{
    clip_editor::ClipEditor,
    events::*,
    job::{
        Job, MoveCameraClipJob, MoveSubtitleClipJob, WysiwygCropImageJob, WysiwygMoveImageJob,
        WysiwygResizeImageJob,
    },
};
use super::types::{ImageFilenameObject, Sequence};
use crate::app::editor::clip_editor::ClipEditorProps;
mod clip_editor;
mod events;
mod job;

pub struct Editor {
    job: Option<Job>,
    timeline: Timeline,
    clip_editor: ClipEditor,
    playback_time: chrono::Duration,
    screen_wh: namui::Wh<f32>,
    image_filename_objects: Vec<ImageFilenameObject>,
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
                    if self.job.is_none() {
                        self.job = Some(Job::MoveCameraClip(MoveCameraClipJob {
                            clip_id: clip_id.clone(),
                            click_anchor_in_global: *global_mouse_xy,
                            last_global_mouse_xy: *global_mouse_xy,
                        }));
                    }
                    self.timeline.selected_clip_id = Some(clip_id.clone());
                }
                EditorEvent::SubtitleClipHeadMouseDownEvent {
                    clip_id,
                    global_mouse_xy,
                    ..
                } => {
                    if self.job.is_none() {
                        self.job = Some(Job::MoveSubtitleClip(MoveSubtitleClipJob {
                            clip_id: clip_id.clone(),
                            click_anchor_in_global: *global_mouse_xy,
                            last_global_mouse_xy: *global_mouse_xy,
                        }));
                    }
                    self.timeline.selected_clip_id = Some(clip_id.clone());
                }
                EditorEvent::ImageFilenameObjectsUpdatedEvent {
                    image_filename_objects,
                } => {
                    self.image_filename_objects = image_filename_objects.to_vec();
                }
                EditorEvent::WysiwygEditorInnerImageMouseDownEvent {
                    mouse_xy,
                    container_size,
                } => {
                    if self.job.is_none() {
                        self.job = Some(Job::WysiwygMoveImage(WysiwygMoveImageJob {
                            start_global_mouse_xy: *mouse_xy,
                            last_global_mouse_xy: *mouse_xy,
                            container_size: *container_size,
                        }));
                    };
                }
                EditorEvent::WysiwygEditorResizerHandleMouseDownEvent {
                    mouse_xy,
                    handle,
                    center_xy,
                    container_size,
                    image_size_ratio,
                } => {
                    if self.job.is_none() {
                        self.job = Some(Job::WysiwygResizeImage(WysiwygResizeImageJob {
                            start_global_mouse_xy: *mouse_xy,
                            last_global_mouse_xy: *mouse_xy,
                            handle: *handle,
                            center_xy: *center_xy,
                            container_size: *container_size,
                            image_size_ratio: *image_size_ratio,
                        }));
                    };
                }
                EditorEvent::WysiwygEditorCropperHandleMouseDownEvent {
                    mouse_xy,
                    handle,
                    container_size,
                } => {
                    if self.job.is_none() {
                        self.job = Some(Job::WysiwygCropImage(WysiwygCropImageJob {
                            start_global_mouse_xy: *mouse_xy,
                            last_global_mouse_xy: *mouse_xy,
                            handle: handle.clone(),
                            container_size: *container_size,
                        }));
                    };
                }
                _ => {}
            }
        } else if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match event {
                NamuiEvent::MouseMove(global_xy) => match self.job {
                    Some(Job::MoveCameraClip(ref mut job)) => {
                        job.last_global_mouse_xy = *global_xy;
                    }
                    Some(Job::MoveSubtitleClip(ref mut job)) => {
                        job.last_global_mouse_xy = *global_xy;
                    }
                    Some(Job::WysiwygMoveImage(ref mut job)) => {
                        job.last_global_mouse_xy = *global_xy;
                    }
                    Some(Job::WysiwygResizeImage(ref mut job)) => {
                        job.last_global_mouse_xy = *global_xy;
                    }
                    Some(Job::WysiwygCropImage(ref mut job)) => {
                        job.last_global_mouse_xy = *global_xy;
                    }
                    _ => {}
                },
                NamuiEvent::MouseUp(global_xy) => {
                    let job = self.job.clone();
                    match job {
                        Some(Job::MoveCameraClip(mut job)) => {
                            job.last_global_mouse_xy = *global_xy;
                            job.execute(&mut self.timeline);
                            self.job = None;
                        }
                        Some(Job::MoveSubtitleClip(mut job)) => {
                            job.last_global_mouse_xy = *global_xy;
                            job.execute(&mut self.timeline);
                            self.job = None;
                        }
                        Some(Job::WysiwygMoveImage(mut job)) => {
                            job.last_global_mouse_xy = *global_xy;
                            job.execute(&mut self.timeline);
                            self.job = None;
                        }
                        Some(Job::WysiwygResizeImage(mut job)) => {
                            job.last_global_mouse_xy = *global_xy;
                            job.execute(&mut self.timeline);
                            self.job = None;
                        }
                        Some(Job::WysiwygCropImage(mut job)) => {
                            job.last_global_mouse_xy = *global_xy;
                            job.execute(&mut self.timeline);
                            self.job = None;
                        }
                        _ => {}
                    }
                }
                &namui::NamuiEvent::ScreenResize(wh) => {
                    self.screen_wh = namui::Wh {
                        width: wh.width as f32,
                        height: wh.height as f32,
                    };
                }
                _ => {}
            }
        };
        self.clip_editor.update(event);
    }
    fn render(&self, _: &Self::Props) -> namui::RenderingTree {
        let selected_clip = self
            .timeline
            .selected_clip_id
            .as_ref()
            .and_then(|id| self.timeline.sequence.get_clip(&id));
        render![
            self.timeline.render(&TimelineProps {
                playback_time: self.playback_time,
                xywh: self.calculate_timeline_xywh(),
                job: &self.job,
            }),
            self.clip_editor.render(&ClipEditorProps {
                selected_clip,
                xywh: XywhRect {
                    x: 0.0,
                    y: 0.0,
                    width: 800.0,
                    height: self.screen_wh.height - 200.0,
                },
                image_filename_objects: &self.image_filename_objects,
                job: &self.job,
            }),
        ]
    }
}

impl Editor {
    pub fn new(screen_wh: namui::Wh<f32>, socket: Socket, sequence: Sequence) -> Self {
        spawn_local({
            let socket = socket.clone();
            async move {
                let result = socket
                    .get_camera_shot_urls(luda_editor_rpc::get_camera_shot_urls::Request {})
                    .await;
                match result {
                    Ok(response) => {
                        let image_filename_objects = response
                            .camera_shot_urls
                            .iter()
                            .map(|url| ImageFilenameObject::new(url))
                            .collect();

                        namui::event::send(Box::new(
                            EditorEvent::ImageFilenameObjectsUpdatedEvent {
                                image_filename_objects,
                            },
                        ))
                    }
                    Err(error) => namui::log(format!("error on get_camera_shot_urls: {:?}", error)),
                }
            }
        });
        Self {
            timeline: Timeline::new(sequence),
            clip_editor: ClipEditor::new(),
            playback_time: chrono::Duration::zero(),
            screen_wh,
            image_filename_objects: vec![],
            job: None,
        }
    }
    fn calculate_timeline_xywh(&self) -> XywhRect<f32> {
        XywhRect {
            x: 0.0,
            y: self.screen_wh.height - 200.0,
            width: self.screen_wh.width,
            height: 200.0,
        }
    }
}
