use crate::color;
use namui::prelude::*;
use namui_prebuilt::scroll_view::{self};
use namui_prebuilt::{button::text_button_fit, simple_rect, table};
use rpc::data::Memo;

#[namui::component]
pub struct MemoListView<'a> {
    pub wh: Wh<Px>,
    pub memos: Vec<Memo>,
    pub user_id: Uuid,
    // pub on_done_clicked: &'a dyn Fn(CutIdMemoId),
    pub on_event: Box<dyn 'a + Fn(Event)>,
}

pub enum Event {
    DoneClicked { cut_id: Uuid, memo_id: Uuid },
}

impl Component for MemoListView<'_> {
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
        let Self {
            wh,
            ref memos,
            user_id,
            // ref on_done_clicked,
            on_event,
        } = self;

        ctx.component(simple_rect(
            wh,
            color::STROKE_NORMAL,
            1.px(),
            color::BACKGROUND,
        ));

        ctx.component(scroll_view::AutoScrollViewWithCtx {
            xy: Xy::zero(),
            scroll_bar_width: 4.px(),
            height: wh.height,
            content: |ctx| {
                table::hooks::vertical(memos.iter().map(|memo| {
                    table::hooks::fit(
                        table::hooks::FitAlign::LeftTop,
                        MemoComponent {
                            width: wh.width,
                            memo,
                            user_id,
                            on_event: Box::new(|event| on_event(event)),
                        },
                    )
                }))(wh, ctx);
            },
        });
        ctx.done()
    }
}

#[namui::component]
struct MemoComponent<'a> {
    width: Px,
    memo: &'a Memo,
    user_id: Uuid,
    on_event: callback!('a, Event),
}
impl Component for MemoComponent<'_> {
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
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

        let content = MemoContent {
            width,
            memo,
            user_id,
            on_event,
        };
        let container_height = ctx
            .ghost_render(content)
            .bounding_box()
            .map_or(0.px(), |bounding_box| bounding_box.height())
            + PADDING * 2.0;

        let container = simple_rect(
            Wh::new(container_width, container_height),
            color::STROKE_NORMAL,
            1.px(),
            color::BACKGROUND,
        );

        ctx.compose(|ctx| {
            ctx.translate((MARGIN, MARGIN))
                .add(container)
                .compose(|ctx| {
                    // ctx.translate((PADDING, PADDING)).add(content);
                });
        })
        .done()
    }
}

#[namui::component]
struct MemoContent<'a> {
    width: Px,
    memo: &'a Memo,
    user_id: Uuid,
    on_event: callback!('a, Event),
}
impl Component for MemoContent<'_> {
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
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
                text_button_fit(
                    BUTTON_HEIGHT,
                    "완료",
                    color::STROKE_NORMAL,
                    color::STROKE_NORMAL,
                    1.px(),
                    Color::TRANSPARENT,
                    PADDING,
                    [MouseButton::Left],
                    move |_| {
                        on_event(Event::DoneClicked { cut_id, memo_id });
                    },
                )
                .with_mouse_cursor(MouseCursor::Pointer),
            ),
            false => None,
        };
        let done_button_width = done_button
            .and_then(|done_button| ctx.ghost_render(done_button).bounding_box())
            .map_or(0.px(), |bounding_box| bounding_box.width());

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
            // ctx.translate((MARGIN, MARGIN))
            //     .add(user_name_label)
            //     .translate((inner_width - done_button_width, 0.px()))
            //     .add(done_button);
        })
        .compose(|ctx| {
            ctx.translate((0.px(), BUTTON_HEIGHT + MARGIN))
                .add(content_text);
        })
        .done()
    }
}
