use namui::Uuid;

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Interacted { entity_id: Uuid },
}
