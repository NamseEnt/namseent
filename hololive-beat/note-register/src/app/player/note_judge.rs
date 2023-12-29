use super::{State, STATE};
use crate::app::note::{Direction, Note};
use namui::prelude::*;
use namui_prebuilt::simple_rect;
use std::collections::HashSet;

const PERFECT_RANGE: Time = Time::Ms(64.0);
const GOOD_RANGE: Time = Time::Ms(256.0);

static JUDGE_CONTEXT: Atom<JudgeContext> = Atom::uninitialized_new();

#[component]
pub struct NoteJudge<'a> {
    pub notes: &'a Vec<Note>,
    pub played_time: Time,
}

impl Component for NoteJudge<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { notes, played_time } = self;

        let (state, _) = ctx.atom(&STATE);
        let (judge_context, set_judge_context) = ctx.atom_init(&JUDGE_CONTEXT, JudgeContext::new);

        let handle_passed_notes = || {
            let last_judged_note_index = judge_context.last_judged_note_index;
            let check_start_time = last_judged_note_index
                .and_then(|index| notes.get(index))
                .map(|last_judged_note| last_judged_note.time - GOOD_RANGE)
                .unwrap_or(Time::Ms(0.0));
            let check_end_time = played_time - GOOD_RANGE;

            let mut passed_note_indexes = Vec::new();
            for (index, note) in notes.iter().enumerate() {
                if note.time < check_start_time {
                    continue;
                }
                if note.time > check_end_time {
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

            let check_start_time = played_time - GOOD_RANGE;
            let check_end_time = played_time + GOOD_RANGE;
            for (index, note) in notes.iter().enumerate() {
                if note.time < check_start_time {
                    continue;
                }
                if note.time > check_end_time {
                    break;
                }

                if note.direction.lane() != direction.lane() {
                    continue;
                }

                if judge_context.judged_note_index.contains(&index) {
                    continue;
                }

                if note.direction != direction {
                    set_judge_context.mutate(move |judge_context| {
                        judge_context.miss_count += 1;
                        judge_context.combo = 0;
                        judge_context.judged_note_index.insert(index);
                        judge_context.last_judged_note_index = Some(index);
                    })
                }

                let time_diff_ms = (note.time - played_time).as_millis().abs();

                if time_diff_ms <= PERFECT_RANGE.as_millis() {
                    set_judge_context.mutate(move |judge_context| {
                        judge_context.perfect_count += 1;
                        judge_context.combo += 1;
                        judge_context.max_combo = judge_context.max_combo.max(judge_context.combo);
                        judge_context.judged_note_index.insert(index);
                        judge_context.last_judged_note_index = Some(index);
                    })
                } else if time_diff_ms <= GOOD_RANGE.as_millis() {
                    set_judge_context.mutate(move |judge_context| {
                        judge_context.good_count += 1;
                        judge_context.combo += 1;
                        judge_context.max_combo = judge_context.max_combo.max(judge_context.combo);
                        judge_context.judged_note_index.insert(index);
                        judge_context.last_judged_note_index = Some(index);
                    })
                }
                namui::log!("{judge_context:#?}");
            }
        };

        ctx.component(
            simple_rect(Wh::zero(), Color::TRANSPARENT, 0.px(), Color::TRANSPARENT).attach_event(
                |event| {
                    if !matches!(*state, State::Play { .. }) {
                        return;
                    }

                    if let Event::ScreenRedraw = event {
                        handle_passed_notes();
                    } else if let Event::KeyDown { event } = event {
                        handle_key_down(event.code);
                    }
                },
            ),
        );

        ctx.done()
    }
}

#[derive(Debug)]
struct JudgeContext {
    perfect_count: usize,
    good_count: usize,
    miss_count: usize,
    max_combo: usize,
    combo: usize,
    last_judged_note_index: Option<usize>,
    judged_note_index: HashSet<usize>,
}
impl JudgeContext {
    fn new() -> Self {
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
