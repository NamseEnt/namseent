use super::*;

impl MemoListView {
    pub fn update(&mut self, event: &namui::Event) {
        self.scroll_view.update(event);
    }
}
