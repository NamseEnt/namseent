mod instrument_player;
mod music_player;
mod note_judge;
mod note_plotter;
mod slider;
mod video_player;

use self::note_plotter::NotePlotter;
use crate::app::{
    color::THEME,
    player::{
        instrument_player::InstrumentPlayer, music_player::MusicPlayer, note_judge::NoteJudge,
        slider::Slider, video_player::VideoPlayer,
    },
    LoadedData,
};
use namui::{prelude::*, time::now};
use namui_prebuilt::{button::TextButtonFit, table::hooks::*};

static STATE: Atom<State> = Atom::uninitialized_new();

#[namui::component]
pub struct Player<'a> {
    pub wh: Wh<Px>,
    pub loaded: &'a LoadedData,
}
impl Component for Player<'_> {
    fn render(self, ctx: &namui::prelude::RenderCtx) -> namui::prelude::RenderDone {
        let Self { wh, loaded } = self;
        let LoadedData {
            notes,
            kick,
            cymbals,
            snare,
            music,
            video,
        } = loaded;

        const TIMING_ZERO_Y_FROM_BOTTOM: Px = px(192.0);
        const NOTE_PLOTTER_WIDTH: Px = px(768.0);
        const BUTTON_HEIGHT: Px = px(64.0);
        const STROKE_WIDTH: Px = px(2.0);
        const BUTTON_SIDE_PADDING: Px = px(16.0);

        let (state, set_state) = ctx.atom_init(&STATE, || State::Stop);
        let (start_offset_ms, set_start_offset) = ctx.state(|| 0.0);
        let px_per_time = Per::new(px(1024.0), 1.sec());
        let now = now();

        let played_time = match *state {
            State::Stop => 0.ms(),
            State::Play { started_time } => now - Instant::new(started_time),
            State::Pause {
                played_time: paused_time,
            } => paused_time,
        };

        ctx.compose(|ctx| {
            horizontal([
                ratio(
                    1,
                    vertical([
                        fixed(BUTTON_HEIGHT, |_, ctx| {
                            ctx.add(TextButtonFit {
                                height: BUTTON_HEIGHT,
                                text: state.button_text(),
                                text_color: THEME.primary.contrast_text,
                                stroke_color: THEME.primary.contrast_text,
                                stroke_width: STROKE_WIDTH,
                                fill_color: THEME.primary.main,
                                side_padding: BUTTON_SIDE_PADDING,
                                mouse_buttons: vec![MouseButton::Left],
                                on_mouse_up_in: &|event| {
                                    if !event.is_local_xy_in() {
                                        return;
                                    }
                                    match *state {
                                        State::Stop => set_state.set(State::Play {
                                            started_time: now
                                                - Instant::new((*start_offset_ms as f64).ms()),
                                        }),
                                        State::Play { .. } => {
                                            set_state.set(State::Pause { played_time })
                                        }
                                        State::Pause { played_time } => {
                                            set_state.set(State::Play {
                                                started_time: now - Instant::new(played_time),
                                            })
                                        }
                                    }
                                },
                            });
                        }),
                        fixed(BUTTON_HEIGHT, |wh, ctx| {
                            ctx.add(Slider {
                                wh,
                                value: *start_offset_ms,
                                min: 0.sec().as_secs_f32(),
                                max: 227.sec().as_secs_f32(),
                                on_change: &|value| {
                                    set_start_offset.set(value);
                                    namui::log!("set: {value}");
                                },
                            });
                        }),
                    ]),
                ),
                fixed(NOTE_PLOTTER_WIDTH, |wh, ctx| {
                    ctx.add(NotePlotter {
                        wh,
                        notes,
                        px_per_time,
                        timing_zero_y_from_bottom: TIMING_ZERO_Y_FROM_BOTTOM,
                        played_time,
                    });
                }),
                ratio(1, |_, _| {}),
            ])(wh, ctx);
        });

        ctx.component(NoteJudge { notes, played_time });

        ctx.component(InstrumentPlayer {
            kick,
            cymbals,
            snare,
        });

        ctx.component(MusicPlayer { music });

        ctx.component(VideoPlayer { wh, video });

        ctx.done()
    }
}

#[derive(Debug)]
enum State {
    Stop,
    Play {
        /// It's App time.
        started_time: Duration,
    },
    Pause {
        /// But It's Music time.
        played_time: Duration,
    },
}
impl State {
    fn button_text(&self) -> &'static str {
        match self {
            State::Stop => "Play",
            State::Play { .. } => "Pause",
            State::Pause { .. } => "Resume",
        }
    }
}
