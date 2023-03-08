mod copy_issue_info;
mod undo_redo;
mod update_data;

use super::*;
use crate::components::sequence_player;
use rpc::data::*;

impl LoadedSequenceEditorPage {
    pub fn update(&mut self, event: &namui::Event) {
        event
            .is::<Event>(|event| {
                match event {
                    Event::AddCutClicked => {
                        self.push_back_cut(Cut::new(uuid()));
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
                    &Event::ScreenEditorCellMouseLeftDown { index, cut_id } => {
                        self.image_select_modal = Some(image_select_modal::ImageSelectModal::new(
                            self.project_id(),
                            cut_id,
                            index,
                            |update| {
                                namui::event::send(Event::ImageUpdated {
                                    cut_id: update.cut_id,
                                    screen_images: update.screen_images,
                                })
                            },
                        ));
                    }
                    &Event::ImageUpdated {
                        cut_id,
                        ref screen_images,
                    } => {
                        let previous_image_ids = self
                            .cut(cut_id)
                            .unwrap()
                            .screen_images
                            .iter()
                            .filter_map(|image| image.as_ref().map(|image| image.id))
                            .collect::<Vec<_>>();

                        let new_image_ids = screen_images
                            .iter()
                            .filter_map(|image| {
                                let image_id = image.as_ref().map(|image| image.id);
                                if let Some(image_id) = image_id {
                                    if !previous_image_ids.contains(&image_id) {
                                        Some(image_id)
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>();

                        self.update_cut(cut_id, |cut| {
                            cut.screen_images = screen_images.clone();
                        });

                        if !new_image_ids.is_empty() {
                            self.recent_selected_image_ids = new_image_ids
                                .clone()
                                .into_iter()
                                .chain(
                                    self.recent_selected_image_ids
                                        .clone()
                                        .into_iter()
                                        .filter(|id| !new_image_ids.contains(id)),
                                )
                                .take(10)
                                .collect();

                            spawn_local({
                                let recent_selected_image_ids =
                                    self.recent_selected_image_ids.clone();
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
                            0,
                        ));
                    }
                    Event::ClosePlayer => {
                        self.sequence_player = None;
                    }
                    &Event::LineRightClicked { global_xy, cut_id } => {
                        /// This is for rust-analyzer, it doesn't run inside of macro vec![].
                        fn vec<T>(items: impl IntoIterator<Item = T>) -> Vec<T> {
                            items.into_iter().collect()
                        }

                        self.context_menu = Some(context_menu::ContextMenu::new(
                            global_xy,
                            [
                                vec([context_menu::Item::new_button("Delete Cut", {
                                    move || {
                                        namui::event::send(Event::DeleteCut { cut_id });
                                    }
                                })]),
                                vec([
                                    context_menu::Item::new_button("Insert Cut Up ↑", {
                                        move || {
                                            namui::event::send(Event::InsertCut {
                                                position: AddCutPosition::Before { cut_id },
                                            });
                                        }
                                    }),
                                    context_menu::Item::new_button("Insert Cut Down ↓", {
                                        move || {
                                            namui::event::send(Event::InsertCut {
                                                position: AddCutPosition::After { cut_id },
                                            });
                                        }
                                    }),
                                ]),
                                vec([context_menu::Item::new_button("Start preview from here", {
                                    move || {
                                        namui::event::send(Event::StartPreviewFromHere { cut_id })
                                    }
                                })]),
                                vec([
                                    context_menu::Item::new_button("Cut whole cut", {
                                        move || namui::event::send(Event::CutTheCut { cut_id })
                                    }),
                                    context_menu::Item::new_button("Copy whole cut", {
                                        move || namui::event::send(Event::CopyTheCut { cut_id })
                                    }),
                                ])
                                .into_iter()
                                .chain(
                                    if self.cut_clipboard.is_some() {
                                        vec([
                                            context_menu::Item::Divider,
                                            context_menu::Item::new_button("Paste Cut Up", {
                                                move || {
                                                    namui::event::send(Event::PasteCutUp { cut_id })
                                                }
                                            }),
                                            context_menu::Item::new_button("Paste Cut Down", {
                                                move || {
                                                    namui::event::send(Event::PasteCutDown {
                                                        cut_id,
                                                    })
                                                }
                                            }),
                                        ])
                                    } else {
                                        vec([])
                                    }
                                    .into_iter(),
                                )
                                .collect(),
                                vec([context_menu::Item::new_button("Copy images", {
                                    move || namui::event::send(Event::CopyImages { cut_id })
                                })])
                                .into_iter()
                                .chain(
                                    if self.images_clipboard.is_some() {
                                        vec([context_menu::Item::new_button("Paste images", {
                                            move || {
                                                namui::event::send(Event::PasteImages { cut_id })
                                            }
                                        })])
                                    } else {
                                        vec([])
                                    }
                                    .into_iter(),
                                )
                                .collect(),
                                vec([context_menu::Item::new_button(
                                    "Copy issue info for this cut",
                                    move || namui::event::send(Event::CopyIssueInfo { cut_id }),
                                )]),
                                vec([context_menu::Item::new_button(
                                    "Copy preview link 🔗",
                                    move || namui::event::send(Event::CopyPreviewLink { cut_id }),
                                )]),
                            ]
                            .join(&context_menu::Item::Divider),
                        ))
                    }
                    &Event::DeleteCut { cut_id } => {
                        self.update_sequence(|sequence| {
                            sequence.cuts.retain(|cut| cut.id() != cut_id);
                        });
                    }
                    &Event::InsertCut { position } => match position {
                        AddCutPosition::Before { cut_id } | AddCutPosition::After { cut_id } => {
                            let index =
                                self.sequence.cuts.iter().position(|cut| cut.id() == cut_id);
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
                    &Event::StartPreviewFromHere { cut_id } => {
                        let cut_index =
                            self.sequence.cuts.iter().position(|cut| cut.id() == cut_id);
                        if let Some(cut_index) = cut_index {
                            self.sequence_player = Some(sequence_player::SequencePlayer::new(
                                self.sequence.clone(),
                                self.project_shared_data.clone(),
                                cut_index,
                            ));
                        }
                    }
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
                    &Event::CutTheCut { cut_id } => {
                        let clip_index =
                            self.sequence.cuts.iter().position(|cut| cut.id() == cut_id);
                        if let Some(clip_index) = clip_index {
                            let clip = self.sequence.cuts[clip_index].clone();
                            self.update_sequence(|sequence| {
                                sequence.cuts.remove(clip_index);
                            });
                            self.cut_clipboard = Some(clip);
                        }
                    }
                    &Event::CopyTheCut { cut_id } => {
                        let clip_index =
                            self.sequence.cuts.iter().position(|cut| cut.id() == cut_id);
                        if let Some(clip_index) = clip_index {
                            let clip = self.sequence.cuts[clip_index].clone();
                            self.cut_clipboard = Some(clip);
                        }
                    }
                    &Event::PasteCutUp { cut_id } => {
                        if let Some(cut_clipboard) = self.cut_clipboard.as_ref() {
                            let clip_index =
                                self.sequence.cuts.iter().position(|cut| cut.id() == cut_id);
                            if let Some(clip_index) = clip_index {
                                let index_to_insert = clip_index;

                                let new_cut = cut_clipboard.duplicate(uuid());
                                self.insert_cut(index_to_insert, new_cut)
                            }
                        }
                    }
                    &Event::PasteCutDown { cut_id } => {
                        if let Some(cut_clipboard) = self.cut_clipboard.as_ref() {
                            let clip_index =
                                self.sequence.cuts.iter().position(|cut| cut.id() == cut_id);
                            if let Some(clip_index) = clip_index {
                                let index_to_insert = clip_index + 1;

                                let new_cut = cut_clipboard.duplicate(uuid());
                                self.insert_cut(index_to_insert, new_cut)
                            }
                        }
                    }
                    &Event::CopyImages { cut_id } => {
                        let cut = self.sequence.cuts.iter().find(|cut| cut.id() == cut_id);
                        if let Some(cut) = cut {
                            self.images_clipboard = Some(cut.screen_images.clone());
                        }
                    }
                    &Event::PasteImages { cut_id } => {
                        if let Some(images_clipboard) = self.images_clipboard.clone() {
                            self.update_cut(cut_id, |cut| {
                                cut.screen_images = images_clipboard;
                            })
                        }
                    }
                    Event::ImageManagerButtonClicked => {
                        self.image_manager_modal =
                            Some(image_manager_modal::ImageManagerModal::new(
                                self.project_shared_data.id(),
                            ));
                    }
                    &Event::CopyIssueInfo { cut_id } => {
                        self.copy_issue_info(cut_id);
                    }
                    &Event::CopyPreviewLink { cut_id } => {
                        let share_preview = crate::share_preview::SharePreview {
                            index: self
                                .sequence
                                .cuts
                                .iter()
                                .position(|cut| cut.id() == cut_id)
                                .unwrap(),
                            sequence_id: self.sequence.id(),
                        };

                        let url = share_preview.url();

                        spawn_local(async move {
                            let result = namui::system::clipboard::write_text(url).await;
                            if let Err(_) = result {
                                namui::event::send(Event::Error(
                                    "Failed to copy to clipboard".to_string(),
                                ));
                            }
                        })
                    }
                    Event::RedoSequenceChange => {
                        self.redo_sequence_change();
                    }
                    Event::UndoSequenceChange => {
                        self.undo_sequence_change();
                    }
                    Event::EscapeKeyDown => {
                        self.context_menu = None;
                        self.character_edit_modal = None;
                        self.image_select_modal = None;
                        self.text_input_selected_cut_id = None;
                        namui::system::text_input::blur();
                    }
                    Event::CtrlEnterKeyDown => {
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

                        self.insert_cut(next_cut_index, Cut::new(uuid()));
                    }
                }
            })
            .is::<text_input::Event>(|event| {
                if let text_input::Event::TextUpdated { id, text } = event {
                    let selected_cut_id =
                        self.line_text_inputs
                            .iter()
                            .find_map(|(cut_id, text_input)| {
                                if text_input.get_id() == *id {
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
                } else if let &text_input::Event::Focus { id } = event {
                    self.text_input_selected_cut_id =
                        self.line_text_inputs
                            .iter()
                            .find_map(|(cut_id, text_input)| {
                                if text_input.get_id() == id {
                                    Some(*cut_id)
                                } else {
                                    None
                                }
                            });
                }
            })
            .is::<character_edit_modal::Event>(|event| match event {
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
            })
            .is::<image_select_modal::Event>(|event| match event {
                image_select_modal::Event::Close => self.image_select_modal = None,
                image_select_modal::Event::Error(error) => {
                    namui::log!("image_select_modal::Event::Error: {error}")
                }
            })
            .is::<crate::components::sync::Event>(|event| match event {
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
            })
            .is::<context_menu::Event>(|event| match event {
                context_menu::Event::Close => {
                    self.context_menu = None;
                }
            })
            .is::<image_manager_modal::Event>(|event| match event {
                image_manager_modal::Event::Close => {
                    self.image_manager_modal = None;
                }
                image_manager_modal::Event::Error(error) => todo!("{}", error),
            });

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
        self.context_menu
            .as_mut()
            .map(|context_menu| context_menu.update(event));
        self.image_manager_modal
            .as_mut()
            .map(|image_manager_modal| image_manager_modal.update(event));
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
    fn push_back_cut(&mut self, cut: Cut) {
        self.insert_cut(self.sequence.cuts.len(), cut);
    }
    fn insert_cut(&mut self, index: usize, cut: Cut) {
        let cut_id = cut.id();

        self.update_sequence(|sequence| {
            sequence.cuts.insert(index, cut);
        });

        let text_input = text_input::TextInput::new();

        text_input.focus();

        self.line_text_inputs.insert(cut_id, text_input);
    }
}
