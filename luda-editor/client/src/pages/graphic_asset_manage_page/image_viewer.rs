use crate::{color, storage::get_project_image_url};
use namui::prelude::*;
use namui_prebuilt::{button, simple_rect, table::hooks::*, typography};

const MODAL_MAX_WH: Wh<Px> = Wh {
    width: px(1280.0),
    height: px(720.0),
};
const MODAL_MIN_MARGIN: Px = px(16.0);
const TITLE_BAR_HEIGHT: Px = px(48.0);

#[component]
pub struct ImageViewer<'a> {
    pub wh: Wh<Px>,
    pub image: &'a rpc::data::ImageWithLabels,
    pub project_id: Uuid,
    pub on_close: &'a dyn Fn(),
}

impl Component for ImageViewer<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            image,
            project_id,
            on_close,
        } = self;

        let wh = ctx.track_eq(&wh);
        let modal_rect = ctx.memo(|| {
            let modal_wh = Wh {
                width: (wh.width - MODAL_MIN_MARGIN * 2.0).min(MODAL_MAX_WH.width),
                height: (wh.height - MODAL_MIN_MARGIN * 2.0).min(MODAL_MAX_WH.height),
            };
            let modal_xy = Xy {
                x: (wh.width - modal_wh.width) / 2.0,
                y: (wh.height - modal_wh.height) / 2.0,
            };
            Rect::from_xy_wh(modal_xy, modal_wh)
        });

        let title_bar = |wh, ctx: &mut ComposeCtx| {
            let background = simple_rect(wh, color::STROKE_NORMAL, 1.px(), Color::TRANSPARENT);

            ctx.add(
                button::TextButton {
                    rect: Rect::from_xy_wh(
                        Xy {
                            x: wh.width - wh.height,
                            y: 0.px(),
                        },
                        Wh::single(wh.height),
                    ),
                    text: "X",
                    text_color: color::STROKE_NORMAL,
                    stroke_color: color::STROKE_NORMAL,
                    stroke_width: 1.px(),
                    fill_color: color::BACKGROUND,
                    mouse_buttons: vec![MouseButton::Left],
                    on_mouse_up_in: Box::new(|_event| {
                        on_close();
                    }),
                }
                .with_mouse_cursor(MouseCursor::Pointer),
            );

            ctx.add(typography::center_text_full_height(
                wh,
                image.id.to_string(),
                color::STROKE_NORMAL,
            ));

            ctx.add(background);
        };

        let content = |wh, ctx: &mut ComposeCtx| {
            let background = simple_rect(wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND);

            ctx.add(get_project_image_url(project_id, image.id).map_or(
                RenderingTree::Empty,
                |cg_thumbnail_image_url| {
                    namui::image(ImageParam {
                        rect: Rect::from_xy_wh(Xy::zero(), wh),
                        source: ImageSource::Url {
                            url: cg_thumbnail_image_url,
                        },
                        style: ImageStyle {
                            fit: ImageFit::Contain,
                            paint: None,
                        },
                    })
                },
            ));

            ctx.add(background);
        };

        ctx.compose(|ctx| {
            let mut ctx = ctx.translate(modal_rect.xy());

            vertical([
                fixed(TITLE_BAR_HEIGHT, |wh, ctx| {
                    title_bar(wh, ctx);
                }),
                ratio(1, |wh, ctx| {
                    content(wh, ctx);
                }),
            ])(modal_rect.wh(), &mut ctx);

            ctx.add(simple_rect(
                modal_rect.wh(),
                color::STROKE_NORMAL,
                1.px(),
                color::BACKGROUND,
            ));
        });

        ctx.component(simple_rect(
            *wh,
            Color::TRANSPARENT,
            0.px(),
            Color::from_u8(0, 0, 0, 128),
        ));

        ctx.done()
    }
}
