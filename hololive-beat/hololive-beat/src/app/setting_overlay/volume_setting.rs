use crate::app::{setting_overlay::slider::Slider, theme::THEME};
use namui::*;
use namui_prebuilt::{table::*, typography::adjust_font_size};

#[component]
pub struct VolumeSetting {
    pub wh: Wh<Px>,
}
impl Component for VolumeSetting {
    fn render(self, ctx: &RenderCtx)  {
        let Self { wh } = self;

        const SLIDER_HEIGHT: Px = px(32.0);

        let volume = namui::media::volume();
        let padding = wh.height / 8;

        ctx.compose(|ctx| {
            vertical([
                ratio(1, |wh, ctx| {
                    ctx.add(text(TextParam {
                        text: "Volume".to_string(),
                        x: wh.width / 2,
                        y: wh.height / 2,
                        align: TextAlign::Center,
                        baseline: TextBaseline::Middle,
                        font: Font {
                            size: adjust_font_size(wh.height),
                            name: THEME.font_name.to_string(),
                        },
                        style: TextStyle {
                            color: THEME.text,
                            ..Default::default()
                        },
                        max_width: None,
                    }));
                }),
                ratio(
                    1,
                    vertical([
                        ratio(1, |_, _| {}),
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
                    ]),
                ),
            ])(wh, ctx);
        });

        
    }
}
