use super::{
    editor::Editor,
    sequence_list::SequenceList,
    storage::Storage,
    types::{AppContext, MetaContainer},
};
use std::sync::Arc;

type PageInitializer<T> = Box<dyn Fn(&AppContext) -> T + Send + Sync>;

pub enum RouterEvent {
    PageChangeToEditorEvent(PageInitializer<Editor>),
    PageChangeToSequenceListEvent(PageInitializer<SequenceList>),
}

pub enum AppEvent {
    Initialized {
        storage: Arc<Storage>,
        meta_container: Arc<MetaContainer>,
    },
}
