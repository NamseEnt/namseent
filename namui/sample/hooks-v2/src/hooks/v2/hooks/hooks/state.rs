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

    let no_state = || state_list.len() <= state_index;

    if no_state() {
        let state = init();

        update_or_push(&mut state_list, state_index, Box::new(state));
    }
    // else if let ContextFor::SetState { set_state_item, .. } = &ctx.context_for {
    //     let sig_id = set_state_item.sig_id();

    //     if sig_id.component_id == instance.component_id
    //         && sig_id.id_type == SigIdType::State
    //         && sig_id.index == state_index
    //     {
    //         match set_state_item {
    //             SetStateItem::Set { sig_id: _, value } => {
    //                 let mut_state = state_list.get_mut(state_index).unwrap();
    //                 let next_value = value.lock().unwrap().take().unwrap();
    //                 *mut_state = next_value;
    //             }
    //             SetStateItem::Mutate { sig_id: _, mutate } => {
    //                 let state = state_list.get_mut(state_index).unwrap();
    //                 let mutate = mutate.lock().unwrap().take().unwrap();
    //                 mutate(state.as_mut());
    //             }
    //         }
    //     }
    // }

    let state: &State = state_list[state_index]
        .as_ref()
        .as_any()
        .downcast_ref()
        .unwrap();

    let state: &State = unsafe { std::mem::transmute(state) };

    let sig_id = SigId {
        id_type: SigIdType::State,
        index: state_index,
        component_id: instance.component_id,
    };

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

impl SetStateItem {
    pub fn sig_id(&self) -> SigId {
        match self {
            SetStateItem::Set { sig_id, .. } => *sig_id,
            SetStateItem::Mutate { sig_id, .. } => *sig_id,
        }
    }
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

pub struct SetState<State: 'static + Debug> {
    sig_id: SigId,
    _state: std::marker::PhantomData<State>,
}

impl<State: 'static + Debug> SetState<State> {
    pub(crate) fn new(sig_id: SigId) -> Self {
        Self {
            sig_id,
            _state: std::marker::PhantomData,
        }
    }
    pub fn set(self, state: State) {
        todo!()
        // channel::send(channel::Item::SetStateItem(SetStateItem::Set {
        //     sig_id: self.sig_id,
        //     value: Arc::new(Mutex::new(Some(Box::new(state)))),
        // }));
    }
    pub fn mutate(self, mutate: impl FnOnce(&mut State) + Send + Sync + 'static) {
        todo!()
        // channel::send(channel::Item::SetStateItem(SetStateItem::Mutate {
        //     sig_id: self.sig_id,
        //     mutate: Arc::new(Mutex::new(Some(Box::new(move |state| {
        //         let state = state.as_any_mut().downcast_mut::<State>().unwrap();
        //         mutate(state);
        //     })))),
        // }));
    }
}
