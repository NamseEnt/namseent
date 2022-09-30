use super::{player_character::new_player, *};
use namui::prelude::*;

pub struct Game {
    pub player_entity_id: Uuid,
    pub state: GameState,
    pub camera: Camera,
    pub ecs_app: crate::ecs::App,
}
impl Game {
    pub fn new() -> Self {
        let character = mock_character();
        let quest_object = mock_quest_object();
        let walls = mock_walls();
        let floors = mock_floor();
        let state = GameState::new();

        let player_entity_id = character.id();

        let mut ecs_app = crate::ecs::App::new();
        ecs_app.add_entity(character);
        ecs_app.add_entity(quest_object);
        ecs_app.add_entities(walls);
        ecs_app.add_entity(floors);

        Self {
            player_entity_id,
            state,
            camera: Camera::new(Some(CameraSubject::Object {
                id: player_entity_id,
            })),
            ecs_app,
        }
    }

    pub fn update(&mut self, event: &dyn std::any::Any) {
        let current_time = now();
        if let Some(_event) = event.downcast_ref::<GameEvent>() {
            // match event {
            //     GameEvent::AddObject(object_constructor) => {
            //         self.object_list.push(object_constructor());
            //     }
            // }
        } else if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match event {
                NamuiEvent::KeyDown(_) => self.handle_character_movement_on_key_event(current_time),
                NamuiEvent::KeyUp(_) => self.handle_character_movement_on_key_event(current_time),
                NamuiEvent::AnimationFrame
                | NamuiEvent::MouseDown(_)
                | NamuiEvent::MouseUp(_)
                | NamuiEvent::MouseMove(_)
                | NamuiEvent::ScreenResize(_)
                | NamuiEvent::Wheel(_)
                | NamuiEvent::DeepLinkOpened(_) => (),
            }
        }
        self.predict_character_movement_if_needed(current_time);
        self.camera.update(event);
    }

    pub fn render(&self) -> namui::RenderingTree {
        let rendering_context = self.create_rendering_context();

        render([
            render_background(),
            self.camera.translate_to_camera_screen(
                &rendering_context,
                render([
                    self.render_in_screen_object_list(&rendering_context),
                    self.render_guide_icon(&rendering_context),
                ]),
            ),
        ])
    }
}

pub enum GameEvent {
    // AddObject(Arc<dyn (Fn() -> crate::ecs::Entity) + Send + Sync>),
}

fn mock_character() -> crate::ecs::Entity {
    new_player(Xy {
        x: 8.tile(),
        y: 6.tile(),
    })
}

fn mock_walls() -> Vec<crate::ecs::Entity> {
    let mut wall = Vec::new();
    for x in 1..10 {
        if x == 5 {
            for y in 1..10 {
                wall.push(new_wall(
                    Xy {
                        x: x.tile(),
                        y: y.tile(),
                    },
                    0.sec(),
                ));
            }
        } else {
            wall.push(new_wall(
                Xy {
                    x: x.tile(),
                    y: 1.tile(),
                },
                0.sec(),
            ));
            wall.push(new_wall(
                Xy {
                    x: x.tile(),
                    y: 9.tile(),
                },
                0.sec(),
            ));
        }
    }
    wall
}

fn mock_floor() -> crate::ecs::Entity {
    let positions = (1..100)
        .into_iter()
        .flat_map(|x| {
            (1..100).into_iter().map(move |y| Xy {
                x: x.tile(),
                y: y.tile(),
            })
        })
        .collect();
    new_floor(positions)
}

fn mock_quest_object() -> crate::ecs::Entity {
    new_wall_with_id(
        known_id::object::FIRST_QUEST_OBJECT,
        Xy {
            x: 10.tile(),
            y: 10.tile(),
        },
        0.sec(),
    )
}
