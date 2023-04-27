use super::*;
use crate::{components::context_menu, pages::sequence_edit_page::loaded::components};

impl CutEditor {
    pub fn update(&mut self, event: &namui::Event) {
        event
            .is::<Event>(|event| match event {
                &Event::MoveCutRequest {
                    cut_id: _,
                    to_prev,
                    focused,
                } => {
                    if focused {
                        self.focus(if to_prev {
                            ClickTarget::CutText
                        } else {
                            ClickTarget::CharacterName
                        })
                    }
                }
                &Event::Click { target } => {
                    self.focus(target);
                }
                Event::ChangeCharacterName { .. }
                | Event::ChangeCutLine { .. }
                | Event::AddNewImage { .. } => {}
            })
            .is::<InternalEvent>(|event| match event {
                InternalEvent::EscapeKeyDown => {
                    self.blur();
                }
                InternalEvent::MouseRightButtonDown { global_xy } => {
                    self.context_menu = Some(ContextMenu::new(
                        *global_xy,
                        [context_menu::Item::new_button("Add Image", move || {
                            namui::event::send(
                                components::character_editor::Event::OpenCharacterEditor {
                                    target: character_editor::EditTarget::NewCharacter,
                                },
                            );
                        })],
                    ));
                }
            })
            .is::<text_input::Event>(|event| match event {
                &text_input::Event::Blur { id } => {
                    if id == self.character_name_input.text_input_id()
                        && self.selected_target == Some(ClickTarget::CharacterName)
                    {
                        self.selected_target = None;
                    } else if id == self.text_input.get_id()
                        && self.selected_target == Some(ClickTarget::CutText)
                    {
                        self.selected_target = None;
                    }
                }
                text_input::Event::TextUpdated { .. }
                | text_input::Event::SelectionUpdated { .. }
                | text_input::Event::KeyDown { .. }
                | text_input::Event::Focus { .. } => {}
            })
            .is::<context_menu::Event>(|event| match event {
                context_menu::Event::Close => {
                    self.context_menu = None;
                }
            });

        self.character_name_input.update(event);
        self.image_wysiwyg_editor.update(event);
    }

    pub fn focus(&mut self, target: ClickTarget) {
        self.selected_target = Some(target);
        match target {
            ClickTarget::CharacterName => {
                self.character_name_input.focus();
            }
            ClickTarget::CutText => {
                self.text_input.focus();
            }
        }
    }

    fn blur(&mut self) {
        namui::log!("blur");
        self.character_name_input.blur();
        self.text_input.blur();
        self.selected_target = None;
    }
}
