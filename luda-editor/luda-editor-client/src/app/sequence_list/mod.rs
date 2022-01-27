mod common;
mod events;
mod list;
mod ops;
mod reload_titles_button;
mod types;
use self::{
    events::SequenceListEvent,
    types::{SequenceLoadStateDetail, SequenceLoadStateMap, SequenceTitlesLoadState},
};
use super::{
    editor::SequencePlayer,
    types::{
        LudaEditorServerCameraAngleImageLoader, Sequence, SubtitlePlayDurationMeasurer, Time, Track,
    },
};
use crate::app::{
    editor::SequencePlayerProps,
    sequence_list::{list::render_list, reload_titles_button::render_reload_titles_button},
};
use luda_editor_rpc::Socket;
use namui::{render, Entity, Wh, XywhRect};
use std::{collections::HashMap, sync::Arc, time::Duration};

const LIST_WIDTH: f32 = 800.0;
const BUTTON_HEIGHT: f32 = 36.0;
const RECT_RADIUS: f32 = 4.0;
const SPACING: f32 = 4.0;
const MARGIN: f32 = 4.0;

pub struct SequenceListProps {
    pub wh: Wh<f32>,
}

pub struct SequenceList {
    sequence_load_state_map: SequenceLoadStateMap,
    sequence_titles_load_state: SequenceTitlesLoadState,
    socket: Socket,
    scroll_y: f32,
    sequence_player: SequencePlayer,
    subtitle_play_duration_measurer: SubtitlePlayDurationMeasurer,
}

impl SequenceList {
    pub fn new(socket: Socket) -> Self {
        let mut sequence_list = Self {
            sequence_load_state_map: HashMap::new(),
            sequence_titles_load_state: SequenceTitlesLoadState {
                started_at: Duration::from_millis(0),
                detail: types::SequenceTitlesLoadStateDetail::Failed {
                    error: "never loaded".to_string(),
                },
            },
            socket,
            scroll_y: 0.0,
            sequence_player: SequencePlayer::new(
                Arc::new(Sequence::default()),
                Box::new(LudaEditorServerCameraAngleImageLoader {}),
            ),
            subtitle_play_duration_measurer: SubtitlePlayDurationMeasurer::new(),
        };
        sequence_list.load_sequence_titles();
        sequence_list
    }
}

impl Entity for SequenceList {
    type Props = SequenceListProps;

    fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<SequenceListEvent>() {
            match event {
                SequenceListEvent::SequenceLoadStateUpdateEvent { path, state } => match state {
                    Some(state) => {
                        if let Some(old_state) = self.sequence_load_state_map.get(path) {
                            if old_state.started_at > state.started_at {
                                return;
                            }
                        }

                        self.sequence_load_state_map
                            .insert(path.clone(), (*state).clone());
                    }
                    None => {
                        self.sequence_load_state_map.remove(path);
                    }
                },
                SequenceListEvent::SequenceTitleButtonClickedEvent { path } => {
                    let should_clear_load_state = self.sequence_load_state_map.get(path).is_some();
                    match should_clear_load_state {
                        true => clear_sequence(path),
                        false => self.load_sequence(path),
                    }
                }
                SequenceListEvent::SequenceTitlesLoadStateUpdateEvent { state } => {
                    let old_state = &self.sequence_titles_load_state;
                    let is_old_state_newer = old_state.started_at > state.started_at;
                    if is_old_state_newer {
                        return;
                    }

                    self.sequence_titles_load_state = state.clone();
                }
                SequenceListEvent::SequenceReloadTitlesButtonClickedEvent => {
                    self.load_sequence_titles()
                }
                SequenceListEvent::ScrolledEvent { scroll_y } => {
                    self.scroll_y = *scroll_y;
                }
                SequenceListEvent::PreviewSliderMovedEvent { path, progress } => {
                    if let Some(load_state) = self.sequence_load_state_map.get(path) {
                        if let SequenceLoadStateDetail::Loaded { sequence } = &load_state.detail {
                            let duration = calculate_sequence_duration(
                                sequence,
                                &self.subtitle_play_duration_measurer,
                            );
                            let moved_time = duration * progress;
                            self.sequence_player.update_sequence(sequence.clone());
                            self.sequence_player.seek(moved_time);
                        }
                    }
                }
            }
        }
        self.sequence_player.update(event);
    }

    fn render(&self, props: &Self::Props) -> namui::RenderingTree {
        let list_wh = Wh {
            width: LIST_WIDTH,
            height: props.wh.height - 2.0 * MARGIN - SPACING - BUTTON_HEIGHT,
        };
        let preview_xywh = XywhRect {
            x: MARGIN + list_wh.width + SPACING,
            y: MARGIN,
            width: props.wh.width - list_wh.width - SPACING - 2.0 * MARGIN,
            height: props.wh.height - 2.0 * MARGIN,
        };

        render![
            namui::translate(
                MARGIN,
                MARGIN,
                render_reload_titles_button(Wh {
                    width: LIST_WIDTH,
                    height: BUTTON_HEIGHT
                })
            ),
            namui::translate(
                MARGIN,
                MARGIN + SPACING + BUTTON_HEIGHT,
                render_list(
                    list_wh,
                    &self.sequence_titles_load_state,
                    &self.sequence_load_state_map,
                    self.scroll_y
                )
            ),
            self.sequence_player.render(&SequencePlayerProps {
                xywh: &preview_xywh,
                language: namui::Language::Ko,
                subtitle_play_duration_measurer: &self.subtitle_play_duration_measurer,
                with_buttons: false,
            })
        ]
    }
}

fn clear_sequence(path: &String) {
    namui::event::send(SequenceListEvent::SequenceLoadStateUpdateEvent {
        path: path.clone(),
        state: None,
    })
}

fn calculate_sequence_duration(
    sequence: &Arc<Sequence>,
    subtitle_play_duration_measurer: &SubtitlePlayDurationMeasurer,
) -> Time {
    sequence
        .tracks
        .iter()
        .fold(Time::zero(), |duration, track| match track.as_ref() {
            Track::Camera(track) => track
                .clips
                .iter()
                .fold(duration, |duration, clip| duration.max(clip.end_at)),
            Track::Subtitle(track) => track.clips.iter().fold(duration, |duration, clip| {
                duration.max(clip.end_at(namui::Language::Ko, subtitle_play_duration_measurer))
            }),
        })
}
