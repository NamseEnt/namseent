use super::{known_id::object::PLAYER_CHARACTER, player_character::new_player, *};
use crate::component::Interactor;
use namui::prelude::*;

pub struct Game {
    pub state: GameState,
    pub ecs_app: crate::ecs::App,
    pub map_loader: MapLoader,
}
impl Game {
    pub fn new_with_mock() -> Self {
        let mut ecs_app = crate::ecs::App::new();

        mock_character(&mut ecs_app);
        mock_quest_object_1(&mut ecs_app);
        mock_quest_object_2(&mut ecs_app);
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

fn mock_character(app: &mut crate::ecs::App) -> &mut crate::ecs::Entity {
    new_player(
        app,
        Xy {
            x: 8.tile(),
            y: 6.tile(),
        },
    )
}

fn mock_quest_object_1(app: &mut crate::ecs::App) -> &mut crate::ecs::Entity {
    new_wall_with_id(
        app,
        known_id::object::FIRST_QUEST_OBJECT,
        vec![Xy {
            x: 10.tile(),
            y: 10.tile(),
        }],
    )
    .add_component(Interactor {
        kind: InteractionKind::Quest,
    })
}

fn mock_quest_object_2(app: &mut crate::ecs::App) -> &mut crate::ecs::Entity {
    new_wall_with_id(
        app,
        known_id::object::SECOND_QUEST_OBJECT,
        vec![Xy {
            x: 6.tile(),
            y: 10.tile(),
        }],
    )
    .add_component(Interactor {
        kind: InteractionKind::Quest,
    })
}
