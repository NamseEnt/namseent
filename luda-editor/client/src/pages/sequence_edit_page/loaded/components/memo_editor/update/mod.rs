use super::*;

impl MemoEditor {
    pub fn update(&mut self, event: &namui::Event) {
        event.is::<InternalEvent>(|event| match event {
            InternalEvent::ChangeText(text) => {
                self.text = text.clone();
            }
            InternalEvent::SaveButtonClicked => {
                let sequence_id = self.sequence_id;
                let cut_id = self.cut_id;
                let content = self.text.clone();
                spawn_local(async move {
                    match crate::RPC
                        .create_memo(rpc::create_memo::Request {
                            sequence_id,
                            cut_id,
                            content,
                        })
                        .await
                    {
                        Ok(response) => {
                            namui::event::send(Event::MemoCreated {
                                memo: response.memo,
                            });
                            namui::event::send(Event::CloseMemoEditor);
                        }
                        Err(error) => namui::log!("Failed to create memo: {:?}", error),
                    };
                });
            }
        });
    }
}
