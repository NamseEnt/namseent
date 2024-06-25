use arc_swap::ArcSwapOption;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct Session {
    user_id: Arc<ArcSwapOption<String>>,
}

impl Session {
    pub(crate) fn new() -> Self {
        Self {
            user_id: Default::default(),
        }
    }
    pub(crate) fn login(&self, user_id: impl ToString) {
        self.user_id.store(Some(user_id.to_string().into()));
    }
    #[allow(dead_code)]
    pub(crate) fn user_id(&self) -> Option<Arc<String>> {
        (*self.user_id.load()).clone()
    }
}
