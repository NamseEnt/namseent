use super::Map;
use namui::{simple_error_impl, spawn_local};

pub struct MapLoader {
    state: MapLoaderState,
}

#[derive(Debug)]
pub enum MapLoaderState {
    Idle,
    Loading,
    Loaded,
    Error(Error),
}

impl MapLoader {
    pub fn new() -> Self {
        Self {
            state: MapLoaderState::Idle,
        }
    }

    pub fn update(&mut self, event: &namui::Event, app: &mut crate::ecs::App) {
        // TODO: This is a mock. We should be able to start loading a map from the outside.
        if let MapLoaderState::Idle = self.state {
            let _ = self.start_load("first".to_string());
        }

        event.is::<InternalEvent>(|event| match event {
            InternalEvent::FailedToReadMapFromBundle => {
                self.state = MapLoaderState::Error(Error::MapNotFound);
            }
            InternalEvent::SerializedMapLoaded(serialized_map) => {
                let Ok(map) = ron::from_str::<Map>(serialized_map) else {
                    self.state = MapLoaderState::Error(Error::InvalidMap);
                    return;
                };
                map.create_entities(app);
                self.state = MapLoaderState::Loaded;
            }
        });
    }

    pub fn start_load(&mut self, map_name: String) -> Result<(), StartLoadError> {
        if let MapLoaderState::Loading = self.state {
            return Err(StartLoadError::AlreadyLoading);
        }

        self.state = MapLoaderState::Loading;
        spawn_local(async move {
            let Ok(serialized_map) = namui::system::file::bundle::read(format!("map/{map_name}.ron").as_str()).await.map(|serialized_map_bytes| {
                String::from_utf8_lossy(serialized_map_bytes.as_ref()).to_string()
            }) else {
                namui::event::send(InternalEvent::FailedToReadMapFromBundle);
                return;
            };
            namui::event::send(InternalEvent::SerializedMapLoaded(serialized_map));
        });
        return Ok(());
    }

    pub fn loaded(&self) -> bool {
        match self.state {
            MapLoaderState::Loaded => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum StartLoadError {
    AlreadyLoading,
}
simple_error_impl!(StartLoadError);

#[derive(Debug)]
pub enum Error {
    MapNotFound,
    InvalidMap,
}
simple_error_impl!(Error);

enum InternalEvent {
    FailedToReadMapFromBundle,
    SerializedMapLoaded(String),
}
