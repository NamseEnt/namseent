use super::{
    known_id, Camera, CameraSubject, Floor, GameObject, GameState, PlayerCharacter,
    RenderGameObjectList, RenderingContext, TileExt, Wall,
};
use namui::prelude::*;
use namui_prebuilt::simple_rect;

pub struct Game {
    pub object_list: Vec<Box<dyn GameObject>>,
    pub state: GameState,
    pub camera: Camera,
}
impl Game {
    pub fn new() -> Self {
        let character = mock_character();
        let quest_object = mock_quest_object();
        let walls = mock_walls();
        let floors = mock_floors();
        let state = GameState::new();
        let mut object_list = Vec::new();
        object_list.push(character);
        object_list.push(quest_object);
        object_list.extend(walls);
        object_list.extend(floors);

        Self {
            object_list,
            state,
            camera: Camera::new(Some(CameraSubject::Object {
                id: known_id::object::PLAYER_CHARACTER_OBJECT,
            })),
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
            .get_in_screen_object_list(&self.object_list, &rendering_context);

        render([
            render_background(screen_size),
            self.camera.render(
                &self.object_list,
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
    // AddObject(Arc<dyn (Fn() -> Box<dyn GameObject>) + Send + Sync>),
}

fn render_background(wh: Wh<Px>) -> namui::RenderingTree {
    render([simple_rect(
        wh,
        Color::TRANSPARENT,
        0.px(),
        Color::grayscale_f01(0.2),
    )])
}

fn mock_character() -> Box<dyn GameObject> {
    Box::new(PlayerCharacter::new(
        Xy {
            x: 8.tile(),
            y: 6.tile(),
        },
        0.sec(),
    ))
}

fn mock_walls() -> Vec<Box<dyn GameObject>> {
    let mut wall: Vec<Box<dyn GameObject>> = Vec::new();
    for x in 1..10 {
        if x == 5 {
            for y in 1..10 {
                wall.push(Box::new(Wall::new(Xy {
                    x: x.tile(),
                    y: y.tile(),
                })));
            }
        } else {
            wall.push(Box::new(Wall::new(Xy {
                x: x.tile(),
                y: 1.tile(),
            })));
            wall.push(Box::new(Wall::new(Xy {
                x: x.tile(),
                y: 9.tile(),
            })));
        }
    }
    wall
}

fn mock_floors() -> Vec<Box<dyn GameObject>> {
    let mut floors: Vec<Box<dyn GameObject>> = Vec::new();
    for x in 1..100 {
        for y in 1..100 {
            floors.push(Box::new(Floor::new(Xy {
                x: x.tile(),
                y: y.tile(),
            })));
        }
    }
    floors
}

fn mock_quest_object() -> Box<dyn GameObject> {
    Box::new(Wall::new_with_id(
        Xy {
            x: 10.tile(),
            y: 10.tile(),
        },
        known_id::object::FIRST_QUEST_OBJECT,
    ))
}
