mod game_ender;
mod note_judge;
mod note_plotter;
mod video_player;

use self::game_ender::GameEnder;

use super::play_state::{LoadedData, PlayState, PLAY_STATE_ATOM};
use crate::app::{
    drummer::Drummer,
    music_play_page::{
        note_judge::NoteJudge, note_plotter::NotePlotter, video_player::VideoPlayer,
    },
    play_state::pause_game,
    setting_overlay::open_setting_overlay,
};
use namui::prelude::*;

#[component]
pub struct MusicPlayPage {
    pub wh: Wh<Px>,
}
impl Component for MusicPlayPage {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh } = self;

        let (state, _set_state) = ctx.atom_init(&PLAY_STATE_ATOM, PlayState::default);
        let now = namui::time::since_start();

        let played_time = match &*state {
            PlayState::Loaded {
                play_time_state, ..
            } => match play_time_state {
                super::play_state::PlayTimeState::Playing {
                    started_time,
                    played_time,
                } => now - started_time + played_time,
                super::play_state::PlayTimeState::Paused { played_time } => *played_time,
                // TODO: music length
                super::play_state::PlayTimeState::Ended => 0.ms(),
            },
            _ => 0.ms(),
        };

        ctx.compose(|ctx| match &*state {
            PlayState::Idle => (),
            PlayState::Loading { .. } => {
                ctx.add(Loading { wh });
            }
            PlayState::Loaded { loaded, .. } => {
                ctx.add(Loaded {
                    wh,
                    played_time,
                    loaded_data: loaded,
                });
            }
        });

        ctx.component(RenderingTree::Empty.attach_event(|event| {
            let Event::KeyDown { event } = event else {
                return;
            };
            if !matches!(event.code, Code::Escape) {
                return;
            }
            pause_game(now);
            open_setting_overlay();
        }));

        ctx.component(GameEnder { played_time });

        ctx.done()
    }
}

#[component]
struct Loading {
    wh: Wh<Px>,
}
impl Component for Loading {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        // TODO
        ctx.done()
    }
}

#[component]
struct Loaded<'a> {
    wh: Wh<Px>,
    played_time: Duration,
    loaded_data: &'a LoadedData,
}
impl Component for Loaded<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            played_time,
            loaded_data,
        } = self;
        let LoadedData {
            notes,
            note_sounds,
            video,
            ..
        } = &loaded_data;

        const NOTE_PLOTTER_HEIGHT: Px = px(256.0);
        const NOTE_WIDTH: Px = px(64.0);
        const TIMING_ZERO_X: Px = px(192.0);
        const DRUMMER_WIDTH: Px = px(384.0);
        let px_per_time = Per::new(NOTE_WIDTH, 33.ms() * 2);
        let perfect_range: Duration = Duration::from_millis(64);
        let good_range: Duration = 256.0.ms();

        ctx.compose(|ctx| {
            let note_plotter_y = wh.height - NOTE_PLOTTER_HEIGHT;
            let drummer_wh = Wh {
                width: DRUMMER_WIDTH,
                height: NOTE_PLOTTER_HEIGHT,
            };
            let note_plotter_wh = Wh {
                width: wh.width - DRUMMER_WIDTH,
                height: NOTE_PLOTTER_HEIGHT,
            };
            ctx.translate((0.px(), note_plotter_y))
                .add(Drummer { wh: drummer_wh })
                .translate((DRUMMER_WIDTH, 0.px()))
                .add(NotePlotter {
                    wh: note_plotter_wh,
                    notes,
                    px_per_time,
                    timing_zero_x: TIMING_ZERO_X,
                    played_time,
                    note_width: NOTE_WIDTH,
                });

            ctx.add(VideoPlayer {
                wh,
                video,
                note_plotter_height: NOTE_PLOTTER_HEIGHT,
            });
        });

        ctx.component(NoteJudge {
            notes,
            played_time,
            perfect_range,
            good_range,
            note_sounds,
        });

        ctx.done()
    }
}
