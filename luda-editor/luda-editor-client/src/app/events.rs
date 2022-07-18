use super::{
    editor::Editor,
    sequence_list::SequenceList,
    storage::GithubStorage,
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
        storage: Arc<dyn GithubStorage>,
        meta_container: Arc<MetaContainer>,
    },
}
