use super::*;

impl AutoCompleteTextInput {
    pub fn update(&mut self, event: &namui::Event) {
        event.is::<InternalEvent>(|event| match event {
            &InternalEvent::ArrowUpDown { next_index } => {
                self.over_item_index = next_index;

                if next_index.is_some() {
                    self.text_input.blur();
                } else {
                    self.text_input.focus();
                }
            }
            &InternalEvent::UpdateItemIndex { over_item_index } => {
                self.over_item_index = over_item_index;
            }
        });
    }
}
