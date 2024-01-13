mod mini_map;
mod track;

use mini_map::MiniMap;
use namui::prelude::*;
use namui_prebuilt::*;
use std::sync::Arc;
use track::Track;

pub fn main() {
    namui::start(|| App)
}

#[namui::component]
struct App;

#[derive(Debug)]
enum DragState {
    None,
    Dragging {
        track_index: usize,
        start_sample_index: usize,
        cursor_sample_index: usize,
    },
}

impl Component for App {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let screen_wh: Wh<Px> = namui::screen::size().into_type();
        const SAMPLE_RATE: u32 = 44100;
        let (audio, set_audio) = ctx.state(|| None);
        let (window_size, set_window_size) = ctx.state(|| SAMPLE_RATE as usize);
        let (start_sample_index, set_start_sample_index) = ctx.state(|| 0_usize);
        let (drag_state, set_drag_state) = ctx.state(|| DragState::None);

        let range = *start_sample_index..(*start_sample_index + *window_size);

        ctx.effect("load raw audio", || {
            namui::spawn(async move {
                let raw_audio: media::audio::RawAudio = namui::media::audio::RawAudio::load(
                    &namui::system::file::bundle::to_real_path("bundle:resources/snare.opus")
                        .unwrap(),
                    Some(SAMPLE_RATE),
                )
                .await
                .unwrap();

                println!("audio loaded: {:?}", raw_audio);

                set_window_size.set(raw_audio.channels[0].len());
                set_audio.set(Some(Arc::new(raw_audio)));
            });
        });

        ctx.compose(|ctx| {
            let Some(audio) = audio.as_ref() else {
                return;
            };

            let audio_length = audio.channels[0].len();

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
                                    audio: audio.clone(),
                                    visual_range: range,
                                    selection_range: match *drag_state {
                                        DragState::None => None,
                                        DragState::Dragging {
                                            track_index: drag_track_index,
                                            start_sample_index,
                                            cursor_sample_index,
                                        } => {
                                            if track_index == drag_track_index {
                                                Some(
                                                    start_sample_index.min(cursor_sample_index)
                                                        ..start_sample_index
                                                            .max(cursor_sample_index),
                                                )
                                            } else {
                                                None
                                            }
                                        }
                                    },
                                }
                                .attach_event(move |event| {
                                    if let DragState::None = *drag_state {
                                        if let Event::MouseDown { event } = event {
                                            if !event.is_local_xy_in() {
                                                return;
                                            }

                                            set_drag_state.set(DragState::Dragging {
                                                track_index,
                                                start_sample_index: sample_index_on_x(
                                                    event.local_xy().x,
                                                ),
                                                cursor_sample_index: sample_index_on_x(
                                                    event.local_xy().x,
                                                ),
                                            });
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
                                if let DragState::Dragging {
                                    track_index,
                                    start_sample_index,
                                    cursor_sample_index: _,
                                } = *drag_state
                                {
                                    set_drag_state.set(DragState::Dragging {
                                        track_index,
                                        start_sample_index,
                                        cursor_sample_index: sample_index_on_x(event.local_xy().x),
                                    });
                                }
                            }
                            Event::MouseUp { event: _ } => {
                                if let DragState::Dragging {
                                    track_index: _,
                                    start_sample_index: _,
                                    cursor_sample_index: _,
                                } = *drag_state
                                {
                                    // TODO
                                    set_drag_state.set(DragState::None);
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
