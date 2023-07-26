use super::*;

pub fn use_state<'a, State: Send + Sync + Debug + 'static>(
    init: impl FnOnce() -> State,
) -> (Signal<'a, State>, SetState<State>) {
    let ctx = ctx();

    let instance = ctx.instance.as_ref();
    let mut state_list = instance.state_list.lock().unwrap();

    let state_index = ctx
        .state_index
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let no_state = || state_list.len() <= state_index;

    if no_state() {
        let state = init();

        update_or_push(&mut state_list, state_index, Box::new(state));
    } else if let ContextFor::SetState { set_state_item, .. } = &ctx.context_for {
        let signal_id = set_state_item.signal_id();

        if signal_id.component_id == instance.component_id
            && signal_id.id_type == SignalIdType::State
            && signal_id.index == state_index
        {
            match set_state_item {
                SetStateItem::Set {
                    signal_id: _,
                    value,
                } => {
                    let mut_state = state_list.get_mut(state_index).unwrap();
                    let next_value = value.lock().unwrap().take().unwrap();
                    *mut_state = next_value;
                }
                SetStateItem::Mutate {
                    signal_id: _,
                    mutate,
                } => {
                    let state = state_list.get_mut(state_index).unwrap();
                    let mutate = mutate.lock().unwrap().take().unwrap();
                    mutate(state.as_mut());
                }
            }
        }
    }

    let state: &State = state_list[state_index]
        .as_ref()
        .as_any()
        .downcast_ref()
        .unwrap();

    let state: &State = unsafe { std::mem::transmute(state) };

    let signal_id = SignalId {
        id_type: SignalIdType::State,
        index: state_index,
        component_id: instance.component_id,
    };

    let set_state = SetState::new(signal_id);

    let signal = Signal::new(state, signal_id);

    (signal, set_state)
}

#[derive(Clone)]
pub(crate) enum SetStateItem {
    Set {
        signal_id: SignalId,
        value: Arc<Mutex<Option<Box<dyn Value>>>>,
    },
    Mutate {
        signal_id: SignalId,
        mutate: Arc<Mutex<Option<Box<dyn FnOnce(&mut (dyn Value)) + Send + Sync>>>>,
    },
}

impl SetStateItem {
    pub fn signal_id(&self) -> SignalId {
        match self {
            SetStateItem::Set { signal_id, .. } => *signal_id,
            SetStateItem::Mutate { signal_id, .. } => *signal_id,
        }
    }
}

impl Debug for SetStateItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetStateItem::Set { signal_id, value } => {
                write!(
                    f,
                    "SetStateItem::Set {{ signal_id: {:?}, value: {:?} }}",
                    signal_id, value,
                )
            }
            SetStateItem::Mutate { signal_id, mutate } => {
                write!(f, "SetStateItem::Mutate {{ signal_id: {:?} }}", signal_id,)
            }
        }
    }
}

pub struct SetState<State: 'static + Debug + Send + Sync> {
    signal_id: SignalId,
    _state: std::marker::PhantomData<State>,
}

impl<State: 'static + Debug + Send + Sync> SetState<State> {
    pub(crate) fn new(signal_id: SignalId) -> Self {
        Self {
            signal_id,
            _state: std::marker::PhantomData,
        }
    }
    pub fn set(self, state: State) {
        channel::send(channel::Item::SetStateItem(SetStateItem::Set {
            signal_id: self.signal_id,
            value: Arc::new(Mutex::new(Some(Box::new(state)))),
        }));
    }
    pub fn mutate(self, mutate: impl FnOnce(&mut State) + Send + Sync + 'static) {
        channel::send(channel::Item::SetStateItem(SetStateItem::Mutate {
            signal_id: self.signal_id,
            mutate: Arc::new(Mutex::new(Some(Box::new(move |state| {
                let state = state.as_any_mut().downcast_mut::<State>().unwrap();
                mutate(state);
            })))),
        }));
    }
}
