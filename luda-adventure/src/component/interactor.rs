use crate::app::game::interaction::InteractionKind;

#[ecs_macro::component]
pub struct Interactor {
    pub kind: InteractionKind,
}
