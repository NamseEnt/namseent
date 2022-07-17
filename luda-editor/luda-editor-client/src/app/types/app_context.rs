use super::MetaContainer;
use crate::app::storage::Storage;
use std::sync::Arc;

pub struct AppContext {
    pub storage: Arc<Storage>,
    pub meta_container: Arc<MetaContainer>,
}
