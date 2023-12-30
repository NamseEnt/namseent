use super::STATE;
use namui::{prelude::*, time::now};

#[component]
pub struct MusicPlayer {}

impl Component for MusicPlayer {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let (state, _) = ctx.atom(&STATE);
        let (music, set_music) = ctx.state::<Option<MediaHandle>>(|| None);

        ctx.effect("load music", || match *state {
            super::State::Stop => {
                if let Some(music) = music.as_ref() {
                    music.stop();
                }
            }
            super::State::Play { started_time } => {
                if let Some(music) = music.as_ref() {
                    music.play(started_time);
                }
            }
            super::State::Pause { played_time } => {
                if let Some(music) = music.as_ref() {
                    music.play(now() - played_time);
                }
            }
        });

        // TODO: Load audio from outside. Need to expose `namui::system::media::MediaHandle` first
        ctx.effect("load music", || {
            namui::spawn(async move {
                let path =
                    namui::system::file::bundle::to_real_path("bundle:you_re_mine.opus").unwrap();
                let music = namui::system::media::new_media(&path).unwrap();
                set_music.set(Some(music));
            });
        });

        ctx.done()
    }
}
