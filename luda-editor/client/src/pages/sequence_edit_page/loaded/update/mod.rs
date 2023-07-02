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
                            move |_| {
                                SEQUENCE_ATOM.update(|sequence| {
                                    sequence.update(SequenceUpdateAction::InsertCut {
                                        cut: Cut::new(uuid()),
                                        after_cut_id: None,
                                    })
                                });
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
                                SEQUENCE_ATOM.update(|sequence| {
                                    sequence.update_cut(
                                        cut_id,
                                        CutUpdateAction::PushScreenGraphic {
                                            graphic_index: uuid(),
                                            screen_graphic: ScreenGraphic::Image(ScreenImage::new(
                                                image_id,
                                            )),
                                        },
                                    )
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
                        let Ok(cg_file) =
                            create_cg(project_id, psd_name, psd_bytes)
                                .await
                                .map_err(|error| {
                                    namui::event::send(InternalEvent::Error(format!(
                                        "create_cg {}",
                                        error.to_string()
                                    )));
                                }) else { return; };
                        CG_FILES_ATOM.update(|cg_files| {
                            cg_files.update_file(cg_file.clone());
                        });

                        let graphic_index = uuid();

                        SEQUENCE_ATOM.update(|sequence| {
                            sequence.update_cut(
                                cut_id,
                                CutUpdateAction::PushScreenGraphic {
                                    graphic_index,
                                    screen_graphic: ScreenGraphic::Cg(ScreenCg::new(&cg_file)),
                                },
                            )
                        });

                        namui::event::send(character_editor::Event::OpenCharacterEditor {
                            target: character_editor::EditTarget::ExistingCharacterPart {
                                cut_id,
                                cg_id: cg_file.id,
                                graphic_index,
                            },
                        });
                    });
                }
                &cut_editor::Event::AddCg { cut_id, ref cg } => {
                    SEQUENCE_ATOM.update(|sequence| {
                        sequence.update_cut(
                            cut_id,
                            CutUpdateAction::PushScreenGraphic {
                                graphic_index: uuid(),
                                screen_graphic: ScreenGraphic::Cg(cg.clone()),
                            },
                        )
                    });
                }
            })
            .is::<components::character_editor::Event>(|event| match event {
                character_editor::Event::MouseDownOutsideCharacterEditor => {
                    self.character_editor = None;
                }
                character_editor::Event::OpenCharacterEditor { target } => {
                    self.character_editor = Some(character_editor::CharacterEditor::new(*target));
                }
            })
            .is::<components::memo_editor::Event>(|event| match event {
                memo_editor::Event::OpenMemoEditor { cut_id } => {
                    self.memo_editor = Some(components::memo_editor::MemoEditor::new(
                        SEQUENCE_ATOM.get_unwrap().id,
                        *cut_id,
                    ));
                }
                memo_editor::Event::CloseMemoEditor => {
                    self.memo_editor = None;
                }
                memo_editor::Event::MemoCreated { memo } => {
                    match self.cut_id_memo_map.get_mut(&memo.cut_id) {
                        Some(memos) => memos.push(memo.clone()),
                        None => {
                            self.cut_id_memo_map.insert(memo.cut_id, vec![memo.clone()]);
                        }
                    }
                }
            })
            .is::<components::memo_list_view::Event>(|event| match event {
                memo_list_view::Event::MemoDeleted { cut_id, memo_id } => {
                    self.cut_id_memo_map
                        .get_mut(cut_id)
                        .map(|memos| memos.retain(|memo| memo.id != *memo_id));
                }
            });

        self.cut_list_view.update(event);
        self.cut_editor.update(event);
        self.character_editor
            .as_mut()
            .map(|editor| editor.update(event));
        self.memo_list_view.update(event);
        self.memo_editor.as_mut().map(|editor| editor.update(event));
    }
}
