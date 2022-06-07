mod timeline;
use self::{
    clip_editor::{camera_clip_editor::image_browser::ImageBrowserFile, ClipEditor},
    events::*,
};
use super::types::{
    meta::{Meta, MetaContainer},
    *,
};
use crate::app::editor::{clip_editor::ClipEditorProps, top_bar::TopBarProps};
use futures::{join, FutureExt};
pub use job::*;
use luda_editor_rpc::Socket;
use namui::prelude::*;
use std::{cmp::Ordering, collections::BTreeSet, sync::Arc};
use timeline::timeline_body::track_body::*;
pub use timeline::*;
use wasm_bindgen_futures::spawn_local;
mod clip_editor;
mod events;
mod job;
mod sequence_player;
pub use sequence_player::{SequencePlay, SequencePlayer, SequencePlayerProps};
mod history;
use history::History;
mod top_bar;
use top_bar::TopBar;
mod clipboard;
use clipboard::Clipboard;
mod context_menu;
use context_menu::*;
mod sequence_saver;
use sequence_saver::SequenceSaver;
mod sheet_sequence_syncer;
use sheet_sequence_syncer::*;
mod clip_select;

pub struct EditorProps {
    pub screen_wh: namui::Wh<f32>,
}

pub struct Editor {
    job: Option<Job>,
    timeline: Timeline,
    clip_editor: Option<ClipEditor>,
    character_image_files: BTreeSet<ImageBrowserFile>,
    background_image_files: BTreeSet<ImageBrowserFile>,
    selected_clip_ids: Arc<BTreeSet<String>>,
    sequence_player: Box<dyn SequencePlay>,
    history: History<Arc<Sequence>>,
    top_bar: TopBar,
    clipboard: Option<Clipboard>,
    language: Language,
    clip_id_to_check_as_click: Option<String>,
    context_menu: Option<ContextMenu>,
    sequence_saver: SequenceSaver,
    sheet_sequence_syncer: SheetSequenceSyncer,
    meta_container: Arc<MetaContainer>,
}

impl namui::Entity for Editor {
    type Props = EditorProps;
    fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<EditorEvent>() {
            match event {
                EditorEvent::ResizableClipBodyMouseDownEvent {
                    clip_id,
                    click_in_time,
                    clicked_part,
                    ..
                } => {
                    self.on_clip_mouse_down(clip_id, click_in_time);

                    if self.job.is_none() {
                        match clicked_part {
                            ResizableClipBodyPart::Sash(sash_direction) => {
                                if self.selected_clip_ids.len() != 1 {
                                    namui::log!("selected_clip_ids.len() should be 1 to resize, but it's {}", self.selected_clip_ids.len());
                                } else {
                                    let clip_id = self.selected_clip_ids.iter().next().unwrap();

                                    self.job = Some(Job::ResizeClip(ResizeClipJob {
                                        clip_id: clip_id.clone(),
                                        click_anchor_in_time: *click_in_time,
                                        last_mouse_position_in_time: *click_in_time,
                                        is_moved: false,
                                        resize_direction: match *sash_direction {
                                            SashDirection::Left => ResizeDirection::Left,
                                            SashDirection::Right => ResizeDirection::Right,
                                        },
                                    }));
                                }
                            }
                            ResizableClipBodyPart::Body => {
                                self.job = Some(Job::MoveClip(MoveClipJob {
                                    clip_ids: self.get_selected_clip_ids().clone(),
                                    click_anchor_in_time: *click_in_time,
                                    last_mouse_position_in_time: *click_in_time,
                                    is_moved: false,
                                }));
                            }
                        }
                    }
                }
                EditorEvent::SubtitleClipHeadMouseDownEvent {
                    clip_id,
                    click_in_time,
                    ..
                } => {
                    self.on_clip_mouse_down(clip_id, click_in_time);

                    if self.job.is_none() {
                        self.job = Some(Job::MoveClip(MoveClipJob {
                            clip_ids: self.get_selected_clip_ids().clone(),
                            click_anchor_in_time: *click_in_time,
                            last_mouse_position_in_time: *click_in_time,
                            is_moved: false,
                        }));
                    }
                }
                EditorEvent::CharacterImageFilesUpdatedEvent {
                    character_image_files,
                } => {
                    self.character_image_files = character_image_files.clone();
                }
                EditorEvent::BackgroundImageFilesUpdatedEvent {
                    background_image_files,
                } => {
                    self.background_image_files = background_image_files.clone();
                }
                EditorEvent::WysiwygEditorInnerImageMouseDownEvent {
                    target,
                    mouse_xy,
                    container_size,
                } => {
                    if self.job.is_none() {
                        self.job = Some(Job::WysiwygMoveImage(WysiwygMoveImageJob {
                            target: target.clone(),
                            clip_id: self
                                .get_single_selected_clip()
                                .unwrap()
                                .get_id()
                                .to_string(),
                            start_global_mouse_xy: *mouse_xy,
                            last_global_mouse_xy: *mouse_xy,
                            container_size: *container_size,
                        }));
                    };
                }
                EditorEvent::WysiwygEditorResizerHandleMouseDownEvent {
                    target,
                    mouse_xy,
                    handle,
                    center_xy,
                    container_size,
                    image_size_ratio,
                } => {
                    if self.job.is_none() {
                        self.job = Some(Job::WysiwygResizeImage(WysiwygResizeImageJob {
                            target: target.clone(),
                            clip_id: self
                                .get_single_selected_clip()
                                .unwrap()
                                .get_id()
                                .to_string(),
                            start_global_mouse_xy: *mouse_xy,
                            last_global_mouse_xy: *mouse_xy,
                            handle: *handle,
                            center_xy: *center_xy,
                            container_size: *container_size,
                            image_size_ratio: *image_size_ratio,
                        }));
                    };
                }
                EditorEvent::CharacterWysiwygEditorCropperHandleMouseDownEvent {
                    mouse_xy,
                    handle,
                    container_size,
                } => {
                    if self.job.is_none() {
                        self.job = Some(Job::WysiwygCropImage(WysiwygCropImageJob {
                            clip_id: self
                                .get_single_selected_clip()
                                .unwrap()
                                .get_id()
                                .to_string(),
                            start_global_mouse_xy: *mouse_xy,
                            last_global_mouse_xy: *mouse_xy,
                            handle: handle.clone(),
                            container_size: *container_size,
                        }));
                    };
                }
                EditorEvent::CharacterImageBrowserSelectEvent {
                    character_pose_emotion,
                } => {
                    let clip = self.get_single_selected_clip().unwrap();

                    clip.as_camera_clip().map(|camera_clip| {
                        if camera_clip
                            .camera_angle
                            .character
                            .as_ref()
                            .map(|character| character.character_pose_emotion.clone())
                            .ne(character_pose_emotion)
                        {
                            self.job = Some(Job::ChangeImage(ChangeImageJob {
                                clip_id: clip.get_id().to_string(),
                                character_pose_emotion: character_pose_emotion.clone(),
                                background_name: camera_clip
                                    .camera_angle
                                    .background
                                    .as_ref()
                                    .map(|background| background.name.clone()),
                            }));
                            self.execute_job();
                        }
                    });
                }
                EditorEvent::BackgroundImageBrowserSelectEvent { background_name } => {
                    let clip = self.get_single_selected_clip().unwrap();

                    clip.as_camera_clip().map(|camera_clip| {
                        if camera_clip
                            .camera_angle
                            .background
                            .as_ref()
                            .map(|background| background.name.clone())
                            .ne(background_name)
                        {
                            self.job = Some(Job::ChangeImage(ChangeImageJob {
                                clip_id: clip.get_id().to_string(),
                                character_pose_emotion: camera_clip
                                    .camera_angle
                                    .character
                                    .as_ref()
                                    .map(|character| character.character_pose_emotion.clone()),
                                background_name: background_name.clone(),
                            }));
                            self.execute_job();
                        }
                    });
                }
                EditorEvent::TimelineTimeRulerClickEvent {
                    click_position_in_time,
                } => {
                    self.sequence_player.seek(*click_position_in_time);
                }
                EditorEvent::TimelineBodyMouseMoveEvent {
                    mouse_position_in_time,
                } => match self.job {
                    Some(Job::MoveClip(ref mut job)) => {
                        job.last_mouse_position_in_time = *mouse_position_in_time;
                        job.is_moved = true;
                    }
                    Some(Job::ResizeClip(ref mut job)) => {
                        job.last_mouse_position_in_time = *mouse_position_in_time;
                        job.is_moved = true;
                    }
                    _ => {}
                },
                EditorEvent::TimelineBodyLeftClickEvent {
                    is_mouse_on_clip,
                    mouse_position_in_time,
                } => {
                    self.sequence_player.seek(*mouse_position_in_time);
                    if !is_mouse_on_clip {
                        let keyboard_manager = &namui::managers().keyboard_manager;
                        if keyboard_manager
                            .any_code_press([namui::Code::ControlLeft, namui::Code::ControlRight])
                        {
                            // do nothing please
                        } else if keyboard_manager
                            .any_code_press([namui::Code::ShiftLeft, namui::Code::ShiftRight])
                        {
                            self.select_all_to_time(mouse_position_in_time);
                        } else {
                            self.deselect_all_clips();
                        }
                    }
                }
                EditorEvent::CameraTrackBodyRightClickEvent {
                    mouse_global_xy,
                    mouse_position_in_time,
                } => {
                    self.open_context_menu(mouse_global_xy, mouse_position_in_time);
                }
                EditorEvent::SubtitleSyncRequestEvent { subtitles } => {
                    self.job = Some(Job::SyncSubtitles(SyncSubtitlesJob {
                        subtitles: subtitles.clone(),
                    }));
                    let result = self.execute_job();
                    namui::event::send(SheetSequenceSyncerEvent::SyncDone(result));
                }
                _ => {}
            }
        } else if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match event {
                NamuiEvent::MouseMove(mouse_event) => match self.job {
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
                NamuiEvent::MouseUp(_) => match self.job {
                    Some(Job::MoveClip(MoveClipJob { is_moved, .. }))
                    | Some(Job::ResizeClip(ResizeClipJob { is_moved, .. })) => {
                        if is_moved {
                            self.execute_job();
                        } else {
                            self.job.take();
                            if let Some(clip_id) = &self.clip_id_to_check_as_click {
                                self.select_only_this_clip(&clip_id.clone());
                            }
                        }
                    }
                    Some(Job::WysiwygMoveImage(_))
                    | Some(Job::WysiwygResizeImage(_))
                    | Some(Job::WysiwygCropImage(_)) => {
                        self.execute_job();
                    }
                    _ => {}
                },
                NamuiEvent::KeyDown(key_event) => {
                    if key_event.code == namui::Code::KeyZ
                        && namui::managers()
                            .keyboard_manager
                            .any_code_press([namui::Code::ControlLeft, namui::Code::ControlRight])
                    {
                        self.undo();
                    } else if key_event.code == namui::Code::KeyY
                        && namui::managers()
                            .keyboard_manager
                            .any_code_press([namui::Code::ControlLeft, namui::Code::ControlRight])
                    {
                        self.redo();
                    } else if key_event.code == namui::Code::KeyC
                        && namui::managers()
                            .keyboard_manager
                            .any_code_press([namui::Code::ControlLeft, namui::Code::ControlRight])
                    {
                        self.copy_to_clipboard();
                    } else if key_event.code == namui::Code::KeyV
                        && namui::managers()
                            .keyboard_manager
                            .any_code_press([namui::Code::ControlLeft, namui::Code::ControlRight])
                    {
                        self.paste_clipboard();
                    } else if key_event.code == namui::Code::Home
                        && namui::managers()
                            .keyboard_manager
                            .any_code_press([namui::Code::ShiftLeft, namui::Code::ShiftRight])
                    {
                        self.select_at_once(Direction::Forward);
                    } else if key_event.code == namui::Code::End
                        && namui::managers()
                            .keyboard_manager
                            .any_code_press([namui::Code::ShiftLeft, namui::Code::ShiftRight])
                    {
                        self.select_at_once(Direction::Backward);
                    } else if key_event.code == namui::Code::Delete
                        && !self.selected_clip_ids.is_empty()
                    {
                        self.job = Some(Job::DeleteCameraClip(DeleteCameraClipJob {
                            clip_ids: self.get_selected_clip_ids().clone(),
                        }));
                        self.execute_job();
                    } else if key_event.code == namui::Code::Space {
                        self.sequence_player.toggle_playback();
                    }
                }
                _ => {}
            }
        } else if let Some(event) = event.downcast_ref::<ContextMenuEvent>() {
            match event {
                ContextMenuEvent::CloseContextMenu(id) => {
                    if self
                        .context_menu
                        .as_ref()
                        .map(|context_menu| context_menu.get_id().to_string())
                        == Some(id.to_string())
                    {
                        self.context_menu = None;
                    }
                }
                ContextMenuEvent::CreateCameraClip(time) => {
                    let new_clip = self.create_default_camera_clip(time);
                    let new_clip_id = new_clip.id.clone();

                    self.job = Some(Job::AddCameraClip(AddCameraClipJob {
                        camera_clip: Arc::new(new_clip),
                        time_to_insert: *time,
                    }));
                    self.execute_job();

                    if self.get_sequence().find_clip(&new_clip_id).is_some() {
                        self.select_only_this_clip(&new_clip_id);
                    }
                }
                _ => {}
            }
        }
        self.timeline.update(event);
        self.clip_editor
            .as_mut()
            .map(|clip_editor| clip_editor.update(event));

        self.sequence_player.update(event);
        self.top_bar.update(event);
        self.context_menu
            .as_mut()
            .map(move |context_menu| context_menu.update(event));
        self.sequence_saver.update(event);
        self.sheet_sequence_syncer.update(event);
    }

    fn render(&self, props: &Self::Props) -> namui::RenderingTree {
        let timeline_xywh = self.calculate_timeline_xywh(&props.screen_wh);
        let top_bar_xywh: XywhRect<f32> = XywhRect {
            x: 0.0,
            y: 0.0,
            width: props.screen_wh.width,
            height: 32.0,
        };
        let clip_editor_xywh = XywhRect {
            x: 0.0,
            y: top_bar_xywh.height,
            width: props.screen_wh.width * 0.5,
            height: props.screen_wh.height - timeline_xywh.height - top_bar_xywh.height,
        };
        let sequence_player_xywh = XywhRect {
            x: clip_editor_xywh.width,
            y: top_bar_xywh.height,
            width: props.screen_wh.width - clip_editor_xywh.width,
            height: clip_editor_xywh.height - top_bar_xywh.height,
        };
        let playback_time = self.sequence_player.get_playback_time();
        let meta = self.get_meta();
        render![
            self.timeline.render(&TimelineProps {
                playback_time: &playback_time,
                xywh: timeline_xywh,
                job: &self.job,
                selected_clip_ids: self.selected_clip_ids.iter().collect::<Vec<_>>().as_slice(),
                sequence: self.get_sequence(),
                subtitle_play_duration_measurer: &self.get_meta(),
            }),
            match &self.clip_editor {
                None => RenderingTree::Empty,
                Some(clip_editor) => {
                    clip_editor.render(&ClipEditorProps {
                        clip: self
                            .selected_clip_ids
                            .iter()
                            .next()
                            .and_then(|id| self.get_sequence().get_clip(&id))
                            .unwrap(),
                        xywh: clip_editor_xywh,
                        character_image_files: &self.character_image_files,
                        background_image_files: &self.background_image_files,
                        job: &self.job,
                    })
                }
            },
            self.sequence_player.render(&SequencePlayerProps {
                xywh: &sequence_player_xywh,
                language: self.language,
                subtitle_play_duration_measurer: &meta,
                with_buttons: true,
                subtitle_character_color_map: &meta.subtitle_character_color_map,
            }),
            self.top_bar.render(&TopBarProps {
                xywh: top_bar_xywh,
                sequence_saver_status: &self.sequence_saver.get_status(),
                sheet_sequence_syncer_status: &self.sheet_sequence_syncer.get_status(),
            }),
            match &self.context_menu {
                Some(context_menu) => context_menu.render(&ContextMenuProps {}),
                None => RenderingTree::Empty,
            },
        ]
    }
}

impl Editor {
    pub fn new(
        socket: Socket,
        sequence: Arc<Sequence>,
        sequence_file_path: &str,
        sequence_title: &str,
        meta_container: Arc<MetaContainer>,
    ) -> Self {
        spawn_local({
            let socket = socket.clone();
            async move {
                let character_image_urls_future = socket
                    .get_character_image_urls(luda_editor_rpc::get_character_image_urls::Request {})
                    .map(|result| match result {
                        Ok(response) => {
                            let character_image_files =
                                convert_character_image_urls_to_character_image_files(
                                    &response.character_image_urls,
                                );

                            namui::event::send(EditorEvent::CharacterImageFilesUpdatedEvent {
                                character_image_files,
                            })
                        }
                        Err(error) => {
                            namui::log(format!("error on get_character_image_urls: {:?}", error))
                        }
                    });

                let background_image_urls_future = socket
                    .get_background_image_urls(
                        luda_editor_rpc::get_background_image_urls::Request {},
                    )
                    .map(|result| match result {
                        Ok(response) => {
                            let background_image_files =
                                convert_background_image_urls_to_background_image_files(
                                    &response.background_image_urls,
                                );

                            namui::event::send(EditorEvent::BackgroundImageFilesUpdatedEvent {
                                background_image_files,
                            })
                        }
                        Err(error) => {
                            namui::log(format!("error on get_background_image_urls: {:?}", error))
                        }
                    });

                join!(character_image_urls_future, background_image_urls_future);
            }
        });
        Self {
            timeline: Timeline::new(),
            character_image_files: BTreeSet::new(),
            background_image_files: BTreeSet::new(),
            job: None,
            clip_editor: None,
            selected_clip_ids: Arc::new(BTreeSet::new()),
            sequence_player: Box::new(SequencePlayer::new(
                sequence.clone(),
                Box::new(LudaEditorServerCameraAngleImageLoader {}),
            )),
            history: History::new(sequence.clone()),
            top_bar: TopBar::new(),
            clipboard: None,
            language: namui::Language::Ko,
            clip_id_to_check_as_click: None,
            context_menu: None,
            sequence_saver: SequenceSaver::new(
                sequence_file_path.clone(),
                sequence.clone(),
                socket.clone(),
            ),
            sheet_sequence_syncer: SheetSequenceSyncer::new(sequence_title),
            meta_container,
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
    fn execute_job(&mut self) -> Result<(), String> {
        let job = &self.job.take();
        if job.is_none() {
            return Err("job is None".to_string());
        }
        let job = job.as_ref().unwrap();
        match job.execute(&self.get_sequence()) {
            Err(reason) => Err(reason),
            Ok(next_sequence) => {
                let next_sequence = Arc::new(next_sequence);
                self.history.push(next_sequence.clone());
                self.on_change_sequence();
                Ok(())
            }
        }
    }
    fn get_sequence(&self) -> &Arc<Sequence> {
        self.history.get()
    }
    fn on_change_sequence(&mut self) {
        let sequence = self.get_sequence().clone();
        self.remove_dangling_selected_clips();
        self.sequence_player.update_sequence(sequence.clone());
        self.sequence_saver.on_change_sequence(sequence.clone());
        namui::event::send(EditorEvent::SequenceUpdateEvent {
            sequence: sequence.clone(),
        });
    }
    fn undo(&mut self) {
        if self.history.undo().is_some() {
            self.on_change_sequence();
        }
    }
    fn redo(&mut self) {
        if self.history.redo().is_some() {
            self.on_change_sequence();
        }
    }

    fn copy_to_clipboard(&mut self) {
        // TODO : Support multiple clips
        let selected_clip = self.get_single_selected_clip();

        match selected_clip {
            None => {}
            Some(Clip::Camera(camera_clip)) => {
                self.clipboard = Some(Clipboard::CameraClip(camera_clip.clone()));
            }
            Some(Clip::Subtitle(_)) => {}
        }
    }

    fn paste_clipboard(&mut self) {
        if self.clipboard.is_none() {
            return;
        }

        match self.clipboard.as_ref().unwrap() {
            Clipboard::CameraClip(camera_clip) => {
                self.job = Some(Job::AddCameraClip(AddCameraClipJob {
                    camera_clip: Arc::new(camera_clip.duplicate()),
                    time_to_insert: self.sequence_player.get_playback_time(),
                }));
                self.execute_job();
            }
        }
    }

    pub(crate) fn get_clip_end_time(&self, clip: &Clip) -> Time {
        match clip {
            Clip::Camera(clip) => clip.end_at,
            Clip::Subtitle(clip) => clip.end_at(self.language, &self.get_meta()),
        }
    }
    fn on_clip_mouse_down(&mut self, clip_id: &str, click_in_time: &Time) {
        self.clip_id_to_check_as_click = None;

        let keyboard_manager = &namui::managers().keyboard_manager;

        if keyboard_manager.any_code_press([namui::Code::ControlLeft, namui::Code::ControlRight]) {
            if self.selected_clip_ids.contains(clip_id) {
                self.deselect_clips(&[clip_id]);
            } else {
                self.multi_select_clip(clip_id);
            }
        } else if keyboard_manager
            .any_code_press([namui::Code::ShiftLeft, namui::Code::ShiftRight])
            && self.is_clip_in_same_track_with_selected_clips(clip_id)
        {
            self.select_all_to_time(click_in_time);
        } else if !self.selected_clip_ids.contains(clip_id) {
            self.select_only_this_clip(clip_id);
        } else {
            self.clip_id_to_check_as_click = Some(clip_id.to_string());
        }
    }

    fn get_selected_clip_track(&self) -> Option<Arc<Track>> {
        self.selected_clip_ids
            .iter()
            .next()
            .and_then(|clip_id| self.get_sequence().find_track_by_clip_id(clip_id))
    }

    fn open_context_menu(
        &mut self,
        context_menu_absolute_left_top: &Xy<f32>,
        mouse_position_in_time: &Time,
    ) {
        self.context_menu = Some(ContextMenu::new(
            context_menu_absolute_left_top,
            mouse_position_in_time,
        ));
    }

    fn create_default_camera_clip(&self, start_at: &Time) -> CameraClip {
        CameraClip {
            id: CameraClip::get_new_id(),
            start_at: *start_at,
            end_at: start_at + Time::from_sec(3.0),
            camera_angle: CameraAngle {
                character: None,
                background: None,
            },
        }
    }
    fn get_meta(&self) -> Meta {
        self.meta_container.get_meta().unwrap()
    }
}

fn convert_character_image_urls_to_character_image_files(
    character_image_urls: &[String],
) -> BTreeSet<ImageBrowserFile> {
    character_image_urls
        .iter()
        .map(|url| ImageBrowserFile::new(url.clone()))
        .collect()
}

fn convert_background_image_urls_to_background_image_files(
    background_image_urls: &[String],
) -> BTreeSet<ImageBrowserFile> {
    background_image_urls
        .iter()
        .map(|url| ImageBrowserFile::new(url.clone()))
        .collect()
}

enum Direction {
    Forward,
    Backward,
}
