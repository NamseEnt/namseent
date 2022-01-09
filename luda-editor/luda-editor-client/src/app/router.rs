use super::{
    events::RouterEvent,
    sequence_list::SequenceList,
    types::{AppContext, Page},
};
use namui::Entity;

pub struct Router {
    page: Page,
    context: AppContext,
}

impl Entity for Router {
    type Props = ();

    fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<RouterEvent>() {
            match &event {
                RouterEvent::PageChangeToEditorEvent(initializer) => {
                    self.page = Page::Editor(initializer(&self.context));
                }
                RouterEvent::PageChangeToSequenceListEvent(initializer) => {
                    self.page = Page::SequenceList(initializer(&self.context));
                }
            }
        }
        match &mut self.page {
            Page::Editor(editor) => editor.update(event),
            Page::SequenceList(sequence_list) => sequence_list.update(event),
        }
    }

    fn render(&self, props: &Self::Props) -> namui::RenderingTree {
        match &self.page {
            Page::Editor(editor) => editor.render(props),
            Page::SequenceList(sequence_list) => sequence_list.render(props),
        }
    }
}

impl Router {
    pub fn new(context: AppContext) -> Self {
        Self {
            page: Page::SequenceList(SequenceList::new(
                context.socket.clone(),
                namui::XywhRect {
                    x: 0.0,
                    y: 0.0,
                    width: context.screen_size.width,
                    height: context.screen_size.height,
                },
            )),
            context,
        }
    }
}
