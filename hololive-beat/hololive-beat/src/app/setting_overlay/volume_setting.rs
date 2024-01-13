use namui::prelude::*;
use namui_prebuilt::{
    table::{self, hooks::*},
    typography::{self, text_fit},
};

use crate::app::setting_overlay::slider::Slider;

#[component]
pub struct VolumeSetting {
    pub wh: Wh<Px>,
}
impl Component for VolumeSetting {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh } = self;

        const SLIDER_HEIGHT: Px = px(32.0);

        let volume = namui::media::volume();
        let padding = wh.height / 16;

        ctx.compose(|ctx| {
            vertical([
                ratio(
                    1,
                    table::hooks::padding(padding, |wh, ctx| {
                        ctx.add(typography::center_text_full_height(
                            wh,
                            "Volume",
                            Color::WHITE,
                        ));
                    }),
                ),
                ratio(
                    1,
                    table::hooks::padding(
                        padding,
                        vertical([
                            ratio(2, |_, _| {}),
                            fixed(
                                SLIDER_HEIGHT,
                                horizontal_padding(padding, |wh, ctx| {
                                    ctx.add(Slider {
                                        wh,
                                        value: volume,
                                        min: 0.0,
                                        max: 1.0,
                                        on_change: &|value| {
                                            namui::media::set_volume(value);
                                        },
                                    });
                                }),
                            ),
                            ratio(1, |_, _| {}),
                            ratio(2, |wh, ctx| {
                                horizontal([
                                    fit(
                                        FitAlign::LeftTop,
                                        text_fit(wh.height, "min", Color::WHITE, 0.px()),
                                    ),
                                    ratio(1, |_, _| {}),
                                    fit(
                                        FitAlign::LeftTop,
                                        text_fit(wh.height, "max", Color::WHITE, 0.px()),
                                    ),
                                ])(wh, ctx);
                            }),
                        ]),
                    ),
                ),
            ])(wh, ctx);
        });

        ctx.done()
    }
}
