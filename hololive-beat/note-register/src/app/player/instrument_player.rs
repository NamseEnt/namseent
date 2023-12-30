use crate::app::note::Direction;
use namui::{prelude::*, time::now};
use namui_prebuilt::simple_rect;

#[component]
pub struct InstrumentPlayer {}

impl Component for InstrumentPlayer {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let (kick_audio, set_kick_audio) = ctx.state(|| None);
        let (cymbals_audio, set_cymbals_audio) = ctx.state(|| None);
        let (snare_audio, set_snare_audio) = ctx.state(|| None);

        // TODO: Load audio from outside. Need to expose `namui::system::media::MediaHandle` first
        ctx.effect("load audio", || {
            namui::spawn(async move {
                let path = namui::system::file::bundle::to_real_path("bundle:kick.mp3").unwrap();
                let audio = namui::system::media::new_media(&path).unwrap();
                set_kick_audio.set(Some(audio));
            });
            namui::spawn(async move {
                let path = namui::system::file::bundle::to_real_path("bundle:cymbals.mp3").unwrap();
                let audio = namui::system::media::new_media(&path).unwrap();
                set_cymbals_audio.set(Some(audio));
            });
            namui::spawn(async move {
                let path = namui::system::file::bundle::to_real_path("bundle:snare.mp3").unwrap();
                let audio = namui::system::media::new_media(&path).unwrap();
                set_snare_audio.set(Some(audio));
            });
        });

        ctx.component(
            simple_rect(Wh::zero(), Color::TRANSPARENT, 0.px(), Color::TRANSPARENT).attach_event(
                |event| {
                    if let Event::KeyDown { event } = event {
                        let Ok(direction) = Direction::try_from(event.code) else {
                            return;
                        };

                        match direction.as_instrument() {
                            crate::app::note::Instrument::Kick => {
                                if let Some(audio) = kick_audio.as_ref() {
                                    audio.clone_independent().unwrap().play(now());
                                }
                            }
                            crate::app::note::Instrument::Snare => {
                                if let Some(audio) = snare_audio.as_ref() {
                                    audio.clone_independent().unwrap().play(now());
                                }
                            }
                            crate::app::note::Instrument::Cymbals => {
                                if let Some(audio) = cymbals_audio.as_ref() {
                                    audio.clone_independent().unwrap().play(now());
                                }
                            }
                        }
                    }
                },
            ),
        )
        .done()
    }
}
