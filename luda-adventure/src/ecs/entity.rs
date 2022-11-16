use super::*;
use ecs_macro::define_component_query_combinations;
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
    pub fn get_component<'entity, T: ComponentQueryCombination<'entity>>(
        &'entity self,
    ) -> Option<T::Output> {
        T::filter(self)
    }
    pub fn get_component_mut<'entity, T: ComponentQueryCombinationMut<'entity>>(
        &'entity mut self,
    ) -> Option<T::Output> {
        T::filter(self)
    }
}

impl<'entity, T0: ComponentQueryArgument<'entity>> ComponentQueryCombination<'entity> for T0 {
    type Output = T0::Output;
    fn filter(entity: &'entity Entity) -> Option<Self::Output> {
        let mut filtered0 = None;

        for component in entity.components.iter() {
            if T0::filter(component, &filtered0) {
                filtered0 = Some(component);
            }
        }

        Some(T0::output(filtered0)?)
    }
}
impl<'entity, T0: ComponentQueryArgumentMut<'entity>> ComponentQueryCombinationMut<'entity> for T0 {
    type Output = T0::Output;
    fn filter(entity: &'entity mut Entity) -> Option<Self::Output> {
        let mut filtered0 = None;

        for component in entity.components.iter_mut() {
            if T0::filter(component, &filtered0) {
                filtered0 = Some(component);
            }
        }

        Some(T0::output(filtered0)?)
    }
}

define_component_query_combinations!();

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
