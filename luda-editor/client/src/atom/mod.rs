use std::sync::{Arc, Mutex, MutexGuard};

#[allow(dead_code)]
pub struct Atom<Atom: Atomic> {
    atom: Mutex<Atom>,
}

#[allow(dead_code)]
impl<TAtom: Atomic> Atom<TAtom> {
    pub fn get(&self) -> MutexGuard<TAtom> {
        self.atom.lock().unwrap()
    }

    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut TAtom),
    {
        let mut atom = self.atom.lock().unwrap();
        f(&mut atom);
        atom.on_update();
        namui::event::send(namui::NamuiEvent::NoUpdateJustRender);
    }

    pub(crate) fn new(atom: TAtom) -> Self {
        Self {
            atom: Mutex::new(atom),
        }
    }
}

pub struct OptionAtom<Atom: Atomic> {
    inner: Mutex<Option<Arc<Atom>>>,
}

impl<TAtom: Atomic> OptionAtom<TAtom> {
    pub fn get_unwrap(&self) -> Arc<TAtom> {
        let inner = self.inner.lock().unwrap();
        inner.as_ref().unwrap().clone()
    }

    pub fn set(&self, atom: TAtom) {
        let mut inner = self.inner.lock().unwrap();
        *inner = Some(Arc::new(atom));
        namui::event::send(namui::NamuiEvent::NoUpdateJustRender);
    }

    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut TAtom),
    {
        let mut inner = self.inner.lock().unwrap();
        let Some(atom) = inner.as_mut() else {
            return;
        };
        let atom = Arc::get_mut(atom).unwrap();
        f(&mut *atom);
        atom.on_update();
        namui::event::send(namui::NamuiEvent::NoUpdateJustRender);
    }

    pub(crate) const fn new() -> Self {
        Self {
            inner: Mutex::new(None),
        }
    }
}

pub trait Atomic {
    fn on_update(&self);
}

impl<T: Atomic> Atomic for Option<T> {
    fn on_update(&self) {
        if let Some(value) = self {
            value.on_update();
        }
    }
}
