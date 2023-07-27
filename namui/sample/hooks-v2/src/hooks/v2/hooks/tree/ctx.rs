use super::*;
use std::collections::VecDeque;

pub(crate) struct Ctx {
    pub(crate) context_for: ContextFor,
    pub(crate) instance: Arc<ComponentInstance>,
    pub(crate) state_index: AtomicUsize,
    pub(crate) effect_index: AtomicUsize,
    pub(crate) memo_index: AtomicUsize,
    pub(crate) as_index: AtomicUsize,
}
unsafe impl Send for Ctx {}
unsafe impl Sync for Ctx {}

impl Ctx {
    pub(crate) fn new(context_for: ContextFor, instance: Arc<ComponentInstance>) -> Self {
        Self {
            context_for,
            instance,
            state_index: Default::default(),
            effect_index: Default::default(),
            memo_index: Default::default(),
            as_index: Default::default(),
        }
    }
}

pub(crate) enum ContextFor {
    Mount,
    Event {
        event_callback: EventCallback,
        children_tree: Vec<ComponentTree>,
    },
    SetState {
        set_state_item: SetStateItem,
        updated_sigs: Arc<Mutex<HashSet<SigId>>>,
        children_tree: VecDeque<ComponentTree>,
    },
}

impl Debug for ContextFor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextFor::Mount => write!(f, "ContextFor::Mount"),
            ContextFor::Event {
                event_callback,
                children_tree: children,
            } => write!(
                f,
                "ContextFor::Event {{ event_callback: {:?}, children: {:?} }}",
                event_callback, children
            ),
            ContextFor::SetState {
                updated_sigs,
                set_state_item,
                children_tree: children,
            } => write!(
                f,
                "ContextFor::SetState {{ updated_sigs: {:?}, set_state_item: {:?}, children: {:?} }}",
                updated_sigs.lock().unwrap(),
                set_state_item,
                children,
            ),
        }
    }
}
