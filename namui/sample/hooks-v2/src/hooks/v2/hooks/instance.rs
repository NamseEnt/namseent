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
    pub(crate) effect_used_signals_list: Mutex<Vec<Vec<SignalId>>>,
    pub(crate) memo_value_list: Mutex<Vec<Arc<dyn Value>>>,
    pub(crate) memo_used_signals_list: Mutex<Vec<Vec<SignalId>>>,
    pub(crate) render_used_signals: Mutex<Vec<SignalId>>,
    pub(crate) map_used_signals_list: Mutex<Vec<Vec<SignalId>>>,
    pub(crate) is_first_render: AtomicBool,
}

impl Debug for ComponentInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            f.debug_struct("ComponentInstance")
                .field("component_id", &self.component_id)
                .field("component_type_id", &self.component_type_id)
                .field("component_type_name", &self.component_type_name)
                .field("state_list", &self.state_list.lock())
                .field(
                    "effect_used_signals_list",
                    &self.effect_used_signals_list.lock().unwrap(),
                )
                .field("memo_value_list", &self.memo_value_list.lock())
                .field(
                    "memo_used_signals_list",
                    &self.memo_used_signals_list.lock().unwrap(),
                )
                .field(
                    "render_used_signals",
                    &self.render_used_signals.lock().unwrap(),
                )
                .field("is_first_render", &self.is_first_render)
                .finish()
        }
    }
}

impl ComponentInstance {
    pub(crate) fn new(
        component_id: usize,
        component_type_id: TypeId,
        component_type_name: &'static str,
    ) -> Self {
        Self {
            component_id,
            component_type_id,
            component_type_name,
            state_list: Default::default(),
            effect_used_signals_list: Default::default(),
            memo_value_list: Default::default(),
            memo_used_signals_list: Default::default(),
            render_used_signals: Default::default(),
            map_used_signals_list: Default::default(),
            is_first_render: AtomicBool::new(true),
        }
    }

    pub(crate) fn push_children_used_signals(&self, used_signals: Vec<SignalId>) {
        self.render_used_signals
            .lock()
            .unwrap()
            .extend(used_signals);
    }

    pub(crate) fn get_all_used_signals(&self) -> Vec<SignalId> {
        let mut used_signals = self.render_used_signals.lock().unwrap().clone();
        used_signals.extend(
            self.effect_used_signals_list
                .lock()
                .unwrap()
                .iter()
                .flatten()
                .copied(),
        );
        used_signals.extend(
            self.memo_used_signals_list
                .lock()
                .unwrap()
                .iter()
                .flatten()
                .copied(),
        );
        used_signals
    }
}
