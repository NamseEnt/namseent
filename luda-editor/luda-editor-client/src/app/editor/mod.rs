mod timeline;
use self::{
    clip_editor::{camera_clip_editor::image_browser::ImageBrowserItem, ClipEditor},
    events::*,
};
use super::types::*;
use crate::app::editor::{clip_editor::ClipEditorProps, top_bar::TopBarProps};
pub use job::*;
use luda_editor_rpc::Socket;
use namui::prelude::*;
use std::{cmp::Ordering, collections::BTreeSet, sync::Arc};
use timeline::timeline_body::track_body::camera_track_body::camera_clip_body::*;
pub use timeline::*;
use wasm_bindgen_futures::spawn_local;
mod clip_editor;
mod events;
mod job;
mod sequence_player;
pub use sequence_player::{SequencePlayer, SequencePlayerProps};
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

pub struct EditorProps {
    pub screen_wh: namui::Wh<f32>,
}

pub struct Editor {
    job: Option<Job>,
    timeline: Timeline,
    clip_editor: Option<ClipEditor>,
    image_filename_objects: Vec<ImageFilenameObject>,
    selected_clip_ids: BTreeSet<String>,
    sequence_player: SequencePlayer,
    subtitle_play_duration_measurer: SubtitlePlayDurationMeasurer,
    history: History<Arc<Sequence>>,
    top_bar: TopBar,
    clipboard: Option<Clipboard>,
    language: Language,
    clip_id_to_check_as_click: Option<String>,
    context_menu: Option<ContextMenu>,
    sequence_saver: SequenceSaver,
    sheet_sequence_syncer: SheetSequenceSyncer,
}

impl namui::Entity for Editor {
    type Props = EditorProps;
    fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<EditorEvent>() {
            match event {
                EditorEvent::CameraClipBodyMouseDownEvent {
                    clip_id,
                    click_in_time,
                    clicked_part,
                    ..
                } => {
                    self.on_clip_mouse_down(clip_id);

                    if self.job.is_none() {
                        match clicked_part {
                            CameraClipBodyPart::Sash(sash_direction) => {
                                if self.selected_clip_ids.len() != 1 {
                                    namui::log!("selected_clip_ids.len() should be 1 to resize, but it's {}", self.selected_clip_ids.len());
                                    return;
                                }
                                let clip_id = self.selected_clip_ids.iter().next().unwrap();

                                self.job = Some(Job::ResizeCameraClip(ResizeCameraClipJob {
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
                            CameraClipBodyPart::Body => {
                                self.job = Some(Job::MoveCameraClip(MoveCameraClipJob {
                                    clip_ids: self.selected_clip_ids.clone(),
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
                    self.on_clip_mouse_down(clip_id);

                    if self.job.is_none() {
                        self.job = Some(Job::MoveSubtitleClip(MoveSubtitleClipJob {
                            clip_ids: self.selected_clip_ids.clone(),
                            click_anchor_in_time: *click_in_time,
                            last_mouse_position_in_time: *click_in_time,
                            is_moved: false,
                        }));
                    }
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
                    mouse_xy,
                    handle,
                    center_xy,
                    container_size,
                    image_size_ratio,
                } => {
                    if self.job.is_none() {
                        self.job = Some(Job::WysiwygResizeImage(WysiwygResizeImageJob {
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
                EditorEvent::WysiwygEditorCropperHandleMouseDownEvent {
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
                EditorEvent::ImageBrowserSelectEvent { selected_item } => match selected_item {
                    ImageBrowserItem::CharacterPoseEmotion(character, pose, emotion) => {
                        let character_pose_emotion =
                            CharacterPoseEmotion(character.clone(), pose.clone(), emotion.clone());
                        let clip = self.get_single_selected_clip().unwrap();
                        if Some(&character_pose_emotion)
                            == clip
                                .as_camera_clip()
                                .map(|camera_clip| &camera_clip.camera_angle.character_pose_emotion)
                        {
                            return;
                        }

                        self.job = Some(Job::ChangeImage(ChangeImageJob {
                            clip_id: clip.get_id().to_string(),
                            character_pose_emotion,
                        }));
                        self.execute_job();
                    }
                    _ => {}
                },
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
                        job.is_moved = true;
                    }
                    Some(Job::MoveSubtitleClip(ref mut job)) => {
                        job.last_mouse_position_in_time = *mouse_position_in_time;
                        job.is_moved = true;
                    }
                    Some(Job::ResizeCameraClip(ref mut job)) => {
                        job.last_mouse_position_in_time = *mouse_position_in_time;
                        job.is_moved = true;
                    }
                    _ => {}
                },
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
                    Some(Job::MoveCameraClip(MoveCameraClipJob { is_moved, .. }))
                    | Some(Job::MoveSubtitleClip(MoveSubtitleClipJob { is_moved, .. }))
                    | Some(Job::ResizeCameraClip(ResizeCameraClipJob { is_moved, .. })) => {
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
                            .any_code_press(&[namui::Code::ControlLeft])
                    {
                        self.undo();
                    } else if key_event.code == namui::Code::KeyY
                        && namui::managers()
                            .keyboard_manager
                            .any_code_press(&[namui::Code::ControlLeft])
                    {
                        self.redo();
                    } else if key_event.code == namui::Code::KeyC
                        && namui::managers()
                            .keyboard_manager
                            .any_code_press(&[namui::Code::ControlLeft])
                    {
                        self.copy_to_clipboard();
                    } else if key_event.code == namui::Code::KeyV
                        && namui::managers()
                            .keyboard_manager
                            .any_code_press(&[namui::Code::ControlLeft])
                    {
                        self.paste_clipboard();
                    } else if key_event.code == namui::Code::Home
                        && namui::managers()
                            .keyboard_manager
                            .any_code_press(&[namui::Code::ShiftLeft])
                    {
                        self.select_at_once(Direction::Forward);
                    } else if key_event.code == namui::Code::End
                        && namui::managers()
                            .keyboard_manager
                            .any_code_press(&[namui::Code::ShiftLeft])
                    {
                        self.select_at_once(Direction::Backward);
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
        render![
            self.timeline.render(&TimelineProps {
                playback_time: &playback_time,
                xywh: timeline_xywh,
                job: &self.job,
                selected_clip_ids: self.selected_clip_ids.iter().collect::<Vec<_>>().as_slice(),
                sequence: self.get_sequence(),
                subtitle_play_duration_measurer: &self.subtitle_play_duration_measurer,
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
                        image_filename_objects: &self.image_filename_objects,
                        job: &self.job,
                    })
                }
            },
            self.sequence_player.render(&SequencePlayerProps {
                xywh: &sequence_player_xywh,
                language: self.language,
                subtitle_play_duration_measurer: &self.subtitle_play_duration_measurer,
                with_buttons: true,
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
    ) -> Self {
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
            selected_clip_ids: BTreeSet::new(),
            sequence_player: SequencePlayer::new(
                sequence.clone(),
                Box::new(LudaEditorServerCameraAngleImageLoader {}),
            ),
            subtitle_play_duration_measurer: SubtitlePlayDurationMeasurer::new(),
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
    fn get_single_selected_clip(&self) -> Option<Clip> {
        self.selected_clip_ids
            .iter()
            .next()
            .and_then(|id| self.get_sequence().get_clip(id))
    }
    fn select_only_this_clip(&mut self, clip_id: &str) {
        self.selected_clip_ids.clear();
        self.selected_clip_ids.insert(clip_id.to_string());
        self.clip_editor = Some(ClipEditor::new(
            &self.get_sequence().get_clip(clip_id).unwrap(),
        ));
    }
    fn multi_select_clip(&mut self, clip_id: &str) {
        if self.selected_clip_ids.is_empty() {
            self.selected_clip_ids.insert(clip_id.to_string());
        } else if !self.selected_clip_ids.contains(clip_id) {
            let sequence = self.get_sequence().clone();
            let selected_clip_track = self.get_selected_clip_track().unwrap();
            let selecting_clip_track = sequence.find_track_by_clip_id(clip_id).unwrap();

            if selected_clip_track.get_id() != selecting_clip_track.get_id() {
                self.selected_clip_ids.clear();
            }
            self.selected_clip_ids.insert(clip_id.to_string());
        }

        if self.selected_clip_ids.len() == 1 {
            self.clip_editor = Some(ClipEditor::new(
                &self.get_sequence().get_clip(clip_id).unwrap(),
            ));
        } else {
            self.clip_editor = None;
        }
    }
    fn deselect_all_clips(&mut self) {
        self.selected_clip_ids.clear();
        self.clip_editor = None;
    }
    fn deselect_clips<T: AsRef<str>>(&mut self, clip_ids: &[T]) {
        for clip_id in clip_ids {
            self.selected_clip_ids.remove(clip_id.as_ref());
        }
        if self.selected_clip_ids.len() == 1 {
            let selected_clip_id = self.selected_clip_ids.iter().next().unwrap();
            self.clip_editor = Some(ClipEditor::new(
                &self.get_sequence().get_clip(selected_clip_id).unwrap(),
            ));
        } else {
            self.clip_editor = None;
        }
    }
    fn remove_dangling_selected_clips(&mut self) {
        let sequence = self.get_sequence().clone();

        let mut clip_ids_to_remove = vec![];
        self.selected_clip_ids
            .iter()
            .filter(|clip_id| sequence.find_track_by_clip_id(clip_id).is_none())
            .for_each(|clip_id| {
                clip_ids_to_remove.push(clip_id.clone());
            });

        self.deselect_clips(&clip_ids_to_remove);
    }

    fn is_clip_in_same_track_with_selected_clips(&self, clip_id: &str) -> bool {
        if self.selected_clip_ids.len() == 0 {
            return false;
        }

        let sequence = self.get_sequence().clone();
        let selected_clip_track = self.get_selected_clip_track().unwrap();
        let clip_track = sequence.find_track_by_clip_id(clip_id).unwrap();

        selected_clip_track.get_id() == clip_track.get_id()
    }

    fn select_all_between_clips<T: AsRef<str>>(&mut self, clip_ids: &[T]) {
        let track = self.get_selected_clip_track().unwrap();

        let clips = clip_ids
            .iter()
            .map(|clip_id| track.find_clip(clip_id.as_ref()).unwrap())
            .collect::<Vec<_>>();

        let most_left_clip = clips
            .iter()
            .min_by(|a, b| a.get_start_time().partial_cmp(&b.get_start_time()).unwrap())
            .unwrap();

        let most_right_clip = clips
            .iter()
            .max_by(|a, b| {
                self.get_clip_end_time(a)
                    .partial_cmp(&self.get_clip_end_time(b))
                    .unwrap()
            })
            .unwrap();

        let mut selected_clip_ids = BTreeSet::new();
        selected_clip_ids.insert(most_left_clip.get_id().to_string());
        selected_clip_ids.insert(most_right_clip.get_id().to_string());

        for clip in track.get_clips() {
            if clip.get_start_time() >= most_left_clip.get_start_time()
                && self.get_clip_end_time(&clip) <= self.get_clip_end_time(most_right_clip)
            {
                selected_clip_ids.insert(clip.get_id().to_string());
            }
        }

        self.selected_clip_ids = selected_clip_ids;
    }

    pub(crate) fn get_clip_end_time(&self, clip: &Clip) -> Time {
        match clip {
            Clip::Camera(clip) => clip.end_at,
            Clip::Subtitle(clip) => {
                clip.end_at(self.language, &self.subtitle_play_duration_measurer)
            }
        }
    }
    fn on_clip_mouse_down(&mut self, clip_id: &str) {
        self.clip_id_to_check_as_click = None;

        let keyboard_manager = &namui::managers().keyboard_manager;

        if keyboard_manager.any_code_press(&[namui::Code::ControlLeft]) {
            if self.selected_clip_ids.contains(clip_id) {
                self.deselect_clips(&[clip_id]);
            } else {
                self.multi_select_clip(clip_id);
            }
        } else if keyboard_manager.any_code_press(&[namui::Code::ShiftLeft])
            && self.is_clip_in_same_track_with_selected_clips(clip_id)
        {
            let mut selected_clip_ids = self.selected_clip_ids.clone();
            selected_clip_ids.insert(clip_id.to_string());
            self.select_all_between_clips(&selected_clip_ids.into_iter().collect::<Vec<_>>());
        } else if !self.selected_clip_ids.contains(clip_id) {
            self.select_only_this_clip(clip_id);
        } else {
            self.clip_id_to_check_as_click = Some(clip_id.to_string());
        }
    }

    fn select_at_once(&mut self, direction: Direction) {
        if self.selected_clip_ids.is_empty() {
            return;
        }

        let track = &*self.get_selected_clip_track().unwrap();

        let start_point_to_select = match direction {
            Direction::Forward => self
                .selected_clip_ids
                .iter()
                .map(|clip_id| {
                    let clip = track.find_clip(clip_id).unwrap();
                    self.get_clip_end_time(&clip)
                })
                .max()
                .unwrap(),
            Direction::Backward => self
                .selected_clip_ids
                .iter()
                .map(|clip_id| track.find_clip(clip_id).unwrap().get_start_time())
                .min()
                .unwrap(),
        };

        let ordering = match direction {
            Direction::Forward => Ordering::Less,
            Direction::Backward => Ordering::Greater,
        };

        track.get_clips().iter().for_each(|clip| {
            let results = [
                clip.get_start_time().partial_cmp(&start_point_to_select),
                self.get_clip_end_time(&clip)
                    .partial_cmp(&start_point_to_select),
            ];

            if results.contains(&Some(ordering)) {
                self.selected_clip_ids.insert(clip.get_id().to_string());
            }
        });
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
                source_01_circumscribed: Circumscribed {
                    center: Xy { x: 0.5, y: 0.5 },
                    radius: 0.5,
                },
                crop_screen_01_rect: LtrbRect {
                    left: 0.0,
                    top: 0.0,
                    right: 1.0,
                    bottom: 1.0,
                },
                character_pose_emotion: self.image_filename_objects[0]
                    .into_character_pose_emotion(),
            },
        }
    }
}

enum Direction {
    Forward,
    Backward,
}
