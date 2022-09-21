use super::*;

impl CharacterEditModal {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            if let Event::CharacterSelected {
                character_id,
                cut_id: _,
            } = event
            {
                self.character_id = Some(character_id.clone());
            }
        } else if let Some(event) = event.downcast_ref::<InternalEvent>() {
            match event {
                InternalEvent::CharacterRightClicked {
                    character_id,
                    mouse_global_xy,
                    name,
                } => {
                    self.context_menu = Some(context_menu::ContextMenu::new(
                        *mouse_global_xy,
                        [context_menu::Item::new("Edit Name", {
                            let character_id = character_id.clone();
                            let name = name.clone();
                            move || {
                                namui::event::send(InternalEvent::CharacterNameEditClicked {
                                    character_id: character_id.clone(),
                                    name: name.clone(),
                                });
                            }
                        })],
                    ));
                }
                InternalEvent::CharacterNameEditClicked { character_id, name } => {
                    self.editing_text_mode = Some(EditingTextMode::CharacterName {
                        character_id: character_id.clone(),
                        text: name.clone(),
                    });
                    self.text_input.focus();
                }
            }
        } else if let Some(event) = event.downcast_ref::<context_menu::Event>() {
            match event {
                context_menu::Event::Close => {
                    self.context_menu = None;
                }
            }
        } else if let Some(event) = event.downcast_ref::<text_input::Event>() {
            match event {
                text_input::Event::Focus { .. }
                | text_input::Event::Blur { .. }
                | text_input::Event::SelectionUpdated { .. } => {}
                text_input::Event::TextUpdated {
                    id,
                    text: updated_text,
                    ..
                } => {
                    if id.eq(self.text_input.get_id()) {
                        if let Some(editing_text_mode) = self.editing_text_mode.as_mut() {
                            match editing_text_mode {
                                EditingTextMode::CharacterName { text, .. } => {
                                    *text = updated_text.clone();
                                }
                            }
                        }
                    }
                }
                text_input::Event::KeyDown { id, code } => {
                    if id.eq(self.text_input.get_id()) && Code::Enter.eq(code) {
                        if let Some(editing_text_mode) = self.editing_text_mode.as_ref() {
                            match editing_text_mode {
                                EditingTextMode::CharacterName { character_id, text } => {
                                    namui::event::send(Event::CharacterNameChanged {
                                        character_id: character_id.clone(),
                                        name: text.clone(),
                                    })
                                }
                            }
                        }
                        self.editing_text_mode = None;
                    }
                }
            }
        }
        self.character_list_view.update(event);
        self.context_menu
            .as_mut()
            .map(|context_menu| context_menu.update(event));
    }
}
