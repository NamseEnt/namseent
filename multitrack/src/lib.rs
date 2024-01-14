mod mini_map;
mod track;

use mini_map::MiniMap;
use namui::prelude::{media::audio::RawAudio, *};
use namui_prebuilt::*;
use std::sync::Arc;
use track::Track;

pub fn main() {
    namui::start(|| App)
}

#[namui::component]
struct App;

#[derive(Debug)]
enum ActionState {
    None,
    Dragging {
        dragging_track_index: usize,
        start_sample_index: usize,
        cursor_sample_index: usize,
    },
    Selected {
        selected_track_index: usize,
        range: std::ops::Range<usize>,
    },
}

impl Component for App {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let screen_wh: Wh<Px> = namui::screen::size().into_type();
        const SAMPLE_RATE: u32 = 44100;
        let (tracks, set_tracks) = ctx.state::<Option<Vec<PtrEqArc<RawAudio>>>>(|| None);
        let (window_size, set_window_size) = ctx.state(|| SAMPLE_RATE as usize);
        let (start_sample_index, set_start_sample_index) = ctx.state(|| 0_usize);
        let (action_state, set_action_state) = ctx.state(|| ActionState::None);

        let range = *start_sample_index..(*start_sample_index + *window_size);

        ctx.effect("load raw audio", || {
            namui::spawn(async move {
                let futures =
                    ["no_drums", "cymbals", "toms", "snare", "kick"].map(|file_name| async move {
                        RawAudio::load(
                            &namui::system::file::bundle::to_real_path(format!(
                                "bundle:resources/{file_name}.opus"
                            ))
                            .unwrap(),
                            Some(SAMPLE_RATE),
                        )
                        .await
                        .unwrap()
                    });
                let tracks = namui::join_all(futures)
                    .await
                    .into_iter()
                    .map(PtrEqArc::new)
                    .collect::<Vec<_>>();

                set_window_size.set(tracks[0].channels[0].len());
                set_tracks.set(Some(tracks));
            });
        });

        ctx.compose(|ctx| {
            let Some(tracks) = tracks.as_ref() else {
                return;
            };

            let move_and_mix =
                |from_track_index: usize, to_track_index: usize, range: std::ops::Range<usize>| {
                    let mut from_track = (*tracks[from_track_index]).clone();
                    let mut to_track = (*tracks[to_track_index]).clone();

                    from_track
                        .channels
                        .iter_mut()
                        .zip(to_track.channels.iter_mut())
                        .for_each(|(from_channel, to_channel)| {
                            from_channel.as_mut_slice()[range.clone()]
                                .iter_mut()
                                .zip(to_channel.as_mut_slice()[range.clone()].iter_mut())
                                .for_each(|(from_sample, to_sample)| {
                                    *to_sample += *from_sample;
                                    *from_sample = 0.0;
                                });
                        });

                    let mut tracks = tracks.clone();
                    tracks[from_track_index] = PtrEqArc::new(from_track);
                    tracks[to_track_index] = PtrEqArc::new(to_track);
                    set_tracks.set(Some(tracks));
                };

            let audio_length = tracks[0].channels[0].len();

            let sample_index_on_x = |x: Px| {
                let cursor_x_ratio = x / screen_wh.width;

                let cursor_sample_index =
                    *start_sample_index as f32 + (*window_size as f32 * cursor_x_ratio);

                cursor_sample_index.clamp(0.0, audio_length as f32) as usize
            };

            table::hooks::vertical([
                table::hooks::fixed(120.px(), |wh, ctx| {
                    ctx.add(MiniMap {
                        wh,
                        length: audio_length,
                        range: range.clone(),
                    });
                }),
                table::hooks::ratio(
                    1,
                    table::hooks::vertical((0..5).map(|track_index| {
                        let range = range.clone();

                        table::hooks::ratio(1, move |wh, ctx| {
                            ctx.add(
                                Track {
                                    wh,
                                    audio: tracks[track_index].clone(),
                                    visual_range: range,
                                    selection_range: match action_state.as_ref() {
                                        ActionState::None => None,
                                        &ActionState::Dragging {
                                            dragging_track_index,
                                            start_sample_index,
                                            cursor_sample_index,
                                        } => {
                                            if track_index == dragging_track_index {
                                                Some(
                                                    start_sample_index.min(cursor_sample_index)
                                                        ..start_sample_index
                                                            .max(cursor_sample_index),
                                                )
                                            } else {
                                                None
                                            }
                                        }
                                        &ActionState::Selected {
                                            selected_track_index,
                                            ref range,
                                        } => {
                                            if track_index == selected_track_index {
                                                Some(range.clone())
                                            } else {
                                                None
                                            }
                                        }
                                    },
                                }
                                .attach_event(move |event| {
                                    if let Event::MouseDown { event } = event {
                                        match action_state.as_ref() {
                                            ActionState::None => {
                                                if !event.is_local_xy_in() {
                                                    return;
                                                }

                                                set_action_state.set(ActionState::Dragging {
                                                    dragging_track_index: track_index,
                                                    start_sample_index: sample_index_on_x(
                                                        event.local_xy().x,
                                                    ),
                                                    cursor_sample_index: sample_index_on_x(
                                                        event.local_xy().x,
                                                    ),
                                                });
                                            }
                                            ActionState::Dragging { .. } => {}
                                            &ActionState::Selected {
                                                selected_track_index,
                                                ref range,
                                            } => {
                                                if !event.is_local_xy_in() {
                                                    return;
                                                }

                                                if system::keyboard::ctrl_press() {
                                                    if track_index != selected_track_index {
                                                        move_and_mix(
                                                            selected_track_index,
                                                            track_index,
                                                            range.clone(),
                                                        );
                                                    }

                                                    set_action_state.set(ActionState::None);
                                                } else {
                                                    set_action_state.set(ActionState::Dragging {
                                                        dragging_track_index: selected_track_index,
                                                        start_sample_index: sample_index_on_x(
                                                            event.local_xy().x,
                                                        ),
                                                        cursor_sample_index: sample_index_on_x(
                                                            event.local_xy().x,
                                                        ),
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }),
                            );
                        })
                    })),
                ),
            ])(screen_wh, ctx);

            ctx.add(
                simple_rect(screen_wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT)
                    .attach_event(|event| {
                        match event {
                            Event::Wheel { event } => {
                                if namui::keyboard::ctrl_press() {
                                    // zoom

                                    let zoom_delta = 1.0 + event.delta_xy.y / 10.0;
                                    let next_window_size =
                                        (zoom_delta * *window_size as f32) as usize;

                                    set_window_size.set(next_window_size);

                                    let cursor_x_ratio = event.local_xy().x / screen_wh.width;

                                    let cursor_sample_index = *start_sample_index as f32
                                        + (*window_size as f32 * cursor_x_ratio);

                                    let next_start_sample_index = cursor_sample_index
                                        - zoom_delta * (*window_size as f32 * cursor_x_ratio);

                                    set_start_sample_index.set(
                                        next_start_sample_index.clamp(
                                            0.0,
                                            audio_length as f32 - next_window_size as f32,
                                        ) as usize,
                                    );
                                } else {
                                    // move

                                    let delta = (event.delta_xy.y / 10.0) * *window_size as f32;
                                    set_start_sample_index.set(
                                        ((*start_sample_index as f32 - delta) as usize)
                                            .clamp(0, audio_length - *window_size),
                                    );
                                }
                            }
                            Event::MouseMove { event } => {
                                if let ActionState::Dragging {
                                    dragging_track_index,
                                    start_sample_index,
                                    cursor_sample_index: _,
                                } = *action_state
                                {
                                    set_action_state.set(ActionState::Dragging {
                                        dragging_track_index,
                                        start_sample_index,
                                        cursor_sample_index: sample_index_on_x(event.local_xy().x),
                                    });
                                }
                            }
                            Event::MouseUp { event } => {
                                if let ActionState::Dragging {
                                    dragging_track_index,
                                    start_sample_index,
                                    cursor_sample_index: _,
                                } = *action_state
                                {
                                    let cursor_sample_index = sample_index_on_x(event.local_xy().x);
                                    set_action_state.set(ActionState::Selected {
                                        selected_track_index: dragging_track_index,
                                        range: cursor_sample_index.min(start_sample_index)
                                            ..cursor_sample_index.max(start_sample_index),
                                    });
                                }
                            }
                            _ => (),
                        }
                    }),
            );
        });

        ctx.done()
    }
}

#[derive(Debug, Clone)]
struct PtrEqArc<T>(Arc<T>);

impl<T> PtrEqArc<T> {
    fn new(value: T) -> Self {
        Self(Arc::new(value))
    }
}
impl<T> std::ops::Deref for PtrEqArc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> PartialEq for PtrEqArc<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}
