use super::*;

pub(crate) struct Context {
    pub(crate) context_for: ContextFor,
    pub(crate) instance: Arc<ComponentInstance>,
    pub(crate) state_index: AtomicUsize,
    pub(crate) effect_index: AtomicUsize,
    pub(crate) memo_index: AtomicUsize,
    pub(crate) map_index: AtomicUsize,
    pub(crate) as_index: AtomicUsize,
}
unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
    pub(crate) fn new(context_for: ContextFor, instance: Arc<ComponentInstance>) -> Self {
        Self {
            context_for,
            instance,
            state_index: Default::default(),
            effect_index: Default::default(),
            memo_index: Default::default(),
            map_index: Default::default(),
            as_index: Default::default(),
        }
    }
}

pub(crate) enum ContextFor {
    Mount,
    Event {
        event_callback: EventCallback,
        children: Vec<ComponentTree>,
    },
    SetState {
        set_state_item: SetStateItem,
        updated_signals: Arc<Mutex<HashSet<SignalId>>>,
        children: Vec<ComponentTree>,
    },
}

impl Debug for ContextFor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextFor::Mount => write!(f, "ContextFor::Mount"),
            ContextFor::Event {
                event_callback,
                children,
            } => write!(
                f,
                "ContextFor::Event {{ event_callback: {:?}, children: {:?} }}",
                event_callback, children
            ),
            ContextFor::SetState {
                updated_signals,
                set_state_item,
                children,
            } => write!(
                f,
                "ContextFor::SetState {{ updated_signals: {:?}, set_state_item: {:?}, children: {:?} }}",
                updated_signals.lock().unwrap(),
                set_state_item,
                children,
            ),
        }
    }
}
