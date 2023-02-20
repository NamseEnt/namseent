use super::*;

impl CutEditor {
    pub fn update(&mut self, event: &namui::Event) {
        event.is::<Event>(|event| match event {
            &Event::MoveCutByTab { cut_id: _, to_prev } => self.focus(if to_prev {
                ClickTarget::CutText
            } else {
                ClickTarget::CharacterName
            }),
            &Event::Click { target } => {
                self.focus(target);
            }
            Event::ChangeCharacterName { .. } | Event::ChangeCutLine { .. } => {}
        });

        self.character_name_input.update(event);
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
