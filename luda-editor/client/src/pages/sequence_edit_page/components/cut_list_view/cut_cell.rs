use super::*;
use crate::*;
use namui::prelude::*;
use namui_prebuilt::*;

#[namui::component]
pub struct CutCell {
    pub wh: Wh<Px>,
    pub index: usize,
    pub cut: Cut,
    pub memo_count: usize,
    pub is_selected: bool,
    pub is_focused: bool,
    pub on_click: &'a dyn Fn(Uuid),
}

impl Component for CutCell {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let &Self {
            wh,
            index,
            ref cut,
            memo_count,
            is_selected,
            is_focused,
            on_click,
        } = self;

        let stroke_color = color::stroke_color(is_selected, is_focused);
        let cut_id = cut.id;
        ctx.use_children(|ctx| {
            ctx.add(transparent_rect(wh).attach_event(|builder| {
                let on_click = on_click.clone();
                builder.on_mouse_down_in(move |event: MouseEvent| {
                    if event.button == Some(MouseButton::Left) {
                        on_click.call(cut_id);
                    }
                });
            }));

            ctx.add(table::hooks::padding(
                12.px(),
                table::hooks::horizontal([
                    table::hooks::fixed(24.px(), |wh| {
                        table::hooks::vertical([
                            table::hooks::fit(
                                table::hooks::FitAlign::LeftTop,
                                typography::body::center_top(
                                    wh.width,
                                    format!("{}", index),
                                    stroke_color,
                                ),
                            ),
                            table::hooks::fixed(4.px(), |_| RenderingTree::Empty),
                            table::hooks::fit(
                                table::hooks::FitAlign::LeftTop,
                                render_comment_badge(wh.width, memo_count, stroke_color),
                            ),
                        ])(wh)
                    }),
                    table::hooks::ratio(1, |wh| {
                        simple_rect(
                            wh,
                            stroke_color,
                            if is_selected { 2.px() } else { 1.px() },
                            Color::BLACK,
                        )
                    }),
                    table::hooks::fixed(8.px(), |_wh| RenderingTree::Empty),
                ]),
            )(wh))
        })
    }
}

fn render_comment_badge(width: Px, memo_count: usize, color: Color) -> RenderingTree {
    if memo_count == 0 {
        return RenderingTree::Empty;
    }

    let memo_count = if memo_count > 9 {
        "9+".to_string()
    } else {
        memo_count.to_string()
    };

    let path_builder = PathBuilder::new()
        .move_to(0.05.px(), 0.05.px())
        .line_to(0.95.px(), 0.05.px())
        .line_to(0.95.px(), 0.7.px())
        .line_to(0.8.px(), 0.7.px())
        .line_to(0.9.px(), 0.8.px())
        .line_to(0.6.px(), 0.7.px())
        .line_to(0.05.px(), 0.7.px())
        .line_to(0.05.px(), 0.05.px())
        .scale(width.as_f32(), width.as_f32());

    let paint_builder = PaintBuilder::new()
        .set_style(PaintStyle::Fill)
        .set_color(color);

    render([
        path(path_builder, paint_builder),
        text(TextParam {
            text: memo_count,
            x: width * 0.5,
            y: width * 0.35,
            align: TextAlign::Center,
            baseline: TextBaseline::Middle,
            font_type: FontType {
                serif: false,
                size: (width * 0.5).into(),
                language: Language::Ko,
                font_weight: FontWeight::BOLD,
            },
            style: TextStyle {
                border: None,
                drop_shadow: None,
                color: color::BACKGROUND,
                background: None,
                line_height_percent: 100.percent(),
                underline: None,
            },
            max_width: width.into(),
        }),
    ])
}
