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
use super::types::{
    CharacterPoseEmotion, Clip, ImageFilenameObject, MutableClip, Sequence, TimePerPixel,
};
use crate::app::editor::clip_editor::ClipEditorProps;
mod clip_editor;
mod events;
mod job;

pub struct EditorProps {
    pub screen_wh: namui::Wh<f32>,
}

pub struct Editor {
    job: Option<Job>,
    timeline: Timeline,
    clip_editor: Option<ClipEditor>,
    playback_time: chrono::Duration,
    image_filename_objects: Vec<ImageFilenameObject>,
    pub selected_clip_id: Option<String>,
    pub sequence: Sequence,
}

impl namui::Entity for Editor {
    type Props = EditorProps;
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
                    self.selected_clip_id = Some(clip_id.clone());
                    self.clip_editor =
                        Some(ClipEditor::new(&self.sequence.get_clip(clip_id).unwrap()));
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
                    self.selected_clip_id = Some(clip_id.clone());
                    self.clip_editor =
                        Some(ClipEditor::new(&self.sequence.get_clip(clip_id).unwrap()));
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
                EditorEvent::TimelineMoveEvent { pixel } => {
                    self.timeline.start_at += pixel * self.timeline.time_per_pixel;
                }
                EditorEvent::TimelineZoomEvent {
                    delta,
                    anchor_x_in_timeline,
                } => {
                    let zoom_by_wheel = |target: &f32, delta: &f32| -> f32 {
                        const STEP: f32 = 400.0;
                        const MIN: f32 = 10.0;
                        const MAX: f32 = 1000.0;

                        let wheel = STEP * (target / 10.0).log2();

                        let next_wheel = wheel + delta;

                        let zoomed = num::clamp(10.0 * 2.0f32.powf(next_wheel / STEP), MIN, MAX);
                        zoomed
                    };
                    let time_of_mouse_position = self.timeline.start_at
                        + anchor_x_in_timeline * self.timeline.time_per_pixel;

                    let next_ms_per_pixel =
                        zoom_by_wheel(&self.timeline.time_per_pixel.ms_per_pixel(), delta);
                    let next_time_per_pixel = TimePerPixel::from_ms_per_pixel(&next_ms_per_pixel);

                    let next_start_at =
                        time_of_mouse_position - anchor_x_in_timeline * next_time_per_pixel;

                    self.timeline.time_per_pixel = next_time_per_pixel;
                    self.timeline.start_at = next_start_at;
                }
                EditorEvent::ImageBrowserSelectEvent { selected_item } => {
                    match selected_item {
                        clip_editor::camera_clip_editor::image_browser::ImageBrowserItem::CharacterPoseEmotion(character, pose, emotion) => {
                            let selected_clip = self
                                .selected_clip_id
                                .as_ref()
                                .and_then(|id| self.sequence.get_mut_clip(&id));
                            if let Some(MutableClip::Camera(camera_clip)) = selected_clip {
                                camera_clip.camera_angle.character_pose_emotion = CharacterPoseEmotion(character.clone(), pose.clone(), emotion.clone());
                            } else {
                                unreachable!();
                            }
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        } else if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match event {
                NamuiEvent::MouseMove(mouse_event) => match self.job {
                    Some(Job::MoveCameraClip(ref mut job)) => {
                        job.last_global_mouse_xy = mouse_event.xy;
                    }
                    Some(Job::MoveSubtitleClip(ref mut job)) => {
                        job.last_global_mouse_xy = mouse_event.xy;
                    }
                    Some(Job::WysiwygMoveImage(ref mut job)) => {
                        job.last_global_mouse_xy = mouse_event.xy;
                    }
                    Some(Job::WysiwygResizeImage(ref mut job)) => {
                        job.last_global_mouse_xy = mouse_event.xy;
                    }
                    Some(Job::WysiwygCropImage(ref mut job)) => {
                        job.last_global_mouse_xy = mouse_event.xy;
                    }
                    _ => {}
                },
                NamuiEvent::MouseUp(mouse_event) => {
                    let job = self.job.clone();
                    match job {
                        // TODO : Make these simple using trait
                        Some(Job::MoveCameraClip(mut job)) => {
                            job.last_global_mouse_xy = mouse_event.xy;
                            job.execute(self);
                            self.job = None;
                        }
                        Some(Job::MoveSubtitleClip(mut job)) => {
                            job.last_global_mouse_xy = mouse_event.xy;
                            job.execute(self);
                            self.job = None;
                        }
                        Some(Job::WysiwygMoveImage(mut job)) => {
                            job.last_global_mouse_xy = mouse_event.xy;
                            job.execute(self);
                            self.job = None;
                        }
                        Some(Job::WysiwygResizeImage(mut job)) => {
                            job.last_global_mouse_xy = mouse_event.xy;
                            job.execute(self);
                            self.job = None;
                        }
                        Some(Job::WysiwygCropImage(mut job)) => {
                            job.last_global_mouse_xy = mouse_event.xy;
                            job.execute(self);
                            self.job = None;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        };
        self.clip_editor
            .as_mut()
            .map(|clip_editor| clip_editor.update(event));
    }

    fn render(&self, props: &Self::Props) -> namui::RenderingTree {
        let selected_clip = self
            .selected_clip_id
            .as_ref()
            .and_then(|id| self.sequence.get_clip(&id));
        render![
            self.timeline.render(&TimelineProps {
                playback_time: self.playback_time,
                xywh: self.calculate_timeline_xywh(&props.screen_wh),
                job: &self.job,
                selected_clip_id: &self.selected_clip_id,
                sequence: &self.sequence,
            }),
            match (selected_clip, &self.clip_editor) {
                (None, None) => RenderingTree::Empty,
                (Some(clip), Some(clip_editor)) => {
                    clip_editor.render(&ClipEditorProps {
                        clip,
                        xywh: XywhRect {
                            x: 0.0,
                            y: 0.0,
                            width: 800.0,
                            height: props.screen_wh.height - 200.0,
                        },
                        image_filename_objects: &self.image_filename_objects,
                        job: &self.job,
                    })
                }
                (None, Some(_)) => unreachable!("clip_editor is Some but selected_clip is None"),
                (Some(_), None) => unreachable!("selected_clip is Some but clip_editor is None"),
            },
        ]
    }
}

impl Editor {
    pub fn new(socket: Socket, sequence: Sequence) -> Self {
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

                        namui::event::send(EditorEvent::ImageFilenameObjectsUpdatedEvent {
                            image_filename_objects,
                        })
                    }
                    Err(error) => namui::log(format!("error on get_camera_shot_urls: {:?}", error)),
                }
            }
        });
        Self {
            timeline: Timeline::new(),
            playback_time: chrono::Duration::zero(),
            image_filename_objects: vec![],
            job: None,
            clip_editor: None,
            selected_clip_id: None,
            sequence,
        }
    }
    fn calculate_timeline_xywh(&self, screen_wh: &namui::Wh<f32>) -> XywhRect<f32> {
        XywhRect {
            x: 0.0,
            y: screen_wh.height - 200.0,
            width: screen_wh.width,
            height: 200.0,
        }
    }
    fn get_selected_clip(&self) -> Option<Clip> {
        self.selected_clip_id
            .as_ref()
            .and_then(|id| self.sequence.get_clip(&id))
    }
}
