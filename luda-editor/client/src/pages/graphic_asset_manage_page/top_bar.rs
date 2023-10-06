use crate::{
    app::notification::{push_notification, Notification},
    color,
    pages::{
        graphic_asset_manage_page::upload_asset::{add_new_cg, add_new_image},
        router::{move_to, Route},
    },
};
use namui::{file::picker::open, prelude::*};
use namui_prebuilt::{button::TextButtonFit, simple_rect, table::hooks::*, typography};
use std::path::PathBuf;

#[component]
pub(super) struct TopBar {
    pub wh: Wh<Px>,
    pub project_id: Uuid,
}

impl Component for TopBar {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        const PADDING: Px = px(8.0);
        let Self { wh, project_id } = self;

        let on_upload_button_clicked = || {
            spawn_local(async move {
                let files = open().await;
                for file in files.into_iter() {
                    let file_name = PathBuf::from(file.name());
                    let extension_name = file_name
                        .extension()
                        .map(|extension_name| extension_name.to_str().unwrap());
                    match extension_name {
                        Some("png") | Some("jpg") | Some("jpeg") => {
                            add_new_image(project_id, file.content().await.to_vec());
                        }
                        Some("psd") => {
                            let psd_name = file.name().trim_end_matches(".psd").to_string();
                            add_new_cg(project_id, psd_name, file.content().await.to_vec())
                        }
                        _ => {
                            push_notification(Notification::error(format!(
                                "Unsupported file type {file_name:?}"
                            )));
                        }
                    }
                }
            });
        };

        let on_back_button_clicked = || move_to(Route::SequenceList { project_id });

        ctx.compose(|ctx| {
            padding(PADDING, |wh, ctx| {
                horizontal([
                    fit(
                        FitAlign::CenterMiddle,
                        TextButtonFit {
                            height: wh.height,
                            text: "Upload",
                            text_color: color::BACKGROUND,
                            stroke_color: color::STROKE_NORMAL,
                            stroke_width: 1.px(),
                            fill_color: color::STROKE_NORMAL,
                            side_padding: PADDING,
                            mouse_buttons: vec![MouseButton::Left],
                            on_mouse_up_in: Box::new(|_| on_upload_button_clicked()),
                        }
                        .with_mouse_cursor(MouseCursor::Pointer),
                    ),
                    ratio(1, |_wh, _ctx| {}),
                    fit(
                        FitAlign::CenterMiddle,
                        TextButtonFit {
                            height: wh.height,
                            text: "Back",
                            text_color: color::BACKGROUND,
                            stroke_color: color::STROKE_NORMAL,
                            stroke_width: 1.px(),
                            fill_color: color::STROKE_NORMAL,
                            side_padding: PADDING,
                            mouse_buttons: vec![MouseButton::Left],
                            on_mouse_up_in: Box::new(|_| on_back_button_clicked()),
                        }
                        .with_mouse_cursor(MouseCursor::Pointer),
                    ),
                ])(wh, ctx);

                ctx.add(typography::title::center(
                    wh,
                    "Graphic Assets",
                    color::STROKE_NORMAL,
                ));
            })(wh, ctx)
        });

        ctx.component(simple_rect(
            wh,
            color::STROKE_NORMAL,
            1.px(),
            color::BACKGROUND,
        ));

        ctx.done()
    }
}
