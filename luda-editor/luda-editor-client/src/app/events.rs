use super::{editor::Editor, sequence_list::SequenceList, types::AppContext};

type PageInitializer<T> = Box<dyn Fn(&AppContext) -> T + Send + Sync>;

pub enum RouterEvent {
    PageChangeToEditorEvent(PageInitializer<Editor>),
    PageChangeToSequenceListEvent(PageInitializer<SequenceList>),
}
