use super::*;

impl CutListView {
    pub fn update(&mut self, event: &namui::Event) {
        self.list_view.update(event);
    }
}
