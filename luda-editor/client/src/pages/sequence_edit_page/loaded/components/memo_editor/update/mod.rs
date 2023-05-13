use super::*;

impl MemoEditor {
    pub fn update(&mut self, event: &namui::Event) {
        event.is::<InternalEvent>(|event| match event {
            InternalEvent::ChangeText(text) => {
                self.text = text.clone();
            }
            InternalEvent::SaveButtonClicked => {
                namui::event::send(Event::AddCutMemo {
                    cut_id: self.cut_id,
                    memo: Memo::new(uuid(), self.text.clone()),
                });
                namui::event::send(Event::CloseMemoEditor);
            }
        });
    }
}
