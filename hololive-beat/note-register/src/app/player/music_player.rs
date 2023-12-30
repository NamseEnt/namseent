use super::STATE;
use namui::{prelude::*, time::now};

#[component]
pub struct MusicPlayer<'a> {
    pub music: &'a MediaHandle,
}

impl Component for MusicPlayer<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { music } = self;

        let (state, _) = ctx.atom(&STATE);

        ctx.effect("load music", || match *state {
            super::State::Stop => {
                music.stop();
            }
            super::State::Play { started_time } => {
                music.play(started_time);
            }
            super::State::Pause { played_time } => {
                music.play(now() - played_time);
            }
        });

        ctx.done()
    }
}
