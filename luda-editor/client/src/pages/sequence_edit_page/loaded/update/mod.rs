use super::*;

impl LoadedSequenceEditorPage {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::AddCutClicked => {
                    self.editor_history_system.mutate_sequence(|sequence| {
                        let new_cut = crate::storage::Cut::new();

                        self.line_text_inputs
                            .insert(new_cut.id().to_string(), text_input::TextInput::new());

                        sequence.cuts.push(new_cut);
                    });
                }
                Event::Error(_) => todo!(),
                Event::EditorHistorySystemUpdated => {
                    let state = self.editor_history_system.get_state();
                    let sequence = state.sequence;

                    sequence.cuts.iter().for_each(|cut| {
                        if self.line_text_inputs.contains_key(cut.id()) {
                            return;
                        }
                        self.line_text_inputs
                            .insert(cut.id().to_string(), text_input::TextInput::new());
                    });
                    self.line_text_inputs
                        .retain(|key, _| sequence.cuts.iter().any(move |cut| cut.id() == key));
                }
            }
        } else if let Some(event) = event.downcast_ref::<text_input::Event>() {
            if let text_input::Event::TextUpdated { text, .. } = event {
                let selected_cut_id =
                    self.line_text_inputs
                        .iter()
                        .find_map(|(cut_id, text_input)| {
                            if text_input.is_focused() {
                                Some(cut_id)
                            } else {
                                None
                            }
                        });
                self.editor_history_system
                    .mutate_cut(selected_cut_id.unwrap(), |cut| {
                        cut.line = text.clone();
                    });
            }
        }
        //         else if let Some(event) = event.downcast_ref::<crate::storage::Event>() {
        //     match event {
        //         crate::storage::Event::Mutated { encoded_update: _ } => {
        //             self.editor_history_system
        //                 .with_sequence(&self.sequence_id, |sequence| {
        //                     self.line_text_inputs.retain(|cut_id, _| {
        //                         sequence.cuts.iter().any(|cut| cut.id() == cut_id)
        //                     });

        //                     for cut in sequence.cuts.iter() {
        //                         if !self.line_text_inputs.contains_key(cut.id()) {
        //                             self.line_text_inputs
        //                                 .insert(cut.id().to_string(), text_input::TextInput::new());
        //                         }
        //                     }
        //                 });
        //         }
        //     }
        // }

        self.cut_list_view.update(event);
        self.line_text_inputs
            .values_mut()
            .for_each(|text_input| text_input.update(event));
    }
}
