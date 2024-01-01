mod color;
mod note;
mod player;

use self::{color::THEME, player::Player};
use crate::app::note::load_notes;
use namui::prelude::*;
use namui_prebuilt::simple_rect;

#[namui::component]
pub struct App {}
impl namui::Component for App {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let (loaded, set_loaded) = ctx.state(|| None);
        let wh = screen::size().into_type::<Px>();

        ctx.effect("Init", || {
            spawn(async move {
                set_loaded.set(Some(LoadedData {
                    notes: load_notes().await,
                    kick: load_media("bundle:kick.mp3"),
                    cymbals: load_media("bundle:cymbals.mp3"),
                    snare: load_media("bundle:snare.mp3"),
                    music: load_media("bundle:you_re_mine.opus"),
                    video: load_media("bundle:you_re_mine.mp4"),
                }));
            });
        });

        ctx.compose(|ctx| {
            if let Some(loaded) = loaded.as_ref() {
                ctx.add(Player { wh, loaded });
            }
        });

        ctx.component(simple_rect(
            wh,
            Color::TRANSPARENT,
            0.px(),
            THEME.background.main,
        ));

        ctx.done()
    }
}

#[derive(Debug)]
pub struct LoadedData {
    notes: Vec<note::Note>,
    kick: MediaHandle,
    cymbals: MediaHandle,
    snare: MediaHandle,
    music: MediaHandle,
    video: MediaHandle,
}

fn load_media(path: &str) -> MediaHandle {
    let path = namui::system::file::bundle::to_real_path(path).unwrap();
    namui::system::media::new_media(&path).unwrap()
}
