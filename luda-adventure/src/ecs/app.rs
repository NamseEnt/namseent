use super::{ComponentCombination, ComponentCombinationMut, Entity};

pub struct App {
    entities: Vec<Entity>,
}

impl App {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }
    pub fn entities(&self) -> impl Iterator<Item = &Entity> {
        self.entities.iter()
    }
    pub fn entities_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.entities.iter_mut()
    }
    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
    pub fn add_entities(&mut self, entities: impl IntoIterator<Item = Entity>) {
        self.entities.extend(entities);
    }
    pub fn query_entities<'a, T: ComponentCombination<'a>>(
        &'a self,
    ) -> Vec<(&'a Entity, T::Output)> {
        let mut query = Vec::new();
        for entity in &self.entities {
            if let Some(component) = T::filter(entity) {
                query.push((entity, component));
            }
        }
        query
    }
    pub fn query_entities_mut<'a, T: ComponentCombinationMut<'a>>(
        &'a mut self,
    ) -> Vec<(&'a Entity, T::Output)> {
        let mut query = Vec::new();
        for entity in &self.entities {
            if let Some(component) = T::filter(entity) {
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
