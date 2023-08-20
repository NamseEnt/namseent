use super::*;
use crate::*;
use namui_prebuilt::*;

impl CutListView {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let is_focused = props.is_focused;
        let selected_cut_id = props.selected_cut_id;

        render([
            simple_rect(props.wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND),
            self.list_view.render(list_view::Props {
                xy: Xy::zero(),
                height: props.wh.height,
                scroll_bar_width: 12.px(),
                item_wh: Wh::new(props.wh.width, 128.px()),
                items: props
                    .cuts
                    .into_iter()
                    .zip(
                        props
                            .cuts
                            .iter()
                            .map(|cut| props.cut_id_memo_map.get(&cut.id)),
                    )
                    .enumerate(),
                item_render: |wh, (index, (cut, memos))| {
                    self.render_cut_cell(
                        wh,
                        index,
                        cut,
                        memos.map_or(0, |memos| memos.len()),
                        &props,
                    )
                },
            }),
        ])
        .attach_event(move |builder| {
            let cuts = props.cuts.clone();
            builder
                .on_mouse_down_in(|event: MouseEvent| {
                    if event.button == Some(MouseButton::Right) {
                        namui::event::send(Event::RightClick {
                            global_xy: event.global_xy,
                        })
                    }
                })
                .on_key_down(move |event: KeyboardEvent| {
                    if !is_focused {
                        return;
                    }

                    let Some(selected_cut_id) = selected_cut_id else {
                        return;
                    };
                    handle_enter_key(&event, selected_cut_id);
                    handle_move_key(&event, &cuts, selected_cut_id);
                });
        })
    }

    fn render_cut_cell(
        &self,
        wh: Wh<Px>,
        index: usize,
        cut: &Cut,
        memo_count: usize,
        props: &Props,
    ) -> RenderingTree {
        let cut_id = cut.id;
        let is_focused = props.is_focused;
        let is_selected = props.selected_cut_id == Some(cut_id);
        let stroke_color = color::stroke_color(is_selected, is_focused);
        table::padding(
            12.px(),
            table::horizontal([
                table::fixed(24.px(), |wh| {
                    table::vertical([
                        table::fit(
                            table::FitAlign::LeftTop,
                            typography::body::center_top(
                                wh.width,
                                format!("{}", index),
                                stroke_color,
                            ),
                        ),
                        table::fixed(4.px(), |_| RenderingTree::Empty),
                        table::fit(
                            table::FitAlign::LeftTop,
                            render_comment_badge(wh.width, memo_count, stroke_color),
                        ),
                    ])(wh)
                }),
                table::ratio(1, |wh| {
                    simple_rect(
                        wh,
                        stroke_color,
                        if is_selected { 2.px() } else { 1.px() },
                        Color::BLACK,
                    )
                }),
                table::fixed(8.px(), |_wh| RenderingTree::Empty),
            ]),
        )(wh)
        .attach_event(move |builder| {
            builder.on_mouse_down_in(move |event: MouseEvent| {
                if event.button == Some(MouseButton::Left) {
                    namui::event::send(Event::ClickCut { cut_id })
                }
            });
        })
    }
}

fn handle_move_key(event: &KeyboardEvent, cuts: &[Cut], selected_cut_id: Uuid) {
    enum UpDown {
        Up,
        Down,
    }
    let direction = match event.code {
        Code::ArrowUp => UpDown::Up,
        Code::ArrowDown => UpDown::Down,
        Code::Tab => {
            if namui::keyboard::any_code_press([Code::ShiftLeft, Code::ShiftRight]) {
                UpDown::Up
            } else {
                UpDown::Down
            }
        }
        _ => return,
    };

    let cut_index = cuts
        .iter()
        .position(|cut| cut.id == selected_cut_id)
        .unwrap();

    let next_cut_id = match direction {
        UpDown::Up => {
            if cut_index == 0 {
                return;
            }
            cuts[cut_index - 1].id
        }
        UpDown::Down => {
            if cut_index == cuts.len() - 1 {
                return;
            }
            cuts[cut_index + 1].id
        }
    };

    namui::event::send(Event::MoveToNextCutByKeyboard { next_cut_id })
}

fn handle_enter_key(event: &KeyboardEvent, selected_cut_id: Uuid) {
    if event.code != Code::Enter {
        return;
    }

    namui::event::send(Event::PressEnterOnCut {
        cut_id: selected_cut_id,
    })
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

    let paint = PaintBuilder::new()
        .set_style(PaintStyle::Fill)
        .set_color(color);

    render([
        path(path_builder, paint),
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
