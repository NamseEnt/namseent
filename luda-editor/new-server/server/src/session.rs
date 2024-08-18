use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub(crate) struct Session {
    user_id: Arc<RwLock<Option<Arc<String>>>>,
}

impl Session {
    pub(crate) fn new() -> Self {
        Self {
            user_id: Default::default(),
        }
    }
    pub(crate) async fn login(&self, user_id: impl ToString) {
        self.user_id
            .write()
            .await
            .replace(user_id.to_string().into());
    }

    pub(crate) async fn user_id(&self) -> Option<Arc<String>> {
        self.user_id.read().await.clone()
    }

    pub(crate) async fn logged_in(&self) -> bool {
        self.user_id.read().await.is_some()
    }
}
