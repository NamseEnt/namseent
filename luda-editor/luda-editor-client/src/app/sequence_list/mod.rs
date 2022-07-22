mod common;
mod events;
mod list;
mod ops;
mod sync_sequences_button;
mod types;

use self::{
    events::SequenceListEvent,
    types::{
        SequenceOpenState, SequenceOpenStateMap, SequencePreviewProgressMap, SequenceSyncState,
        SequencesSyncStateDetail,
    },
};
use super::{
    editor::{Editor, SequencePlayer},
    events::RouterEvent,
    storage::GithubStorage,
    types::{CameraAngleImageLoader, Sequence, SubtitlePlayDurationMeasure, Track},
};
use crate::app::{
    editor::{SequencePlay, SequencePlayerProps},
    sequence_list::{list::render_list, sync_sequences_button::render_sync_sequences_button},
};
use namui::prelude::*;
use std::{collections::HashMap, sync::Arc};
use wasm_bindgen_futures::spawn_local;

const LIST_WIDTH: Px = px(800.0);
const BUTTON_HEIGHT: Px = px(36.0);
const RECT_RADIUS: Px = px(4.0);
const SPACING: Px = px(4.0);
const MARGIN: Px = px(4.0);

pub struct SequenceListProps<'a> {
    pub wh: Wh<Px>,
    pub subtitle_play_duration_measurer: &'a dyn SubtitlePlayDurationMeasure,
    pub subtitle_character_color_map: &'a HashMap<String, Color>,
}

pub struct SequenceList {
    sequences_sync_state: SequenceSyncState,
    storage: Arc<dyn GithubStorage>,
    scroll_y: Px,
    sequence_player: SequencePlayer,
    sequence_preview_progress_map: SequencePreviewProgressMap,
    sequence_open_state_map: SequenceOpenStateMap,
    opened_sequence_title: Option<String>,
    error_message: Option<String>,
}

impl SequenceList {
    pub fn new(
        storage: Arc<dyn GithubStorage>,
        camera_angle_image_loader: Arc<dyn CameraAngleImageLoader>,
    ) -> Self {
        let mut sequence_list = Self {
            sequences_sync_state: SequenceSyncState {
                started_at: Time::Ms(0.0),
                detail: types::SequencesSyncStateDetail::Loading,
            },
            storage: storage.clone(),
            scroll_y: px(0.0),
            sequence_player: SequencePlayer::new(
                Arc::new(Sequence::default()),
                camera_angle_image_loader,
            ),
            sequence_preview_progress_map: HashMap::new(),
            sequence_open_state_map: SequenceOpenStateMap::new(),
            opened_sequence_title: None,
            error_message: None,
        };
        sequence_list.load_local_sequences();
        sequence_list
    }
}

impl SequenceList {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<SequenceListEvent>() {
            match event {
                SequenceListEvent::SequenceTitleButtonClickedEvent { title } => {
                    self.opened_sequence_title = Some(title.clone());
                }
                SequenceListEvent::SequencesSyncStateUpdateEvent { state } => {
                    let old_state = &self.sequences_sync_state;
                    let is_old_state_newer = old_state.started_at > state.started_at;
                    if is_old_state_newer {
                        return;
                    }

                    self.sequences_sync_state = state.clone();
                    self.error_message = match &state.detail {
                        SequencesSyncStateDetail::Failed { error } => Some(error.clone()),
                        _ => None,
                    };
                }
                SequenceListEvent::SyncSequencesButtonClickedEvent => {
                    self.error_message = None;
                    self.sync_sequences_from_google_spreadsheet()
                }
                SequenceListEvent::ScrolledEvent { scroll_y } => {
                    self.scroll_y = *scroll_y;
                }
                SequenceListEvent::PreviewSliderMovedEvent { title, progress } => {
                    if let SequencesSyncStateDetail::Loaded { title_sequence_map } =
                        &self.sequences_sync_state.detail
                    {
                        let sequence = title_sequence_map.get(title).unwrap();
                        let duration = calculate_sequence_duration(sequence);
                        let moved_time = duration * progress;
                        self.sequence_player.update_sequence(sequence.clone());
                        self.sequence_player.seek(moved_time);
                        self.sequence_preview_progress_map
                            .insert(title.clone(), *progress);
                    }
                }
                SequenceListEvent::SequenceOpenButtonClickedEvent { title, sequence } => {
                    open_sequence(
                        title.clone(),
                        sequence.clone(),
                        self.storage.clone(),
                        self.sequence_open_state_map.get(title),
                    )
                }
                SequenceListEvent::SequenceOpenStateChangedEvent { title, state } => {
                    self.sequence_open_state_map
                        .set(title.clone(), state.clone());
                }
            }
        }
        self.sequence_player.update(event);
    }

    pub fn render(&self, props: &SequenceListProps) -> namui::RenderingTree {
        let list_wh = Wh {
            width: LIST_WIDTH,
            height: props.wh.height - 2.0 * MARGIN - SPACING - BUTTON_HEIGHT,
        };
        let preview_rect = Rect::Xywh {
            x: MARGIN + list_wh.width + SPACING,
            y: MARGIN,
            width: props.wh.width - list_wh.width - SPACING - 2.0 * MARGIN,
            height: props.wh.height - 2.0 * MARGIN,
        };

        render([
            namui::translate(
                MARGIN,
                MARGIN,
                render_sync_sequences_button(Wh {
                    width: LIST_WIDTH,
                    height: BUTTON_HEIGHT,
                }),
            ),
            namui::translate(
                MARGIN,
                MARGIN + SPACING + BUTTON_HEIGHT,
                render_list(
                    list_wh,
                    &self.sequences_sync_state,
                    &self.sequence_preview_progress_map,
                    self.scroll_y,
                    &self.opened_sequence_title,
                    &self.sequence_open_state_map,
                ),
            ),
            self.sequence_player.render(&SequencePlayerProps {
                rect: &preview_rect,
                language: namui::Language::Ko,
                subtitle_play_duration_measurer: props.subtitle_play_duration_measurer,
                with_buttons: false,
                subtitle_character_color_map: props.subtitle_character_color_map,
            }),
        ])
    }
}

fn calculate_sequence_duration(sequence: &Arc<Sequence>) -> Time {
    sequence
        .tracks
        .iter()
        .fold(Time::Ms(0.0), |duration, track| match track.as_ref() {
            Track::Camera(track) => track
                .clips
                .iter()
                .fold(duration, |duration, clip| duration.max(clip.end_at)),
            Track::Subtitle(_) => duration,
        })
}

fn open_sequence(
    title: String,
    sequence: Arc<Sequence>,
    storage: Arc<dyn GithubStorage>,
    open_state: &SequenceOpenState,
) {
    if let SequenceOpenState::Opening = open_state {
        return;
    }
    namui::event::send(SequenceListEvent::SequenceOpenStateChangedEvent {
        title: title.clone(),
        state: SequenceOpenState::Opening,
    });
    spawn_local(async move {
        match storage.lock_sequence(title.as_str()).await {
            Ok(lock_info) => {
                namui::event::send(SequenceListEvent::SequenceOpenStateChangedEvent {
                    title: title.clone(),
                    state: SequenceOpenState::Opening,
                });
                namui::event::send(RouterEvent::PageChangeToEditorEvent(Box::new(
                    move |context| {
                        let lock_info = lock_info.clone();
                        let sequence = sequence.clone();
                        Editor::new(
                            context.storage.clone(),
                            sequence,
                            title.as_str(),
                            context.meta_container.clone(),
                            context.camera_angle_image_loader.clone(),
                            lock_info,
                        )
                    },
                )));
            }
            Err(error) => namui::event::send(SequenceListEvent::SequenceOpenStateChangedEvent {
                title: title.clone(),
                state: SequenceOpenState::Failed {
                    message: format!("{:#?}", error),
                },
            }),
        }
    });
}
