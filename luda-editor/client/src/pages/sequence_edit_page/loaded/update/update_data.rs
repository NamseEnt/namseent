use super::*;
use rpc::data::*;

impl LoadedSequenceEditorPage {
    pub fn update_sequence(&mut self, f: impl FnOnce(&mut Sequence)) {
        let previous_sequence = self.sequence.clone();
        f(&mut self.sequence);

        let patch = get_patch(&previous_sequence, &self.sequence);
        if let Err(error) = patch {
            namui::event::send(Event::Error(format!(
                "UpdateReceived get_patch {}",
                error.to_string()
            )));
            return;
        }
        let patch = patch.unwrap();
        if patch.0.len() == 0 {
            namui::log!("No patch. Are you sure?");
            return;
        }

        self.send_patch(patch.clone(), PatchType::Sequence);
        self.merge_or_push_patch(patch);
        self.undo_stack.clear();
    }
    fn merge_or_push_patch(&mut self, patch: revert_json_patch::RevertablePatch) {
        if let Some(last) = self.patch_stack.last_mut() {
            if patch.0.len() == 1 && last.0.len() == 1 {
                if let revert_json_patch::RevertablePatchOperation::Replace {
                    operation: patch_opeation,
                    ..
                } = patch.0.get(0).unwrap()
                {
                    if let revert_json_patch::RevertablePatchOperation::Replace {
                        operation: last_operation,
                        ..
                    } = last.0.get_mut(0).unwrap()
                    {
                        if patch_opeation.path == last_operation.path {
                            last_operation.value = patch_opeation.value.clone();
                            return;
                        }
                    }
                }
            }
        }

        self.patch_stack.push(patch);
    }
    pub fn update_cut(&mut self, cut_id: Uuid, f: impl FnOnce(&mut Cut)) {
        self.update_sequence(|sequence| {
            let cut = sequence.cuts.iter_mut().find(|cut| cut_id == cut.id());
            if let Some(cut) = cut {
                f(cut);
            }
        });
    }
    pub fn update_project_shared_data(&mut self, f: impl FnOnce(&mut ProjectSharedData)) {
        let prev = self.project_shared_data.clone();
        f(&mut self.project_shared_data);

        let patch = get_patch(&prev, &self.project_shared_data);
        if let Err(error) = patch {
            namui::event::send(Event::Error(format!(
                "UpdateReceived get_patch {}",
                error.to_string()
            )));
            return;
        }
        let patch = patch.unwrap();
        if patch.0.len() == 0 {
            namui::log!("No patch. Are you sure?");
            return;
        }

        self.send_patch(patch, PatchType::ProjectSharedData);
    }
    pub fn send_patch(&mut self, patch: rpc::json_patch::RevertablePatch, patch_type: PatchType) {
        match patch_type {
            PatchType::Sequence => self
                .sequence_syncer
                .push_patch(patch.to_patch(), self.sequence.clone()),
            PatchType::ProjectSharedData => self
                .project_shared_data_syncer
                .push_patch(patch.to_patch(), self.project_shared_data.clone()),
        }
    }
}

pub enum PatchType {
    Sequence,
    ProjectSharedData,
}

fn get_patch(
    prev: impl serde::Serialize,
    next: impl serde::Serialize,
) -> Result<rpc::json_patch::RevertablePatch, Box<dyn std::error::Error>> {
    let patch = rpc::json_patch::diff_revertable(
        &serde_json::to_value(prev)?,
        &serde_json::to_value(next)?,
    );
    Ok(patch)
}
