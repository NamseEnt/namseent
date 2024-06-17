use super::*;
use crate::color;
use namui::*;
use namui_prebuilt::{
    button::{self},
    simple_rect,
    table::{self, padding},
};
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
    NameQuickSlotButtonClicked,
}

impl Component for SideBar<'_> {
    fn render(self, ctx: &RenderCtx)  {
        let Self {
            wh,
            project_id,
            user_id,
            selected_cut,
            memos,
            on_event,
        } = self;

        const GRAPHIC_LIST_VIEW_HEIGHT: Px = px(384.0);
        const NAME_QUICK_SLOT_BUTTON_CONTAINER_HEIGHT: Px = px(42.0);
        const NAME_QUICK_SLOT_BUTTON_CONTAINER_PADDING: Px = px(8.0);

        ctx.compose(|ctx| {
            table::vertical([
                table::fixed(
                    NAME_QUICK_SLOT_BUTTON_CONTAINER_HEIGHT,
                    padding(NAME_QUICK_SLOT_BUTTON_CONTAINER_PADDING, |wh, ctx| {
                        ctx.add(
                            button::TextButton {
                                rect: wh.to_rect(),
                                text: "Name Quick Slot",
                                text_color: color::STROKE_NORMAL,
                                stroke_color: color::STROKE_NORMAL,
                                stroke_width: 1.px(),
                                fill_color: color::BACKGROUND,
                                mouse_buttons: vec![MouseButton::Left],
                                on_mouse_up_in: &|_event| {
                                    on_event(Event::NameQuickSlotButtonClicked);
                                },
                            }
                            .with_mouse_cursor(MouseCursor::Pointer),
                        );
                    }),
                ),
                table::ratio(1, |wh, ctx| {
                    ctx.add(memo_list_view::MemoListView {
                        wh,
                        memos,
                        user_id,
                        on_event: &|event| on_event(Event::MemoListView(event)),
                    });
                }),
                table::fixed(GRAPHIC_LIST_VIEW_HEIGHT, |wh, ctx| {
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

        
    }
}
