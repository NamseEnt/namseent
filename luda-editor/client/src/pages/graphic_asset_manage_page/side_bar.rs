use super::Tab;
use crate::{color, pages::graphic_asset_manage_page::TAB_ATOM};
use namui::prelude::*;
use namui_prebuilt::{simple_rect, table::hooks::*, typography};

#[component]
pub(super) struct SideBar {
    pub wh: Wh<Px>,
}

impl Component for SideBar {
    fn render(self, ctx: &RenderCtx)  {
        let Self { wh } = self;

        const PADDING: Px = px(8.0);
        const MARGIN: Px = px(4.0);
        const ITEM_HEIGHT: Px = px(36.0);

        ctx.compose(|ctx| {
            vertical_padding(PADDING, |wh, ctx| {
                vertical([
                    fixed(ITEM_HEIGHT, |wh, ctx| {
                        ctx.add(TabButton {
                            wh,
                            tab: Tab::Image,
                        });
                    }),
                    fixed(MARGIN, |_wh, _ctx| {}),
                    fixed(ITEM_HEIGHT, |wh, ctx| {
                        ctx.add(TabButton { wh, tab: Tab::Cg });
                    }),
                ])(wh, ctx);
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

#[component]
struct TabButton {
    wh: Wh<Px>,
    tab: Tab,
}
impl Component for TabButton {
    fn render(self, ctx: &RenderCtx)  {
        let Self { wh, tab } = self;

        const PADDING: Px = px(4.0);

        let (current_tab, set_tab) = ctx.atom(&TAB_ATOM);

        let selected = *current_tab == tab;
        let (text_color, fill_color) = match selected {
            true => (color::BACKGROUND, color::STROKE_NORMAL),
            false => (color::STROKE_NORMAL, color::BACKGROUND),
        };
        let on_clicked = || {
            set_tab.set(tab);
        };

        ctx.compose(|ctx| {
            padding(PADDING, |wh, ctx| {
                ctx.add(typography::title::left(
                    wh.height,
                    tab.to_string(),
                    text_color,
                ));
            })(wh, ctx);
        });

        ctx.component(
            simple_rect(wh, color::STROKE_NORMAL, 1.px(), fill_color)
                .attach_event(|event| {
                    if let Event::MouseDown { event } = event {
                        if !event.is_local_xy_in() {
                            return;
                        }
                        on_clicked();
                    }
                })
                .with_mouse_cursor(MouseCursor::Pointer),
        );

        
    }
}
