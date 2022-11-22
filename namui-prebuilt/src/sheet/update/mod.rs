use super::*;

impl<Row, Column> Sheet<Row, Column> {
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
            }
        }
        self.vh_list_view.update(event);
    }
}
