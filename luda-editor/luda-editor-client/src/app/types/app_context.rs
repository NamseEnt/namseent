use super::MetaContainer;
use crate::app::storage::GithubStorage;
use std::sync::Arc;

pub struct AppContext {
    pub storage: Arc<dyn GithubStorage>,
    pub meta_container: Arc<MetaContainer>,
}
