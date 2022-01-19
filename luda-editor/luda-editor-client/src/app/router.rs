use super::{
    editor::EditorProps,
    events::RouterEvent,
    sequence_list::{SequenceList, SequenceListProps},
    types::{AppContext, Page},
};
use namui::{Entity, Wh};

pub struct RouterProps {
    pub screen_wh: Wh<f32>,
}

pub struct Router {
    page: Page,
    context: AppContext,
}

impl Entity for Router {
    type Props = RouterProps;

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
            Page::Editor(editor) => editor.render(&EditorProps {
                screen_wh: props.screen_wh,
            }),
            Page::SequenceList(sequence_list) => sequence_list.render(&SequenceListProps {
                wh: props.screen_wh,
            }),
        }
    }
}

impl Router {
    pub fn new(context: AppContext) -> Self {
        Self {
            page: Page::SequenceList(SequenceList::new(context.socket.clone())),
            context,
        }
    }
}
