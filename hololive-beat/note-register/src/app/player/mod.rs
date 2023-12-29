mod note_judge;
mod note_plotter;

use self::note_plotter::NotePlotter;
use super::note::Note;
use crate::app::{color::THEME, player::note_judge::NoteJudge};
use namui::prelude::*;
use namui_prebuilt::{button::TextButtonFit, table::hooks::*};

static STATE: Atom<State> = Atom::uninitialized_new();

#[namui::component]
pub struct Player<'a> {
    pub wh: Wh<Px>,
    pub notes: &'a Vec<Note>,
}
impl Component for Player<'_> {
    fn render(self, ctx: &namui::prelude::RenderCtx) -> namui::prelude::RenderDone {
        let Self { wh, notes } = self;

        const TIMING_ZERO_X: Px = px(256.0);
        const NOTE_PLOTTER_HEIGHT: Px = px(384.0);
        const BUTTON_HEIGHT: Px = px(64.0);
        const STROKE_WIDTH: Px = px(2.0);
        const BUTTON_SIDE_PADDING: Px = px(16.0);

        let (state, set_state) = ctx.atom_init(&STATE, || State::Stop);
        let px_per_time = Per::new(px(512.0), Time::Sec(1.0));
        let now = now();

        let played_time = match *state {
            State::Stop => Time::Sec(0.0),
            State::Play { started_time } => now - started_time,
            State::Pause {
                played_time: paused_time,
            } => paused_time,
        };

        ctx.compose(|ctx| {
            vertical([
                fixed(NOTE_PLOTTER_HEIGHT, |wh, ctx| {
                    ctx.add(NotePlotter {
                        wh,
                        notes,
                        px_per_time,
                        timing_zero_x: TIMING_ZERO_X,
                        played_time,
                    });
                }),
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
                                State::Stop => set_state.set(State::Play { started_time: now }),
                                State::Play { .. } => set_state.set(State::Pause { played_time }),
                                State::Pause { played_time } => set_state.set(State::Play {
                                    started_time: now - played_time,
                                }),
                            }
                        },
                    });
                }),
            ])(wh, ctx);
        });

        ctx.component(NoteJudge { notes, played_time });

        ctx.done()
    }
}

#[derive(Debug)]
enum State {
    Stop,
    Play {
        /// It's App time.
        started_time: Time,
    },
    Pause {
        /// But It's Note map time.
        played_time: Time,
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
