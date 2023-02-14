use crate::{app::game::Game, ecs};
use namui::{file::local_storage, simple_error_impl, spawn_local};
use serde::{Deserialize, Serialize};

pub struct SaveLoad {
    pub state: SaveLoadState,
}

impl SaveLoad {
    pub fn new() -> Self {
        Self {
            state: SaveLoadState::Idle,
        }
    }

    pub fn update(&mut self, event: &namui::Event, game: &mut Game) {
        event.is::<InternalEvent>(|event| match event {
            InternalEvent::Saved | InternalEvent::Loaded => {
                self.state = SaveLoadState::Idle;
            }
            InternalEvent::SerializedGameFetched(serialized_game) => {
                let ecs_app = ecs::App::load(&serialized_game.serialized_ecs_app);
                if let Err(error) = ecs_app {
                    namui::event::send(InternalEvent::Failed(error.to_string()));
                    return;
                }
                let ecs_app = ecs_app.unwrap();

                let state = ron::from_str(&serialized_game.serialized_game_state);
                if let Err(error) = state {
                    namui::event::send(InternalEvent::Failed(error.to_string()));
                    return;
                }
                let state = state.unwrap();

                game.ecs_app.clear_entities();
                game.ecs_app = ecs_app;
                game.state = state;
                namui::event::send(InternalEvent::Loaded);
            }
            InternalEvent::Failed(error) => self.state = SaveLoadState::Failed(error.clone()),
        });
    }

    fn request_save(&mut self, game: &mut Game) -> Result<(), Box<dyn std::error::Error>> {
        let serialized_game_state = ron::to_string(&game.state).unwrap();
        let serialized_ecs_app = game.ecs_app.save();
        let serialized_game = SerializedGame {
            serialized_game_state,
            serialized_ecs_app,
        };
        let serialized_game =
            ron::to_string(&serialized_game).map_err(|error| error.to_string())?;
        spawn_local(async move {
            match local_storage::write("saved_game", serialized_game).await {
                Ok(_) => namui::event::send(InternalEvent::Saved),
                Err(error) => namui::event::send(InternalEvent::Failed(error.to_string())),
            }
        });
        return Ok(());
    }

    fn request_load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match self.state {
            SaveLoadState::Idle | SaveLoadState::Failed(_) => {
                spawn_local(async move {
                    let serialized_game = get_serialized_game().await;
                    if let Err(error) = serialized_game {
                        namui::event::send(InternalEvent::Failed(error.to_string()));
                        return;
                    }
                    let serialized_game = serialized_game.unwrap();
                    namui::event::send(InternalEvent::SerializedGameFetched(serialized_game));
                });
                Ok(())
            }
            _ => Err(SaveLoadError::Busy.into()),
        }
    }
}

async fn get_serialized_game() -> Result<SerializedGame, Box<dyn std::error::Error>> {
    let serialized_game = local_storage::read("saved_game")
        .await
        .map_err(|error| error.to_string())
        .and_then(|bytes| String::from_utf8(bytes).map_err(|error| error.to_string()))
        .and_then(|string| {
            ron::from_str::<SerializedGame>(string.as_str()).map_err(|error| error.to_string())
        })?;
    Ok(serialized_game)
}

pub enum SaveLoadState {
    Idle,
    Saving,
    Loading,
    Failed(String),
}

enum InternalEvent {
    Saved,
    Loaded,
    SerializedGameFetched(SerializedGame),
    Failed(String),
}

#[derive(Debug)]
pub enum SaveLoadError {
    Busy,
}
simple_error_impl!(SaveLoadError);

#[derive(Serialize, Deserialize)]
struct SerializedGame {
    serialized_game_state: String,
    serialized_ecs_app: String,
}
