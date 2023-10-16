use crate::app::notification::{push_notification, Notification};
use namui::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct NameQuickSlot {
    sequence_id: Uuid,
    names: HashMap<usize, String>,
}
impl NameQuickSlot {
    pub async fn load_from_cache(sequence_id: Uuid) -> anyhow::Result<Self> {
        cache::get_serde(&Self::cache_key(sequence_id))
            .await
            .map(|names| Self {
                sequence_id,
                names: names.unwrap_or_default(),
            })
            .map_err(|error| anyhow!(error))
    }
    fn save_to_cache(&self) {
        let string = serde_json::to_string(&self.names).unwrap();
        let cache_key = Self::cache_key(self.sequence_id);
        spawn_local(async move {
            let Err(error) = cache::set(&cache_key, string.as_bytes()).await else {
                return;
            };
            push_notification(Notification::error(format!(
                "NameQuickSlot cache set error: {error}"
            )));
        });
    }
    pub fn get_name(&self, index: usize) -> Option<&String> {
        self.names.get(&index)
    }
    pub fn set_name(&mut self, index: usize, name: String) {
        self.save_to_cache();
        self.names.insert(index, name);
    }
    fn cache_key(sequence_id: Uuid) -> String {
        const PREFIX: &str = "name_quick_slot/";
        format!("{PREFIX}{sequence_id}",)
    }
    pub fn clear_cache(sequence_id: Uuid) {
        let cache_key = Self::cache_key(sequence_id);
        spawn_local(async move {
            let Err(error) = cache::delete(&cache_key).await else {
                return;
            };
            push_notification(Notification::error(format!(
                "NameQuickSlot cache clear error: {error}"
            )));
        });
    }
}
