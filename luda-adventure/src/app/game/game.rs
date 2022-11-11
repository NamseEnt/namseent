use super::{known_id::object::PLAYER_CHARACTER, player_character::new_player, *};
use namui::prelude::*;

pub struct Game {
    pub state: GameState,
    pub ecs_app: crate::ecs::App,
}
impl Game {
    pub fn new_with_mock() -> Self {
        let character = mock_character();
        let quest_object = mock_quest_object();
        let map_entities = Map::mock().create_entities();
        let mut state = GameState::new();
        state.camera.subject = CameraSubject::Object {
            id: PLAYER_CHARACTER,
        };

        let mut ecs_app = crate::ecs::App::new();
        ecs_app.add_entity(character);
        ecs_app.add_entity(quest_object);
        ecs_app.add_entities(map_entities);

        Self { state, ecs_app }
    }
    pub fn new() -> Self {
        Self {
            state: GameState::new(),
            ecs_app: crate::ecs::App::new(),
        }
    }

    pub fn update(&mut self, event: &dyn std::any::Any) {
        self.state.tick.current_time = now();
        self.set_character_movement_according_to_user_input(event);
        self.evaluate_ticks();
    }

    pub fn render(&self) -> namui::RenderingTree {
        let rendering_context = self.create_rendering_context();

        render([
            render_background(),
            self.translate_to_camera_screen(
                &rendering_context,
                render([
                    self.render_in_screen_object_list(&rendering_context),
                    self.render_guide_icon(&rendering_context),
                ]),
            ),
        ])
    }
}

fn mock_character() -> crate::ecs::Entity {
    new_player(Xy {
        x: 8.tile(),
        y: 6.tile(),
    })
}

fn mock_quest_object() -> crate::ecs::Entity {
    new_wall_with_id(
        known_id::object::FIRST_QUEST_OBJECT,
        vec![Xy {
            x: 10.tile(),
            y: 10.tile(),
        }],
    )
}
