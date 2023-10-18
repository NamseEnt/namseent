use super::{atom::*, components::*, sequence::SequenceWrapped};
use crate::{
    components::context_menu::{if_context_menu_for, open_context_menu},
    pages::sequence_edit_page::components::character_editor::EditTarget,
};
use namui::prelude::*;
use namui_prebuilt::*;
use rpc::data::*;
use std::collections::HashMap;

#[namui::component]
pub struct LoadedSequenceEditorPage {
    pub wh: Wh<Px>,
    pub project_shared_data: ProjectSharedData,
    pub cut_id_memos_map: HashMap<Uuid, Vec<Memo>>,
    pub user_id: Uuid,
    pub sequence: Sequence,
    pub cg_files: Vec<CgFile>,
    pub images: Vec<ImageWithLabels>,
}

#[derive(Debug)]
enum ContextMenu {
    CutListView,
}

impl Component for LoadedSequenceEditorPage {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            project_shared_data,
            cut_id_memos_map,
            user_id,
            sequence,
            wh,
            cg_files,
            images,
        } = self;
        let (sequence, set_seqenece) =
            ctx.atom_init(&SEQUENCE_ATOM, || SequenceWrapped::new(sequence.clone()));
        let (cg_files, _set_cg_files) = ctx.atom_init(&CG_FILES_ATOM, || cg_files.clone());
        let (_images, _set_cg_files) = ctx.atom_init(&IMAGES_ATOM, || images.clone());

        let (selected_cut_id, set_selected_cut_id) = ctx.state::<Option<Uuid>>(|| None);
        let (focused_component, set_focused_component) = ctx.atom_init(&FOCUSED_COMPONENT, || None);
        let (character_editor_target, set_character_editor_target) =
            ctx.state::<Option<EditTarget>>(|| None);
        let (cut_id_memos_map, set_cut_id_memos_map) = ctx.state(|| cut_id_memos_map.clone());
        let (editing_memo, set_editing_memo) = ctx.state(|| None);
        let (image_picker_open, set_image_picker_open) = ctx.state(|| false);

        let selected_cut = selected_cut_id.and_then(|id| sequence.cuts.iter().find(|c| c.id == id));
        let project_id = project_shared_data.id();
        let sequence_id = sequence.id;

        enum InternalEvent {
            CutListView { event: cut_list_view::Event },
            CutEditor { event: cut_editor::Event },
            CharacterEdtior { event: character_editor::Event },
            MemoEditor { event: memo_editor::Event },
            ImagePicker { event: image_picker::Event },
            SideBar { event: side_bar::Event },
        }
        let on_internal_event = &|event: InternalEvent| match event {
            InternalEvent::CutListView { event } => match event {
                cut_list_view::Event::PressEnterOnCut { cut_id } => {
                    assert_eq!(*focused_component, Some(FocusableComponent::CutListView));
                    assert_eq!(*selected_cut_id, Some(cut_id));

                    set_focused_component.set(Some(FocusableComponent::CutEditor));
                }
                cut_list_view::Event::MoveToNextCutByKeyboard { next_cut_id } => {
                    assert_eq!(*focused_component, Some(FocusableComponent::CutListView));

                    set_selected_cut_id.set(Some(next_cut_id));
                }
                cut_list_view::Event::ClickCut { cut_id } => {
                    set_selected_cut_id.set(Some(cut_id));
                    set_focused_component.set(Some(FocusableComponent::CutListView));
                }
                cut_list_view::Event::RightClick { global_xy } => {
                    open_context_menu(global_xy, ContextMenu::CutListView);
                    set_focused_component.set(Some(FocusableComponent::CutListView));
                }
            },
            InternalEvent::CutEditor { event } => match event {
                cut_editor::Event::AddMemo { cut_id } => {
                    set_editing_memo.set(Some(SequenceIdCutId {
                        sequence_id,
                        cut_id,
                    }))
                }
                cut_editor::Event::ClickCharacterEdit { edit_target } => {
                    set_character_editor_target.set(Some(edit_target));
                }
                cut_editor::Event::Focus => {
                    set_focused_component.set(Some(FocusableComponent::CutEditor))
                }
                cut_editor::Event::AddImageButtonClicked => {
                    set_image_picker_open.set(true);
                }
                _ => {}
            },
            InternalEvent::CharacterEdtior { event } => match event {
                character_editor::Event::Close => set_character_editor_target.set(None),
                character_editor::Event::CgChangeButtonClicked => {
                    if let Some(EditTarget::ExistingCharacterPart {
                        cut_id,
                        graphic_index,
                        ..
                    }) = *character_editor_target
                    {
                        set_character_editor_target.set(Some(EditTarget::ExistingCharacter {
                            cut_id,
                            graphic_index,
                        }));
                    }
                }
                character_editor::Event::ChangeEditTarget { edit_target } => {
                    set_character_editor_target.set(Some(edit_target));
                }
            },
            InternalEvent::MemoEditor { event } => match event {
                memo_editor::Event::Close => set_editing_memo.set(None),
                memo_editor::Event::SaveButtonClicked {
                    sequence_id,
                    cut_id,
                    content,
                } => {
                    spawn_local(async move {
                        match crate::RPC
                            .create_memo(rpc::create_memo::Request {
                                sequence_id,
                                cut_id,
                                content,
                            })
                            .await
                        {
                            Ok(response) => {
                                let memo = response.memo;
                                set_cut_id_memos_map.mutate(move |cut_id_memos_map| {
                                    match cut_id_memos_map.get_mut(&cut_id) {
                                        Some(memos) => memos.push(memo),
                                        None => {
                                            cut_id_memos_map.insert(memo.cut_id, vec![memo]);
                                        }
                                    }
                                });
                                set_editing_memo.set(None);
                            }
                            Err(error) => {
                                namui::log!("Failed to create memo: {:?}", error)
                            }
                        };
                    });
                }
            },
            InternalEvent::ImagePicker { event } => match event {
                image_picker::Event::Close => {
                    set_image_picker_open.set(false);
                }
            },
            InternalEvent::SideBar { event } => match event {
                side_bar::Event::MemoListView(event) => match event {
                    memo_list_view::Event::DoneClicked { cut_id, memo_id } => {
                        spawn_local(async move {
                            match crate::RPC
                                .delete_memo(rpc::delete_memo::Request {
                                    sequence_id,
                                    memo_id,
                                })
                                .await
                            {
                                Ok(_) => {
                                    set_cut_id_memos_map.mutate(move |cut_id_memos_map| {
                                        cut_id_memos_map
                                            .get_mut(&cut_id)
                                            .unwrap()
                                            .retain(|memo| memo.id != memo_id);
                                    });
                                }
                                Err(error) => {
                                    namui::log!("Failed to delete memo: {:?}", error)
                                }
                            };
                        });
                    }
                },
            },
        };
        let side_bar_cell: table::hooks::TableCell = {
            const SIDE_BAR_WIDTH: Px = px(256.0);

            let memos = selected_cut.and_then(|cut| cut_id_memos_map.get(&cut.id));

            table::hooks::fixed(SIDE_BAR_WIDTH, move |wh, ctx| {
                ctx.add(side_bar::SideBar {
                    wh,
                    project_id,
                    user_id,
                    selected_cut,
                    memos,
                    on_event: Box::new(|event| {
                        on_internal_event(InternalEvent::SideBar { event });
                    }),
                });
            })
        };
        let character_editor_cell = {
            const CHARACTER_EDITOR_WIDTH: Px = px(496.0);

            match *character_editor_target {
                Some(character_editor_target) => {
                    table::hooks::fixed(CHARACTER_EDITOR_WIDTH, move |wh, ctx| {
                        ctx.add(character_editor::CharacterEditor {
                            wh,
                            project_id,
                            edit_target: character_editor_target,
                            cut: selected_cut,
                            on_event: boxed(|event: character_editor::Event| {
                                on_internal_event(InternalEvent::CharacterEdtior { event })
                            }),
                        });
                    })
                }
                None => table::hooks::empty(),
            }
        };
        let image_picker_cell = {
            const IMAGE_PICKER_WIDTH: Px = px(496.0);

            match (*image_picker_open, selected_cut) {
                (true, Some(cut)) => table::hooks::fixed(IMAGE_PICKER_WIDTH, move |wh, ctx| {
                    ctx.add(image_picker::ImagePicker {
                        wh,
                        project_id,
                        cut,
                        on_event: &|event| on_internal_event(InternalEvent::ImagePicker { event }),
                    });
                }),
                _ => table::hooks::empty(),
            }
        };

        let cut_list_view_cell = |wh, ctx: &mut ComposeCtx| {
            ctx.add(cut_list_view::CutListView {
                wh,
                cuts: &sequence.cuts,
                selected_cut_id: *selected_cut_id,
                is_focused: *focused_component == Some(FocusableComponent::CutListView),
                cut_id_memos_map: cut_id_memos_map.as_ref(),
                on_event: Box::new(|event| on_internal_event(InternalEvent::CutListView { event })),
                project_id,
                cg_files: &cg_files,
            });
        };

        let cut_editor_cell = move |wh, ctx: &mut ComposeCtx| {
            if let Some(selected_cut) = selected_cut {
                ctx.add(cut_editor::CutEditor {
                    wh,
                    cut: selected_cut,
                    cuts: &sequence.cuts,
                    is_focused: *focused_component == Some(FocusableComponent::CutEditor),
                    project_id,
                    cg_files: &cg_files,
                    on_event: Box::new(|event| {
                        on_internal_event(InternalEvent::CutEditor { event })
                    }),
                });
            }
        };

        if_context_menu_for::<ContextMenu>(|context_menu, builder| match context_menu {
            ContextMenu::CutListView => builder.add_button("Add Cut", || {
                set_seqenece.mutate(|sequence| {
                    sequence.update(SequenceUpdateAction::InsertCut {
                        cut: Cut::new(uuid()),
                        after_cut_id: None,
                    })
                })
            }),
        });

        ctx.compose(|ctx| {
            if let Some(SequenceIdCutId {
                sequence_id,
                cut_id,
            }) = *editing_memo
            {
                ctx.add(memo_editor::MemoEditor {
                    sequence_id,
                    cut_id,
                    on_event: Box::new(|event| {
                        on_internal_event(InternalEvent::MemoEditor { event })
                    }),
                });
            }
        });

        ctx.compose(|ctx| {
            table::hooks::horizontal([
                table::hooks::fixed(220.px(), cut_list_view_cell),
                table::hooks::ratio(4, cut_editor_cell),
                character_editor_cell,
                side_bar_cell,
                image_picker_cell,
            ])(wh, ctx)
        });

        ctx.done()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CutIdMemoId {
    pub cut_id: Uuid,
    pub memo_id: Uuid,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SequenceIdCutId {
    pub sequence_id: Uuid,
    pub cut_id: Uuid,
}
