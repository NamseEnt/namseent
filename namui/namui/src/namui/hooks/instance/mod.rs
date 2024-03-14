#[cfg(target_family = "wasm")]
mod inspect;

use super::*;
use crate::*;
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct ComponentInstance {
    pub(crate) component_instance_id: usize,
    pub(crate) state_list: Vec<Box<dyn Value>>,
    pub(crate) mut_state_list: Vec<Box<dyn Value>>,
    pub(crate) effect_used_sigs_list: Vec<Vec<SigId>>,
    #[derivative(Debug = "ignore")]
    pub(crate) effect_clean_up_list: Vec<CleanUpFnOnce>,
    pub(crate) interval_last_call_at_list: Vec<Instant>,
    pub(crate) memo_value_list: Vec<Box<dyn Value>>,
    pub(crate) memo_used_sigs_list: Vec<Vec<SigId>>,
    pub(crate) render_used_sigs: Vec<SigId>,
    pub(crate) track_eq_value_list: Vec<Box<dyn Value>>,
    pub(crate) is_first_render: bool,
}

impl Drop for ComponentInstance {
    fn drop(&mut self) {
        self.effect_clean_up_list.iter_mut().for_each(|clean_up| {
            if let Some(clean_up) = clean_up.take() {
                clean_up();
            }
        });
    }
}

impl ComponentInstance {
    pub(crate) fn new() -> Self {
        let component_instance_id = {
            static COMPONENT_INSTANCE_ID: AtomicUsize = AtomicUsize::new(0);
            COMPONENT_INSTANCE_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        };
        Self {
            component_instance_id,
            state_list: Default::default(),
            mut_state_list: Default::default(),
            effect_used_sigs_list: Default::default(),
            effect_clean_up_list: Default::default(),
            interval_last_call_at_list: Default::default(),
            memo_value_list: Default::default(),
            memo_used_sigs_list: Default::default(),
            render_used_sigs: Default::default(),
            track_eq_value_list: Default::default(),
            is_first_render: true,
        }
    }

<<<<<<< HEAD
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
=======
    pub(crate) fn after_render(&mut self) {
        self.is_first_render = false;
>>>>>>> 091808c9 (Init)
    }
}
