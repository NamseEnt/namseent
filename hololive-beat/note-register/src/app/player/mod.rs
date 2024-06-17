mod music_player;
mod note_judge;
mod note_plotter;
mod slider;
mod video_player;

use self::{note_judge::JudgeContext, note_plotter::NotePlotter};
use crate::app::{
    color::THEME,
    player::{
        music_player::MusicPlayer, note_judge::NoteJudge, slider::Slider, video_player::VideoPlayer,
    },
    LoadedData,
};
use namui::{prelude::*, time::since_start};
use namui_prebuilt::{button::TextButtonFit, table::*};

static STATE: Atom<State> = Atom::uninitialized_new();
static JUDGE_CONTEXT: Atom<JudgeContext> = Atom::uninitialized_new();

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
            note_sounds,
            music,
            video,
        } = loaded;

        const NOTE_PLOTTER_HEIGHT: Px = px(256.0);
        const BUTTON_HEIGHT: Px = px(64.0);
        const STROKE_WIDTH: Px = px(2.0);
        const BUTTON_SIDE_PADDING: Px = px(16.0);
        const NOTE_WIDTH: Px = px(64.0);
        const TIMING_ZERO_X: Px = px(192.0);

        let (state, set_state) = ctx.atom_init(&STATE, || State::Stop);
        let _ = ctx.atom_init(&JUDGE_CONTEXT, JudgeContext::new);
        let (start_offset_ms, set_start_offset) = ctx.state(|| 0.0);
        let px_per_time = Per::new(NOTE_WIDTH, 33.ms() * 2);
        let now = since_start();
        let perfect_range: Duration = Duration::from_millis(64);
        let good_range: Duration = 256.0.ms();

        let played_time = match *state {
            State::Stop => 0.ms(),
            State::Play {
                started_time,
                played_time,
            } => now - started_time + played_time,
            State::Pause { played_time } => played_time,
        };

        ctx.compose(|ctx| {
            vertical([
                ratio(1, |wh, ctx| {
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
                                            started_time: now,
                                            played_time: (*start_offset_ms as f64).ms(),
                                        }),
                                        State::Play { .. } => {
                                            set_state.set(State::Pause { played_time })
                                        }
                                        State::Pause { played_time } => {
                                            set_state.set(State::Play {
                                                started_time: now,
                                                played_time,
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
                                min: 0.sec().as_secs_f32() * 1000.0,
                                max: 227.sec().as_secs_f32() * 1000.0,
                                on_change: &|value| {
                                    set_start_offset.set(value);
                                    namui::log!("set: {value}");
                                },
                            });
                        }),
                    ])(wh, ctx);
                    ctx.add(VideoPlayer { wh, video });
                }),
                fixed(
                    NOTE_PLOTTER_HEIGHT,
                    horizontal([
                        fixed(384.px(), |_, _| {
                            // character
                        }),
                        ratio(1, |wh, ctx| {
                            ctx.add(NotePlotter {
                                wh,
                                notes,
                                px_per_time,
                                timing_zero_x: TIMING_ZERO_X,
                                played_time,
                                note_width: NOTE_WIDTH,
                            });
                        }),
                    ]),
                ),
            ])(wh, ctx);
        });

        ctx.component(NoteJudge {
            notes,
            played_time,
            perfect_range,
            good_range,
            note_sounds,
        });

        ctx.component(MusicPlayer { music });

        
    }
}

#[derive(Debug)]
enum State {
    Stop,
    Play {
        /// It's App time.
        started_time: Duration,
        played_time: Duration,
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
