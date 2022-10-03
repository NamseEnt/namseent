use super::*;

impl LoadedSequenceEditorPage {
    pub fn revert_sequence(&mut self) {
        let patch = self.patch_stack.pop();
        if patch.is_none() {
            return;
        }
        let patch = patch.unwrap();

        if patch.0.len() == 0 {
            return;
        }

        let mut sequence_value = serde_json::to_value(&self.sequence).unwrap();
        rpc::json_patch::patch(&mut sequence_value, &patch.to_reversed_patch()).unwrap();

        self.sequence = serde_json::from_value(sequence_value).unwrap();

        self.sequence_syncer
            .push_patch(patch.to_patch(), self.sequence.clone())
    }
}
