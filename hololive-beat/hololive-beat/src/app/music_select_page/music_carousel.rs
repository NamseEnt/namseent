use crate::app::{
    components::{IconButton, TextButton},
    music::MusicMetadata,
    theme::THEME,
};
use keyframe::num_traits::Signed;
use namui::*;
use namui_prebuilt::{simple_rect, table::*, typography::adjust_font_size};
use std::f32::consts::PI;

#[component]
pub struct MusicCarousel<'a> {
    pub wh: Wh<Px>,
    pub musics: &'a Vec<MusicMetadata>,
    pub selected: f32,
}

impl Component for MusicCarousel<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            musics,
            selected,
        } = self;

        const PADDING: Px = px(8.0);

        // offset should be -0.5 ~ 0.5
        let offset = selected.round() - selected;
        let music_card_wh = {
            let height = wh.height * 0.8;
            let width = height * 16.0 / 9.0;
            Wh::new(width, height)
        };
        let musics_near_selected: [Option<&MusicMetadata>; 5] = {
            if musics.is_empty() {
                [None; 5]
            } else {
                let round = selected.round() - 2.0;
                let modulo = round % musics.len() as f32;
                let start_index = match modulo.is_positive() {
                    true => modulo as usize,
                    false => (modulo + musics.len() as f32) as usize,
                };
                let mut iter = musics.iter().cycle().skip(start_index);
                [
                    iter.next(),
                    iter.next(),
                    iter.next(),
                    iter.next(),
                    iter.next(),
                ]
            }
        };
        let offsets = [-2.0, -1.0, 0.0, 1.0, 2.0]
            .into_iter()
            .map(|main_offset| main_offset + offset);

        ctx.compose(|ctx| {
            ctx.translate((wh.width / 2, wh.height - 64.px()))
                .translate(((-128).px(), 0.px()))
                .add(TextButton {
                    wh: Wh::new(256.px(), 128.px()),
                    text: "start".to_string(),
                    on_click: &|| {},
                    focused: true,
                })
                .translate((256.px(), 32.px()))
                .add(EnterIcon {
                    wh: Wh::new(64.px(), 64.px()),
                });
        });

        ctx.compose(|ctx| {
            horizontal([
                ratio_no_clip(
                    1,
                    padding_no_clip(PADDING, |wh, ctx| {
                        ctx.add(ArrowButton { wh, left: true });
                    }),
                ),
                fixed_no_clip(music_card_wh.width, |_, _| {}),
                ratio_no_clip(
                    1,
                    padding_no_clip(PADDING, |wh, ctx| {
                        ctx.add(ArrowButton { wh, left: false });
                    }),
                ),
            ])(wh, ctx);
        });

        ctx.compose(|ctx| {
            let ctx = ctx.translate((wh.width / 2, music_card_wh.height / 2));
            for (music, offset) in musics_near_selected.into_iter().zip(offsets) {
                ctx.add(MusicCard {
                    music_card_wh,
                    offset,
                    music,
                });
            }
        });
    }
}

#[component]
struct MusicCard<'a> {
    pub music_card_wh: Wh<Px>,
    /// offset -2.5 ~ 2.5
    pub offset: f32,
    pub music: Option<&'a MusicMetadata>,
}
impl Component for MusicCard<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            music_card_wh,
            offset,
            music,
        } = self;

        let rotation = (offset * 5.0).deg();
        let center_xy = {
            let alpha = music_card_wh.width.as_f32() * 0.625 / f32::sin(2.5 / 180.0 * PI);
            let y = alpha - alpha * rotation.cos();
            let x = alpha * rotation.sin();
            Xy::new(x, y).into_type()
        };
        let opacity = 1.0 - (offset.abs() - 1.0).clamp(0.0, 1.0);
        let color = Color::BLACK.with_alpha((opacity * 255.0) as u8);

        ctx.compose(|ctx| {
            let ctx = ctx
                .translate(center_xy)
                .rotate(rotation)
                .translate((music_card_wh / 2).as_xy() * -1);

            ctx.compose(|ctx| {
                if let Some(music) = music {
                    ctx.add(ImageRender {
                        rect: Rect::zero_wh(music_card_wh),
                        source: ImageSource::Url {
                            url: music.thumbnail_url(),
                        },
                        fit: ImageFit::Cover,
                        paint: Some(Paint::new(color)),
                    });
                }
            });
            ctx.add(path(
                Path::new().add_rect(Rect::zero_wh(music_card_wh)),
                Paint::new(color)
                    .set_style(PaintStyle::Fill)
                    .set_mask_filter(MaskFilter::Blur {
                        blur_style: BlurStyle::Outer,
                        sigma: 8.0,
                    })
                    .set_blend_mode(BlendMode::Multiply),
            ));
        });
    }
}

#[component]
struct ArrowButton {
    pub wh: Wh<Px>,
    pub left: bool,
}
impl Component for ArrowButton {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, left } = self;

        const ARROW_WH: Wh<Px> = Wh {
            width: px(192.0),
            height: px(192.0),
        };

        let (mouse_hover, set_mouse_hover) = ctx.state(|| false);

        let rect = Rect::Xywh {
            x: if left {
                wh.width - ARROW_WH.width
            } else {
                0.px()
            },
            y: (wh.height - ARROW_WH.height) / 2,
            width: ARROW_WH.width,
            height: ARROW_WH.height,
        };

        ctx.compose(|ctx| {
            ctx.translate(rect.xy()).add(IconButton {
                wh: rect.wh(),
                // https://fontawesome.com/v5/icons/angle-double-left?f=classic&s=solid
                // https://fontawesome.com/v5/icons/angle-double-right?f=classic&s=solid
                text: if left { "" } else { "" }.to_string(),
                on_click: &|| {},
                focused: *mouse_hover,
            });
        });

        ctx.add(
            simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT).attach_event(|event| {
                let Event::MouseMove { event } = event else {
                    return;
                };
                let hovering = event.is_local_xy_in();
                if *mouse_hover == hovering {
                    return;
                }
                set_mouse_hover.set(hovering);
            }),
        );
    }
}

#[component]
struct EnterIcon {
    pub wh: Wh<Px>,
}
impl Component for EnterIcon {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;

        ctx.compose(|ctx| {
            ctx.translate((wh.width / 2, wh.height * 0.75))
                .rotate((90.0).deg())
                .add(TextDrawCommand {
                    // https://fontawesome.com/v5/icons/level-down-alt?f=classic&s=solid
                    text: "".to_string(),
                    font: Font {
                        size: adjust_font_size(wh.height),
                        name: THEME.icon_font_name.to_string(),
                    },
                    x: 0.px(),
                    y: 0.px(),
                    paint: Paint::new(THEME.text),
                    align: TextAlign::Center,
                    baseline: TextBaseline::Middle,
                    max_width: None,
                    line_height_percent: 100.percent(),
                    underline: None,
                });
        });

        ctx.add(path(
            Path::new()
                .move_to(1.0.px(), 0.0.px())
                .line_to(1.0.px(), 1.0.px())
                .line_to(0.0.px(), 1.0.px())
                .line_to(0.0.px(), 0.625.px())
                .line_to(0.25.px(), 0.625.px())
                .line_to(0.25.px(), 0.625.px())
                .line_to(0.25.px(), 0.0.px())
                .close()
                .scale(wh.width.as_f32(), wh.height.as_f32()),
            Paint::new(THEME.text.with_alpha(128)),
        ));
    }
}
