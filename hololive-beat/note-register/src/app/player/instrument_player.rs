use crate::app::note::Direction;
use namui::{prelude::*, time::now};
use namui_prebuilt::simple_rect;

#[component]
pub struct InstrumentPlayer<'a> {
    pub kick: &'a MediaHandle,
    pub cymbals: &'a MediaHandle,
    pub snare: &'a MediaHandle,
}

impl Component for InstrumentPlayer<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            kick,
            cymbals,
            snare,
        } = self;

        ctx.component(
            simple_rect(Wh::zero(), Color::TRANSPARENT, 0.px(), Color::TRANSPARENT).attach_event(
                |event| {
                    if let Event::KeyDown { event } = event {
                        let Ok(direction) = Direction::try_from(event.code) else {
                            return;
                        };

                        match direction.as_instrument() {
                            crate::app::note::Instrument::Kick => {
                                kick.clone_independent().unwrap().play(now());
                            }
                            crate::app::note::Instrument::Snare => {
                                snare.clone_independent().unwrap().play(now());
                            }
                            crate::app::note::Instrument::Cymbals => {
                                cymbals.clone_independent().unwrap().play(now());
                            }
                        }
                    }
                },
            ),
        )
        .done()
    }
}
