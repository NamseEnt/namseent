use crate::app::play_state::{stop_game, PlayState, PlayTimeState, PLAY_STATE_ATOM};
use namui::prelude::*;

#[component]
pub struct GameEnder {
    pub played_time: Duration,
}
impl Component for GameEnder {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { played_time } = self;

        let (state, _) = ctx.atom(&PLAY_STATE_ATOM);

        ctx.component(RenderingTree::Empty.attach_event(|event| {
            let Event::ScreenRedraw = event else {
                return;
            };
            let PlayState::Loaded {
                music,
                play_time_state: PlayTimeState::Playing { .. },
                ..
            } = &*state
            else {
                return;
            };
            if played_time > music.length.sec() {
                stop_game();
            }
        }));

        ctx.done()
    }
}
