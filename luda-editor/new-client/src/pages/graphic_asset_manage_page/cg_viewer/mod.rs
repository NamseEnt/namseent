mod part_picker;

use self::part_picker::PartPicker;
use crate::{color, components::cg_render::CgRender, storage::get_project_cg_thumbnail_image_url};
use namui::*;
use namui_prebuilt::{button, simple_rect, table::hooks::*, typography};
use rpc::data::{CgFile, ScreenCg};
use std::ops::Deref;

const MODAL_MAX_WH: Wh<Px> = Wh {
    width: px(1280.0),
    height: px(720.0),
};
const MODAL_MIN_MARGIN: Px = px(16.0);
const TITLE_BAR_HEIGHT: Px = px(48.0);
const PICKER_WIDTH: Px = px(512.0);

pub enum Event {
    Close,
    UnselectCgPart {
        cg_part_name: String,
    },
    TurnOnCgPartVariant {
        cg_part_name: String,
        cg_part_variant_name: String,
    },
    TurnOffCgPartVariant {
        cg_part_name: String,
        cg_part_variant_name: String,
    },
}

pub struct CgViewer<'a> {
    pub wh: Wh<Px>,
    pub project_id: Uuid,
    pub cg_file: &'a CgFile,
    pub screen_cg: &'a ScreenCg,
    pub on_event: &'a dyn Fn(Event),
}

impl Component for CgViewer<'_> {
    fn render(self, ctx: &RenderCtx)  {
        let Self {
            wh,
            project_id,
            cg_file,
            screen_cg,
            on_event,
        } = self;

        let modal_rect = {
            let modal_wh = Wh {
                width: (wh.width - MODAL_MIN_MARGIN * 2.0).min(MODAL_MAX_WH.width),
                height: (wh.height - MODAL_MIN_MARGIN * 2.0).min(MODAL_MAX_WH.height),
            };
            let modal_xy = Xy {
                x: (wh.width - modal_wh.width) / 2.0,
                y: (wh.height - modal_wh.height) / 2.0,
            };
            Rect::from_xy_wh(modal_xy, modal_wh)
        };

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
                    on_mouse_up_in: &|_event| {
                        on_event(Event::Close);
                    },
                }
                .with_mouse_cursor(MouseCursor::Pointer),
            );

            ctx.add(typography::center_text_full_height(
                wh,
                &cg_file.name,
                color::STROKE_NORMAL,
            ));

            ctx.add(background);
        };

        let preview = |wh: Wh<Px>, ctx: &mut ComposeCtx| {
            let background = simple_rect(wh, color::STROKE_NORMAL, 1.px(), Color::TRANSPARENT);

            ctx.add(RenderCgContainFit {
                wh,
                project_id,
                screen_cg,
                cg_file,
            });

            ctx.add(background);
        };

        let picker = |wh, ctx: &mut ComposeCtx| {
            let background = simple_rect(wh, color::STROKE_NORMAL, 1.px(), Color::TRANSPARENT);
            ctx.add(PartPicker {
                wh,
                cg_file,
                project_id,
                screen_cg,
                on_event,
            });
            ctx.add(background);
        };

        let content = |wh, ctx: &mut ComposeCtx| {
            let background = simple_rect(wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND);

            horizontal([ratio(1, preview), fixed(PICKER_WIDTH, picker)])(wh, ctx);

            ctx.add(background);
        };

        ctx.compose(|ctx| {
            let mut ctx = ctx.translate(modal_rect.xy());

            vertical([
                fixed(TITLE_BAR_HEIGHT, |wh, ctx| {
                    title_bar(wh, ctx);
                }),
                ratio(1, content),
            ])(modal_rect.wh(), &mut ctx);

            ctx.add(
                simple_rect(
                    modal_rect.wh(),
                    color::STROKE_NORMAL,
                    1.px(),
                    color::BACKGROUND,
                )
                .attach_event(|event| {
                    if let namui::Event::MouseDown { event } = event {
                        if !event.is_local_xy_in() {
                            return;
                        }
                        event.stop_propagation();
                    }
                }),
            );
        });

        ctx.component(
            simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::from_u8(0, 0, 0, 128)).attach_event(
                |event| {
                    if let namui::Event::MouseDown { event } = event {
                        if !event.is_local_xy_in() {
                            return;
                        }
                        event.stop_propagation();
                        on_event(Event::Close);
                    }
                },
            ),
        );

        
    }
}

struct RenderCgContainFit<'a> {
    wh: Wh<Px>,
    project_id: Uuid,
    screen_cg: &'a ScreenCg,
    cg_file: &'a rpc::data::CgFile,
}
impl Component for RenderCgContainFit<'_> {
    fn render(self, ctx: &RenderCtx)  {
        let Self {
            wh,
            project_id,
            screen_cg,
            cg_file,
        } = self;

        let cg_thumbnail_image =
            ctx.image(&get_project_cg_thumbnail_image_url(project_id, cg_file.id).unwrap());
        let rect = cg_thumbnail_image
            .deref()
            .as_ref()
            .and_then(|cg_thumbnail_image| cg_thumbnail_image.as_ref().ok())
            .map(|cg_thumbnail_image| calculate_cg_rect(wh, cg_thumbnail_image.wh));

        ctx.compose(|ctx| {
            let Some(rect) = rect else {
                return;
            };
            ctx.add(CgRender {
                rect,
                project_id,
                screen_cg,
                cg_file,
            });
        });

        
    }
}

fn calculate_cg_rect(container_wh: Wh<Px>, cg_wh: Wh<Px>) -> Rect<Px> {
    let container_ratio = container_wh.width / container_wh.height;
    let cg_ratio = cg_wh.width / cg_wh.height;
    let contain_fit_cg_wh = if container_ratio > cg_ratio {
        Wh {
            width: container_wh.height * cg_ratio,
            height: container_wh.height,
        }
    } else {
        Wh {
            width: container_wh.width,
            height: container_wh.width / cg_ratio,
        }
    };
    let xy = Xy {
        x: (container_wh.width - contain_fit_cg_wh.width) / 2.0,
        y: (container_wh.height - contain_fit_cg_wh.height) / 2.0,
    };
    Rect::from_xy_wh(xy, contain_fit_cg_wh)
}
