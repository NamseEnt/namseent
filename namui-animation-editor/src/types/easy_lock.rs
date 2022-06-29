#![allow(dead_code)]

use parking_lot::ReentrantMutex;
use std::sync::Arc;

pub type EasyLock<T> = Arc<ReentrantMutex<T>>;
pub fn new_easy_lock<T>(t: T) -> EasyLock<T> {
    Arc::new(ReentrantMutex::new(t))
}
