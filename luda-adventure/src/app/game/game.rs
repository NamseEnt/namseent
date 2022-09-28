use super::{player_character::new_player, *};
use namui::prelude::*;
use namui_prebuilt::simple_rect;

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
        let floors = mock_floors();
        let state = GameState::new();

        let player_entity_id = character.id();

        let mut ecs_app = crate::ecs::App::new();
        ecs_app.add_entity(character);
        ecs_app.add_entity(quest_object);
        ecs_app.add_entities(walls);
        ecs_app.add_entities(floors);

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
        let screen_size = namui::screen::size();
        let px_per_tile = Per::new(32.px(), 1.tile());
        let rendering_context = RenderingContext {
            current_time: now(),
            px_per_tile,
            screen_size: Wh {
                width: px_per_tile.invert() * screen_size.width,
                height: px_per_tile.invert() * screen_size.height,
            },
        };

        let in_screen_object_list = self
            .camera
            .get_in_screen_object_list(&self.ecs_app, &rendering_context);

        render([
            render_background(screen_size),
            self.camera.render(
                &self.ecs_app,
                &rendering_context,
                render([
                    in_screen_object_list.render(&self.state, &rendering_context),
                    self.render_guide_icon(&rendering_context),
                ]),
            ),
        ])
    }
}

pub enum GameEvent {
    // AddObject(Arc<dyn (Fn() -> crate::ecs::Entity) + Send + Sync>),
}

fn render_background(wh: Wh<Px>) -> namui::RenderingTree {
    render([simple_rect(
        wh,
        Color::TRANSPARENT,
        0.px(),
        Color::grayscale_f01(0.2),
    )])
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

fn mock_floors() -> Vec<crate::ecs::Entity> {
    let mut floors: Vec<crate::ecs::Entity> = Vec::new();
    for x in 1..100 {
        for y in 1..100 {
            floors.push(new_floor(Xy {
                x: x.tile(),
                y: y.tile(),
            }));
        }
    }
    floors
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
