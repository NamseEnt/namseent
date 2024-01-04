#[cfg(target_family = "wasm")]
mod inspect;

use super::*;
use crate::*;
use std::{collections::HashMap, fmt::Debug, rc::Rc};

#[derive(Debug)]
pub struct ComponentInstance {
    pub(crate) component_id: usize,
    #[allow(dead_code)]
    pub(crate) component_type_name: &'static str,
}

pub(crate) struct RealComponentInstance {
    pub(crate) state_list: Vec<Box<dyn Value>>,
    pub(crate) effect_used_sigs_list: Vec<Vec<SigId>>,
    pub(crate) effect_clean_up_list: Vec<CleanUpFnOnce>,
    pub(crate) memo_value_list: Vec<Box<dyn Value>>,
    pub(crate) memo_used_sigs_list: Vec<Vec<SigId>>,
    pub(crate) track_eq_value_list: Vec<Box<dyn Value>>,
    pub(crate) is_first_render: bool,
    is_rendered_on_this_tick: bool,
    children_instances: HashMap<(KeyVec, &'static str), Rc<ComponentInstance>>,
    pub(crate) debug_bounding_box: Option<Rect<Px>>,
}

static mut COMPONENT_INSTANCES: OnceLock<HashMap<usize, RealComponentInstance>> = OnceLock::new();
pub(crate) fn component_instance(component_id: usize) -> &'static RealComponentInstance {
    unsafe {
        COMPONENT_INSTANCES
            .get()
            .unwrap()
            .get(&component_id)
            .unwrap()
    }
}
pub(crate) fn component_instance_mut(component_id: usize) -> &'static mut RealComponentInstance {
    unsafe {
        COMPONENT_INSTANCES
            .get_mut()
            .unwrap()
            .get_mut(&component_id)
            .unwrap()
    }
}
pub(crate) fn init_component_instances() {
    unsafe {
        COMPONENT_INSTANCES.get_or_init(HashMap::new);
    }
}

impl Drop for ComponentInstance {
    fn drop(&mut self) {
        let effect_clean_up_list = &mut self.self_mut().effect_clean_up_list;
        for clean_up in effect_clean_up_list.iter_mut() {
            if let Some(clean_up) = std::mem::take(clean_up) {
                clean_up();
            }
        }

        unsafe {
            COMPONENT_INSTANCES
                .get_mut()
                .unwrap()
                .remove(&self.component_id);
        }
    }
}

impl ComponentInstance {
    pub(crate) fn new(component_type_name: &'static str) -> Self {
        let component_id = new_component_id();
        unsafe {
            COMPONENT_INSTANCES.get_mut().unwrap().insert(
                component_id,
                RealComponentInstance {
                    state_list: Default::default(),
                    effect_used_sigs_list: Default::default(),
                    effect_clean_up_list: Default::default(),
                    memo_value_list: Default::default(),
                    memo_used_sigs_list: Default::default(),
                    track_eq_value_list: Default::default(),
                    is_first_render: true,
                    is_rendered_on_this_tick: false,
                    children_instances: Default::default(),
                    debug_bounding_box: Default::default(),
                },
            );
        }
        Self {
            component_id,
            component_type_name,
        }
    }
    pub(crate) fn self_ref(&self) -> &'static RealComponentInstance {
        component_instance(self.component_id)
    }
    pub(crate) fn self_mut(&self) -> &'static mut RealComponentInstance {
        component_instance_mut(self.component_id)
    }
    pub(crate) fn get_or_create_child_instance(
        &self,
        key_vec: KeyVec,
        component_type_name: &'static str,
    ) -> Rc<ComponentInstance> {
        let key = (key_vec, component_type_name);

        self.self_mut()
            .children_instances
            .entry(key)
            .or_insert_with(|| Rc::new(ComponentInstance::new(component_type_name)))
            .clone()
    }

    pub(crate) fn before_render(&self) {
        self.self_mut().is_rendered_on_this_tick = true;
    }

    pub(crate) fn after_render(&self) {
        self.self_mut().is_first_render = false;
    }

    pub(crate) fn clear_unrendered_chidlren(&self) {
        let children = &mut self.self_mut().children_instances;
        children.retain(|_, child| {
            std::mem::replace(&mut child.self_mut().is_rendered_on_this_tick, false)
        });
        children
            .values()
            .for_each(|child| child.clear_unrendered_chidlren());
    }
    pub(crate) fn set_debug_bounding_box(&self, debug_bounding_box: Option<Rect<Px>>) {
        self.self_mut().debug_bounding_box = debug_bounding_box;
    }
}

fn new_component_id() -> usize {
    static COMPONENT_ID: AtomicUsize = AtomicUsize::new(0);
    COMPONENT_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}
