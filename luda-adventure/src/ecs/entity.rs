use super::*;
use namui::Uuid;

pub struct Entity {
    id: Uuid,
    app_id: Uuid,
    drop_functions: Vec<Box<dyn FnOnce()>>,
}

impl Entity {
    pub fn new(app_id: Uuid) -> Self {
        Self::with_id(app_id, Uuid::new_v4())
    }
    pub fn with_id(app_id: Uuid, entity_id: Uuid) -> Self {
        Self {
            id: entity_id,
            app_id,
            drop_functions: Vec::new(),
        }
    }
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn add_component<T: Component>(&mut self, component: T) -> &mut Self {
        let id = self.id;
        let app_id = self.app_id;
        component.insert(self.app_id, id);
        self.drop_functions
            .push(Box::new(move || T::drop(app_id, id)));
        self
    }
    pub fn get_component<T: ComponentCombination>(&self) -> Option<T> {
        T::filter(self.app_id, &self)
    }
    pub fn get_component_mut<T: ComponentCombinationMut>(&mut self) -> Option<T> {
        T::filter(self.app_id, self)
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
