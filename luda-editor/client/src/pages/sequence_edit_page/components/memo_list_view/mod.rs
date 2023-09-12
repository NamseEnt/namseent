use crate::color;
use namui::prelude::*;
use namui_prebuilt::button::TextButtonFitAlign;
use namui_prebuilt::scroll_view::{self};
use namui_prebuilt::{simple_rect, table, transparent_rect};
use rpc::data::Memo;

#[namui::component]
pub struct MemoListView<'a> {
    pub wh: Wh<Px>,
    pub memos: Option<&'a Vec<Memo>>,
    pub user_id: Uuid,
    pub on_event: Box<dyn 'a + Fn(Event)>,
}

pub enum Event {
    DoneClicked { cut_id: Uuid, memo_id: Uuid },
}

impl Component for MemoListView<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            ref memos,
            user_id,
            on_event,
        } = self;

        ctx.component(scroll_view::AutoScrollViewWithCtx {
            scroll_bar_width: 4.px(),
            wh,
            content: |ctx| {
                let Some(memos) = memos else {
                    return;
                };
                table::hooks::vertical(memos.iter().map(|memo| {
                    table::hooks::fit(
                        table::hooks::FitAlign::LeftTop,
                        MemoComponent {
                            width: wh.width,
                            memo,
                            user_id,
                            on_event: &on_event,
                        },
                    )
                }))(wh, ctx);
            },
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

#[namui::component]
struct MemoComponent<'a> {
    width: Px,
    memo: &'a Memo,
    user_id: Uuid,
    on_event: &'a dyn Fn(Event),
}
impl Component for MemoComponent<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        const MARGIN: Px = px(8.0);
        const PADDING: Px = px(8.0);

        let Self {
            width,
            memo,
            user_id,
            on_event,
        } = self;

        let container_width = width - MARGIN * 2.0;

        let mut container_height = None;

        ctx.compose(|ctx| {
            ctx.translate((MARGIN, MARGIN))
                .compose(|ctx| {
                    ctx.translate((PADDING, PADDING)).compose(|ctx| {
                        let bounding_box = ctx.add_and_get_bounding_box(MemoContent {
                            width,
                            memo,
                            user_id,
                            on_event,
                        });
                        container_height = Some(
                            bounding_box.map_or(0.px(), |bounding_box| bounding_box.height())
                                + PADDING * 2.0,
                        );
                    });
                })
                .add(simple_rect(
                    Wh::new(container_width, container_height.unwrap()),
                    color::STROKE_NORMAL,
                    1.px(),
                    color::BACKGROUND,
                ));
        })
        .done()
    }
}

#[namui::component]
struct MemoContent<'a> {
    width: Px,
    memo: &'a Memo,
    user_id: Uuid,
    on_event: &'a dyn Fn(Event),
}
impl Component for MemoContent<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        const MARGIN: Px = px(8.0);
        const PADDING: Px = px(8.0);
        const BUTTON_HEIGHT: Px = px(24.0);
        let Self {
            width,
            memo,
            user_id,
            on_event,
        } = self;

        let container_width = width - MARGIN * 2.0;
        let inner_width = container_width - PADDING * 2.0;
        let memo_id = memo.id;
        let cut_id = memo.cut_id;

        let user_name_label = text(TextParam {
            text: memo.user_name.clone(),
            x: 0.px(),
            y: BUTTON_HEIGHT * 0.5,
            align: TextAlign::Left,
            baseline: TextBaseline::Middle,
            font: Font {
                size: 16.int_px(),
                name: "NotoSansKR-Bold".to_string(),
            },
            style: TextStyle {
                border: None,
                color: color::STROKE_NORMAL,
                drop_shadow: None,
                background: None,
                line_height_percent: 100.percent(),
                underline: None,
            },
            max_width: None,
        });

        let done_button = match memo.user_id == user_id {
            true => Some(
                TextButtonFitAlign {
                    wh: Wh::new(inner_width, BUTTON_HEIGHT),
                    align: TextAlign::Right,
                    text: "완료",
                    text_color: color::STROKE_NORMAL,
                    stroke_color: color::STROKE_NORMAL,
                    stroke_width: 1.px(),
                    fill_color: Color::TRANSPARENT,
                    side_padding: PADDING,
                    mouse_buttons: vec![MouseButton::Left],
                    on_mouse_up_in: Box::new(|_| {
                        on_event(Event::DoneClicked { cut_id, memo_id });
                    }),
                }
                .with_mouse_cursor(MouseCursor::Pointer),
            ),
            false => None,
        };

        let content_text = text(TextParam {
            text: memo.content.clone(),
            x: 0.px(),
            y: 0.px(),
            align: TextAlign::Left,
            baseline: TextBaseline::Top,
            font: Font {
                size: 16.int_px(),
                name: "NotoSansKR-Regular".to_string(),
            },
            style: TextStyle {
                border: None,
                drop_shadow: None,
                color: color::STROKE_NORMAL,
                background: None,
                line_height_percent: 175.percent(),
                underline: None,
            },
            max_width: Some(inner_width),
        });

        ctx.compose(|ctx| {
            ctx.translate((MARGIN, MARGIN))
                .add(user_name_label)
                .add(done_button);
        })
        .compose(|ctx| {
            ctx.translate((0.px(), BUTTON_HEIGHT + MARGIN * 3))
                .add(content_text);
        })
        .component(transparent_rect(Wh::new(
            inner_width + MARGIN * 2,
            BUTTON_HEIGHT + MARGIN * 4,
        )))
        .done()
    }
}
