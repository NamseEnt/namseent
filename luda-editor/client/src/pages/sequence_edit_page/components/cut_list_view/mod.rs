mod cut_cell;
use crate::{
    app::notification,
    clipboard::{LudaEditorClipboardItem, TryReadLudaEditorClipboardItem},
    color,
    pages::sequence_edit_page::atom::SEQUENCE_ATOM,
};
use cut_cell::*;
use namui::prelude::*;
use namui_prebuilt::*;
use rpc::data::{CgFile, Cut, Memo, MoveCutAction};
use std::collections::HashMap;

static DRAGGING_CONTEXT: Atom<Option<DraggingContext>> = Atom::uninitialized_new();

#[namui::component]
pub struct CutListView<'a> {
    pub wh: Wh<Px>,
    pub cuts: &'a Vec<Cut>,
    pub selected_cut_id: Option<Uuid>,
    pub is_focused: bool,
    pub cut_id_memos_map: &'a HashMap<Uuid, Vec<Memo>>,
    pub on_event: Box<dyn 'a + Fn(Event)>,
    pub project_id: Uuid,
    pub cg_files: &'a Vec<CgFile>,
}

pub enum Event {
    PressEnterOnCut { cut_id: Uuid },
    MoveToNextCutByKeyboard { next_cut_id: Uuid },
    ClickCut { cut_id: Uuid },
    RightClick { global_xy: Xy<Px> },
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
            project_id,
            cg_files,
        } = self;
        let on_event = on_event.as_ref();
        let item_wh = Wh::new(wh.width, 128.px());
        let (dragging, set_dragging) = ctx.atom_init(&DRAGGING_CONTEXT, || None);
        let (scroll_y, set_scroll_y) = ctx.state(|| 0.px());

        let on_key_down = {
            move |event: KeyboardEvent| {
                if !is_focused {
                    return;
                }

                let ctrl_press = namui::keyboard::ctrl_press();
                if ctrl_press && event.code == Code::KeyV {
                    spawn_local(async move {
                        if let Ok(items) = clipboard::read().await {
                            for item in items {
                                if let Some(mut cut) =
                                    TryReadLudaEditorClipboardItem::<Cut>::try_read_from_clipboard(
                                        &item,
                                    )
                                    .await
                                {
                                    cut.id = uuid();
                                    SEQUENCE_ATOM.mutate(move |sequence| {
                                        sequence.update(
                                            rpc::data::SequenceUpdateAction::InsertCut {
                                                cut,
                                                after_cut_id: selected_cut_id,
                                            },
                                        )
                                    });
                                };
                            }
                        }
                    });
                }

                let Some(selected_cut_id) = selected_cut_id else {
                    return;
                };

                if event.code == Code::Enter {
                    on_event(Event::PressEnterOnCut {
                        cut_id: selected_cut_id,
                    });
                } else if ctrl_press && event.code == Code::KeyC {
                    if let Some(selected_cut) = cuts.iter().find(|cut| cut.id == selected_cut_id) {
                        let selected_cut = selected_cut.clone();
                        spawn_local(async move {
                            if let Err(error) = selected_cut.write_to_clipboard().await {
                                notification::error!("{error}").push();
                            };
                        });
                    };
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
                    on_event(Event::MoveToNextCutByKeyboard { next_cut_id });
                }
            }
        };
        let on_mouse_move = |event: MouseEvent| {
            if dragging.is_some() {
                let cursor_located_cut_index = {
                    let local_y_in_content = event.local_xy().y + *scroll_y;
                    ((local_y_in_content / item_wh.height).round() as usize).clamp(0, cuts.len())
                };
                set_dragging.mutate(move |dragging| {
                    let Some(dragging) = dragging else {
                        return;
                    };
                    dragging.end_index = cursor_located_cut_index;
                })
            }
        };
        let on_mouse_up = |_event: MouseEvent| {
            if let Some(dragging) = *dragging {
                let after_cut_id = dragging
                    .end_index
                    .checked_sub(1)
                    .and_then(|index| cuts.get(index))
                    .map(|cut| cut.id);
                if let Ok(move_cut_action) = MoveCutAction::new(dragging.cut_id, after_cut_id) {
                    SEQUENCE_ATOM.mutate(move |sequence| sequence.update(move_cut_action.into()));
                }
            }
            set_dragging.set(None);
        };

        ctx.compose(|ctx| {
            const PADDING: Px = px(24.0);
            const STROKE_WIDTH: Px = px(4.0);
            let Some(dragging) = *dragging else {
                return;
            };
            let cursor_y = item_wh.height * ((dragging.end_index) as f32) - *scroll_y;
            let path = Path::new()
                .move_to(PADDING, cursor_y)
                .line_to(item_wh.width - PADDING, cursor_y);
            let paint = Paint::new()
                .set_style(PaintStyle::Stroke)
                .set_stroke_width(STROKE_WIDTH)
                .set_stroke_cap(StrokeCap::Round)
                .set_color(color::STROKE_FOCUS);
            ctx.add(namui::path(path, paint));
        });

        ctx.component(list_view::ListView {
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
                            on_click: boxed(|cut_id: Uuid| on_event(Event::ClickCut { cut_id })),
                            project_id,
                            cg_files,
                        },
                    )
                })
                .collect(),
            scroll_y: *scroll_y,
            set_scroll_y,
        });

        ctx.component(
            simple_rect(wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND).attach_event(
                move |event| match event {
                    namui::Event::MouseDown { event } => {
                        if event.is_local_xy_in() && event.button == Some(MouseButton::Right) {
                            on_event(Event::RightClick {
                                global_xy: event.global_xy,
                            });
                        }
                    }
                    namui::Event::KeyDown { event } => {
                        on_key_down(event);
                    }
                    namui::Event::MouseMove { event } => {
                        on_mouse_move(event);
                    }
                    namui::Event::MouseUp { event } => {
                        on_mouse_up(event);
                    }
                    _ => {}
                },
            ),
        );
        ctx.done()
    }
}

#[derive(Debug, Clone, Copy)]
struct DraggingContext {
    cut_id: Uuid,
    thumbnail_clicked_offset_xy: Xy<Px>,
    end_index: usize,
}
