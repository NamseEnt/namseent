mod login;

use crate::{
    pages::{router, sequence_list_page::SequenceListPage},
    storage::{EditorHistorySystem, Storage, SystemTree},
    sync,
};
use crdt::{HistorySystem, List};
use namui::prelude::*;
use namui_prebuilt::*;

pub enum App {
    LoggingIn,
    Initializing,
    Initialized {
        editor_history_system: EditorHistorySystem,
        router: router::Router,
        syncer: sync::Syncer,
    },
}
impl App {
    pub fn new() -> Self {
        login::check_token();
        App::LoggingIn
    }
}

enum Event {
    LoggedIn(Storage),
    Initialized {
        storage: Storage,
        encoded_history_system: Box<[u8]>,
    },
}

impl namui::Entity for App {
    type Props = ();

    fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::LoggedIn(storage) => {
                    *self = App::Initializing;
                    start_initialize(storage.clone());
                }
                Event::Initialized {
                    encoded_history_system,
                    storage,
                } => {
                    if let App::Initialized { .. } = self {
                        panic!("already initialized");
                    }
                    let editor_history_system =
                        EditorHistorySystem::new(HistorySystem::decode(&encoded_history_system));
                    *self = App::Initialized {
                        editor_history_system: editor_history_system.clone(),
                        router: router::Router::new(router::Route::SequenceListPage(
                            SequenceListPage::new(editor_history_system, storage.clone()),
                        )),
                        syncer: sync::Syncer::new(storage.clone(), encoded_history_system.clone()),
                    };
                }
            }
        } else if let Some(event) = event.downcast_ref::<sync::Event>() {
            match event {
                sync::Event::NewHistorySystem { encoded } => match self {
                    App::LoggingIn | App::Initializing => {
                        unreachable!()
                    }
                    App::Initialized {
                        editor_history_system,
                        ..
                    } => {
                        editor_history_system.merge(encoded);
                        save_history_system_to_cache(editor_history_system.encode());
                    }
                },
            }
        } else if let Some(event) = event.downcast_ref::<crate::storage::Event>() {
            match event {
                crate::storage::Event::Mutated { encoded_update } => match self {
                    App::LoggingIn | App::Initializing => {
                        unreachable!()
                    }
                    App::Initialized {
                        editor_history_system,
                        syncer,
                        ..
                    } => syncer.send(encoded_update),
                },
            }
        }

        self.update_login(event);

        match self {
            App::LoggingIn | App::Initializing => {}
            App::Initialized { router, .. } => {
                router.update(event);
            }
        }
    }
    fn render(&self, _: &Self::Props) -> namui::RenderingTree {
        let wh = namui::screen::size();
        render([
            simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::BLACK),
            match &self {
                App::LoggingIn => typography::body::center(wh, "Logging in...", Color::BLACK),
                App::Initializing => typography::body::center(wh, "Initializing...", Color::BLACK),
                App::Initialized { router, syncer, .. } => router.render(router::Props {
                    wh,
                    sync_send_status: syncer.get_sync_status(),
                }),
            },
        ])
    }
}

fn save_history_system_to_cache(encoded_history_system: Box<[u8]>) {
    namui::spawn_local(async move {
        namui::cache::set("HistorySystem", &encoded_history_system)
            .await
            .unwrap()
    });
}

fn start_initialize(storage: Storage) {
    namui::spawn_local(async move {
        let cached = namui::cache::get("HistorySystem").await.unwrap();
        let encoded_history_system = if let Some(encoded_history_system) = cached {
            encoded_history_system
        } else {
            let encoded_history_system: Box<[u8]> = match storage.get().await {
                Ok(history_system) => history_system.encode().into(),
                Err(error) => match error {
                    editor_core::storage::GetError::NotExists => HistorySystem::new(SystemTree {
                        sequence_list: List::new([]),
                    })
                    .encode()
                    .into(),
                    editor_core::storage::GetError::Unknown(error) => {
                        panic!("fail to get data from storage, {}", error)
                    }
                },
            };

            save_history_system_to_cache(encoded_history_system.clone());

            encoded_history_system
        };

        namui::event::send(Event::Initialized {
            encoded_history_system,
            storage,
        });
    });
}
