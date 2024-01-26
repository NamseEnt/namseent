use crate::app::{
    components::{IconButton, TextButton},
    music::MusicMetadata,
    theme::THEME,
};
use namui::prelude::*;
use namui_prebuilt::{table::hooks::*, typography::adjust_font_size};

#[component]
pub struct MusicCarousel<'a> {
    pub wh: Wh<Px>,
    pub musics: &'a Vec<MusicMetadata>,
    pub selected: usize,
}

impl Component for MusicCarousel<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            musics,
            selected,
        } = self;

        const PADDING: Px = px(8.0);

        let (prev, selected, next) = {
            if musics.is_empty() {
                (None, None, None)
            } else {
                let prev_index = selected.checked_sub(1).unwrap_or(musics.len() - 1);
                let mut iter = musics.iter().cycle().skip(prev_index);
                (iter.next(), iter.next(), iter.next())
            }
        };
        let music_card_wh = {
            let height = wh.height * 0.8;
            let width = height * 16.0 / 9.0;
            Wh::new(width, height)
        };
        let (prev_xy, selected_xy, next_xy) = {
            let music_card_center = music_card_wh / 2.0;
            let center_x = wh.width / 2;
            let side_y = wh.height - music_card_center.height;
            (
                Xy::new(center_x - (music_card_wh.width * 1.25), side_y),
                Xy::new(center_x, music_card_center.height),
                Xy::new(center_x + (music_card_wh.width * 1.25), side_y),
            )
        };

        ctx.compose(|ctx| {
            ctx.translate((wh.width / 2, wh.height - 64.px()))
                .translate(((-128).px(), 0.px()))
                .add(TextButton {
                    wh: Wh::new(256.px(), 128.px()),
                    text: "start".to_string(),
                    on_click: &|| {},
                })
                .translate((256.px(), 32.px()))
                .add(EnterIcon {
                    wh: Wh::new(64.px(), 64.px()),
                });
        });

        ctx.compose(|ctx| {
            horizontal([
                ratio(
                    1,
                    padding(PADDING, |wh, ctx| {
                        ctx.add(ArrowButton { wh, left: true });
                    }),
                ),
                fixed(music_card_wh.width, |_, _| {}),
                ratio(
                    1,
                    padding(PADDING, |wh, ctx| {
                        ctx.add(ArrowButton { wh, left: false });
                    }),
                ),
            ])(wh, ctx);
        });

        ctx.component(MusicCard {
            wh: music_card_wh,
            center_xy: selected_xy,
            rotate: Angle::Degree(0.0),
            music: selected,
        });
        ctx.component(MusicCard {
            wh: music_card_wh,
            center_xy: prev_xy,
            rotate: Angle::Degree(-2.5),
            music: prev,
        });
        ctx.component(MusicCard {
            wh: music_card_wh,
            center_xy: next_xy,
            rotate: Angle::Degree(2.5),
            music: next,
        });

        ctx.done()
    }
}

#[component]
struct MusicCard<'a> {
    pub wh: Wh<Px>,
    pub center_xy: Xy<Px>,
    pub rotate: Angle,
    pub music: Option<&'a MusicMetadata>,
}
impl Component for MusicCard<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            center_xy,
            rotate,
            music,
        } = self;

        ctx.compose(|ctx| {
            let mut ctx = ctx
                .translate(center_xy)
                .rotate(rotate)
                .translate((wh / 2).as_xy() * -1);

            ctx.compose(|ctx| {
                if let Some(music) = music {
                    ctx.add(image(ImageParam {
                        rect: Rect::zero_wh(wh),
                        source: ImageSource::Url {
                            url: music.thumbnail_url(),
                        },
                        style: ImageStyle {
                            fit: ImageFit::Cover,
                            paint: None,
                        },
                    }));
                }
            });
            ctx.add(path(
                Path::new().add_rect(Rect::zero_wh(wh)),
                Paint::new(Color::BLACK)
                    .set_style(PaintStyle::Fill)
                    .set_mask_filter(MaskFilter::Blur {
                        blur: Blur::Outer { sigma: 8.0 },
                    })
                    .set_blend_mode(BlendMode::Multiply),
            ));
        });

        ctx.done()
    }
}

#[component]
struct ArrowButton {
    pub wh: Wh<Px>,
    pub left: bool,
}
impl Component for ArrowButton {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh, left } = self;

        const ARROW_WH: Wh<Px> = Wh {
            width: px(192.0),
            height: px(192.0),
        };

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
            });
        });

        ctx.done()
    }
}

#[component]
struct EnterIcon {
    pub wh: Wh<Px>,
}
impl Component for EnterIcon {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh } = self;

        ctx.compose(|ctx| {
            ctx.translate((wh.width / 2, wh.height * 0.75))
                .rotate(Angle::Degree(90.0))
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

        ctx.component(path(
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

        ctx.done()
    }
}
