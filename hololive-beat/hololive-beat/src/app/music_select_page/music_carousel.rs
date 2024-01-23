use crate::app::{
    components::{IconButton, TextButton},
    music::MusicMetadata,
};
use namui::prelude::*;
use namui_prebuilt::table::hooks::*;

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
                .add(image(ImageParam {
                    rect: Rect::Xywh {
                        x: 256.px(),
                        y: 32.px(),
                        width: 64.px(),
                        height: 64.px(),
                    },
                    source: ImageSource::Url {
                        url: Url::parse("bundle:ui/enter.png").unwrap(),
                    },
                    style: ImageStyle {
                        fit: ImageFit::Contain,
                        paint: None,
                    },
                }));
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
