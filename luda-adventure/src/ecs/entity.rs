use super::*;
use std::sync::atomic::AtomicUsize;

pub struct Entity {
    id: usize,
    drop_functions: Vec<Box<dyn FnOnce()>>,
}

static mut ID: AtomicUsize = AtomicUsize::new(0);
impl Entity {
    pub fn new() -> Self {
        Self {
            id: unsafe { ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed) },
            drop_functions: Vec::new(),
        }
    }
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn add_component<T: Component>(mut self, component: T) -> Self {
        let id = self.id;
        component.insert(id);
        self.drop_functions.push(Box::new(move || T::drop(id)));
        self
    }
    pub fn get_component<T: ComponentCombination>(&self) -> Option<T> {
        T::filter(&self)
    }
    pub fn get_component_mut<T: ComponentCombinationMut>(&mut self) -> Option<T> {
        T::filter(self)
    }
}
impl Drop for Entity {
    fn drop(&mut self) {
        for drop_function in self.drop_functions.drain(..) {
            drop_function();
        }
    }
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Entity {}
