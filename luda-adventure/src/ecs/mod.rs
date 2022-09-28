mod entity;

pub use entity::*;
use namui::Uuid;

pub trait Component {
    fn insert(self, id: Uuid);
    fn drop(id: Uuid);
}

pub trait ComponentCombination {
    fn filter(entity: &Entity) -> Option<Self>
    where
        Self: Sized;
}

pub trait ComponentCombinationMut {
    fn filter(entity: &mut Entity) -> Option<Self>
    where
        Self: Sized;
}
pub struct App {
    systems: Vec<Box<dyn Fn(&Vec<Entity>)>>,
    entities: Vec<Entity>,
}

impl App {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
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
    pub fn add_system<'a, T, F>(&'a mut self, system_func: F)
    where
        F: Fn(Vec<T>) + 'static,
        T: ComponentCombination,
    {
        let wrapped_system_func = Box::new(move |entities: &Vec<Entity>| {
            let components = get_components::<T>(entities);
            system_func(components);
        });
        self.systems.push(wrapped_system_func);
    }
    pub fn run(&self, entities: &Vec<Entity>) {
        for system in &self.systems {
            system(entities);
        }
    }
    pub fn query_entities<T: ComponentCombination>(&self) -> Vec<(&Entity, T)> {
        let mut query = Vec::new();
        for entity in &self.entities {
            if let Some(component) = T::filter(entity) {
                query.push((entity, component));
            }
        }
        query
    }
    pub fn query_entities_mut<T: ComponentCombinationMut>(&mut self) -> Vec<(&mut Entity, T)> {
        let mut query = Vec::new();
        for entity in &mut self.entities {
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

pub fn get_components<T: ComponentCombination>(entities: &Vec<Entity>) -> Vec<T> {
    let mut components = Vec::new();
    for entity in entities {
        if let Some(component) = T::filter(entity) {
            components.push(component);
        }
    }
    components
}

pub fn get_component<T: ComponentCombination>(entity: &Entity) -> Option<T> {
    T::filter(entity)
}

ecs_macro::define_combinations!();
