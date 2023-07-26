use super::*;

pub(crate) enum SetStateItem {
    Set {
        signal_id: SignalId,
        value: Arc<dyn Value>,
    },
    Mutate {
        signal_id: SignalId,
        mutate: Box<dyn FnOnce(&mut (dyn Value)) + Send + Sync>,
    },
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
    pub fn set(self, state: State) {
        channel::send(channel::Item::SetStateItem(SetStateItem::Set {
            signal_id: self.signal_id,
            value: Arc::new(state),
        }));
    }
    pub fn mutate(self, mutate: impl FnOnce(&mut State) + Send + Sync + 'static) {
        channel::send(channel::Item::SetStateItem(SetStateItem::Mutate {
            signal_id: self.signal_id,
            mutate: Box::new(move |state| {
                let state = state.as_any_mut().downcast_mut::<State>().unwrap();
                mutate(state);
            }),
        }));
    }
}

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

        update_or_push(&mut state_list, state_index, Arc::new(state));
    }

    let state: Arc<State> = Arc::downcast(state_list[state_index].clone().as_arc()).unwrap();
    let state: &State = unsafe { &*Arc::as_ptr(&state) };

    let signal_id = SignalId {
        id_type: SignalIdType::State,
        index: state_index,
        component_id: instance.component_id,
    };

    let set_state = SetState {
        signal_id,
        _state: std::marker::PhantomData,
    };

    let signal = Signal::new(state, signal_id);

    (signal, set_state)
}
