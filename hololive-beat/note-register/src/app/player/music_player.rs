use super::STATE;
use namui::{prelude::*, time::since_start};

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
                music.stop().unwrap();
            }
            super::State::Play {
                started_time,
                played_time,
            } => {
                let music = music.clone();
                namui::spawn(async move {
                    let offset = since_start() - started_time + played_time;
                    music.seek_to(offset).unwrap();
                    music.wait_for_preload().await.unwrap();
                    music.play().unwrap();
                });
            }
            super::State::Pause { played_time } => {
                music.pause().unwrap();
                music.seek_to(played_time).unwrap();
            }
        });

        ctx.done()
    }
}
