use super::*;
use crate::{
    components::context_menu::{if_context_menu_for, open_context_menu},
    pages::sequence_edit_page::atom::SEQUENCE_ATOM,
    *,
};
use rpc::data::SequenceUpdateAction;

#[namui::component]
pub struct CutCell<'a> {
    pub wh: Wh<Px>,
    pub index: usize,
    pub cut: Cut,
    pub memo_count: usize,
    pub is_selected: bool,
    pub is_focused: bool,
    pub on_click: callback!('a, Uuid),
}

enum ContextMenu {
    CutCell { cut_id: Uuid },
}

impl Component for CutCell<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            index,
            cut,
            memo_count,
            is_selected,
            is_focused,
            on_click,
        } = self;

        let stroke_color = color::stroke_color(is_selected, is_focused);
        let cut_id = cut.id;
        let (dragging, set_dragging) = ctx.atom(&DRAGGING_CONTEXT);

        if_context_menu_for::<ContextMenu>(|context_menu, builder| match context_menu {
            &ContextMenu::CutCell { cut_id } => builder.add_button("Delete Cut", || {
                SEQUENCE_ATOM.mutate(move |sequence| {
                    sequence.update(SequenceUpdateAction::DeleteCut { cut_id })
                });
            }),
        });

        ctx.compose(|ctx| {
            let Some(dragging) = *dragging else {
                return;
            };
            if dragging.cut_id != cut_id {
                return;
            }
            ctx.add(
                simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::from_u8(0, 0, 0, 128))
                    .attach_event(|event| {
                        if let namui::Event::MouseDown { event } = event {
                            if event.is_local_xy_in() {
                                event.stop_propagation();
                            }
                        }
                    })
                    .with_mouse_cursor(MouseCursor::Grabbing),
            );
        });

        ctx.component(transparent_rect(wh).attach_event(|event| {
            if let namui::Event::MouseDown { event } = event {
                if event.is_local_xy_in() && event.button == Some(MouseButton::Left) {
                    on_click(cut_id);
                }
            }
        }));

        ctx.compose(|ctx| {
            table::hooks::padding(
                12.px(),
                table::hooks::horizontal([
                    table::hooks::fixed(24.px(), |wh, ctx| {
                        table::hooks::vertical([
                            table::hooks::fit(
                                table::hooks::FitAlign::LeftTop,
                                typography::body::center_top(
                                    wh.width,
                                    format!("{}", index),
                                    stroke_color,
                                ),
                            ),
                            table::hooks::fixed(4.px(), |_, _| {}),
                            table::hooks::fit(
                                table::hooks::FitAlign::LeftTop,
                                render_comment_badge(wh.width, memo_count, stroke_color),
                            ),
                        ])(wh, ctx)
                    }),
                    table::hooks::ratio(1, |wh, ctx| {
                        let thumbnail = simple_rect(
                            wh,
                            stroke_color,
                            if is_selected { 2.px() } else { 1.px() },
                            Color::BLACK,
                        );
                        match *dragging {
                            Some(dragging) if dragging.cut_id == cut_id => {
                                ctx.on_top()
                                    .absolute(
                                        mouse::position() - dragging.thumbnail_clicked_offset_xy,
                                    )
                                    .add(thumbnail);
                            }
                            _ => {
                                ctx.add(thumbnail).attach_event(|event| {
                                    if let namui::Event::MouseDown { event } = event {
                                        if event.is_local_xy_in() {
                                            if event.button == Some(MouseButton::Left) {
                                                set_dragging.set(Some(DraggingContext {
                                                    cut_id,
                                                    thumbnail_clicked_offset_xy: event.local_xy(),
                                                    end_index: index,
                                                }));
                                            }
                                            if event.button == Some(MouseButton::Right) {
                                                open_context_menu(
                                                    event.global_xy,
                                                    ContextMenu::CutCell { cut_id },
                                                );
                                                event.stop_propagation();
                                            }
                                        }
                                    }
                                });
                            }
                        };
                    }),
                    table::hooks::fixed(8.px(), |_wh, _ctx| {}),
                ]),
            )(wh, ctx)
        });

        ctx.done()
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

    let path_builder = Path::new()
        .move_to(0.05.px(), 0.05.px())
        .line_to(0.95.px(), 0.05.px())
        .line_to(0.95.px(), 0.7.px())
        .line_to(0.8.px(), 0.7.px())
        .line_to(0.9.px(), 0.8.px())
        .line_to(0.6.px(), 0.7.px())
        .line_to(0.05.px(), 0.7.px())
        .line_to(0.05.px(), 0.05.px())
        .scale(width.as_f32(), width.as_f32());

    let paint = Paint::new().set_style(PaintStyle::Fill).set_color(color);

    render([
        text(TextParam {
            text: memo_count,
            x: width * 0.5,
            y: width * 0.35,
            align: TextAlign::Center,
            baseline: TextBaseline::Middle,
            font: Font {
                size: (width * 0.5).into(),
                name: "NotoSansKR-Bold".to_string(),
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
        path(path_builder, paint),
    ])
}
