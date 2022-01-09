use super::{editor::Editor, sequence_list::SequenceList, types::AppContext};

type PageInitializer<T> = Box<dyn Fn(&AppContext) -> T + Send + Sync>;

pub enum RouterEvent {
    PageChangeToEditorEvent(PageInitializer<Editor>),
    // TODO: this gonna be used to make back button in editor
    #[allow(dead_code)]
    PageChangeToSequenceListEvent(PageInitializer<SequenceList>),
}
