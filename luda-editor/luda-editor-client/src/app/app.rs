use super::{
    authentication::{Authentication, AuthenticationEvent, AuthenticationProps},
    events::AppEvent,
    github_api::GithubAPiClient,
    router::RouterProps,
    storage::{GithubStorage, Storage},
    types::{AppContext, MetaContainer},
    Router,
};
use namui::prelude::*;
use std::sync::Arc;
use wasm_bindgen_futures::spawn_local;

pub struct App {
    stage: AppStage,
}

impl namui::Entity for App {
    type Props = ();
    fn render(&self, _: &Self::Props) -> namui::RenderingTree {
        let screen_size = namui::screen::size();
        match &self.stage {
            AppStage::Initialize { authentication } => {
                authentication.render(&AuthenticationProps { wh: screen_size })
            }
            AppStage::Ready {
                router,
                meta_container,
            } => match meta_container.get_meta() {
                Some(meta) => router.render(&RouterProps {
                    screen_wh: Wh {
                        width: screen_size.width,
                        height: screen_size.height,
                    },
                    meta: &meta,
                }),
                None => namui::RenderingTree::Empty,
            },
        }
    }
    fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<AppEvent>() {
            match event {
                AppEvent::Initialized {
                    storage,
                    meta_container,
                } => {
                    let context = AppContext {
                        storage: storage.clone(),
                        meta_container: meta_container.clone(),
                    };
                    self.stage = AppStage::Ready {
                        router: Router::new(context),
                        meta_container: meta_container.clone(),
                    };
                    meta_container.start_reloading();
                }
            }
        }
        match &mut self.stage {
            AppStage::Initialize { authentication } => {
                if let Some(event) = event.downcast_ref::<AuthenticationEvent>() {
                    if let AuthenticationEvent::LoginSucceeded { github_api_client } = event {
                        initialize_app(github_api_client.clone());
                    }
                }
                authentication.update(event);
            }
            AppStage::Ready {
                router,
                meta_container,
            } => {
                meta_container.update(event);
                router.update(event);
            }
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            stage: AppStage::Initialize {
                authentication: Authentication::new(),
            },
        }
    }
}

enum AppStage {
    Initialize {
        authentication: Authentication,
    },
    Ready {
        router: Router,
        meta_container: Arc<MetaContainer>,
    },
}

fn initialize_app(github_api_client: Arc<GithubAPiClient>) {
    spawn_local(async move {
        let storage = Arc::new(Storage::new(github_api_client.clone()));
        let meta_container = Arc::new(MetaContainer::new(None, storage.clone()));
        storage.init().await.unwrap();
        namui::event::send(AppEvent::Initialized {
            storage,
            meta_container,
        });
    });
}
