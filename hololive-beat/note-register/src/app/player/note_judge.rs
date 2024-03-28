use super::{State, JUDGE_CONTEXT, STATE};
use crate::app::note::{Direction, Note};
use namui::*;
use namui_prebuilt::simple_rect;
use std::collections::HashSet;

const AUTO_PLAY: bool = false;

#[component]
pub struct NoteJudge<'a> {
    pub notes: &'a Vec<Note>,
    pub played_time: Duration,
    pub perfect_range: Duration,
    pub good_range: Duration,
    pub note_sounds: &'a Vec<FullLoadOnceAudio>,
}

impl Component for NoteJudge<'_> {
    fn render(self, ctx: &RenderCtx)  {
        let Self {
            notes,
            played_time,
            perfect_range,
            good_range,
            note_sounds,
        } = self;

        let (state, _) = ctx.atom(&STATE);
        let (judge_context, set_judge_context) = ctx.atom(&JUDGE_CONTEXT);

        let auto_play = || {
            let check_start_time = played_time - perfect_range;
            let check_end_time = played_time + perfect_range;
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

                note_sounds.get(index).cloned().unwrap().play().unwrap();

                set_judge_context.mutate(move |judge_context| {
                    judge_context.perfect_count += 1;
                    judge_context.combo += 1;
                    judge_context.max_combo = judge_context.max_combo.max(judge_context.combo);
                    judge_context.judged_note_index.insert(index);
                    judge_context.last_judged_note_index = Some(index);
                });
            }
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
            set_judge_context.mutate(move |judge_context| {
                judge_context.miss_count += passed_note_indexes.len();
                judge_context.combo = 0;
                judge_context.last_judged_note_index = passed_note_indexes.last().cloned();
                for index in passed_note_indexes {
                    judge_context.judged_note_index.insert(index);
                }
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
                    set_judge_context.mutate(move |judge_context| {
                        judge_context.perfect_count += 1;
                        judge_context.combo += 1;
                        judge_context.max_combo = judge_context.max_combo.max(judge_context.combo);
                        judge_context.judged_note_index.insert(index);
                        judge_context.last_judged_note_index = Some(index);
                    });
                    break;
                }

                set_judge_context.mutate(move |judge_context| {
                    judge_context.good_count += 1;
                    judge_context.combo += 1;
                    judge_context.max_combo = judge_context.max_combo.max(judge_context.combo);
                    judge_context.judged_note_index.insert(index);
                    judge_context.last_judged_note_index = Some(index);
                });
                break;
            }
        };

        ctx.component(
            simple_rect(Wh::zero(), Color::TRANSPARENT, 0.px(), Color::TRANSPARENT).attach_event(
                |event| {
                    if !matches!(*state, State::Play { .. }) {
                        return;
                    }

                    if let Event::ScreenRedraw = event {
                        if AUTO_PLAY {
                            auto_play();
                        }
                        handle_passed_notes();
                    } else if let Event::KeyDown { event } = event {
                        handle_key_down(event.code);
                    }
                },
            ),
        );

        
    }
}

#[derive(Debug)]
pub struct JudgeContext {
    pub perfect_count: usize,
    pub good_count: usize,
    pub miss_count: usize,
    pub max_combo: usize,
    pub combo: usize,
    pub last_judged_note_index: Option<usize>,
    pub judged_note_index: HashSet<usize>,
}
impl JudgeContext {
    pub fn new() -> Self {
        Self {
            perfect_count: 0,
            good_count: 0,
            miss_count: 0,
            max_combo: 0,
            combo: 0,
            last_judged_note_index: None,
            judged_note_index: HashSet::new(),
        }
    }
}
