use super::*;

#[derive(Clone)]
pub(crate) enum ContextFor {
    Mount,
    Event {
        event_callback: EventCallback,
    },
    SetState {
        updated_signals: Arc<Mutex<HashSet<SignalId>>>,
    },
}

impl Debug for ContextFor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextFor::Mount => write!(f, "ContextFor::Mount"),
            ContextFor::Event { event_callback } => write!(
                f,
                "ContextFor::Event {{ event_callback: {:?} }}",
                event_callback
            ),
            ContextFor::SetState { updated_signals } => write!(
                f,
                "ContextFor::SetState {{ updated_signals: {:?} }}",
                updated_signals.lock().unwrap()
            ),
        }
    }
}

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
