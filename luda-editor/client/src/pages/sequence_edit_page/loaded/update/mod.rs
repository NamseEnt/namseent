mod undo_redo;
mod update_data;

use super::*;
use crate::components::sequence_player;
use rpc::data::*;

impl LoadedSequenceEditorPage {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::AddCutClicked => {
                    self.push_back_new_cut();
                }
                Event::Error(error) => {
                    todo!("error: {error}")
                }
                &Event::CharacterCellClicked { cut_id, global_xy } => {
                    match self.sequence.cuts.iter().find(|cut| cut.id() == cut_id) {
                        Some(cut) => {
                            self.character_edit_modal =
                                Some(character_edit_modal::CharacterEditModal::new(
                                    cut_id,
                                    cut.character_id,
                                    global_xy,
                                ));
                        }
                        None => {
                            namui::log!("cut not found: {cut_id}");
                        }
                    }
                }
                &Event::ScreenEditorCellClicked { index, cut_id } => {
                    self.image_select_modal = Some(image_select_modal::ImageSelectModal::new(
                        self.project_id(),
                        move |image_id| {
                            namui::event::send(Event::ScreenEditorConfirmClicked {
                                index,
                                cut_id,
                                image_id,
                            });
                        },
                    ));
                }
                &Event::ScreenEditorConfirmClicked {
                    index,
                    cut_id,
                    image_id,
                } => {
                    self.image_select_modal = None;
                    self.update_cut(cut_id, |cut| cut.screen_image_ids[index] = image_id);
                    if let Some(image_id) = image_id {
                        self.recent_selected_image_ids.retain(|id| id.ne(&image_id));
                        self.recent_selected_image_ids.push_front(image_id);
                        while self.recent_selected_image_ids.len() > 10 {
                            self.recent_selected_image_ids.pop_back();
                        }

                        spawn_local({
                            let recent_selected_image_ids = self.recent_selected_image_ids.clone();
                            async move {
                                let result = namui::cache::set_serde(
                                    "recent_selected_image_ids",
                                    &recent_selected_image_ids,
                                )
                                .await;
                                if let Err(error) = result {
                                    namui::log!(
                                        "failed to save recent_selected_image_ids: {error}"
                                    );
                                    namui::event::send(Event::Error(error.to_string()));
                                }
                            }
                        });
                    }
                }
                Event::UpdateRecentSelectedImageIds { image_ids } => {
                    self.recent_selected_image_ids = image_ids.clone();
                }
                Event::PreviewButtonClicked => {
                    self.sequence_player = Some(sequence_player::SequencePlayer::new(
                        self.sequence.clone(),
                        self.project_shared_data.clone(),
                    ));
                }
                Event::ClosePlayer => {
                    self.sequence_player = None;
                }
                &Event::LineRightClicked { global_xy, cut_id } => {
                    self.context_menu = Some(context_menu::ContextMenu::new(
                        global_xy,
                        [
                            context_menu::Item::new("Delete Cut", {
                                move || {
                                    namui::event::send(Event::DeleteCut { cut_id });
                                }
                            }),
                            context_menu::Item::new("Insert Cut Up", {
                                move || {
                                    namui::event::send(Event::InsertCut {
                                        position: AddCutPosition::Before { cut_id },
                                    });
                                }
                            }),
                            context_menu::Item::new("Insert Cut Down", {
                                move || {
                                    namui::event::send(Event::InsertCut {
                                        position: AddCutPosition::After { cut_id },
                                    });
                                }
                            }),
                        ],
                    ))
                }
                &Event::DeleteCut { cut_id } => {
                    self.update_sequence(|sequence| {
                        sequence.cuts.retain(|cut| cut.id() != cut_id);
                    });
                }
                &Event::InsertCut { position } => match position {
                    AddCutPosition::Before { cut_id } | AddCutPosition::After { cut_id } => {
                        let index = self.sequence.cuts.iter().position(|cut| cut.id() == cut_id);
                        if let Some(index) = index {
                            let cut_id = uuid();

                            self.update_sequence(|sequence| {
                                let new_cut = Cut::new(cut_id);
                                let new_cut_index = match position {
                                    AddCutPosition::Before { .. } => index,
                                    AddCutPosition::After { .. } => index + 1,
                                };
                                sequence.cuts.insert(new_cut_index, new_cut);
                            });

                            self.line_text_inputs
                                .insert(cut_id, text_input::TextInput::new());
                        }
                    }
                },
                Event::DownloadButtonClicked => {
                    let project_shared_data_json =
                        serde_json::to_string(&self.project_shared_data).unwrap();
                    let sequence_json = serde_json::to_string(&self.sequence).unwrap();
                    let project_id = self.project_id();
                    let sequence_name = self.sequence.name.clone();
                    spawn_local(async move {
                        namui::system::file::download(
                            format!("project_{project_id}.json"),
                            project_shared_data_json,
                        )
                        .await
                        .unwrap();

                        namui::system::file::download(
                            format!("sequence_{sequence_name}_{project_id}.json"),
                            sequence_json,
                        )
                        .await
                        .unwrap();
                    });
                }
            }
        } else if let Some(event) = event.downcast_ref::<text_input::Event>() {
            if let text_input::Event::TextUpdated { id, text } = event {
                let selected_cut_id =
                    self.line_text_inputs
                        .iter()
                        .find_map(|(cut_id, text_input)| {
                            if text_input.get_id() == id {
                                Some(cut_id)
                            } else {
                                None
                            }
                        });

                if let Some(selected_cut_id) = selected_cut_id {
                    self.update_cut(*selected_cut_id, |cut| {
                        cut.line = text.clone();
                    });
                }
            }
        } else if let Some(event) = event.downcast_ref::<character_edit_modal::Event>() {
            match event {
                &character_edit_modal::Event::CharacterSelected {
                    cut_id,
                    character_id,
                } => {
                    self.update_cut(cut_id, move |cut| {
                        cut.character_id = Some(character_id);
                    });
                    self.character_edit_modal = None;
                }
                character_edit_modal::Event::AddCharacterClicked => {
                    self.update_project_shared_data(|project_shared_data| {
                        let mut new_character = Character::new(uuid());
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

                        match character {
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
        } else if let Some(event) = event.downcast_ref::<image_select_modal::Event>() {
            match event {
                image_select_modal::Event::Close => self.image_select_modal = None,
                image_select_modal::Event::Error(_) => todo!(),
            }
        } else if let Some(event) = event.downcast_ref::<crate::components::sync::Event>() {
            match event {
                crate::components::sync::Event::UpdateReceived { patch, id } => {
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
        } else if let Some(event) = event.downcast_ref::<context_menu::Event>() {
            match event {
                context_menu::Event::Close => {
                    self.context_menu = None;
                }
            }
        } else if let Some(event) = event.downcast_ref::<namui::event::NamuiEvent>() {
            if let NamuiEvent::KeyDown(event) = event {
                if code_composites_on(
                    event,
                    [
                        vec![Code::ControlLeft, Code::KeyY],
                        vec![Code::ControlLeft, Code::ShiftLeft, Code::KeyZ],
                    ],
                ) && !self.is_any_line_text_input_focused()
                {
                    self.redo_sequence_change();
                } else if code_composites_on(event, [vec![Code::ControlLeft, Code::KeyZ]])
                    && !self.is_any_line_text_input_focused()
                {
                    self.undo_sequence_change();
                } else if code_composites_on(event, [vec![Code::Escape]]) {
                    self.context_menu = None;
                    self.character_edit_modal = None;
                    self.image_select_modal = None;
                    namui::system::text_input::blur();
                } else if code_composites_on(event, [vec![Code::ControlLeft, Code::Enter]])
                    && self.is_any_line_text_input_focused()
                {
                    let focused_cut_id = self
                        .line_text_inputs
                        .iter()
                        .find_map(|(id, text_input)| match text_input.is_focused() {
                            true => Some(id),
                            false => None,
                        })
                        .unwrap();

                    let next_cut_index = self
                        .sequence
                        .cuts
                        .iter()
                        .position(|cut| cut.id().eq(focused_cut_id))
                        .unwrap()
                        + 1;

                    self.insert_new_cut(next_cut_index);
                }

                fn code_composites_on(
                    event: &RawKeyboardEvent,
                    iter: impl IntoIterator<Item = Vec<Code>>,
                ) -> bool {
                    iter.into_iter().any(|codes| {
                        codes
                            .into_iter()
                            .all(|code| event.pressing_codes.contains(&code))
                    })
                }
            }
        }

        self.cut_list_view.update(event);
        self.character_edit_modal
            .as_mut()
            .map(|character_edit_modal| character_edit_modal.update(event));
        self.image_select_modal
            .as_mut()
            .map(|image_select_modal| image_select_modal.update(event));
        self.sequence_player
            .as_mut()
            .map(|sequence_player| sequence_player.update(event));
    }
    fn on_sequence_updated_by_server(&mut self) {
        self.renew_line_text_inputs();
    }
    fn renew_line_text_inputs(&mut self) {
        self.line_text_inputs
            .retain(|cut_id, _| self.sequence.cuts.iter().any(|cut| cut.id() == *cut_id));

        for cut in self.sequence.cuts.iter() {
            if !self.line_text_inputs.contains_key(&cut.id()) {
                self.line_text_inputs
                    .insert(cut.id(), text_input::TextInput::new());
            }
        }
    }
    fn is_any_line_text_input_focused(&self) -> bool {
        self.line_text_inputs
            .iter()
            .any(|(_, text_input)| text_input.is_focused())
    }
    fn push_back_new_cut(&mut self) {
        self.insert_new_cut(self.sequence.cuts.len());
    }
    fn insert_new_cut(&mut self, index: usize) {
        let cut_id = uuid();

        self.update_sequence(|sequence| {
            let new_cut = Cut::new(cut_id);
            sequence.cuts.insert(index, new_cut);
        });

        let text_input = text_input::TextInput::new();

        text_input.focus();

        self.line_text_inputs.insert(cut_id, text_input);
    }
}
