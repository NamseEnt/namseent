use crate::hooks::{component_tree::Key, update::invoke_update};
use std::{
    any::Any,
    collections::{HashMap, HashSet},
    fmt::Debug,
    sync::{atomic::AtomicUsize, Arc, Mutex, OnceLock},
};

pub struct Atom<T> {
    id: OnceLock<usize>,
    value: OnceLock<Arc<Mutex<Arc<T>>>>,
    init: fn() -> T,
}

static mut ATOM_VALUE_MAP: OnceLock<HashMap<usize, Arc<dyn Any>>> = OnceLock::new();
static mut ATOM_STATED_COMPONENT_KEYS: OnceLock<HashMap<usize, HashSet<Key>>> = OnceLock::new();
static COMPONENT_KEY: OnceLock<Key> = OnceLock::new();

pub(crate) fn set_up_atom_before_render(key: Key) {
    let _ = COMPONENT_KEY.set(key);
}

impl<T: 'static + Any + Clone + PartialEq + Debug + Send + Sync> Atom<T> {
    pub const fn new(init: fn() -> T) -> Self {
        Self {
            id: OnceLock::new(),
            value: OnceLock::new(),
            init,
        }
    }
    pub fn state<'a>(&'a self) -> (&'a T, SetAtomState<T>) {
        static ID: AtomicUsize = AtomicUsize::new(0);

        let value = {
            self.value
                .get_or_init(|| Arc::new(Mutex::new(Arc::new((self.init)()))))
                .lock()
                .unwrap()
                .clone()
        };
        let id = *self.id.get_or_init(|| {
            let id = ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

            unsafe {
                ATOM_VALUE_MAP.get_or_init(|| HashMap::new());
                let value = self.value.get().unwrap().clone();
                ATOM_VALUE_MAP.get_mut().unwrap().insert(id, value);
            }

            id
        });

        unsafe {
            ATOM_STATED_COMPONENT_KEYS.get_or_init(|| HashMap::new());
            let component_key = COMPONENT_KEY.get().unwrap();
            ATOM_STATED_COMPONENT_KEYS
                .get_mut()
                .unwrap()
                .entry(id)
                .or_insert_with(HashSet::new)
                .insert(*component_key);
        }

        let set_state = SetAtomState::new(id);

        let value_ptr = Arc::as_ptr(&value);
        let value_ref = unsafe { &*value_ptr };
        (value_ref, set_state)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct SetAtomState<T> {
    _marker: std::marker::PhantomData<T>,
    atom_id: usize,
}

impl<T: Clone> Copy for SetAtomState<T> {}

impl<T: 'static + Any + Clone + PartialEq + Debug> SetAtomState<T> {
    pub fn invoke(&self, next_state_fn: impl FnOnce(&mut T)) {
        self.update_atom_value(next_state_fn);
        self.invoke_updates();
    }

    fn new(atom_id: usize) -> Self {
        Self {
            _marker: std::marker::PhantomData,
            atom_id,
        }
    }
    fn update_atom_value(&self, next_state_fn: impl FnOnce(&mut T)) {
        unsafe {
            let atom_value = ATOM_VALUE_MAP
                .get_mut()
                .unwrap()
                .get_mut(&self.atom_id)
                .unwrap();
            let atom_value = atom_value.downcast_ref::<Mutex<Arc<T>>>().unwrap();
            let mut atom_value = atom_value.lock().unwrap();
            let atom_value = Arc::make_mut(&mut *atom_value);
            next_state_fn(&mut *atom_value);
        }
    }

    fn invoke_updates(&self) {
        let component_keys = unsafe {
            ATOM_STATED_COMPONENT_KEYS
                .get_mut()
                .unwrap()
                .get(&self.atom_id)
                .unwrap()
        };
        let source = Arc::new(());
        for component_key in component_keys {
            invoke_update(*component_key, source.clone())
        }
    }
}
