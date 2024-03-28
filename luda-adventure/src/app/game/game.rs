use super::{
    known_id::object::PLAYER_CHARACTER, render::render_background, save_load::SaveLoad, *,
};
use namui::*;
use std::collections::HashSet;

pub struct Game {
    pub state: GameState,
    pub ecs_app: crate::ecs::App,
    pub map_loader: map::MapLoader,
    image_loader: image_loader::ImageLoader,
    menu: menu::Menu,
    save_load: SaveLoad,
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
            map_loader: map::MapLoader::new(),
            image_loader: image_loader::ImageLoader::new(),
            menu: menu::Menu::new(),
            save_load: SaveLoad::new(),
        }
    }
    pub fn new() -> Self {
        Self {
            state: GameState::new(),
            ecs_app: crate::ecs::App::new(),
            map_loader: map::MapLoader::new(),
            image_loader: image_loader::ImageLoader::new(),
            menu: menu::Menu::new(),
            save_load: SaveLoad::new(),
        }
    }

    pub fn update(&mut self, event: &namui::Event) {
        self.state.tick.current_time = now() + self.state.tick.time_offset;
        self.handle_interaction(event);
        self.set_character_movement_according_to_user_input(event);

        event.is::<NamuiEvent>(|event| {
            if let NamuiEvent::AnimationFrame = event {
                self.evaluate_ticks();
            };
        });

        self.map_loader.update(event, &mut self.ecs_app);
        self.image_loader.update(event);
        self.menu.update(event);
        self.save_load
            .update(event, &mut self.ecs_app, &mut self.state, &self.map_loader);
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
                    self.render_interaction_guide(&rendering_context),
                ]),
            ),
            self.menu.render(),
            self.key_handler(),
        ])
    }

    fn key_handler(&self) -> RenderingTree {
        RenderingTree::Empty.attach_event(|builder| {
            builder
                .on_key_down(|event: KeyboardEvent| {
                    namui::event::send(Event::KeyDown {
                        code: event.code,
                        pressing_codes: event.pressing_codes.clone(),
                    });
                })
                .on_key_up(|event: KeyboardEvent| {
                    namui::event::send(Event::KeyUp {
                        code: event.code,
                        pressing_codes: event.pressing_codes.clone(),
                    });
                });
        })
    }
}

pub enum Event {
    KeyDown {
        code: Code,
        pressing_codes: HashSet<Code>,
    },
    KeyUp {
        code: Code,
        pressing_codes: HashSet<Code>,
    },
}
