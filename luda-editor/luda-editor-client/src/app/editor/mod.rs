mod timeline;
use self::{clip_editor::ClipEditor, events::*, job::*};
use super::types::*;
use crate::app::editor::{clip_editor::ClipEditorProps, sequence_player::SequencePlayerProps};
use luda_editor_rpc::Socket;
use namui::prelude::*;
use std::sync::Arc;
pub use timeline::*;
use wasm_bindgen_futures::spawn_local;
mod clip_editor;
mod events;
mod job;
mod sequence_player;
use sequence_player::SequencePlayer;
mod history;
use history::History;

pub struct EditorProps {
    pub screen_wh: namui::Wh<f32>,
}

pub struct Editor {
    job: Option<Job>,
    timeline: Timeline,
    clip_editor: Option<ClipEditor>,
    image_filename_objects: Vec<ImageFilenameObject>,
    pub selected_clip_id: Option<String>,
    sequence_player: SequencePlayer,
    subtitle_play_duration_measurer: SubtitlePlayDurationMeasurer,
    history: History<Arc<Sequence>>,
}

impl namui::Entity for Editor {
    type Props = EditorProps;
    fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<EditorEvent>() {
            match event {
                EditorEvent::CameraClipBodyMouseDownEvent {
                    clip_id,
                    click_in_time,
                    ..
                } => {
                    if self.job.is_none() {
                        self.job = Some(Job::MoveCameraClip(MoveCameraClipJob {
                            clip_id: clip_id.clone(),
                            click_anchor_in_time: *click_in_time,
                            last_mouse_position_in_time: *click_in_time,
                        }));
                    }
                    self.selected_clip_id = Some(clip_id.clone());
                    self.clip_editor = Some(ClipEditor::new(
                        &self.get_sequence().get_clip(clip_id).unwrap(),
                    ));
                }
                EditorEvent::SubtitleClipHeadMouseDownEvent {
                    clip_id,
                    click_in_time,
                    ..
                } => {
                    if self.job.is_none() {
                        self.job = Some(Job::MoveSubtitleClip(MoveSubtitleClipJob {
                            clip_id: clip_id.clone(),
                            click_anchor_in_time: *click_in_time,
                            last_mouse_position_in_time: *click_in_time,
                        }));
                    }
                    self.selected_clip_id = Some(clip_id.clone());
                    self.clip_editor = Some(ClipEditor::new(
                        &self.get_sequence().get_clip(clip_id).unwrap(),
                    ));
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
                            clip_id: self.selected_clip_id.clone().unwrap(),
                            start_global_mouse_xy: *mouse_xy,
                            last_global_mouse_xy: *mouse_xy,
                            container_size: *container_size,
                        }));
                    };
                }
                // EditorEvent::WysiwygEditorResizerHandleMouseDownEvent {
                //     mouse_xy,
                //     handle,
                //     center_xy,
                //     container_size,
                //     image_size_ratio,
                // } => {
                //     if self.job.is_none() {
                //         self.job = Some(Job::WysiwygResizeImage(WysiwygResizeImageJob {
                //             start_global_mouse_xy: *mouse_xy,
                //             last_global_mouse_xy: *mouse_xy,
                //             handle: *handle,
                //             center_xy: *center_xy,
                //             container_size: *container_size,
                //             image_size_ratio: *image_size_ratio,
                //         }));
                //     };
                // }
                // EditorEvent::WysiwygEditorCropperHandleMouseDownEvent {
                //     mouse_xy,
                //     handle,
                //     container_size,
                // } => {
                //     if self.job.is_none() {
                //         self.job = Some(Job::WysiwygCropImage(WysiwygCropImageJob {
                //             start_global_mouse_xy: *mouse_xy,
                //             last_global_mouse_xy: *mouse_xy,
                //             handle: handle.clone(),
                //             container_size: *container_size,
                //         }));
                //     };
                // }
                EditorEvent::ImageBrowserSelectEvent { selected_item } => {
                    match selected_item {
                        clip_editor::camera_clip_editor::image_browser::ImageBrowserItem::CharacterPoseEmotion(character, pose, emotion) => {
                            todo!("Make it as job.");
                            // let selected_clip = self
                            //     .selected_clip_id
                            //     .as_ref()
                            //     .and_then(|id| self.sequence.get_mut_clip(&id));
                            // if let Some(MutableClip::Camera(camera_clip)) = selected_clip {
                            //     camera_clip.camera_angle.character_pose_emotion = CharacterPoseEmotion(character.clone(), pose.clone(), emotion.clone());
                            // } else {
                            //     unreachable!();
                            // }
                        },
                        _ => {}
                    }
                }
                EditorEvent::TimelineTimeRulerClickEvent {
                    click_position_in_time,
                } => {
                    self.sequence_player.seek(*click_position_in_time);
                }
                EditorEvent::TimelineBodyMouseMoveEvent {
                    mouse_position_in_time,
                } => match self.job {
                    Some(Job::MoveCameraClip(ref mut job)) => {
                        job.last_mouse_position_in_time = *mouse_position_in_time;
                    }
                    Some(Job::MoveSubtitleClip(ref mut job)) => {
                        job.last_mouse_position_in_time = *mouse_position_in_time;
                    }
                    _ => {}
                },
                _ => {}
            }
        } else if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match event {
                NamuiEvent::MouseMove(mouse_event) => match self.job {
                    Some(Job::WysiwygMoveImage(ref mut job)) => {
                        job.last_global_mouse_xy = mouse_event.xy;
                    }
                    // Some(Job::WysiwygResizeImage(ref mut job)) => {
                    //     job.last_global_mouse_xy = mouse_event.xy;
                    // }
                    // Some(Job::WysiwygCropImage(ref mut job)) => {
                    //     job.last_global_mouse_xy = mouse_event.xy;
                    // }
                    _ => {}
                },
                NamuiEvent::MouseUp(_) => {
                    match self.job {
                        // TODO : Make these simple using trait
                        Some(Job::MoveCameraClip(_)) => {
                            self.execute_job();
                        }
                        Some(Job::MoveSubtitleClip(_)) => {
                            self.execute_job();
                        }
                        Some(Job::WysiwygMoveImage(_)) => {
                            self.execute_job();
                        }
                        // Some(Job::WysiwygResizeImage(mut job)) => {
                        //     job.last_global_mouse_xy = mouse_event.xy;
                        //     job.execute(self);
                        //     self.job = None;
                        // }
                        // Some(Job::WysiwygCropImage(mut job)) => {
                        //     job.last_global_mouse_xy = mouse_event.xy;
                        //     job.execute(self);
                        //     self.job = None;
                        // }
                        _ => {}
                    }
                }
                NamuiEvent::KeyDown(key_event) => {
                    if key_event.code == namui::Code::KeyZ
                        && namui::managers()
                            .keyboard_manager
                            .any_code_press(&[namui::Code::ControlLeft])
                    {
                        self.undo();
                    } else if key_event.code == namui::Code::KeyY
                        && namui::managers()
                            .keyboard_manager
                            .any_code_press(&[namui::Code::ControlLeft])
                    {
                        self.redo();
                    }
                }
                _ => {}
            }
        };
        self.timeline.update(event);
        self.clip_editor
            .as_mut()
            .map(|clip_editor| clip_editor.update(event));

        self.sequence_player.update(event);
    }

    fn render(&self, props: &Self::Props) -> namui::RenderingTree {
        let selected_clip = self
            .selected_clip_id
            .as_ref()
            .and_then(|id| self.get_sequence().get_clip(&id));

        let timeline_xywh = self.calculate_timeline_xywh(&props.screen_wh);
        let clip_editor_xywh = XywhRect {
            x: 0.0,
            y: 0.0,
            width: props.screen_wh.width * 0.5,
            height: props.screen_wh.height - timeline_xywh.height,
        };
        let sequence_player_xywh = XywhRect {
            x: clip_editor_xywh.width,
            y: 0.0,
            width: props.screen_wh.width - clip_editor_xywh.width,
            height: clip_editor_xywh.height,
        };
        let playback_time = self.sequence_player.get_playback_time();
        render![
            self.timeline.render(&TimelineProps {
                playback_time: &playback_time,
                xywh: timeline_xywh,
                job: &self.job,
                selected_clip_id: &self.selected_clip_id,
                sequence: self.get_sequence(),
                subtitle_play_duration_measurer: &self.subtitle_play_duration_measurer,
            }),
            match (selected_clip, &self.clip_editor) {
                (None, None) => RenderingTree::Empty,
                (Some(clip), Some(clip_editor)) => {
                    clip_editor.render(&ClipEditorProps {
                        clip,
                        xywh: clip_editor_xywh,
                        image_filename_objects: &self.image_filename_objects,
                        job: &self.job,
                    })
                }
                (None, Some(_)) => unreachable!("clip_editor is Some but selected_clip is None"),
                (Some(_), None) => unreachable!("selected_clip is Some but clip_editor is None"),
            },
            self.sequence_player.render(&SequencePlayerProps {
                xywh: &sequence_player_xywh,
                language: namui::Language::Ko, // TODO
                subtitle_play_duration_measurer: &self.subtitle_play_duration_measurer,
            }),
        ]
    }
}

impl Editor {
    pub fn new(socket: Socket, sequence: Arc<Sequence>) -> Self {
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
            image_filename_objects: vec![],
            job: None,
            clip_editor: None,
            selected_clip_id: None,
            sequence_player: SequencePlayer::new(
                sequence.clone(),
                Box::new(LudaEditorServerCameraAngleImageLoader {}),
            ),
            subtitle_play_duration_measurer: SubtitlePlayDurationMeasurer::new(),
            history: History::new(sequence.clone()),
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
            .and_then(|id| self.get_sequence().get_clip(&id))
    }
    fn execute_job(&mut self) {
        let job = &self.job.take();
        if job.is_none() {
            panic!("job is None");
        }
        let job = job.as_ref().unwrap();
        match job.execute(&self.get_sequence()) {
            Err(reason) => {
                namui::log(format!("job execute failed: {:?}", reason));
            }
            Ok(next_sequence) => {
                let next_sequence = Arc::new(next_sequence);
                self.history.push(next_sequence.clone());
                self.sequence_player.update_sequence(next_sequence.clone());
            }
        }
    }
    fn get_sequence(&self) -> &Arc<Sequence> {
        self.history.get()
    }
    fn undo(&mut self) {
        self.history.undo();
        self.sequence_player
            .update_sequence(self.get_sequence().clone());
    }
    fn redo(&mut self) {
        self.history.redo();
        self.sequence_player
            .update_sequence(self.get_sequence().clone());
    }
}
