use crate::app::game::InteractionKind;

#[ecs_macro::component]
pub struct Interactor {
    pub kind: InteractionKind,
}
