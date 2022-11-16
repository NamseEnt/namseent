use super::*;
use ecs_macro::define_component_combinations;
use namui::Uuid;

pub struct Entity {
    id: Uuid,
    components: Vec<Box<dyn ContainedComponent>>,
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
        self.components
            .push(Box::new(ComponentContainer::new(component)));
        self
    }
    pub fn get_component<'entity, T: ComponentCombination<'entity>>(
        &'entity self,
    ) -> Option<T::Output> {
        T::filter(self)
    }
    pub fn get_component_mut<'entity, T: ComponentCombinationMut<'entity>>(
        &'entity mut self,
    ) -> Option<T::Output> {
        T::filter(self)
    }
}

impl<'entity, T: Component + 'static + Sized> ComponentCombination<'entity> for T {
    type Output = &'entity T;
    fn filter(entity: &'entity Entity) -> Option<Self::Output>
    where
        Self: Sized,
    {
        let mut component1 = None;

        for component in entity.components.iter() {
            if component.as_any().is::<ComponentContainer<T>>() {
                component1 = Some(component);
            }
        }

        Some(
            component1?
                .as_any()
                .downcast_ref::<ComponentContainer<T>>()?
                .as_ref(),
        )
    }
}
impl<'entity, T: Component + 'static> ComponentCombinationMut<'entity> for T {
    type Output = &'entity mut T;
    fn filter(entity: &'entity mut Entity) -> Option<Self::Output>
    where
        Self: Sized,
    {
        let mut component1 = None;

        for component in entity.components.iter_mut() {
            if component.as_any_mut().is::<ComponentContainer<T>>() {
                component1 = Some(component);
            }
        }

        Some(
            component1?
                .as_any_mut()
                .downcast_mut::<ComponentContainer<T>>()?
                .as_ref_mut(),
        )
    }
}

pub trait ComponentCombination<'entity> {
    type Output;
    fn filter(entity: &'entity Entity) -> Option<Self::Output>
    where
        Self: Sized;
}
pub trait ComponentCombinationMut<'entity> {
    type Output;
    fn filter(entity: &'entity mut Entity) -> Option<Self::Output>
    where
        Self: Sized;
}
define_component_combinations!();

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
