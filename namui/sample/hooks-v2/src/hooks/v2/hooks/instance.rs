use super::*;
use std::{
    any::TypeId,
    fmt::Debug,
    sync::{atomic::AtomicBool, Mutex},
};

pub(crate) struct ComponentInstance {
    pub(crate) component_id: usize,
    pub(crate) component_type_id: TypeId,
    pub(crate) component_type_name: &'static str,
    pub(crate) state_list: Mutex<Vec<Box<dyn Value>>>,
    pub(crate) effect_used_sigs_list: Mutex<Vec<Vec<SigId>>>,
    pub(crate) memo_value_list: Mutex<Vec<Box<dyn Value>>>,
    pub(crate) memo_used_sigs_list: Mutex<Vec<Vec<SigId>>>,
    pub(crate) render_used_sigs: Mutex<Vec<SigId>>,
    pub(crate) is_first_render: AtomicBool,
}

impl Debug for ComponentInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentInstance")
            .field("component_id", &self.component_id)
            .field("component_type_id", &self.component_type_id)
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
            .field("is_first_render", &self.is_first_render)
            .finish()
    }
}

impl ComponentInstance {
    pub(crate) fn new(component: &dyn Component) -> Self {
        Self {
            component_id: new_component_id(),
            component_type_id: component.static_type_id(),
            component_type_name: component.static_type_name(),
            state_list: Default::default(),
            effect_used_sigs_list: Default::default(),
            memo_value_list: Default::default(),
            memo_used_sigs_list: Default::default(),
            render_used_sigs: Default::default(),
            is_first_render: AtomicBool::new(true),
        }
    }
}

fn new_component_id() -> usize {
    static COMPONENT_ID: AtomicUsize = AtomicUsize::new(0);
    COMPONENT_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}
