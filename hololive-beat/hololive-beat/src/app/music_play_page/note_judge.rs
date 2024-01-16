use crate::app::{
    note::{Direction, Note},
    play_state::{PlayState, PlayTimeState, PLAY_STATE_ATOM},
};
use namui::prelude::*;

use super::judge_indicator::indicate_judge;

#[component]
pub struct NoteJudge<'a> {
    pub notes: &'a Vec<Note>,
    pub played_time: Duration,
    pub perfect_range: Duration,
    pub good_range: Duration,
    pub note_sounds: &'a Vec<FullLoadOnceAudio>,
}

impl Component for NoteJudge<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            notes,
            played_time,
            perfect_range,
            good_range,
            note_sounds,
        } = self;

        let (state, set_state) = ctx.atom(&PLAY_STATE_ATOM);

        ctx.compose(|ctx| {
            let PlayState::Loaded {
                judge_context,
                play_time_state: PlayTimeState::Playing { .. },
                ..
            } = &*state
            else {
                return;
            };

            let handle_passed_notes = || {
                let last_judged_note_index = judge_context.last_judged_note_index;
                let check_start_time = last_judged_note_index
                    .and_then(|index| notes.get(index))
                    .map(|last_judged_note| last_judged_note.start_time - good_range)
                    .unwrap_or(0.ms());
                let check_end_time = played_time - good_range;

                let mut passed_note_indexes = Vec::new();
                for (index, note) in notes.iter().enumerate() {
                    if note.start_time < check_start_time {
                        continue;
                    }
                    if note.start_time > check_end_time {
                        break;
                    }

                    if judge_context.judged_note_index.contains(&index) {
                        continue;
                    }
                    passed_note_indexes.push(index);
                }

                if passed_note_indexes.is_empty() {
                    return;
                }
                set_state.mutate(move |state| {
                    let PlayState::Loaded {
                        judge_context,
                        play_time_state: PlayTimeState::Playing { .. },
                        ..
                    } = &mut *state
                    else {
                        return;
                    };
                    judge_context.miss_count += passed_note_indexes.len();
                    judge_context.combo = 0;
                    judge_context.last_judged_note_index = passed_note_indexes.last().cloned();
                    for index in passed_note_indexes {
                        judge_context.judged_note_index.insert(index);
                    }
                    indicate_judge(super::judge_indicator::Judge::Miss);
                });
            };

            let handle_key_down = |key: Code| {
                let Ok(direction) = Direction::try_from(key) else {
                    return;
                };

                let check_start_time = played_time - good_range;
                let check_end_time = played_time + good_range;
                for (index, note) in notes.iter().enumerate() {
                    if note.start_time < check_start_time {
                        continue;
                    }
                    if note.start_time > check_end_time {
                        break;
                    }

                    if note.direction.lane() != direction.lane() {
                        continue;
                    }

                    if judge_context.judged_note_index.contains(&index) {
                        continue;
                    }

                    let time_diff = (played_time - note.start_time).abs();
                    if time_diff > good_range {
                        continue;
                    }

                    note_sounds.get(index).cloned().unwrap().play().unwrap();

                    if time_diff <= perfect_range {
                        set_state.mutate(move |state| {
                            let PlayState::Loaded {
                                judge_context,
                                play_time_state: PlayTimeState::Playing { .. },
                                ..
                            } = &mut *state
                            else {
                                return;
                            };
                            judge_context.perfect_count += 1;
                            judge_context.combo += 1;
                            judge_context.max_combo =
                                judge_context.max_combo.max(judge_context.combo);
                            judge_context.judged_note_index.insert(index);
                            judge_context.last_judged_note_index = Some(index);
                            indicate_judge(super::judge_indicator::Judge::Perfect {
                                combo: judge_context.combo,
                            });
                        });
                        break;
                    }

                    set_state.mutate(move |state| {
                        let PlayState::Loaded {
                            judge_context,
                            play_time_state: PlayTimeState::Playing { .. },
                            ..
                        } = &mut *state
                        else {
                            return;
                        };
                        judge_context.good_count += 1;
                        judge_context.combo += 1;
                        judge_context.max_combo = judge_context.max_combo.max(judge_context.combo);
                        judge_context.judged_note_index.insert(index);
                        judge_context.last_judged_note_index = Some(index);
                        indicate_judge(super::judge_indicator::Judge::Good {
                            combo: judge_context.combo,
                        });
                    });
                    break;
                }
            };

            ctx.add(RenderingTree::Empty.attach_event(|event| {
                if let Event::ScreenRedraw = event {
                    handle_passed_notes();
                } else if let Event::KeyDown { event } = event {
                    handle_key_down(event.code);
                }
            }));
        });

        ctx.done()
    }
}
