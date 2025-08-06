use crate::app::game::{self, Game, interaction};
use interaction::nearest_entity;
use namui::Code;

impl Game {
    pub fn handle_interaction(&mut self, event: &namui::Event) {
        event.is::<game::Event>(|event| match event {
            &game::Event::KeyDown {
                code,
                pressing_codes: _,
            } => {
                if code != Code::KeyZ {
                    return;
                }
                let interactive_object_list = self.get_interactive_object_with_distance();
                let Some((entity_id, kind)) = nearest_entity(&interactive_object_list) else {
                    return;
                };
                namui::event::send(interaction::Event::Interacted { entity_id, kind });
            }
            game::Event::KeyUp { .. } => {}
        });
    }
}
