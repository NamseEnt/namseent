use crate::level::{load_level_list, Level};
use namui::*;
use namui_prebuilt::{simple_rect, table::hooks::*};

const TABLE_ROW: usize = 3;
const ITEM_PER_ROW: usize = 4;
const SPACING: Px = px(8.);

pub struct LevelSelect;

impl Component for LevelSelect {
    fn render(self, ctx: &RenderCtx) {
        let screen_wh = screen::size().into_type::<Px>();

        let levels = load_levels(ctx);
        let (page, set_page) = ctx.state(|| 0_usize);

        let rows = levels.chunks(ITEM_PER_ROW).collect::<Vec<_>>();
        let pages = rows.chunks(TABLE_ROW).collect::<Vec<_>>();
        let page_len = pages.len();
        let table_wh = {
            let scale_factor = Px::min(&(screen_wh.width / 16.), screen_wh.height / 9.) * 2 / 3;
            Wh {
                width: scale_factor * 16.,
                height: scale_factor * 9.,
            }
        };
        let thumbnail_wh = Wh {
            width: table_wh.width / ITEM_PER_ROW as f32,
            height: table_wh.height / TABLE_ROW as f32,
        };
        let Some(current_page) = pages.get(*page) else {
            return;
        };

        let left_button = |wh: Wh<Px>, ctx: ComposeCtx| {
            let button_wh = Wh {
                width: wh.height / 15.,
                height: wh.height / 5.,
            };
            let path = Path::new()
                .move_to(1.px(), 0.px())
                .line_to(1.px(), 1.px())
                .line_to(0.px(), 0.5.px())
                .line_to(1.px(), 0.px())
                .close()
                .scale(button_wh.width.as_f32(), button_wh.height.as_f32());
            let paint = Paint::new(Color::WHITE).set_style(PaintStyle::Fill);
            let mut offset = wh - button_wh;
            offset.height /= 2.;

            ctx.translate(offset.as_xy())
                .add(PathDrawCommand { path, paint }.attach_event(|event| {
                    let Event::MouseDown { event } = event else {
                        return;
                    };
                    event.stop_propagation();
                    set_page.mutate(move |page| {
                        *page = (page_len as isize + (*page as isize) - 1) as usize % page_len;
                    });
                }));
        };

        let right_button = |wh: Wh<Px>, ctx: ComposeCtx| {
            let button_wh = Wh {
                width: wh.height / 15.,
                height: wh.height / 5.,
            };
            let path = Path::new()
                .move_to(0.px(), 0.px())
                .line_to(1.px(), 0.5.px())
                .line_to(0.px(), 1.px())
                .line_to(0.px(), 0.px())
                .close()
                .scale(button_wh.width.as_f32(), button_wh.height.as_f32());
            let paint = Paint::new(Color::WHITE).set_style(PaintStyle::Fill);
            let mut offset = wh - button_wh;
            offset.height /= 2.;
            offset.width = 0.px();

            ctx.translate(offset.as_xy())
                .add(PathDrawCommand { path, paint }.attach_event(|event| {
                    let Event::MouseDown { event } = event else {
                        return;
                    };
                    event.stop_propagation();
                    set_page.mutate(move |page| {
                        *page = (*page + 1) % page_len;
                    });
                }));
        };

        ctx.compose(|ctx| {
            horizontal([
                ratio(1, left_button),
                fixed(
                    table_wh.width,
                    vertical([
                        ratio(1, |_, _| {}),
                        fixed(
                            table_wh.height,
                            vertical(current_page.iter().map(|row| {
                                fixed(
                                    thumbnail_wh.height,
                                    horizontal(row.iter().map(|level| {
                                        fixed(
                                            thumbnail_wh.width,
                                            padding(SPACING, |wh, ctx| {
                                                ctx.add(Thumbnail { level, wh });
                                            }),
                                        )
                                    })),
                                )
                            })),
                        ),
                        ratio(1, |_, _| {}),
                    ]),
                ),
                ratio(1, right_button),
            ])(screen_wh, ctx);
        });
    }
}

fn load_levels<'a>(ctx: &'a RenderCtx) -> Sig<'a, Vec<Level>, &'a Vec<Level>> {
    let (levels, set_levels) = ctx.state(Vec::new);

    ctx.effect("load levels", || {
        let set_levels = set_levels.cloned();
        namui::spawn(async move {
            let level_list = load_level_list().await.unwrap();
            set_levels.set(level_list);
        });
    });

    levels
}

pub struct Thumbnail<'a> {
    level: &'a Level,
    wh: Wh<Px>,
}
impl Component for Thumbnail<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { level, wh } = self;
        let image = ctx.image(&level.image_path());

        ctx.add(simple_rect(
            wh,
            Color::grayscale_alpha_f01(0.375, 1.0),
            2.px(),
            Color::TRANSPARENT,
        ));

        ctx.compose(|ctx| {
            let Some(Ok(image)) = image.as_ref() else {
                return;
            };

            ctx.add(ImageDrawCommand {
                rect: wh.to_rect(),
                source: image.src.clone(),
                fit: ImageFit::Contain,
                paint: None,
            });

            ctx.add(simple_rect(
                wh,
                Color::TRANSPARENT,
                0.px(),
                Color::grayscale_alpha_f01(0.0, 0.8),
            ));

            ctx.add(ImageDrawCommand {
                rect: wh.to_rect(),
                source: image.src.clone(),
                fit: ImageFit::Cover,
                paint: Some(
                    Paint::new(Color::BLACK).set_image_filter(ImageFilter::Blur {
                        sigma_xy: Xy::single(16.0),
                        tile_mode: Some(TileMode::Mirror),
                        input: None,
                        crop_rect: None,
                    }),
                ),
            });
        });

        ctx.add(simple_rect(
            wh,
            Color::TRANSPARENT,
            0.px(),
            Color::grayscale_alpha_f01(0.0, 0.5),
        ));
    }
}
