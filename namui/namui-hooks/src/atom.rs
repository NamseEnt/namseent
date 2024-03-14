use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

#[derive(Debug)]
pub struct Atom<T> {
    initialized: AtomicBool,
    index: AtomicUsize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Atom<T> {
    pub const fn uninitialized() -> Self {
        Self {
            initialized: AtomicBool::new(false),
            index: AtomicUsize::new(0),
            _phantom: std::marker::PhantomData,
        }
    }

    pub(crate) fn get_index(&self) -> usize {
        assert!(self.initialized.load(Ordering::Relaxed));
        self.index.load(Ordering::Relaxed)
    }

    pub(crate) fn init_index(&self) -> usize {
        static NEXT_INDEX: AtomicUsize = AtomicUsize::new(0);
        if self.initialized.load(Ordering::Relaxed) {
            return self.index.load(Ordering::Relaxed);
        }

        let next_index = NEXT_INDEX.fetch_add(1, Ordering::Relaxed);
        self.index.store(next_index, Ordering::Relaxed);
        self.initialized.store(true, Ordering::Relaxed);

        next_index
    }
}
