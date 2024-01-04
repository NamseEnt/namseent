mod color;
mod note;
mod player;

use self::{color::THEME, player::Player};
use crate::app::note::load_notes;
use futures::join;
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
                let music = load_media("bundle:you_re_mine.opus");
                let video = load_media("bundle:you_re_mine.mp4");
                let (notes, kick, cymbals, snare) = join!(
                    load_notes(),
                    load_full_load_once_audio("bundle:kick.opus"),
                    load_full_load_once_audio("bundle:cymbals.opus"),
                    load_full_load_once_audio("bundle:snare.opus"),
                );

                let note_sounds = {
                    notes
                        .iter()
                        .map(|note| {
                            let instrument = match note.instrument {
                                note::Instrument::Kick => &kick,
                                note::Instrument::Snare => &snare,
                                note::Instrument::Cymbals => &cymbals,
                            };
                            instrument.slice(note.start_time..note.end_time).unwrap()
                        })
                        .collect()
                };

                set_loaded.set(Some(LoadedData {
                    notes,
                    note_sounds,
                    music,
                    video,
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
    note_sounds: Vec<FullLoadOnceAudio>,
    music: MediaHandle,
    video: MediaHandle,
}

fn load_media(path: &str) -> MediaHandle {
    let path = namui::system::file::bundle::to_real_path(path).unwrap();
    namui::system::media::new_media(&path).unwrap()
}

async fn load_full_load_once_audio(path: &str) -> FullLoadOnceAudio {
    let path = namui::system::file::bundle::to_real_path(path).unwrap();
    namui::system::media::new_full_load_once_audio(&path)
        .await
        .unwrap()
}
