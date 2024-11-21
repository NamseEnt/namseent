use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub(crate) struct Session {
    user_id: Arc<RwLock<Option<Arc<u128>>>>,
}

impl Session {
    pub(crate) fn new() -> Self {
        Self {
            user_id: Default::default(),
        }
    }
    pub(crate) async fn login(&self, user_id: u128) {
        self.user_id.write().await.replace(user_id.into());
    }

    pub(crate) async fn user_id(&self) -> Option<u128> {
        self.user_id.read().await.clone().map(|id| *id)
    }

    pub(crate) async fn logged_in(&self) -> bool {
        self.user_id.read().await.is_some()
    }
}
