use super::*;

pub struct Atom<T: Debug + Send + Sync + 'static> {
    value_index: Mutex<Option<(T, usize)>>,
}

impl<T: Debug + Send + Sync + 'static> Atom<T> {
    pub const fn uninitialized_new() -> Self {
        Self {
            value_index: Mutex::new(None),
        }
    }
    pub const fn new(value: T) -> Self {
        Self {
            value_index: Mutex::new(Some((value, 0))),
        }
    }
    fn sig_id(&self) -> SigId {
        let value_index = self.value_index.lock().unwrap();
        let (_, index) = value_index.as_ref().unwrap();
        SigId {
            id_type: SigIdType::Atom,
            index: *index,
            component_id: 0,
        }
    }
    pub fn set(&self, value: T) {
        channel::send(channel::Item::SetStateItem(SetStateItem::Set {
            sig_id: self.sig_id(),
            value: Arc::new(Mutex::new(Some(Box::new(value)))),
        }));
    }
    pub fn mutate(&self, mutate: impl FnOnce(&mut T) + Send + Sync + 'static) {
        channel::send(channel::Item::SetStateItem(SetStateItem::Mutate {
            sig_id: self.sig_id(),
            mutate: Arc::new(Mutex::new(Some(Box::new(move |value| {
                let value = value.as_any_mut().downcast_mut::<T>().unwrap();
                mutate(value);
            })))),
        }));
    }
    pub fn get(&self) -> &T {
        let value_index = self.value_index.lock().unwrap();
        let (atom_value, _) = value_index.as_ref().unwrap();
        let value: &T = unsafe { std::mem::transmute(atom_value) };
        value
    }
}

static ATOM_INDEX: AtomicUsize = AtomicUsize::new(0);

pub fn use_atom_init<'a, T: Any + Send + Sync + Debug>(
    atom: &'static Atom<T>,
    init: impl FnOnce() -> T,
) -> (Sig<'a, T>, SetState<T>) {
    let mut value_index = atom.value_index.lock().unwrap();
    let (atom_value, atom_index) = value_index.get_or_insert_with(|| {
        let value = init();
        let index = ATOM_INDEX.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        (value, index)
    });

    if let ContextFor::SetState { set_state_item, .. } = &ctx().context_for {
        let sig_id = set_state_item.sig_id();

        if sig_id.id_type == SigIdType::Atom && sig_id.index == *atom_index {
            match set_state_item {
                SetStateItem::Set { sig_id: _, value } => {
                    let next_value: Box<T> = value
                        .lock()
                        .unwrap()
                        .take()
                        .unwrap()
                        .as_box()
                        .downcast()
                        .unwrap();
                    let next_value: T = *next_value;
                    *atom_value = next_value;
                }
                SetStateItem::Mutate { sig_id: _, mutate } => {
                    let mutate = mutate.lock().unwrap().take().unwrap();
                    mutate(atom_value.as_value_mut());
                }
            }
        }
    }

    let value: &T = unsafe { std::mem::transmute(atom_value) };

    let sig_id = SigId {
        id_type: SigIdType::Atom,
        index: *atom_index,
        component_id: 0,
    };

    let set_state = SetState::new(sig_id);

    let sig = Sig::new(value, sig_id);

    (sig, set_state)
}
