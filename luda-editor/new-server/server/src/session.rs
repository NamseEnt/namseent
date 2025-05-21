use arc_swap::ArcSwapOption;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) struct Session {
    user_id: Arc<ArcSwapOption<u128>>,
}

impl Session {
    pub(crate) fn new() -> Self {
        Self {
            user_id: Default::default(),
        }
    }
    pub(crate) fn login(&self, user_id: u128) {
        self.user_id.store(Some(user_id.into()));
    }

    pub(crate) fn user_id(&self) -> Option<u128> {
        self.user_id.load().as_ref().map(|id| **id)
    }

    pub(crate) fn logged_in(&self) -> bool {
        self.user_id.load().is_some()
    }
}
