use super::*;
use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{atomic::AtomicBool, Mutex},
};

pub struct ComponentInstance {
    pub(crate) component_id: usize,
    // pub(crate) component_type_id: StaticTypeId,
    pub(crate) component_type_name: &'static str,
    pub(crate) state_list: Mutex<Vec<Box<dyn Value>>>,
    pub(crate) effect_used_sigs_list: Mutex<Vec<Vec<SigId>>>,
    pub(crate) memo_value_list: Mutex<Vec<Box<dyn Value>>>,
    pub(crate) memo_used_sigs_list: Mutex<Vec<Vec<SigId>>>,
    pub(crate) render_used_sigs: Mutex<Vec<SigId>>,
    pub(crate) track_eq_value_list: Mutex<Vec<Box<dyn Value>>>,
    pub(crate) is_first_render: AtomicBool,
    children_instances: Mutex<HashMap<KeyVec, Arc<ComponentInstance>>>,
}

impl Debug for ComponentInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentInstance")
            .field("component_id", &self.component_id)
            // .field("component_type_id", &self.component_type_id)
            .field("component_type_name", &self.component_type_name)
            .field("state_list", &self.state_list.lock())
            .field(
                "effect_used_sigs_list",
                &self.effect_used_sigs_list.lock().unwrap(),
            )
            .field("memo_value_list", &self.memo_value_list.lock())
            .field(
                "memo_used_sigs_list",
                &self.memo_used_sigs_list.lock().unwrap(),
            )
            .field("render_used_sigs", &self.render_used_sigs.lock().unwrap())
            .field(
                "track_eq_value_list",
                &self.track_eq_value_list.lock().unwrap(),
            )
            .field("is_first_render", &self.is_first_render)
            .finish()
    }
}

impl ComponentInstance {
    pub(crate) fn new(component_type_name: &'static str) -> Self {
        Self {
            component_id: new_component_id(),
            // component_type_id: component.static_type_id(),
            component_type_name,
            state_list: Default::default(),
            effect_used_sigs_list: Default::default(),
            memo_value_list: Default::default(),
            memo_used_sigs_list: Default::default(),
            render_used_sigs: Default::default(),
            track_eq_value_list: Default::default(),
            is_first_render: AtomicBool::new(true),
            children_instances: Default::default(),
        }
    }
    pub(crate) fn get_or_create_child_instance(
        &self,
        key: KeyVec,
        component_type_name: &'static str,
    ) -> Arc<ComponentInstance> {
        // TODO: Remove unused key's children instances

        self.children_instances
            .lock()
            .unwrap()
            .entry(key)
            .or_insert_with(|| Arc::new(ComponentInstance::new(component_type_name)))
            .clone()
    }

    pub(crate) fn after_render(&self) {
        self.is_first_render
            .store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

fn new_component_id() -> usize {
    static COMPONENT_ID: AtomicUsize = AtomicUsize::new(0);
    COMPONENT_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}
