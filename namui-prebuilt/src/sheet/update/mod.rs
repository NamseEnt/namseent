use super::*;

impl Sheet {
    pub fn update(&mut self, event: &namui::Event) {
        if let Some(event) = event.downcast_ref::<InternalEvent>() {
            match event {
                &InternalEvent::CellMouseLeftDown { cell_index } => {
                    if self.selections.contains(&cell_index) {
                        self.editing_cell = Some(cell_index);
                        self.text_input.focus();
                    } else {
                        self.editing_cell = None;
                        self.selections.clear();
                        self.selections.insert(cell_index);
                    }
                }
                InternalEvent::CtrlCDown { clipboard_items } => {
                    self.clip_board = Some(clipboard_items.clone());
                }
                InternalEvent::CtrlVDown => {
                    let selection_left_top = self
                        .selections
                        .iter()
                        .min_by_key(|selection| (selection.row, selection.column))
                        .unwrap();
                }
            }
        }
        self.vh_list_view.update(event);
    }
}
