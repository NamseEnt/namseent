mod entity;

pub use entity::*;

pub trait Component {
    fn insert(self, id: usize);
    fn drop(id: usize);
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
    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
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
}

pub fn get_components<'entity, T: ComponentCombination>(entities: &Vec<Entity>) -> Vec<T> {
    let mut components = Vec::new();
    for entity in entities {
        if let Some(component) = T::filter(entity) {
            components.push(component);
        }
    }
    components
}

pub fn get_component<'entity, T: ComponentCombination>(entity: &Entity) -> Option<T> {
    T::filter(entity)
}

impl<'entity, TA: ComponentCombination, TB: ComponentCombination> ComponentCombination
    for (TA, TB)
{
    fn filter(entity: &Entity) -> Option<Self> {
        let a = TA::filter(entity)?;
        let b = TB::filter(entity)?;
        Some((a, b))
    }
}

impl<'entity, TA: ComponentCombinationMut, TB: ComponentCombinationMut> ComponentCombinationMut
    for (TA, TB)
{
    fn filter(entity: &mut Entity) -> Option<Self> {
        let a = TA::filter(entity)?;
        let b = TB::filter(entity)?;
        Some((a, b))
    }
}
