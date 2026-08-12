use crate::game_state::card_service::CardServiceBehavior;
use crate::game_state::item::ItemBehavior;
use crate::game_state::upgrade::UpgradeBehavior;
use crate::game_state::{GameFlow, GameState};
use crate::shop::ShopSlot;
use namui::*;
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};
use std::sync::{Mutex, OnceLock};

const STORAGE_KEY: &str = "tower-defense-encyclopedia";
const VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq, Eq, State)]
pub(crate) enum Discovery {
    Item(String),
    CardService(String),
    Treasure(String),
}

#[derive(Clone, Debug, Default, SerdeSerialize, SerdeDeserialize, State)]
pub(crate) struct DiscoveryState {
    #[serde(skip)]
    revision: u64,
    #[serde(default)]
    pub(crate) version: u32,
    #[serde(default)]
    pub(crate) items: Vec<String>,
    #[serde(default)]
    pub(crate) card_services: Vec<String>,
    #[serde(default)]
    pub(crate) treasures: Vec<String>,
    #[serde(skip)]
    pub(crate) loaded: bool,
    #[serde(skip)]
    dirty: bool,
}

impl DiscoveryState {
    fn sanitized(mut self) -> Self {
        self.version = VERSION;
        deduplicate(&mut self.items);
        deduplicate(&mut self.card_services);
        deduplicate(&mut self.treasures);
        self.loaded = false;
        self.dirty = false;
        self
    }

    fn insert(&mut self, discovery: Discovery) -> bool {
        let inserted = match discovery {
            Discovery::Item(key) => insert_key(&mut self.items, key),
            Discovery::CardService(key) => insert_key(&mut self.card_services, key),
            Discovery::Treasure(key) => insert_key(&mut self.treasures, key),
        };
        if inserted {
            self.revision = self.revision.wrapping_add(1);
        }
        inserted
    }

    fn merge(&mut self, other: &DiscoveryState) -> bool {
        let mut inserted = false;
        for key in &other.items {
            inserted |= self.insert(Discovery::Item(key.clone()));
        }
        for key in &other.card_services {
            inserted |= self.insert(Discovery::CardService(key.clone()));
        }
        for key in &other.treasures {
            inserted |= self.insert(Discovery::Treasure(key.clone()));
        }
        inserted
    }

    fn take_dirty_snapshot(&mut self, headless: bool) -> Option<Self> {
        if headless || !self.loaded || !self.dirty {
            return None;
        }
        self.dirty = false;
        Some(self.clone())
    }
}

fn insert_key(keys: &mut Vec<String>, key: String) -> bool {
    if keys.iter().any(|known| known == &key) {
        return false;
    }
    keys.push(key);
    true
}

fn deduplicate(keys: &mut Vec<String>) {
    keys.sort();
    keys.dedup();
}

pub(crate) async fn load_async() -> DiscoveryState {
    let raw = namui::system::kv_store::get(STORAGE_KEY).await;
    raw.and_then(|bytes| String::from_utf8(bytes).ok())
        .and_then(|raw| serde_json::from_str::<DiscoveryState>(&raw).ok())
        .unwrap_or_default()
        .sanitized()
}

struct SaveQueue {
    latest: Option<String>,
    worker_running: bool,
}

static SAVE_QUEUE: OnceLock<Mutex<SaveQueue>> = OnceLock::new();

pub(crate) fn request_save(snapshot: DiscoveryState) {
    let Ok(serialized) = serde_json::to_string(&snapshot) else {
        return;
    };
    let queue = SAVE_QUEUE.get_or_init(|| {
        Mutex::new(SaveQueue {
            latest: None,
            worker_running: false,
        })
    });
    let should_start_worker = {
        let Ok(mut queue) = queue.lock() else {
            return;
        };
        queue.latest = Some(serialized);
        if queue.worker_running {
            false
        } else {
            queue.worker_running = true;
            true
        }
    };
    if should_start_worker {
        spawn(save_worker());
    }
}

async fn save_worker() {
    loop {
        let serialized = {
            let queue = SAVE_QUEUE.get().expect("save queue initialized");
            let Ok(mut queue) = queue.lock() else {
                return;
            };
            let Some(serialized) = queue.latest.take() else {
                queue.worker_running = false;
                return;
            };
            serialized
        };
        namui::system::kv_store::put(STORAGE_KEY, Some(serialized.as_bytes())).await;
    }
}

impl GameState {
    pub(crate) fn discover_item(&mut self, item: &crate::game_state::item::Item) {
        self.discover(Discovery::Item(item.key().to_string()));
    }

    pub(crate) fn discover_card_service(
        &mut self,
        card_service: &crate::game_state::card_service::CardService,
    ) {
        self.discover(Discovery::CardService(card_service.key().to_string()));
    }

    pub(crate) fn discover_treasure(&mut self, upgrade: crate::game_state::upgrade::Upgrade) {
        self.discover(Discovery::Treasure(upgrade.key().to_string()));
    }

    pub(crate) fn discover_inventory_items(&mut self) {
        let items = self
            .items
            .iter()
            .map(|item| item.item.clone())
            .collect::<Vec<_>>();
        for item in items {
            self.discover_item(&item);
        }
    }

    pub(crate) fn discover_shop_slot(&mut self, slot: &ShopSlot) {
        match slot {
            ShopSlot::Item { item, .. } => self.discover_item(item),
            ShopSlot::CardService { card_service, .. } => self.discover_card_service(card_service),
            ShopSlot::Upgrade { upgrade, .. } => self.discover_treasure(*upgrade),
        }
    }

    pub(crate) fn discover_shop(&mut self) {
        let slots = match &self.flow {
            GameFlow::Shopping(flow) => flow.shop.slots.clone(),
            _ => return,
        };
        for slot in slots {
            self.discover_shop_slot(&slot.slot);
        }
    }

    pub(crate) fn discover_treasure_options(&mut self) {
        let options = match &self.flow {
            GameFlow::TreasureSelection(flow) => flow.options.clone(),
            _ => return,
        };
        for upgrade in options {
            self.discover_treasure(upgrade);
        }
    }

    pub(crate) fn merge_loaded_discoveries(&mut self, loaded: DiscoveryState) {
        let runtime = self.discovery.clone();
        let mut merged = loaded;
        let runtime_inserted = merged.merge(&runtime);
        merged.version = VERSION;
        merged.loaded = true;
        merged.dirty = runtime_inserted;
        merged.revision = merged.revision.wrapping_add(1);
        self.discovery = merged;
    }

    pub(crate) fn preserve_discoveries_from(&mut self, previous: &DiscoveryState) {
        let runtime = self.discovery.clone();
        let mut merged = previous.clone();
        let runtime_inserted = merged.merge(&runtime);
        merged.loaded = previous.loaded;
        merged.dirty = previous.dirty || (previous.loaded && runtime_inserted);
        self.discovery = merged;
    }

    pub(crate) fn persist_discoveries_if_dirty(&mut self) {
        if let Some(snapshot) = self.discovery.take_dirty_snapshot(self.is_headless()) {
            request_save(snapshot);
        }
    }

    fn discover(&mut self, discovery: Discovery) {
        if self.is_headless() || !self.discovery.insert(discovery) {
            return;
        }
        if self.discovery.loaded {
            self.discovery.dirty = true;
        }
    }
}

impl PartialEq for DiscoveryState {
    fn eq(&self, other: &Self) -> bool {
        self.revision == other.revision
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovery_state_deduplicates_events() {
        let mut state = DiscoveryState::default();
        assert!(state.insert(Discovery::Item("lump_sugar".to_string())));
        assert!(!state.insert(Discovery::Item("lump_sugar".to_string())));
        assert_eq!(state.items, vec!["lump_sugar"]);
    }

    #[test]
    fn discovery_state_equality_uses_revision() {
        let mut state = DiscoveryState::default();
        let before = state.clone();
        assert_eq!(state, before);
        state.insert(Discovery::Item("lump_sugar".to_string()));
        assert_ne!(state, before);
    }

    #[test]
    fn loading_merges_runtime_discoveries_and_marks_new_entries_dirty() {
        let mut game_state = crate::game_state::create_initial_game_state();
        game_state.merge_loaded_discoveries(DiscoveryState::default());
        assert!(game_state.discovery.loaded);
        assert!(game_state.discovery.dirty);
        assert!(!game_state.discovery.items.is_empty());
    }

    #[test]
    fn headless_game_state_does_not_collect_discoveries() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let before = game_state.discovery.clone();
        game_state.headless = true;
        game_state.discover(Discovery::Item("lump_sugar".to_string()));
        assert_eq!(game_state.discovery, before);
    }
}
