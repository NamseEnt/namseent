use super::InteractionKind;
use namui::Uuid;

#[derive(Debug, Clone)]
pub enum Event {
    Interacted {
        entity_id: Uuid,
        kind: InteractionKind,
    },
}
