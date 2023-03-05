use super::*;

impl CutEditor {
    pub fn update(&mut self, event: &namui::Event) {
        event
            .is::<Event>(|event| match event {
                &Event::MoveCutByTab { cut_id: _, to_prev } => self.focus(if to_prev {
                    ClickTarget::CutText
                } else {
                    ClickTarget::CharacterName
                }),
                &Event::Click { target } => {
                    self.focus(target);
                }
                Event::ChangeCharacterName { .. }
                | Event::ChangeCutLine { .. }
                | Event::AddNewImage { .. } => {}
            })
            .is::<InternalEvent>(|event| match event {
                &InternalEvent::ClickImage { image_id } => {
                    self.edit_mode = EditMode::Image;
                }
                InternalEvent::ClickImageOutside => {
                    if let EditMode::Image = self.edit_mode {
                        self.edit_mode = EditMode::Idle;
                    }
                }
            })
            .is::<text_input::Event>(|event| match event {
                &text_input::Event::Focus { id } => {
                    if id == self.character_name_input.text_input_id() {
                        self.edit_mode = EditMode::CharacterName;
                    } else if id == self.text_input.get_id() {
                        self.edit_mode = EditMode::CutText;
                    }
                }
                &text_input::Event::Blur { id } => {
                    if id == self.character_name_input.text_input_id() {
                        self.edit_mode = EditMode::Idle;
                    } else if id == self.text_input.get_id() {
                        self.edit_mode = EditMode::Idle;
                    }
                }
                text_input::Event::TextUpdated { .. }
                | text_input::Event::SelectionUpdated { .. }
                | text_input::Event::KeyDown { .. } => {}
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
}
