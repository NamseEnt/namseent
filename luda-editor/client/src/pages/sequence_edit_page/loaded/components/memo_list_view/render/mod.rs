use super::*;
use crate::color;
use namui::prelude::*;
use namui_prebuilt::{button::text_button_fit, simple_rect, table};

impl MemoListView {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        render([
            simple_rect(props.wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND),
            self.scroll_view.render(&scroll_view::Props {
                xy: Xy::zero(),
                height: props.wh.height,
                scroll_bar_width: 4.px(),
                content: table::vertical(props.memos.iter().map(|memo| {
                    table::fit(
                        table::FitAlign::LeftTop,
                        render_memo(props.wh.width, props.cut_id, memo),
                    )
                }))(props.wh),
            }),
        ])
    }
}

fn render_memo(width: Px, cut_id: Uuid, memo: &Memo) -> RenderingTree {
    const MARGIN: Px = px(8.0);
    const PADDING: Px = px(8.0);
    const BUTTON_HEIGHT: Px = px(24.0);

    let container_width = width - MARGIN * 2.0;
    let inner_width = container_width - PADDING * 2.0;
    let memo_id = memo.id;

    let done_button = text_button_fit(
        BUTTON_HEIGHT,
        "완료",
        color::STROKE_NORMAL,
        color::STROKE_NORMAL,
        1.px(),
        Color::TRANSPARENT,
        PADDING,
        [MouseButton::Left],
        move |_| namui::event::send(super::Event::RemoveCutMemo { cut_id, memo_id }),
    )
    .with_mouse_cursor(MouseCursor::Pointer);

    let content_text = text(TextParam {
        text: memo.content.clone(),
        x: 0.px(),
        y: 0.px(),
        align: TextAlign::Left,
        baseline: TextBaseline::Top,
        font_type: FontType {
            serif: false,
            size: 16.int_px(),
            language: Language::Ko,
            font_weight: FontWeight::REGULAR,
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

    let content = render([
        translate(
            inner_width - done_button.get_bounding_box().unwrap().width(),
            0.px(),
            done_button,
        ),
        translate(0.px(), BUTTON_HEIGHT + MARGIN, content_text),
    ]);

    let container_height = content.get_bounding_box().unwrap().height() + PADDING * 2.0;

    let container = simple_rect(
        Wh::new(container_width, container_height),
        color::STROKE_NORMAL,
        1.px(),
        color::BACKGROUND,
    );

    translate(
        MARGIN,
        MARGIN,
        render([container, translate(PADDING, PADDING, content)]),
    )
}
