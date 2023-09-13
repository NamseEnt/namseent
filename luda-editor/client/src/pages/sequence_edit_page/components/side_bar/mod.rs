use super::*;
use crate::color;
use namui::prelude::*;
use namui_prebuilt::{simple_rect, table};
use rpc::data::{Cut, Memo};

#[component]
pub struct SideBar<'a> {
    pub wh: Wh<Px>,
    pub project_id: Uuid,
    pub user_id: Uuid,
    pub selected_cut: Option<&'a Cut>,
    pub memos: Option<&'a Vec<Memo>>,
    pub on_event: Box<dyn 'a + Fn(Event)>,
}

pub enum Event {
    MemoListView(memo_list_view::Event),
}

impl Component for SideBar<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        const GRAPHIC_LIST_VIEW_HEIGHT: Px = px(384.0);

        let Self {
            wh,
            project_id,
            user_id,
            selected_cut,
            memos,
            on_event,
        } = self;

        ctx.compose(|ctx| {
            table::hooks::vertical([
                table::hooks::ratio(1, |wh, ctx| {
                    ctx.add(memo_list_view::MemoListView {
                        wh,
                        memos,
                        user_id,
                        on_event: &|event| on_event(Event::MemoListView(event)),
                    });
                }),
                table::hooks::fixed(GRAPHIC_LIST_VIEW_HEIGHT, |wh, ctx| {
                    ctx.add(graphic_list_view::GraphicListView {
                        project_id,
                        wh,
                        selected_cut,
                    });
                }),
            ])(wh, ctx);
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
