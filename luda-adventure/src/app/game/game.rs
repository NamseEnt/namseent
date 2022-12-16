use super::{known_id::object::PLAYER_CHARACTER, *};
use namui::prelude::*;

pub struct Game {
    pub state: GameState,
    pub ecs_app: crate::ecs::App,
    pub map_loader: MapLoader,
}
impl Game {
    pub fn new_with_mock() -> Self {
        let ecs_app = crate::ecs::App::new();
        let mut state = GameState::new();
        state.camera.subject = CameraSubject::Object {
            id: PLAYER_CHARACTER,
        };

        Self {
            state,
            ecs_app,
            map_loader: MapLoader::new(),
        }
    }
    pub fn new() -> Self {
        Self {
            state: GameState::new(),
            ecs_app: crate::ecs::App::new(),
            map_loader: MapLoader::new(),
        }
    }

    pub fn update(&mut self, event: &namui::Event) {
        self.state.tick.current_time = now();
        self.handle_interaction(event);
        self.set_character_movement_according_to_user_input(event);
        self.evaluate_ticks();
        self.map_loader.update(event, &mut self.ecs_app);
    }

    pub fn render(&self) -> namui::RenderingTree {
        let rendering_context = self.create_rendering_context();

        render([
            render_background(),
            self.translate_to_camera_screen(
                &rendering_context,
                render([
                    self.render_in_screen_object_list(&self.state, &rendering_context),
                    self.render_quest_guide(&rendering_context),
                    self.render_interaction_guide(&self.state, &rendering_context),
                ]),
            ),
        ])
    }
}
