use super::Map;
use crate::app::game::{interaction, new_player, Tile, TileExt};
use namui::{prelude::*, simple_error_impl};

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
            let _ = self.start_load("first".to_string(), Xy::new(8.tile(), 6.tile()));
        }

        event
            .is::<InternalEvent>(|event| match event {
                InternalEvent::FailedToReadMapFromBundle => {
                    self.state = MapLoaderState::Error(Error::MapNotFound);
                }
                InternalEvent::SerializedMapLoaded(serialized_map, player_xy) => {
                    let Ok(map) = ron::from_str::<Map>(serialized_map) else {
                        self.state = MapLoaderState::Error(Error::InvalidMap);
                        return;
                    };
                    app.clear_entities();
                    map.create_entities(app);
                    new_player(app, *player_xy);
                    self.state = MapLoaderState::Loaded;
                }
            })
            .is::<interaction::Event>(|event| {
                let interaction::Event::Interacted { kind, .. } = event;
                let interaction::InteractionKind::MapTeleport { map_name, player_xy } = kind else {
                    return;
                };
                let _ = self.start_load(map_name.clone(), *player_xy);
            });
    }

    pub fn start_load(
        &mut self,
        map_name: String,
        player_xy: Xy<Tile>,
    ) -> Result<(), StartLoadError> {
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
            namui::event::send(InternalEvent::SerializedMapLoaded(
                serialized_map,
                player_xy,
            ));
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
    SerializedMapLoaded(String, Xy<Tile>),
}
