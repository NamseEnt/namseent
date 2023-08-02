use super::*;

pub(crate) fn handle_use_state<'a, State: Send + Sync + Debug + 'static>(
    ctx: &'a RenderCtx,
    init: impl FnOnce() -> State,
) -> (Sig<'a, State>, SetState<State>) {
    let instance = ctx.instance.as_ref();
    let mut state_list = instance.state_list.lock().unwrap();

    let state_index = ctx
        .state_index
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let sig_id = SigId {
        id_type: SigIdType::State,
        index: state_index,
        component_id: instance.component_id,
    };

    let no_state = || state_list.len() <= state_index;

    if no_state() {
        let state = init();

        update_or_push(&mut state_list, state_index, Box::new(state));
    }

    let state: &State = state_list[state_index]
        .as_ref()
        .as_any()
        .downcast_ref()
        .unwrap();

    let state: &State = unsafe { std::mem::transmute(state) };

    let set_state = SetState::new(sig_id);

    let sig = Sig::new(state, sig_id);

    (sig, set_state)
}

#[derive(Clone)]
pub(crate) enum SetStateItem {
    Set {
        sig_id: SigId,
        value: Arc<Mutex<Option<Box<dyn Value>>>>,
    },
    Mutate {
        sig_id: SigId,
        mutate: Arc<Mutex<Option<Box<dyn FnOnce(&mut (dyn Value)) + Send + Sync>>>>,
    },
}

impl Debug for SetStateItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetStateItem::Set { sig_id, value } => {
                write!(
                    f,
                    "SetStateItem::Set {{ sig_id: {:?}, value: {:?} }}",
                    sig_id, value,
                )
            }
            SetStateItem::Mutate { sig_id, mutate: _ } => {
                write!(f, "SetStateItem::Mutate {{ sig_id: {:?} }}", sig_id,)
            }
        }
    }
}

pub struct SetState<State: 'static + Debug + Send + Sync> {
    sig_id: SigId,
    _state: std::marker::PhantomData<State>,
}

impl<State: 'static + Debug + Send + Sync> Debug for SetState<State> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SetState {{ sig_id: {:?} }}", self.sig_id,)
    }
}

impl<State: 'static + Debug + Send + Sync> Clone for SetState<State> {
    fn clone(&self) -> Self {
        Self {
            sig_id: self.sig_id,
            _state: std::marker::PhantomData,
        }
    }
}

impl<State: 'static + Debug + Send + Sync> Copy for SetState<State> {}

impl<State: 'static + Debug + Send + Sync> SetState<State> {
    pub(crate) fn new(sig_id: SigId) -> Self {
        Self {
            sig_id,
            _state: std::marker::PhantomData,
        }
    }
    pub fn set(self, state: State) {
        channel::send(channel::Item::SetStateItem(SetStateItem::Set {
            sig_id: self.sig_id,
            value: Arc::new(Mutex::new(Some(Box::new(state)))),
        }));
    }
    pub fn mutate(self, mutate: impl FnOnce(&mut State) + Send + Sync + 'static) {
        channel::send(channel::Item::SetStateItem(SetStateItem::Mutate {
            sig_id: self.sig_id,
            mutate: Arc::new(Mutex::new(Some(Box::new(move |state| {
                let state = state.as_any_mut().downcast_mut::<State>().unwrap();
                mutate(state);
            })))),
        }));
    }
}
