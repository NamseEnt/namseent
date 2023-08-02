mod background_with_event;

use super::*;
use crate::{color, components::sequence_player, pages::sequence_edit_page::atom::SEQUENCE_ATOM};
use background_with_event::*;
use namui::prelude::*;
use namui_prebuilt::*;
use rpc::data::{CgFile, Cut, CutUpdateAction, ScreenCg};
use std::collections::{BTreeSet, VecDeque};

#[namui::component]
pub struct CutEditor<'a> {
    pub wh: Wh<Px>,
    pub cut: &'a Cut,
    pub cuts: &'a Vec<Cut>,
    pub is_focused: bool,
    pub project_id: Uuid,
    pub cg_files: &'a Vec<CgFile>,
    // pub on_click_character_edit: &'a dyn Fn(character_editor::EditTarget),
    pub on_event: &'a dyn Fn(Event2),
}

pub enum Event2 {
    MoveCut {
        cut_id: Uuid,
    },
    ClickCharacterEdit {
        edit_target: character_editor::EditTarget,
    },
}

impl Component for CutEditor<'_> {
    fn render<'a>(&'a self, ctx: RenderCtx<'a>) -> RenderDone {
        let &Self {
            wh,
            ref cut,
            ref cuts,
            is_focused,
            project_id,
            ref cg_files,
            ref on_event,
        } = self;
        let on_event = on_event.clone();
        let (selected_target, set_selected_target) = ctx.use_state::<Option<ClickTarget>>(|| None);
        let (input_req_queue, set_input_req_queue) = ctx.use_state(|| VecDeque::new());
        let (text_input, set_text_input) = ctx.use_state(|| TextInput::new());
        // image_wysiwyg_editor: wysiwyg_editor::WysiwygEditor,
        // context_menu: Option<ContextMenu>,
        let character_name_candidates = use_memo(|| get_character_name_candidates(cuts, cut));

        let content_rect = sequence_player::get_inner_content_rect(wh);
        let cut_id = cut.id;
        let prev_cut_id = prev_cut_id(&cuts, cut_id);
        let next_cut_id = next_cut_id(&cuts, cut_id);

        let focus = closure(move |target: ClickTarget| {
            set_selected_target.set(Some(target));
            match target {
                ClickTarget::CharacterName => {
                    set_input_req_queue
                        .mutate(|x| x.push_back(auto_complete_text_input::Request::Focus));
                }
                ClickTarget::CutText => {
                    text_input.focus();
                }
            }
        });

        let blur = closure(|| {
            namui::log!("blur");
            set_input_req_queue.mutate(|x| x.push_back(auto_complete_text_input::Request::Blur));
            text_input.blur();
            set_selected_target.set(None);
        });

        let move_cut_request = closure(move |up_down: UpDown| {
            if !is_focused {
                return;
            }

            if let Some(selected_target) = *selected_target {
                let target = match (selected_target, up_down) {
                    (ClickTarget::CharacterName, UpDown::Up) => {
                        prev_cut_id.is_some().then(|| ClickTarget::CutText)
                    }
                    (ClickTarget::CharacterName, UpDown::Down) => Some(ClickTarget::CutText),
                    (ClickTarget::CutText, UpDown::Up) => Some(ClickTarget::CharacterName),
                    (ClickTarget::CutText, UpDown::Down) => {
                        next_cut_id.is_some().then(|| ClickTarget::CharacterName)
                    }
                };
                if let Some(target) = target {
                    focus(target);
                }
            }

            match (up_down, prev_cut_id, next_cut_id) {
                (UpDown::Up, Some(cut_id), _) | (UpDown::Down, _, Some(cut_id)) => {
                    on_event.call(Event2::MoveCut { cut_id });
                }
                _ => {}
            }
        });

        ctx.add(BackgroundWithEvent {
            cut: cut.clone(),
            wh,
            is_selecting_target: selected_target.is_some(),
            prev_cut_id,
            next_cut_id,
            on_event: on_event.map(move |e| match e {
                background_with_event::Event::MoveCutRequest { up_down } => {
                    move_cut_request(up_down)
                }
            }),
        });

        let character_name_side = |wh| {
            let content: Box<dyn Component> = {
                let character_name = cut.character_name.clone();
                if is_focused && *selected_target == Some(ClickTarget::CharacterName) {
                    let focus = focus.clone();
                    let move_cut_request = move_cut_request.clone();
                    Box::new(auto_complete_text_input::AutoCompleteTextInput {
                        text: character_name,
                        wh,
                        candidates: character_name_candidates.clone(),
                        on_event: closure(move |e| match e {
                            auto_complete_text_input::Event::TextChange { text } => {
                                SEQUENCE_ATOM.mutate(|sequence| {
                                    sequence.update_cut(
                                        cut_id,
                                        CutUpdateAction::ChangeCharacterName { name: text.clone() },
                                    )
                                });
                            }
                            auto_complete_text_input::Event::EditDone => {
                                focus(ClickTarget::CutText)
                            }
                            auto_complete_text_input::Event::KeyDown { event } => {
                                if event.code == Code::Tab && !event.is_composing {
                                    event.prevent_default();
                                    if namui::keyboard::any_code_press([
                                        Code::ShiftLeft,
                                        Code::ShiftRight,
                                    ]) {
                                        move_cut_request(UpDown::Up)
                                    } else {
                                        focus(ClickTarget::CutText)
                                    }
                                } else if event.code == Code::Escape {
                                    namui::event::send(InternalEvent::EscapeKeyDown)
                                }
                            }
                            auto_complete_text_input::Event::ReqQueuePopFront => {
                                set_input_req_queue.invoke(|x| {
                                    x.pop_front();
                                });
                            }
                        }),
                        req_queue: input_req_queue.clone(),
                    })
                } else {
                    Box::new(text(TextParam {
                        text: character_name,
                        x: 0.px(),
                        y: wh.height / 2.0,
                        align: TextAlign::Left,
                        baseline: TextBaseline::Middle,
                        font_type: sequence_player::CHARACTER_NAME_FONT,
                        style: sequence_player::character_name_text_style(1.one_zero()),
                        max_width: Some(wh.width),
                    }))
                }
            };
            (transparent_rect(wh), content).attach_event(|builder| {
                let focus = focus.clone();
                builder.on_mouse_down_in(move |event: MouseEvent| {
                    if event.button == Some(MouseButton::Left) {
                        focus(ClickTarget::CharacterName)
                    }
                });
            })
        };

        let cut_text_side = |wh: Wh<Px>| {
            let line_text = cut.line.clone();
            let cut_id = cut.id;
            let content: Box<dyn Component> = if is_focused
                && *selected_target == Some(ClickTarget::CutText)
            {
                let focus = focus.clone();
                let move_cut_request = move_cut_request.clone();
                Box::new(
                    text_input.render(text_input::Props {
                        rect: wh.to_rect(),
                        text: line_text,
                        text_align: TextAlign::Left,
                        text_baseline: TextBaseline::Top,
                        font_type: sequence_player::CUT_TEXT_FONT,
                        style: text_input::Style {
                            text: sequence_player::cut_text_style(1.one_zero()),
                            rect: RectStyle {
                                stroke: Some(RectStroke {
                                    color: color::STROKE_FOCUS,
                                    width: 2.px(),
                                    border_position: BorderPosition::Middle,
                                }),
                                fill: Some(RectFill {
                                    color: color::BACKGROUND,
                                }),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        event_handler: Some(
                            text_input::EventHandler::new()
                                .on_text_updated(move |text| {
                                    SEQUENCE_ATOM.mutate(move |sequence| {
                                        sequence.update_cut(
                                            cut_id,
                                            CutUpdateAction::ChangeCutLine { line: text.clone() },
                                        )
                                    });
                                })
                                .on_key_down(move |event: KeyDownEvent| {
                                    if event.code == Code::Tab {
                                        event.prevent_default();
                                        if namui::keyboard::shift_press() {
                                            focus(ClickTarget::CharacterName)
                                        } else {
                                            move_cut_request(UpDown::Down)
                                        }
                                    } else if event.code == Code::Escape {
                                        namui::event::send(InternalEvent::EscapeKeyDown)
                                    }
                                }),
                        ),
                    }),
                )
            } else {
                Box::new(text(TextParam {
                    text: line_text,
                    x: 0.px(),
                    y: 0.px(),
                    align: TextAlign::Left,
                    baseline: TextBaseline::Top,
                    font_type: sequence_player::CUT_TEXT_FONT,
                    style: sequence_player::cut_text_style(1.one_zero()),
                    max_width: Some(wh.width),
                }))
            };

            (transparent_rect(wh), content).attach_event(|builder| {
                let focus = focus.clone();
                builder.on_mouse_down_in(move |event: MouseEvent| {
                    if event.button == Some(MouseButton::Left) {
                        focus(ClickTarget::CutText)
                    }
                });
            })
        };

        ctx.add(hooks::translate(
            content_rect.x(),
            content_rect.y(),
            (
                simple_rect(
                    content_rect.wh(),
                    color::STROKE_NORMAL,
                    1.px(),
                    color::BACKGROUND,
                ),
                wysiwyg_editor::WysiwygEditor {
                    wh: content_rect.wh(),
                    screen_graphics: cut.screen_graphics.clone(),
                    project_id,
                    cut_id,
                    cg_files: cg_files.clone(),
                    on_click_character_edit: on_click_character_edit.clone(),
                },
                sequence_player::render_text_box(content_rect.wh()),
                sequence_player::render_over_text_hooks(
                    content_rect.wh(),
                    character_name_side,
                    cut_text_side,
                ),
            ),
        ));

        // render([

        //     context_menu
        //         .as_ref()
        //         .map_or(RenderingTree::Empty, |context_menu| context_menu.render()),
        // ])})
        ctx.done()
    }
}

pub enum Event {
    AddNewImage {
        png_bytes: Vec<u8>,
        cut_id: Uuid,
    },
    AddNewCg {
        psd_bytes: Vec<u8>,
        psd_name: String,
        cut_id: Uuid,
    },
    AddCg {
        cut_id: Uuid,
        cg: ScreenCg,
    },
}

enum InternalEvent {
    EscapeKeyDown,
    MouseRightButtonDown { global_xy: Xy<Px>, cut_id: Uuid },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClickTarget {
    CharacterName,
    CutText,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UpDown {
    Up,
    Down,
}

impl CutEditor {
    pub fn focus_character_name(&mut self) {
        // focus(ClickTarget::CharacterName);
    }
}

fn prev_cut_id(cuts: &Vec<Cut>, cut_id: Uuid) -> Option<Uuid> {
    cuts.iter().enumerate().find_map(|(i, cut)| {
        if cut.id == cut_id {
            if i == 0 {
                None
            } else {
                Some(cuts[i - 1].id)
            }
        } else {
            None
        }
    })
}

fn next_cut_id(cuts: &Vec<Cut>, cut_id: Uuid) -> Option<Uuid> {
    cuts.iter().enumerate().find_map(|(i, cut)| {
        if cut.id == cut_id {
            if i == cuts.len() - 1 {
                None
            } else {
                Some(cuts[i + 1].id)
            }
        } else {
            None
        }
    })
}

fn get_character_name_candidates(cuts: &[Cut], current_cut: &Cut) -> Vec<String> {
    let character_name = &current_cut.character_name;

    let mut candidates = cuts
        .iter()
        .enumerate()
        .map(|(index, cut)| (index, cut.character_name.clone()))
        .filter(|(_, name)| name != character_name)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let cut_index = cuts
        .iter()
        .position(|cut| cut.id == current_cut.id)
        .unwrap();

    candidates.sort_by_key(|(index, _)| (cut_index as isize - *index as isize).abs());

    candidates
        .into_iter()
        .map(|(_, name)| name)
        .collect::<Vec<_>>()
}
