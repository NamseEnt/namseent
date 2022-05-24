use std::sync::{Arc, RwLock, RwLockReadGuard};

#[derive(Debug, Clone)]
pub struct ReadOnlyLock<T> {
    lock: Arc<RwLock<T>>,
}

impl<T> ReadOnlyLock<T> {
    pub fn read(&self) -> RwLockReadGuard<T> {
        self.lock.read().unwrap()
    }

    pub fn new(lock: Arc<RwLock<T>>) -> ReadOnlyLock<T> {
        ReadOnlyLock { lock }
    }
}
