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
                items: props.cuts.into_iter().enumerate(),
                item_render: |wh, (index, cut)| self.render_cut_cell(wh, index, cut, &props),
            }),
        ])
        .attach_event(move |builder| {
            let cuts = props.cuts.clone();
            builder
                .on_mouse_down_in(|event| {
                    if event.button == Some(MouseButton::Right) {
                        namui::event::send(Event::RightClick {
                            global_xy: event.global_xy,
                        })
                    }
                })
                .on_key_down(move |event| {
                    if !is_focused {
                        return;
                    }

                    let Some(selected_cut_id) = selected_cut_id else {
                        return;
                    };
                    handle_enter_key(event, selected_cut_id);
                    handle_move_key(event, &cuts, selected_cut_id);
                });
        })
    }

    fn render_cut_cell(&self, wh: Wh<Px>, index: usize, cut: &Cut, props: &Props) -> RenderingTree {
        let cut_id = cut.id();
        let is_focused = props.is_focused;
        let is_selected = props.selected_cut_id == Some(cut_id);
        let stroke_color = color::stroke_color(is_selected, is_focused);
        table::padding(
            12.px(),
            table::horizontal([
                table::fixed(24.px(), |wh| {
                    typography::body::center_top(wh.width, format!("{}", index), stroke_color)
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
            builder.on_mouse_down_in(move |event| {
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
        .position(|cut| cut.id() == selected_cut_id)
        .unwrap();

    let next_cut_id = match direction {
        UpDown::Up => {
            if cut_index == 0 {
                return;
            }
            cuts[cut_index - 1].id()
        }
        UpDown::Down => {
            if cut_index == cuts.len() - 1 {
                return;
            }
            cuts[cut_index + 1].id()
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
