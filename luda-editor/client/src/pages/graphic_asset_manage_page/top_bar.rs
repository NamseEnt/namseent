use crate::{
    color,
    pages::{
        graphic_asset_manage_page::upload_asset::upload_file,
        router::{Route, move_to},
    },
};
use futures::future::join_all;
use namui::{file::picker::open, prelude::*};
use namui_prebuilt::{button::TextButtonFit, simple_rect, table::*, typography};

pub(super) struct TopBar {
    pub wh: Wh<Px>,
    pub project_id: Uuid,
}

impl Component for TopBar {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, project_id } = self;

        const PADDING: Px = px(8.0);

        let on_upload_button_clicked = || {
            spawn_local(async move {
                let files = open().await;
                join_all(files.iter().map(|file| upload_file(file, project_id))).await;
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
                            on_mouse_up_in: &|_| on_upload_button_clicked(),
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
                            on_mouse_up_in: &|_| on_back_button_clicked(),
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
    }
}
