mod update_data;

use super::*;
use rpc::data::*;

impl LoadedSequenceEditorPage {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::AddCutClicked => {
                    let new_cut = Cut::new(nanoid());
                    let cut_id = new_cut.id().to_string();

                    self.update_sequence(|sequence| {
                        sequence.cuts.push(new_cut);
                    });

                    self.line_text_inputs
                        .insert(cut_id, text_input::TextInput::new());
                }
                Event::Error(error) => {
                    todo!("error: {error}")
                }
                Event::CharacterCellClicked { cut_id } => {
                    match self.sequence.cuts.iter().find(|cut| cut.id().eq(cut_id)) {
                        Some(cut) => {
                            self.character_edit_modal =
                                Some(character_edit_modal::CharacterEditModal::new(
                                    cut_id.clone(),
                                    cut.character_id.clone(),
                                ));
                        }
                        None => {
                            namui::log!("cut not found: {cut_id}");
                        }
                    }
                }
            }
        } else if let Some(event) = event.downcast_ref::<text_input::Event>() {
            if let text_input::Event::TextUpdated { id, text } = event {
                let selected_cut_id =
                    self.line_text_inputs
                        .iter()
                        .find_map(|(cut_id, text_input)| {
                            if text_input.get_id().eq(id) {
                                Some(cut_id.clone())
                            } else {
                                None
                            }
                        });

                if let Some(selected_cut_id) = selected_cut_id {
                    self.update_cut(selected_cut_id, |cut| {
                        cut.line = text.clone();
                    });
                }
            }
        } else if let Some(event) = event.downcast_ref::<character_edit_modal::Event>() {
            match event {
                character_edit_modal::Event::CharacterSelected {
                    cut_id,
                    character_id,
                } => {
                    self.update_cut(cut_id, move |cut| {
                        cut.character_id = Some(character_id.clone());
                    });
                    self.character_edit_modal = None;
                }
                character_edit_modal::Event::AddCharacterClicked => {
                    self.update_project_shared_data(|project_shared_data| {
                        let mut new_character = Character::new(nanoid());
                        new_character.name = "New Character".to_string();
                        project_shared_data.characters.push(new_character)
                    });
                }
                character_edit_modal::Event::CharacterNameChanged { character_id, name } => self
                    .update_project_shared_data(|project_shared_data| {
                        let character = project_shared_data
                            .characters
                            .iter_mut()
                            .find(|character| character.id().eq(character_id));

                        match { character } {
                            Some(character) => {
                                character.name = name.clone();
                            }
                            None => {
                                namui::log!(
                                    "[CharacterNameChanged] character {character_id} not found"
                                )
                            }
                        }
                    }),
                character_edit_modal::Event::Close => {
                    self.character_edit_modal = None;
                }
            }
        } else if let Some(event) = event.downcast_ref::<crate::sync::Event>() {
            match event {
                crate::sync::Event::UpdateReceived { patch, id } => {
                    if patch.0.len() > 0 {
                        if self.sequence_syncer.id().eq(id) {
                            let sequence = std::mem::take(&mut self.sequence);
                            let mut sequence_json = serde_json::to_value(sequence).unwrap();

                            let result = rpc::json_patch::patch(&mut sequence_json, patch);
                            if let Err(error) = result {
                                namui::event::send(Event::Error(format!(
                                    "UpdateReceived rpc::json_patch::patch {}",
                                    error.to_string()
                                )));
                                return;
                            }

                            let result = serde_json::from_value::<Sequence>(sequence_json);
                            if let Err(error) = result {
                                namui::event::send(Event::Error(format!(
                                    "UpdateReceived serde_json::from_value::<Sequence> {}",
                                    error.to_string()
                                )));
                                return;
                            }

                            self.sequence = result.unwrap();

                            self.on_sequence_updated_by_server();
                        }
                    }
                }
            }
        }

        self.cut_list_view.update(event);
        self.character_edit_modal
            .as_mut()
            .map(|character_edit_modal| character_edit_modal.update(event));
    }
    fn on_sequence_updated_by_server(&mut self) {
        self.renew_line_text_inputs();
    }
    fn renew_line_text_inputs(&mut self) {
        self.line_text_inputs
            .retain(|cut_id, _| self.sequence.cuts.iter().any(|cut| cut.id() == cut_id));

        for cut in self.sequence.cuts.iter() {
            if !self.line_text_inputs.contains_key(cut.id()) {
                self.line_text_inputs
                    .insert(cut.id().to_string(), text_input::TextInput::new());
            }
        }
    }
}
