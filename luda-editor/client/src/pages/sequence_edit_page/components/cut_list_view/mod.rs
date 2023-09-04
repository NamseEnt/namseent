mod cut_cell;
use crate::color;
use cut_cell::*;
use namui::prelude::*;
use namui_prebuilt::*;
use rpc::data::{Cut, Memo};
use std::collections::HashMap;

#[namui::component]
pub struct CutListView<'a> {
    pub wh: Wh<Px>,
    pub cuts: &'a Vec<Cut>,
    pub selected_cut_id: Option<Uuid>,
    pub is_focused: bool,
    pub cut_id_memos_map: &'a HashMap<Uuid, Vec<Memo>>,
    pub on_event: Box<dyn 'a + Fn(Event)>,
}

#[allow(clippy::enum_variant_names)]
pub enum Event {
    OnPressEnterOnCut { cut_id: Uuid },
    OnMoveToNextCutByKeyboard { next_cut_id: Uuid },
    OnClickCutEvent { cut_id: Uuid },
    OnRightClickEvent { global_xy: Xy<Px> },
}
impl Component for CutListView<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            cuts,
            selected_cut_id,
            is_focused,
            cut_id_memos_map,
            on_event,
        } = self;
        let on_event = on_event.as_ref();

        let on_key_down = {
            move |event: KeyboardEvent| {
                if !is_focused {
                    return;
                }
                let Some(selected_cut_id) = selected_cut_id else {
                    return;
                };
                if event.code == Code::Enter {
                    on_event(Event::OnPressEnterOnCut {
                        cut_id: selected_cut_id,
                    });
                } else {
                    enum UpDown {
                        Up,
                        Down,
                    }
                    let direction = match event.code {
                        Code::ArrowUp => UpDown::Up,
                        Code::ArrowDown => UpDown::Down,
                        Code::Tab => {
                            if namui::keyboard::any_code_press([Code::ShiftLeft, Code::ShiftRight])
                            {
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
                    on_event(Event::OnMoveToNextCutByKeyboard { next_cut_id });
                }
            }
        };

        let item_wh = Wh::new(wh.width, 128.px());
        ctx.component(list_view::ListView {
            xy: Xy::zero(),
            height: wh.height,
            scroll_bar_width: 12.px(),
            item_wh,
            items: cuts
                .iter()
                .zip(cuts.iter().map(|cut| cut_id_memos_map.get(&cut.id)))
                .enumerate()
                .map(|(index, (cut, memos))| {
                    (
                        cut.id.to_string(),
                        CutCell {
                            wh: item_wh,
                            index,
                            cut: cut.clone(),
                            memo_count: memos.map_or(0, |memos| memos.len()),
                            is_selected: selected_cut_id == Some(cut.id),
                            is_focused,
                            on_click: boxed(|cut_id: Uuid| {
                                on_event(Event::OnClickCutEvent { cut_id })
                            }),
                        },
                    )
                })
                .collect(),
        });
        ctx.component(
            simple_rect(wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND).attach_event(
                move |event| match event {
                    namui::Event::MouseDown { event } => {
                        if event.is_local_xy_in() && event.button == Some(MouseButton::Right) {
                            on_event(Event::OnRightClickEvent {
                                global_xy: event.global_xy,
                            });
                        }
                    }
                    namui::Event::KeyDown { event } => {
                        on_key_down(event);
                    }
                    _ => {}
                },
            ),
        );
        ctx.done()
    }
}
