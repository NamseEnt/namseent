// use super::*;
// use crate::pages::sequence_edit_page::{
//     cg_files_atom::CG_FILES_ATOM, sequence_atom::SEQUENCE_ATOM,
// };
// use rpc::data::ScreenCg;

// impl CharacterEditor {
//     pub fn update(&mut self, event: &namui::Event) {
//         event.is::<InternalEvent>(|event| {
//             match event {
//                 InternalEvent::OpenTooltip { global_xy, text } => {
//                     self.tooltip = Some(Tooltip {
//                         global_xy: *global_xy,
//                         text: text.clone(),
//                     });
//                 }
//                 InternalEvent::CloseTooltip => {
//                     self.tooltip = None;
//                 }
//                 InternalEvent::CgChangeButtonClicked => {
//                     if let EditTarget::ExistingCharacterPart {
//                         cut_id,
//                         graphic_index,
//                         ..
//                     } = self.edit_target
//                     {
//                         self.edit_target = EditTarget::ExistingCharacter {
//                             cut_id,
//                             graphic_index,
//                         };
//                     }
//                 }
//                 &InternalEvent::CgThumbnailClicked { cg_id } => match self.edit_target {
//                     EditTarget::NewCharacter { cut_id } => {
//                         let cg_files = CG_FILES_ATOM.get_unwrap();
//                         let Some(cg_file) = cg_files
//                             .iter()
//                             .find(|file| file.id == cg_id) else {
//                                 return;
//                             };

//                         let graphic_index: Uuid = Uuid::new_v4();

//                         SEQUENCE_ATOM.mutate(|sequence| {
//                             sequence.update_cut(
//                                 cut_id,
//                                 CutUpdateAction::PushScreenGraphic {
//                                     graphic_index,
//                                     screen_graphic: ScreenGraphic::Cg(ScreenCg::new(cg_file)),
//                                 },
//                             )
//                         });
//                         namui::event::send(InternalEvent::FocusCg {
//                             cut_id,
//                             cg_id,
//                             graphic_index,
//                         });
//                     }
//                     EditTarget::ExistingCharacter {
//                         cut_id,
//                         graphic_index,
//                     } => {
//                         let cg_files = CG_FILES_ATOM.get_unwrap();
//                         let Some(cg_file) = cg_files
//                             .iter()
//                             .find(|file| file.id == cg_id) else {
//                                 return;
//                             };

//                         SEQUENCE_ATOM.mutate(|sequence| {
//                             sequence.update_cut(
//                                 cut_id,
//                                 CutUpdateAction::ChangeCgKeepCircumscribed {
//                                     graphic_index,
//                                     cg: ScreenCg::new(cg_file),
//                                 },
//                             )
//                         });
//                         namui::event::send(InternalEvent::FocusCg {
//                             cut_id,
//                             cg_id,
//                             graphic_index,
//                         });
//                     }
//                     _ => {}
//                 },
//                 &InternalEvent::FocusCg {
//                     cut_id,
//                     cg_id,
//                     graphic_index,
//                 } => {
//                     self.edit_target = EditTarget::ExistingCharacterPart {
//                         cut_id,
//                         cg_id,
//                         graphic_index,
//                     };
//                 }
//             };
//         });
//         self.scroll_view.update(event);
//     }
// }
