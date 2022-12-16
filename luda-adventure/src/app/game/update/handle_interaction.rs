use crate::app::game::{interaction, Game};
use interaction::nearest_entity;
use namui::{Code, NamuiEvent};

impl Game {
    pub fn handle_interaction(&mut self, event: &namui::Event) {
        event.is::<NamuiEvent>(|event| match event {
            NamuiEvent::KeyDown(event) => {
                if event.code != Code::KeyZ {
                    return;
                }
                let interactive_object_list = self.get_interactive_object_with_distance();
                let Some((entity_id, kind)) = nearest_entity(&interactive_object_list) else {
                    return;
                };
                namui::event::send(interaction::Event::Interacted { entity_id, kind });
            }
            _ => (),
        });
    }
}
