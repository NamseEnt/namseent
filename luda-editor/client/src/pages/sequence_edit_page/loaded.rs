use super::{atom::*, components::*, sequence::SequenceWrapped};
use crate::{components::*, pages::sequence_edit_page::components::character_editor::EditTarget};
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
}

impl Component for LoadedSequenceEditorPage {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let &Self {
            ref project_shared_data,
            ref cut_id_memos_map,
            user_id,
            ref sequence,
            wh,
            ref cg_files,
        } = self;
        let (sequence, set_seuqnece) =
            ctx.use_atom_init(&SEQUENCE_ATOM, || SequenceWrapped::new(sequence.clone()));
        // let (cg_files, _set_cg_files) = use_atom_init(&CG_FILES_ATOM, || cg_files.clone());

        let (selected_cut_id, set_selected_cut_id) = ctx.use_state::<Option<Uuid>>(|| None);
        // let (focused_component, set_focused_component) = ctx.use_state(|| None);
        // let (context_menu, set_context_menu) =
        //     ctx.use_state::<Option<context_menu::ContextMenu2>>(|| None);
        // let (character_editor_target, set_character_editor_target) =
        //     ctx.use_state::<Option<EditTarget>>(|| None);
        // let (cut_id_memos_map, set_cut_id_memos_map) = ctx.use_state(|| cut_id_memos_map.clone());
        // let (editing_memo, set_editing_memo) = ctx.use_state(|| None);

        let selected_cut = selected_cut_id.and_then(|id| sequence.cuts.iter().find(|c| c.id == id));
        // let project_id = project_shared_data.id();
        // let sequence_id = sequence.id;

        // enum InternalEvent {
        //     CutListViewEvent { event: cut_list_view::Event },
        //     CutEditorEvent { event: cut_editor::Event2 },
        //     CharacterEdtiorEvent { event: character_editor::Event },
        //     MemoListViewEvent { event: memo_list_view::Event },
        //     MemoEditorEvent { event: memo_editor::Event },
        // }
        ctx.use_children(|ctx| {
            // let memo_list_view_cell: table::hooks::TableCell = {
            //     const MEMO_WINDOW_WIDTH: Px = px(256.0);

            //     let memos = selected_cut.and_then(|cut| self.cut_id_memos_map.get(&cut.id));

            //     match memos {
            //         Some(memos) if !memos.is_empty() => {
            //             table::hooks::fixed(MEMO_WINDOW_WIDTH, move |wh| {
            //                 memo_list_view::MemoListView {
            //                     wh,
            //                     memos: memos.clone(),
            //                     user_id,
            //                     on_event: ctx.event_with_param(|event| {
            //                         Some(InternalEvent::MemoListViewEvent { event })
            //                     }),
            //                 }
            //             })
            //         }
            //         _ => table::hooks::empty(),
            //     }
            // };

            // let character_editor_cell: table::hooks::TableCell = {
            //     const CHARACTER_EDITOR_WIDTH: Px = px(496.0);

            //     match *character_editor_target {
            //         Some(character_editor_target) => {
            //             table::hooks::fixed(CHARACTER_EDITOR_WIDTH, move |wh| {
            //                 character_editor::CharacterEditor {
            //                     wh,
            //                     project_id,
            //                     edit_target: character_editor_target,
            //                     cut: selected_cut.cloned(),
            //                     on_event: ctx.event_with_param(|event| {
            //                         Some(InternalEvent::CharacterEdtiorEvent { event })
            //                     }),
            //                 }
            //             })
            //         }
            //         None => table::hooks::empty(),
            //     }
            // };

            // let cut_list_view_cell = |wh| cut_list_view::CutListView {
            //     wh,
            //     cuts: sequence.cuts.clone(),
            //     selected_cut_id: *selected_cut_id,
            //     is_focused: *focused_component == Some(FocusableComponent::CutListView),
            //     cut_id_memos_map: *cut_id_memos_map.clone(),
            //     on_event: ctx
            //         .event_with_param(|event| Some(InternalEvent::CutListViewEvent { event })),
            // };

            // let cut_editor_cell = |wh| {
            //     selected_cut.map(|selected_cut| cut_editor::CutEditor {
            //         wh,
            //         cut: selected_cut,
            //         cuts: &sequence.cuts,
            //         is_focused: *focused_component == Some(FocusableComponent::CutEditor),
            //         project_id,
            //         cg_files: &cg_files,
            //         on_event: ctx
            //             .event_with_param(|event| Some(InternalEvent::CutEditorEvent { event })),
            //     })
            // };

            ctx.add(table::hooks::horizontal([
                // table::hooks::fixed(220.px(), cut_list_view_cell),
                // table::hooks::ratio(4, cut_editor_cell),
                // character_editor_cell,
                // memo_list_view_cell,
            ])(wh));

            // if let Some(SequenceIdCutId {
            //     sequence_id,
            //     cut_id,
            // }) = *editing_memo
            // {
            //     ctx.add(memo_editor::MemoEditor {
            //         wh,
            //         sequence_id,
            //         cut_id,
            //         on_event: ctx
            //             .event_with_param(|event| Some(InternalEvent::MemoEditorEvent { event })),
            //     })
            // }

            // if let Some(context_menu) = *context_menu {
            //     ctx.add(context_menu);
            // }

            ctx.done()
        })
        // ctx.use_children(
        //     |event| match event {
        //         InternalEvent::CutListViewEvent { event } => match event {
        //             cut_list_view::Event::OnPressEnterOnCut { cut_id } => {
        //                 assert_eq!(*focused_component, Some(FocusableComponent::CutListView));
        //                 assert_eq!(*selected_cut_id, Some(cut_id));

        //                 set_focused_component.set(Some(FocusableComponent::CutEditor));
        //                 todo!("self.cut_editor.focus_character_name();")
        //             }
        //             cut_list_view::Event::OnMoveToNextCutByKeyboard { next_cut_id } => {
        //                 assert_eq!(*focused_component, Some(FocusableComponent::CutListView));

        //                 set_selected_cut_id.set(Some(next_cut_id));
        //             }
        //             cut_list_view::Event::OnClickCutEvent { cut_id } => {
        //                 set_selected_cut_id.set(Some(cut_id));
        //                 set_focused_component.set(Some(FocusableComponent::CutListView));
        //             }
        //             cut_list_view::Event::OnRightClickEvent { global_xy } => {
        //                 set_context_menu.set(Some(context_menu::ContextMenu2::new(
        //                     global_xy,
        //                     [context_menu::Item::new_button(
        //                         "Add Cut",
        //                         set_seuqnece.map_mutate_callback(|sequence| {
        //                             sequence.update(SequenceUpdateAction::InsertCut {
        //                                 cut: Cut::new(uuid()),
        //                                 after_cut_id: None,
        //                             })
        //                         }),
        //                     )],
        //                     set_context_menu.map_set_callback(None),
        //                 )));
        //                 set_focused_component.set(Some(FocusableComponent::CutListView));
        //             }
        //         },
        //         InternalEvent::CutEditorEvent { event } => todo!(),
        //         InternalEvent::CharacterEdtiorEvent { event } => match event {
        //             character_editor::Event::Close => set_character_editor_target.set(None),
        //             character_editor::Event::CgChangeButtonClicked => {
        //                 if let Some(EditTarget::ExistingCharacterPart {
        //                     cut_id,
        //                     graphic_index,
        //                     ..
        //                 }) = *character_editor_target
        //                 {
        //                     set_character_editor_target.set(Some(EditTarget::ExistingCharacter {
        //                         cut_id,
        //                         graphic_index,
        //                     }));
        //                 }
        //             }
        //             character_editor::Event::ChangeEditTarget { edit_target } => {
        //                 set_character_editor_target.set(Some(edit_target));
        //             }
        //         },
        //         InternalEvent::MemoListViewEvent { event } => match event {
        //             memo_list_view::Event::DoneClicked { cut_id, memo_id } => {
        //                 spawn_local(async move {
        //                     match crate::RPC
        //                         .delete_memo(rpc::delete_memo::Request {
        //                             sequence_id,
        //                             memo_id,
        //                         })
        //                         .await
        //                     {
        //                         Ok(_) => {
        //                             set_cut_id_memos_map.mutate(move |cut_id_memos_map| {
        //                                 cut_id_memos_map
        //                                     .get_mut(&cut_id)
        //                                     .unwrap()
        //                                     .retain(|memo| memo.id != memo_id);
        //                             });
        //                         }
        //                         Err(error) => {
        //                             namui::log!("Failed to delete memo: {:?}", error)
        //                         }
        //                     };
        //                 });
        //             }
        //         },
        //         InternalEvent::MemoEditorEvent { event } => match event {
        //             memo_editor::Event::Close => set_editing_memo.set(None),
        //             memo_editor::Event::SaveButtonClicked {
        //                 sequence_id,
        //                 cut_id,
        //                 content,
        //             } => {
        //                 spawn_local(async move {
        //                     match crate::RPC
        //                         .create_memo(rpc::create_memo::Request {
        //                             sequence_id,
        //                             cut_id,
        //                             content,
        //                         })
        //                         .await
        //                     {
        //                         Ok(response) => {
        //                             let memo = response.memo;
        //                             set_cut_id_memos_map.mutate(move |cut_id_memos_map| {
        //                                 match cut_id_memos_map.get_mut(&cut_id) {
        //                                     Some(memos) => memos.push(memo),
        //                                     None => {
        //                                         cut_id_memos_map.insert(memo.cut_id, vec![memo]);
        //                                     }
        //                                 }
        //                             });
        //                             set_editing_memo.set(None);
        //                         }
        //                         Err(error) => {
        //                             namui::log!("Failed to create memo: {:?}", error)
        //                         }
        //                     };
        //                 });
        //             }
        //         },
        //     },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FocusableComponent {
    CutListView,
    CutEditor,
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
