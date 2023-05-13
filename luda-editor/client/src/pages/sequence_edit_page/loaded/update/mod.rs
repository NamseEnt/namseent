mod undo_redo;
mod update_data;

use super::{
    components::{cg_upload::create_cg, image_upload::create_image},
    *,
};
use rpc::data::*;

impl LoadedSequenceEditorPage {
    pub fn update(&mut self, event: &namui::Event) {
        event
            .is::<InternalEvent>(|event| match event {
                InternalEvent::Error(error) => {
                    todo!("error: {error}")
                }
                InternalEvent::ListViewContextMenuAddCutClicked => {
                    let cut_id = uuid();

                    self.update_sequence(|sequence| {
                        let new_cut = Cut::new(cut_id);
                        sequence.cuts.push(new_cut);
                    });
                }
                &InternalEvent::ImageUploaded { cut_id, image_id } => {
                    self.update_cut(cut_id, |cut| {
                        cut.screen_graphics
                            .push(ScreenGraphic::Image(ScreenImage::new(image_id)))
                    });
                }
                &InternalEvent::CgUploaded { cut_id, cg_id } => {
                    let new_cg_graphic_index = self
                        .sequence
                        .cuts
                        .iter()
                        .find(|cut| cut.id() == cut_id)
                        .unwrap()
                        .screen_graphics
                        .len();
                    self.update_cut(cut_id, |cut| {
                        cut.screen_graphics
                            .push(ScreenGraphic::Cg(ScreenCg::new(cg_id, vec![])))
                    });
                    self.character_editor = Some(character_editor::CharacterEditor::new(
                        self.project_id(),
                        character_editor::EditTarget::ExistingCharacterPart {
                            cut_id,
                            cg_id,
                            graphic_index: new_cg_graphic_index,
                        },
                    ))
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
                                namui::event::send(InternalEvent::Error(format!(
                                    "UpdateReceived rpc::json_patch::patch {}",
                                    error.to_string()
                                )));
                                return;
                            }

                            let result = serde_json::from_value::<Sequence>(sequence_json);
                            if let Err(error) = result {
                                namui::event::send(InternalEvent::Error(format!(
                                    "UpdateReceived serde_json::from_value::<Sequence> {}",
                                    error.to_string()
                                )));
                                return;
                            }

                            self.sequence = result.unwrap();
                        }
                    }
                }
            })
            .is::<context_menu::Event>(|event| match event {
                context_menu::Event::Close => {
                    self.context_menu = None;
                }
            })
            .is::<cut_list_view::Event>(|event| match event {
                cut_list_view::Event::RightClick { global_xy } => {
                    self.context_menu = Some(context_menu::ContextMenu::new(
                        *global_xy,
                        [context_menu::Item::new_button("Add Cut", {
                            move || {
                                namui::event::send(InternalEvent::ListViewContextMenuAddCutClicked);
                            }
                        })],
                    ));
                    self.focused_component = Some(FocusableComponent::CutListView);
                }
                &cut_list_view::Event::ClickCut { cut_id } => {
                    self.selected_cut_id = Some(cut_id);
                    self.focused_component = Some(FocusableComponent::CutListView);
                }
                &cut_list_view::Event::MoveToNextCutByKeyboard { next_cut_id } => {
                    assert_eq!(
                        self.focused_component,
                        Some(FocusableComponent::CutListView)
                    );
                    self.selected_cut_id = Some(next_cut_id);
                }
                &cut_list_view::Event::PressEnterOnCut { cut_id } => {
                    assert_eq!(
                        self.focused_component,
                        Some(FocusableComponent::CutListView)
                    );
                    assert_eq!(self.selected_cut_id, Some(cut_id));

                    self.focused_component = Some(FocusableComponent::CutEditor);
                    self.cut_editor.focus_character_name();
                }
            })
            .is::<cut_editor::Event>(|event| match event {
                cut_editor::Event::ChangeCharacterName { name, cut_id } => {
                    self.update_cut(*cut_id, |cut| {
                        cut.character_name = name.clone();
                    });
                }
                cut_editor::Event::ChangeCutLine { text, cut_id } => {
                    self.update_cut(*cut_id, |cut| {
                        cut.line = text.clone();
                    });
                }
                &cut_editor::Event::MoveCutRequest {
                    cut_id,
                    to_prev: _,
                    focused: _,
                } => {
                    self.selected_cut_id = Some(cut_id);
                }
                cut_editor::Event::Click { target: _ } => {
                    self.focused_component = Some(FocusableComponent::CutEditor);
                }
                &cut_editor::Event::AddNewImage {
                    ref png_bytes,
                    cut_id,
                } => {
                    let project_id = self.project_id();
                    let png_bytes = png_bytes.clone();
                    spawn_local(async move {
                        match create_image(project_id, png_bytes).await {
                            Ok(image_id) => {
                                namui::event::send(InternalEvent::ImageUploaded {
                                    cut_id,
                                    image_id,
                                });
                            }
                            Err(error) => {
                                namui::event::send(InternalEvent::Error(format!(
                                    "create_image {}",
                                    error.to_string()
                                )));
                            }
                        };
                    });
                }
                cut_editor::Event::AddNewCg {
                    psd_name,
                    psd_bytes,
                    cut_id,
                } => {
                    let project_id = self.project_id();
                    let psd_bytes = psd_bytes.clone();
                    let psd_name = psd_name.clone();
                    let cut_id = cut_id.clone();
                    spawn_local(async move {
                        match create_cg(project_id, psd_name, psd_bytes).await {
                            Ok(cg_id) => {
                                namui::event::send(InternalEvent::CgUploaded { cut_id, cg_id })
                            }
                            Err(error) => {
                                namui::event::send(InternalEvent::Error(format!(
                                    "create_cg {}",
                                    error.to_string()
                                )));
                            }
                        };
                    });
                }
                cut_editor::Event::AddCg { cut_id, cg } => {
                    let cg = cg.clone();
                    self.update_cut(*cut_id, |cut| {
                        cut.screen_graphics.push(ScreenGraphic::Cg(cg))
                    });
                }
            })
            .is::<wysiwyg_editor::Event>(|event| match event {
                &wysiwyg_editor::Event::UpdateCutGraphics {
                    cut_id,
                    ref callback,
                } => {
                    self.update_cut(cut_id, |cut| callback(&mut cut.screen_graphics));
                }
                wysiwyg_editor::Event::UpdateSequenceGraphics { callback } => {
                    self.update_sequence(|sequence| {
                        sequence.cuts.iter_mut().for_each(|cut| {
                            callback(&mut cut.screen_graphics);
                        });
                    });
                }
            })
            .is::<components::character_editor::Event>(|event| match event {
                character_editor::Event::MouseDownOutsideCharacterEditor => {
                    self.character_editor = None;
                }
                character_editor::Event::OpenCharacterEditor { target } => {
                    self.character_editor = Some(character_editor::CharacterEditor::new(
                        self.project_id(),
                        *target,
                    ));
                }
                &character_editor::Event::UpdateCutGraphics {
                    cut_id,
                    ref callback,
                } => {
                    self.update_cut(cut_id, |cut| callback(&mut cut.screen_graphics));
                }
            });

        self.cut_list_view.update(event);
        self.cut_editor.update(event);
        self.character_editor
            .as_mut()
            .map(|editor| editor.update(event));
        self.memo_list_view.update(event);
    }
}
