use crate::{
    color,
    pages::router::{move_to, Route},
};
use namui::*;
use namui_prebuilt::{button::TextButtonFit, simple_rect, table::*, typography};

pub(super) struct TopBar {
    pub wh: Wh<Px>,
    pub project_id: Uuid,
}

impl Component for TopBar {
    fn render(self, ctx: &RenderCtx)  {
        let Self { wh, project_id } = self;

        const PADDING: Px = px(8.0);

        ctx.compose(|ctx| {
            padding(PADDING, |wh, ctx| {
                horizontal([
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
                            on_mouse_up_in: &|_| move_to(Route::SequenceList { project_id }),
                        }
                        .with_mouse_cursor(MouseCursor::Pointer),
                    ),
                ])(wh, ctx);

                ctx.add(typography::title::center(
                    wh,
                    "Project ACL",
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
