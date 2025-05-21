use crate::app::notification;
use namui::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct NameQuickSlot {
    project_id: Uuid,
    names: HashMap<usize, String>,
}
impl NameQuickSlot {
    pub async fn load_from_cache(project_id: Uuid) -> anyhow::Result<Self> {
        cache::get_serde(&Self::cache_key(project_id))
            .await
            .map(|names| Self {
                project_id,
                names: names.unwrap_or_default(),
            })
            .map_err(|error| anyhow!(error))
    }
    fn save_to_cache(&self) {
        let string = serde_json::to_string(&self.names).unwrap();
        let cache_key = Self::cache_key(self.project_id);
        spawn_local(async move {
            let Err(error) = cache::set(&cache_key, string.as_bytes()).await else {
                return;
            };
            notification::error!("Failed to save NameQuickSlot, refresh and try again: {error}")
                .push();
        });
    }
    pub fn get_name(&self, index: usize) -> Option<&String> {
        self.names.get(&index)
    }
    pub fn set_name(&mut self, index: usize, name: String) {
        self.save_to_cache();
        self.names.insert(index, name);
    }
    fn cache_key(project_id: Uuid) -> String {
        const PREFIX: &str = "name_quick_slot/";
        format!("{PREFIX}{project_id}",)
    }
    // TODO: Clear name quick slot cache on project delete
    // pub fn clear_cache(project_id: Uuid) {
    //     let cache_key = Self::cache_key(project_id);
    //     spawn_local(async move {
    //         let Err(error) = cache::delete(&cache_key).await else {
    //             return;
    //         };
    //         push_notification(Notification::error(format!(
    //             "Failed to clear NameQuickSlot, check browser's indexedDB and clear manually if you needed : {error}"
    //         )));
    //     });
    // }
}
