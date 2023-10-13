use crate::app::notification::{push_notification, Notification};
use namui::prelude::*;
use std::collections::HashMap;

pub static NAME_QUICK_SLOT: Atom<NameQuickSlot> = Atom::uninitialized_new();

const LOCAL_STORAGE_KEY: &str = "/name_quick_slot";

#[derive(Clone, Debug)]
pub struct NameQuickSlot {
    names: HashMap<usize, String>,
}
impl NameQuickSlot {
    pub async fn load_from_cache() -> anyhow::Result<Self> {
        cache::get_serde(LOCAL_STORAGE_KEY)
            .await
            .map(|names| Self {
                names: names.unwrap_or_default(),
            })
            .map_err(|error| anyhow!(error))
    }
    fn save_to_cache(&self) {
        let string = serde_json::to_string(&self.names).unwrap();
        spawn_local(async move {
            let Err(error) = cache::set(LOCAL_STORAGE_KEY, string.as_bytes()).await else {
                return;
            };
            push_notification(Notification::error(error.to_string()));
        });
    }
    pub fn get_name(&self, index: usize) -> Option<&String> {
        self.names.get(&index)
    }
    pub fn set_name(&mut self, index: usize, name: String) {
        self.save_to_cache();
        self.names.insert(index, name);
    }
}
