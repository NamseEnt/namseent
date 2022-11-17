mod entity;

pub use entity::*;
use namui::Uuid;

pub trait Component {
    fn insert(self, app_id: Uuid, entity_id: Uuid);
    fn drop(app_id: Uuid, entity_id: Uuid);
}

pub trait ComponentCombination {
    fn filter(app_id: Uuid, entity: &Entity) -> Option<Self>
    where
        Self: Sized;
}

pub trait ComponentCombinationMut {
    fn filter(app_id: Uuid, entity: &mut Entity) -> Option<Self>
    where
        Self: Sized;
}
pub struct App {
    id: Uuid,
    entities: Vec<Entity>,
}

impl App {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            entities: Vec::new(),
        }
    }
    pub fn new_entity(&mut self) -> &mut Entity {
        self.new_entity_with_id(Uuid::new_v4())
    }
    pub fn new_entity_with_id(&mut self, entity_id: Uuid) -> &mut Entity {
        let entity = Entity::with_id(self.id, entity_id);
        self.entities.push(entity);
        self.entities.last_mut().unwrap()
    }
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn entities(&self) -> impl Iterator<Item = &Entity> {
        self.entities.iter()
    }
    pub fn entities_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.entities.iter_mut()
    }
    // pub fn add_entity(&mut self, entity: Entity) {
    //     self.entities.push(entity);
    // }
    // pub fn add_entities(&mut self, entities: impl IntoIterator<Item = Entity>) {
    //     self.entities.extend(entities);
    // }
    pub fn query_entities<T: ComponentCombination>(&self) -> Vec<(&Entity, T)> {
        let mut query = Vec::new();
        for entity in &self.entities {
            if let Some(component) = T::filter(self.id, entity) {
                query.push((entity, component));
            }
        }
        query
    }
    pub fn query_entities_mut<T: ComponentCombinationMut>(&mut self) -> Vec<(&mut Entity, T)> {
        let mut query = Vec::new();
        for entity in &mut self.entities {
            if let Some(component) = T::filter(self.id, entity) {
                query.push((entity, component));
            }
        }
        query
    }
}

impl std::fmt::Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App")
            .field("entities", &self.entities)
            .finish()
    }
}

ecs_macro::define_combinations!();
