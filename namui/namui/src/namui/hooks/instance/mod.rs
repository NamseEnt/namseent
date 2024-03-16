#[cfg(target_family = "wasm")]
mod inspect;

use super::*;
use crate::*;
use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{atomic::AtomicBool, Mutex},
};

pub struct ComponentInstance {
    pub(crate) component_id: usize,
    pub(crate) component_type_name: &'static str,
    pub(crate) state_list: Mutex<Vec<Box<dyn Value>>>,
    pub(crate) effect_used_sigs_list: Mutex<Vec<Vec<SigId>>>,
    pub(crate) effect_clean_up_list: Mutex<Vec<CleanUpFnOnce>>,
    pub(crate) memo_value_list: Mutex<Vec<Box<dyn Value>>>,
    pub(crate) memo_used_sigs_list: Mutex<Vec<Vec<SigId>>>,
    pub(crate) render_used_sigs: Mutex<Vec<SigId>>,
    pub(crate) track_eq_value_list: Mutex<Vec<Box<dyn Value>>>,
    pub(crate) is_first_render: AtomicBool,
    is_rendered_on_this_tick: AtomicBool,
    children_instances: Mutex<HashMap<(KeyVec, &'static str), Arc<ComponentInstance>>>,
    pub(crate) debug_bounding_box: Mutex<Option<Rect<Px>>>,
}

unsafe impl Send for ComponentInstance {}
unsafe impl Sync for ComponentInstance {}

impl Debug for ComponentInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentInstance")
            .field("component_id", &self.component_id)
            .field("component_type_name", &self.component_type_name)
            .field("state_list", &self.state_list.lock())
            .field(
                "effect_used_sigs_list",
                &self.effect_used_sigs_list.lock().unwrap(),
            )
            .field(
                "effect_clean_up_list",
                &self
                    .effect_clean_up_list
                    .lock()
                    .unwrap()
                    .iter()
                    .map(|clean_up| clean_up.is_some())
                    .collect::<Vec<_>>(),
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
            .field("is_rendered_on_this_tick", &self.is_rendered_on_this_tick)
            .field("children_instances", &self.children_instances.lock())
            .field("debug_bounding_box", &self.debug_bounding_box.lock())
            .finish()
    }
}

impl Drop for ComponentInstance {
    fn drop(&mut self) {
        let mut effect_clean_up_list = self.effect_clean_up_list.lock().unwrap();
        for clean_up in effect_clean_up_list.iter_mut() {
            if let Some(clean_up) = std::mem::take(clean_up) {
                clean_up();
            }
        }
    }
}

impl ComponentInstance {
    pub(crate) fn new(component_type_name: &'static str) -> Self {
        Self {
            component_id: new_component_id(),
            component_type_name,
            state_list: Default::default(),
            effect_used_sigs_list: Default::default(),
            effect_clean_up_list: Default::default(),
            memo_value_list: Default::default(),
            memo_used_sigs_list: Default::default(),
            render_used_sigs: Default::default(),
            track_eq_value_list: Default::default(),
            is_first_render: AtomicBool::new(true),
            is_rendered_on_this_tick: Default::default(),
            children_instances: Default::default(),
            debug_bounding_box: Default::default(),
        }
    }
    pub(crate) fn get_or_create_child_instance(
        &self,
        key_vec: KeyVec,
        component_type_name: &'static str,
    ) -> Arc<ComponentInstance> {
        let key = (key_vec, component_type_name);

        self.children_instances
            .lock()
            .unwrap()
            .entry(key)
            .or_insert_with(|| Arc::new(ComponentInstance::new(component_type_name)))
            .clone()
    }

    pub(crate) fn before_render(&self) {
        self.is_rendered_on_this_tick
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }

    pub(crate) fn after_render(&self) {
        self.is_first_render
            .store(false, std::sync::atomic::Ordering::SeqCst);
    }

    pub(crate) fn clear_unrendered_children(&self) {
        let mut children = self.children_instances.lock().unwrap();
        children.retain(|_, child| {
            child
                .is_rendered_on_this_tick
                .swap(false, std::sync::atomic::Ordering::SeqCst)
        });
        children
            .values()
            .for_each(|child| child.clear_unrendered_children());
    }
}

fn new_component_id() -> usize {
    static COMPONENT_ID: AtomicUsize = AtomicUsize::new(0);
    COMPONENT_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}
