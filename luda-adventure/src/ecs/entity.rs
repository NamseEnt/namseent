use super::*;
use namui::Uuid;

pub struct Entity {
    id: Uuid,
    drop_functions: Vec<Box<dyn FnOnce()>>,
}

impl Entity {
    pub fn new() -> Self {
        Self::with_id(Uuid::new_v4())
    }
    pub fn with_id(id: Uuid) -> Self {
        Self {
            id,
            drop_functions: Vec::new(),
        }
    }
    pub fn id(&self) -> Uuid {
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

impl std::fmt::Debug for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Add debug component of entity
        f.debug_struct("Entity").field("id", &self.id).finish()
    }
}
