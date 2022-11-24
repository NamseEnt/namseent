use super::*;

impl ScreenEditor {
    pub fn update(&mut self, event: &namui::Event) {
        self.wysiwyg_editor.update(event);
    }
}
