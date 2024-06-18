mod background_with_event;

use super::*;
use crate::{
    color,
    components::{context_menu::*, sequence_player},
    pages::sequence_edit_page::atom::SEQUENCE_ATOM,
};
use background_with_event::*;
use namui::*;
use namui_prebuilt::*;
use rpc::data::{CgFile, Cut, CutUpdateAction, SequenceUpdateAction};
use std::collections::BTreeSet;

pub struct CutEditor<'a> {
    pub wh: Wh<Px>,
    pub cut: &'a Cut,
    pub cuts: &'a Vec<Cut>,
    pub is_focused: bool,
    pub project_id: Uuid,
    pub cg_files: &'a Vec<CgFile>,
    pub on_event: Box<dyn 'a + Fn(Event)>,
}

pub enum Event {
    MoveCut {
        cut_id: Uuid,
    },
    ClickCharacterEdit {
        edit_target: character_editor::EditTarget,
    },
    AddMemo {
        cut_id: Uuid,
    },
    Focus,
    AddImageButtonClicked,
}

#[derive(Debug)]
enum ContextMenu {
    CutEditor { cut_id: Uuid },
}

impl Component for CutEditor<'_> {
    fn render(self, ctx: &RenderCtx)  {
        let Self {
            wh,
            cut,
            cuts,
            is_focused,
            project_id,
            cg_files,
            on_event,
        } = self;

        let cut_text_input_instance = namui::text_input::TextInputInstance::new(ctx);
        let cut_name_input_instance = namui::text_input::TextInputInstance::new(ctx);
        let character_name_candidates = ctx.memo(|| get_character_name_candidates(cuts, cut));

        let content_rect = sequence_player::get_inner_content_rect(wh);
        let cut_id: Uuid = cut.id;
        let prev_cut_id = prev_cut_id(cuts, cut_id);
        let next_cut_id = next_cut_id(cuts, cut_id);
        let selected_target = ctx.track_eq(&if cut_text_input_instance.focused() {
            Some(ClickTarget::CutText)
        } else if cut_name_input_instance.focused() {
            Some(ClickTarget::CharacterName)
        } else {
            None
        });

        ctx.effect("Focus on selected_target", || {
            match selected_target.as_ref() {
                Some(_) => on_event(Event::Focus),
                None => {}
            }
        });

        let move_cut_request = &|up_down: UpDown| {
            if !is_focused {
                return;
            }

            if let Some(selected_target) = *selected_target {
                let target = match (selected_target, up_down) {
                    (ClickTarget::CharacterName, UpDown::Up) => {
                        prev_cut_id.is_some().then_some(ClickTarget::CutText)
                    }
                    (ClickTarget::CharacterName, UpDown::Down) => Some(ClickTarget::CutText),
                    (ClickTarget::CutText, UpDown::Up) => Some(ClickTarget::CharacterName),
                    (ClickTarget::CutText, UpDown::Down) => {
                        next_cut_id.is_some().then_some(ClickTarget::CharacterName)
                    }
                };
                if let Some(target) = target {
                    match target {
                        ClickTarget::CharacterName => {
                            cut_name_input_instance.focus();
                        }
                        ClickTarget::CutText => {
                            cut_text_input_instance.focus();
                        }
                    }
                }
            }

            match (up_down, prev_cut_id, next_cut_id) {
                (UpDown::Up, Some(cut_id), _) | (UpDown::Down, _, Some(cut_id)) => {
                    on_event(Event::MoveCut { cut_id });
                }
                _ => {}
            }
        };

        let on_internal_event = |event| match event {
            InternalEvent::MouseRightButtonDown { global_xy, cut_id } => {
                open_context_menu(global_xy, ContextMenu::CutEditor { cut_id });
            }
        };

        if_context_menu_for::<ContextMenu>(|context_menu, builder| match context_menu {
            &ContextMenu::CutEditor { cut_id } => builder
                .add_button(
                    "Add Cg",
                    Box::new(|| {
                        on_event(Event::ClickCharacterEdit {
                            edit_target: character_editor::EditTarget::NewCharacter { cut_id },
                        })
                    }),
                )
                .add_button(
                    "Add Image",
                    Box::new(|| on_event(Event::AddImageButtonClicked)),
                )
                .add_button("Add Memo", Box::new(|| on_event(Event::AddMemo { cut_id }))),
        });

        let character_name_side = |wh, ctx: &mut ComposeCtx| {
            let character_name = cut.character_name.clone();
            let focused = is_focused && *selected_target == Some(ClickTarget::CharacterName);
            ctx.add(auto_complete_text_input::AutoCompleteTextInput {
                text_input_instance: cut_name_input_instance,
                text: character_name,
                wh,
                candidates: character_name_candidates,
                style: text_input::Style {
                    text: sequence_player::character_name_text_style(1.one_zero()),
                    rect: RectStyle {
                        stroke: match focused {
                            true => Some(RectStroke {
                                color: color::STROKE_FOCUS,
                                width: 2.px(),
                                border_position: BorderPosition::Middle,
                            }),
                            false => None,
                        },
                        fill: match focused {
                            true => Some(RectFill {
                                color: color::BACKGROUND,
                            }),
                            false => None,
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                },
                on_event: &|e| match e {
                    text_input::Event::TextUpdated { text } => {
                        let name = text.to_string();
                        SEQUENCE_ATOM.mutate(move |sequence| {
                            sequence
                                .update_cut(cut_id, CutUpdateAction::ChangeCharacterName { name })
                        });
                    }
                    text_input::Event::KeyDown { event } => {
                        if event.is_composing {
                            return;
                        }
                        if event.code == Code::Tab {
                            if namui::keyboard::any_code_press([Code::ShiftLeft, Code::ShiftRight])
                            {
                                move_cut_request(UpDown::Up)
                            } else {
                                cut_text_input_instance.focus();
                            }
                        } else if event.code == Code::Escape {
                            cut_name_input_instance.blur();
                        }
                    }
                    text_input::Event::SelectionUpdated { selection: _ } => {}
                },
            })
            .attach_event(|event| {
                if let namui::Event::MouseDown { event } = event {
                    if !event.is_local_xy_in() {
                        cut_name_input_instance.blur()
                    }
                }
            });
        };

        let cut_text_side = |wh: Wh<Px>, ctx: &mut ComposeCtx| {
            let line_text = cut.line.clone();
            let cut_id = cut.id;

            let focused = is_focused && *selected_target == Some(ClickTarget::CutText);
            let move_cut_request = *move_cut_request;
            ctx.add(TextInput {
                instance: cut_text_input_instance,
                rect: wh.to_rect(),
                text: line_text,
                text_align: TextAlign::Left,
                text_baseline: TextBaseline::Top,
                font: sequence_player::cut_text_font(),
                style: text_input::Style {
                    text: sequence_player::cut_text_style(1.one_zero()),
                    rect: RectStyle {
                        stroke: match focused {
                            true => Some(RectStroke {
                                color: color::STROKE_FOCUS,
                                width: 2.px(),
                                border_position: BorderPosition::Middle,
                            }),
                            false => None,
                        },
                        fill: match focused {
                            true => Some(RectFill {
                                color: color::BACKGROUND,
                            }),
                            false => None,
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                },
                prevent_default_codes: vec![Code::Tab],
                on_event: &|event| match event {
                    text_input::Event::TextUpdated { text } => {
                        let line = text.to_string();
                        SEQUENCE_ATOM.mutate(move |sequence| {
                            sequence.update_cut(cut_id, CutUpdateAction::ChangeCutLine { line })
                        });
                    }
                    text_input::Event::SelectionUpdated { selection: _ } => {}
                    text_input::Event::KeyDown { event } => {
                        if !event.is_composing {
                            let shift_press = namui::keyboard::shift_press();
                            let ctrl_press = namui::keyboard::ctrl_press();

                            if event.code == Code::Tab {
                                if shift_press {
                                    cut_name_input_instance.focus();
                                } else {
                                    move_cut_request(UpDown::Down)
                                }
                            } else if event.code == Code::Escape {
                                cut_text_input_instance.blur();
                            }

                            if ctrl_press && shift_press && event.code == Code::Enter {
                                event.prevent_default();
                                let left_most_cursor =
                                    event.selection_start.min(event.selection_end);
                                SEQUENCE_ATOM.mutate(move |sequence| {
                                    sequence.update(SequenceUpdateAction::SplitCutText {
                                        cut_id,
                                        new_cut_id: uuid(),
                                        split_at: left_most_cursor,
                                    })
                                });
                            }
                        }
                    }
                },
            })
            .attach_event(|event| {
                if let namui::Event::MouseDown { event } = event {
                    if !event.is_local_xy_in() {
                        cut_text_input_instance.blur()
                    }
                }
            });
        };

        ctx.component(
            simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT).attach_event(|event| {
                if let namui::Event::MouseDown { event } = event {
                    if event.is_local_xy_in() {
                        on_event(Event::Focus)
                    }
                }
            }),
        );

        ctx.compose(|ctx| {
            let mut ctx = ctx.translate(content_rect.xy());

            sequence_player::render_over_text_hooks(
                &mut ctx,
                content_rect.wh(),
                character_name_side,
                cut_text_side,
            );

            ctx.add(sequence_player::render_text_box(content_rect.wh()))
                .add(wysiwyg_editor::WysiwygEditor {
                    wh: content_rect.wh(),
                    screen_graphics: cut.screen_graphics.clone(),
                    project_id,
                    cut_id,
                    cg_files: cg_files.clone(),
                    on_click_character_edit: Box::new(|edit_target| {
                        on_event(Event::ClickCharacterEdit { edit_target })
                    }),
                })
                .add(simple_rect(
                    content_rect.wh(),
                    color::STROKE_NORMAL,
                    1.px(),
                    color::BACKGROUND,
                ));
        });

        ctx.component(BackgroundWithEvent {
            cut,
            wh,
            is_selecting_target: selected_target.is_some(),
            on_event: Box::new(|event| match event {
                background_with_event::Event::MoveCutRequest { up_down } => {
                    move_cut_request(up_down)
                }
            }),
            on_internal_event: &on_internal_event,
            project_id,
        });

        
    }
}

enum InternalEvent {
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

fn prev_cut_id(cuts: &[Cut], cut_id: Uuid) -> Option<Uuid> {
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
