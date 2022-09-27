use std::any::Any;

pub struct Entity {
    pub components: Vec<Box<dyn Any>>,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }
    pub fn add_component<T: Any>(mut self, component: T) -> Self {
        self.components.push(Box::new(component));
        self
    }
}

pub trait ComponentCombination<'entity> {
    fn filter(entity: &'entity Entity) -> Option<Self>
    where
        Self: Sized;
}

macro_rules! register_component {
    ($name: ident) => {
        impl<'entity> $crate::ecs::ComponentCombination<'entity> for &'entity $name {
            fn filter(entity: &'entity $crate::ecs::Entity) -> Option<Self> {
                for component in &entity.components {
                    if let Some(component) = component.downcast_ref::<$name>() {
                        return Some(component);
                    }
                }
                None
            }
        }
    };
}

pub(crate) use register_component;

pub struct App<'entity> {
    systems: Vec<Box<dyn Fn(&'entity Vec<Entity>)>>,
    entities: Vec<Entity>,
}

impl<'entity> App<'entity> {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
            entities: Vec::new(),
        }
    }
    pub fn add_entity(mut self, entity: Entity) -> Self {
        self.entities.push(entity);
        self
    }
    pub fn add_system<'a, T, F>(&'a mut self, system_func: F)
    where
        F: Fn(Vec<T>) + 'static,
        T: ComponentCombination<'entity>,
    {
        let wrapped_system_func = Box::new(move |entities: &'entity Vec<Entity>| {
            let components = get_components::<T>(entities);
            system_func(components);
        });
        self.systems.push(wrapped_system_func);
    }
    pub fn run(&self, entities: &'entity Vec<Entity>) {
        for system in &self.systems {
            system(entities);
        }
    }
}

fn get_components<'entity, T: ComponentCombination<'entity>>(
    entities: &'entity Vec<Entity>,
) -> Vec<T> {
    let mut components = Vec::new();
    for entity in entities {
        if let Some(component) = T::filter(entity) {
            components.push(component);
        }
    }
    components
}

impl<'entity, TA: ComponentCombination<'entity>, TB: ComponentCombination<'entity>>
    ComponentCombination<'entity> for (TA, TB)
{
    fn filter(entity: &'entity Entity) -> Option<Self> {
        let a = TA::filter(entity)?;
        let b = TB::filter(entity)?;
        Some((a, b))
    }
}
