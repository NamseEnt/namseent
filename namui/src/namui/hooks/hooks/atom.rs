use super::*;
use std::sync::OnceLock;

pub struct Atom<T: Debug + Send + Sync + 'static> {
    _t: std::marker::PhantomData<T>,
    value_index: OnceLock<Arc<Mutex<(Box<dyn Value>, usize)>>>,
}

impl<T: Debug + Send + Sync + 'static> Clone for Atom<T> {
    fn clone(&self) -> Self {
        Self {
            _t: Default::default(),
            value_index: self.value_index.clone(),
        }
    }
}

static ATOMS: OnceLock<Arc<Mutex<Vec<Atom<()>>>>> = OnceLock::new();

impl<T: Debug + Send + Sync + 'static> Atom<T> {
    pub const fn uninitialized_new() -> Self {
        Self {
            _t: std::marker::PhantomData,
            value_index: OnceLock::new(),
        }
    }
    pub fn get_or_init(&self, init: impl FnOnce() -> T) -> &T {
        let value_index = self.value_index.get_or_init(|| {
            let mut atoms = ATOMS.get_or_init(|| Default::default()).lock().unwrap();

            atoms.push(self.as_no_generic());

            let index = atoms.len() - 1;

            Arc::new(Mutex::new((Box::new(init()), index)))
        });
        self.value_to_ref(&value_index.lock().unwrap().0)
    }
    pub fn get(&self) -> &T {
        let value_index = self.value_index.get().unwrap().lock().unwrap();
        self.value_to_ref(&value_index.0)
    }
    pub fn set(&self, value: T) {
        channel::send(channel::Item::SetStateItem(SetStateItem::Set {
            sig_id: self.sig_id(),
            value: Arc::new(Mutex::new(Some(Box::new(value)))),
        }));
    }
    // TODO: NEED THINKING - Should be 'static or 'a for this mutation?
    pub fn mutate(&self, mutate: impl FnOnce(&mut T) + Send + Sync + 'static) {
        channel::send(channel::Item::SetStateItem(SetStateItem::Mutate {
            sig_id: self.sig_id(),
            mutate: Arc::new(Mutex::new(Some(Box::new(move |value| {
                let value = value.as_any_mut().downcast_mut::<T>().unwrap();
                mutate(value);
            })))),
        }));
    }

    fn value_to_ref(&self, value: &Box<dyn Value>) -> &T {
        let value: &T = value.as_ref().as_any().downcast_ref().unwrap();
        let value: &T = unsafe { std::mem::transmute(value) };
        value
    }
    fn sig_id(&self) -> SigId {
        let value_index = self.value_index.get().unwrap().lock().unwrap();
        let index = value_index.1;
        SigId {
            id_type: SigIdType::Atom,
            index,
            component_id: 0,
        }
    }
    fn as_no_generic(&self) -> Atom<()> {
        Atom {
            _t: std::marker::PhantomData,
            value_index: self.value_index.clone(),
        }
    }
}

pub fn use_atom_init<'a, T: Any + Send + Sync + Debug>(
    atom: &'static Atom<T>,
    init: impl FnOnce() -> T,
) -> (Sig<'a, T>, SetState<T>) {
    (
        Sig::new(atom.get_or_init(init), atom.sig_id()),
        SetState::new(atom.sig_id()),
    )
}

pub fn use_atom<'a, T: Any + Send + Sync + Debug>(
    atom: &'static Atom<T>,
) -> (Sig<'a, T>, SetState<T>) {
    (
        Sig::new(atom.get(), atom.sig_id()),
        SetState::new(atom.sig_id()),
    )
}

pub(crate) fn set_atom_value(index: usize, value: Box<dyn Value>) {
    ATOMS.get().unwrap().lock().unwrap()[index]
        .value_index
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .0 = value;
}

pub(crate) fn mutate_atom_value(
    index: usize,
    mutate: Box<dyn FnOnce(&mut dyn Value) + Send + Sync>,
) {
    let atoms = ATOMS.get().unwrap().lock().unwrap();
    let mut value_index = atoms[index].value_index.get().unwrap().lock().unwrap();
    let value = value_index.0.as_mut();
    mutate(value);
}
