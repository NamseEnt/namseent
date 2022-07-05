use super::{
    editor::EditorProps,
    events::RouterEvent,
    sequence_list::{SequenceList, SequenceListProps},
    types::{meta::Meta, AppContext, Page},
};
use namui::prelude::*;

pub struct RouterProps<'a> {
    pub screen_wh: Wh<Px>,
    pub meta: &'a Meta,
}

pub struct Router {
    page: Page,
    context: AppContext,
}

impl Router {
    pub fn update(&mut self, event: &dyn std::any::Any) {
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

    pub fn render(&self, props: &RouterProps) -> namui::RenderingTree {
        match &self.page {
            Page::Editor(editor) => editor.render(&EditorProps {
                screen_wh: props.screen_wh,
            }),
            Page::SequenceList(sequence_list) => sequence_list.render(&SequenceListProps {
                wh: props.screen_wh,
                subtitle_play_duration_measurer: &props.meta.clone(),
                subtitle_character_color_map: &props.meta.subtitle_character_color_map,
            }),
        }
    }
    pub fn new(context: AppContext) -> Self {
        Self {
            page: Page::SequenceList(SequenceList::new(context.socket.clone())),
            context,
        }
    }
}
