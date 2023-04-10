use super::{map::MapLoader, GameState};
use crate::ecs;
use namui::{file::local_storage, simple_error_impl, spawn_local, Time, TimeExt};
use serde::{Deserialize, Serialize};

const AUTOSAVE_MINIMUM_TERM: Time = Time::Sec(5.0);

pub struct SaveLoad {
    pub state: SaveLoadState,
    last_autosave_time: Time,
    auto_save: bool,
}

impl SaveLoad {
    pub fn new() -> Self {
        Self {
            state: SaveLoadState::Idle,
            last_autosave_time: 0.ms(),
            auto_save: false,
        }
    }

    pub fn update(
        &mut self,
        event: &namui::Event,
        ecs_app: &mut ecs::App,
        game_state: &mut GameState,
        map_loader: &MapLoader,
    ) {
        self.try_auto_save(ecs_app, game_state, map_loader);

        event.is::<InternalEvent>(|event| match event {
            InternalEvent::Saved | InternalEvent::Loaded => {
                self.state = SaveLoadState::Idle;
            }
            InternalEvent::SerializedGameFetched(serialized_game) => {
                self.apply_serialized_game(ecs_app, game_state, serialized_game);
                namui::event::send(Event::Loaded);
            }
            InternalEvent::Failed(error) => self.state = SaveLoadState::Failed(error.clone()),
            InternalEvent::AutoSaveOnRequested => {
                self.auto_save = true;
                let _ = self.save(ecs_app, game_state, map_loader);
            }
            InternalEvent::SaveRequested => {
                let _ = self.save(ecs_app, game_state, map_loader);
            }
            InternalEvent::LoadRequested => {
                let _ = self.load();
            }
        });
    }

    fn save(
        &mut self,
        ecs_app: &ecs::App,
        game_state: &GameState,
        map_loader: &MapLoader,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !map_loader.loaded() {
            return Err(SaveLoadError::Busy.into());
        }
        let serialized_game_state = ron::to_string(&game_state).unwrap();
        let serialized_ecs_app = ecs_app.save();
        let serialized_game = SerializedGame {
            serialized_game_state,
            serialized_ecs_app,
        };
        let serialized_game =
            ron::to_string(&serialized_game).map_err(|error| error.to_string())?;
        spawn_local(async move {
            match local_storage::write("/saved_game", serialized_game).await {
                Ok(_) => namui::event::send(InternalEvent::Saved),
                Err(error) => namui::event::send(InternalEvent::Failed(error.to_string())),
            }
        });
        return Ok(());
    }

    fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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

    fn apply_serialized_game(
        &mut self,
        ecs_app: &mut ecs::App,
        game_state: &mut GameState,
        serialized_game: &SerializedGame,
    ) {
        ecs_app.clear_entities();

        let saved_state: Result<GameState, _> =
            ron::from_str(&serialized_game.serialized_game_state);
        if let Err(error) = saved_state {
            namui::event::send(InternalEvent::Failed(error.to_string()));
            return;
        }
        let saved_state = saved_state.unwrap();

        let saved_ecs_app = ecs::App::load(&serialized_game.serialized_ecs_app);
        if let Err(error) = saved_ecs_app {
            namui::event::send(InternalEvent::Failed(error.to_string()));
            return;
        }
        let saved_ecs_app = saved_ecs_app.unwrap();

        let new_time_offset = get_new_time_offset(&saved_state, game_state);

        *ecs_app = saved_ecs_app;
        *game_state = saved_state;

        game_state.tick.time_offset = new_time_offset;

        namui::event::send(InternalEvent::Loaded);
    }

    fn try_auto_save(
        &mut self,
        ecs_app: &mut ecs::App,
        game_state: &mut GameState,
        map_loader: &MapLoader,
    ) {
        if self.auto_save
            && (self.last_autosave_time + AUTOSAVE_MINIMUM_TERM <= game_state.tick.current_time)
        {
            self.last_autosave_time = game_state.tick.current_time;
            let _ = self.save(ecs_app, game_state, map_loader);
        }
    }

    pub fn request_auto_save_on() {
        namui::event::send(InternalEvent::AutoSaveOnRequested);
    }

    pub fn request_save() {
        namui::event::send(InternalEvent::SaveRequested)
    }

    pub fn request_load() {
        namui::event::send(InternalEvent::LoadRequested)
    }
}

async fn get_serialized_game() -> Result<SerializedGame, Box<dyn std::error::Error>> {
    let serialized_game = local_storage::read("/saved_game")
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

pub enum Event {
    Loaded,
}

enum InternalEvent {
    Saved,
    Loaded,
    SerializedGameFetched(SerializedGame),
    Failed(String),
    AutoSaveOnRequested,
    SaveRequested,
    LoadRequested,
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

fn get_new_time_offset(saved_state: &GameState, current_state: &GameState) -> Time {
    saved_state.tick.current_time - current_state.tick.current_time + current_state.tick.time_offset
}

#[cfg(test)]
mod test {
    use namui::TimeExt;
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::app::game::GameState;

    #[test]
    #[wasm_bindgen_test]
    fn calibrate_time_offset() {
        // Saved when now() is 1500ms
        let mut saved = GameState::new();
        saved.tick.current_time = 2000.ms();
        saved.tick.time_offset = 500.ms();

        // Load start when now() is 15ms
        let mut current = GameState::new();
        current.tick.current_time = 20.ms();
        current.tick.time_offset = 5.ms();

        let new_time_offset = super::get_new_time_offset(&saved, &current);

        assert_eq!(saved.tick.current_time, new_time_offset + 15.ms());
    }
}
