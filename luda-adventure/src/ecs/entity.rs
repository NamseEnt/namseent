use super::*;
use namui::Uuid;
use std::cell::{Ref, RefCell, RefMut};

pub struct Entity {
    id: Uuid,
    components: Vec<Box<dyn WrappedComponent>>,
}

impl Entity {
    pub fn new() -> Self {
        Self::with_id(Uuid::new_v4())
    }
    pub fn with_id(id: Uuid) -> Self {
        Self {
            id,
            components: Vec::new(),
        }
    }
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn add_component<T: Component + 'static>(mut self, component: T) -> Self {
        self.components.push(Box::new(RefCell::new(component)));
        self
    }
    pub fn get_component<T: Component + 'static>(&self) -> Option<Ref<T>> {
        self.components
            .iter()
            .find_map(|component| component.as_any().downcast_ref::<RefCell<T>>())
            .map(|component| component.borrow())
    }
    pub fn get_component_mut<T: Component + 'static>(&self) -> Option<RefMut<T>> {
        self.components
            .iter()
            .find_map(|component| component.as_any().downcast_ref::<RefCell<T>>())
            .map(|component| component.borrow_mut())
    }
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Entity {}

impl std::fmt::Debug for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Entity")
            .field("id", &self.id)
            .field("components", &self.components)
            .finish()
    }
}
