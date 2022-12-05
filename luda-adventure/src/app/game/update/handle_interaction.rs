use crate::app::game::{interaction, Game};
use interaction::nearest_entity_id;
use namui::{Code, NamuiEvent};

impl Game {
    pub fn handle_interaction(&mut self, event: &namui::Event) {
        event.is::<NamuiEvent>(|event| match event {
            NamuiEvent::KeyDown(event) => {
                if event.code != Code::KeyZ {
                    return;
                }
                let interactive_object_list =
                    self.get_interactive_object_with_distance(&self.state);
                let Some(nearest_entity_id) = nearest_entity_id(&interactive_object_list) else {
                    return;
                };
                namui::event::send(interaction::Event::Interacted {
                    entity_id: nearest_entity_id,
                });
            }
            _ => (),
        });
    }
}
