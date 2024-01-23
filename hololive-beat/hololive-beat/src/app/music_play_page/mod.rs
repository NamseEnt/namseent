mod game_ender;
mod game_result_overlay;
mod judge_indicator;
mod note_judge;
mod note_plotter;
mod video_player;

use self::game_ender::GameEnder;
use super::{
    music::MusicSpeedMap,
    play_state::{JudgeContext, LoadedData, PlayState, PlayTimeState, PLAY_STATE_ATOM},
};
use crate::app::{
    drummer::Drummer,
    music_play_page::{
        game_result_overlay::GameResultOverlay, judge_indicator::JudgeIndicator,
        note_judge::NoteJudge, note_plotter::NotePlotter, video_player::VideoPlayer,
    },
    play_state::pause_game,
    setting_overlay::open_setting_overlay,
};
use namui::prelude::*;

#[component]
pub struct MusicPlayPage<'a> {
    pub wh: Wh<Px>,
    pub music_speed_map: Option<&'a MusicSpeedMap>,
}
impl Component for MusicPlayPage<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            music_speed_map,
        } = self;

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
            PlayState::Loaded {
                loaded,
                play_time_state,
                judge_context,
                music,
                ..
            } => {
                let px_per_time = {
                    let speed = music_speed_map
                        .map(|music_speed_map| music_speed_map.get(&music.id))
                        .unwrap_or_default();
                    Per::new(speed * 256.0.px(), 1.sec())
                };
                Per::new(256.px(), 1.sec());
                ctx.add(Loaded {
                    wh,
                    played_time,
                    loaded_data: loaded,
                    play_time_state,
                    judge_context,
                    music_id: &music.id,
                    px_per_time,
                });
            }
        });

        ctx.on_raw_event(|event| {
            let RawEvent::KeyDown { event } = event else {
                return;
            };
            if !matches!(event.code, Code::Escape) {
                return;
            }
            pause_game(now);
            open_setting_overlay();
        });

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
    play_time_state: &'a PlayTimeState,
    judge_context: &'a JudgeContext,
    music_id: &'a str,
    px_per_time: Per<Px, Duration>,
}
impl Component for Loaded<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            played_time,
            loaded_data,
            play_time_state,
            judge_context,
            music_id,
            px_per_time,
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
        let perfect_range: Duration = Duration::from_millis(64);
        let good_range: Duration = 256.0.ms();

        ctx.compose(|ctx| {
            if !matches!(play_time_state, PlayTimeState::Ended) {
                return;
            }
            ctx.add(GameResultOverlay {
                wh,
                judge_context,
                music_id,
            });
        });

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
            let judge_indicator_wh = Wh {
                width: NOTE_PLOTTER_HEIGHT * 2,
                height: NOTE_PLOTTER_HEIGHT,
            };
            ctx.translate((drummer_wh.width, note_plotter_y))
                .scale(Xy::new(-1.0, 1.0))
                .add(Drummer { wh: drummer_wh });

            ctx.translate((
                DRUMMER_WIDTH + note_plotter_wh.width - judge_indicator_wh.width,
                note_plotter_y,
            ))
            .add(JudgeIndicator {
                wh: judge_indicator_wh,
            });

            ctx.translate((DRUMMER_WIDTH, note_plotter_y))
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
