use super::{update_data::PatchType, *};

impl LoadedSequenceEditorPage {
    pub fn undo_sequence_change(&mut self) {
        let patch = self.patch_stack.pop();
        if patch.is_none() {
            return;
        }
        let patch = patch.unwrap();

        if patch.0.len() == 0 {
            return;
        }

        self.apply_patch(patch.reverse());

        self.add_patch_to_undo_stack(patch);
    }

    pub fn redo_sequence_change(&mut self) {
        let last_undoed_patch = self.pop_undoed_patch();
        if last_undoed_patch.is_none() {
            return;
        }
        let last_undoed_patch = last_undoed_patch.unwrap();

        if last_undoed_patch.0.len() == 0 {
            return;
        }

        self.apply_patch(last_undoed_patch.clone());
        self.patch_stack.push(last_undoed_patch);
    }

    fn apply_patch(&mut self, patch: rpc::json_patch::RevertablePatch) {
        let mut sequence_value = serde_json::to_value(&self.sequence).unwrap();
        rpc::json_patch::patch(&mut sequence_value, &patch.to_patch()).unwrap();

        self.sequence = serde_json::from_value(sequence_value).unwrap();

        self.send_patch(patch, PatchType::Sequence);
    }

    fn pop_undoed_patch(&mut self) -> Option<rpc::json_patch::RevertablePatch> {
        self.undo_stack.pop()
    }

    fn add_patch_to_undo_stack(&mut self, patch: revert_json_patch::RevertablePatch) {
        self.undo_stack.push(patch);
    }
}
