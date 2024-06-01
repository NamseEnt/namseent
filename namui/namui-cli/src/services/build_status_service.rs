use crate::{types::ErrorMessage, util::print_build_result};
use std::sync::Arc;
use tokio::sync::RwLock;

pub enum BuildState {
    Building,
    Done {
        compile_error_messages: Vec<ErrorMessage>,
        cli_error_messages: Vec<String>,
    },
}

pub struct BuildStatusService {
    build_state_map: RwLock<std::collections::BTreeMap<BuildStatusCategory, BuildState>>,
}

impl BuildStatusService {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            build_state_map: RwLock::new(std::collections::BTreeMap::new()),
        })
    }
    pub async fn build_started(&self, category: BuildStatusCategory) {
        self.build_state_map
            .write()
            .await
            .insert(category, BuildState::Building);
        self.on_state_change().await;
    }
    pub async fn build_finished(
        &self,
        category: BuildStatusCategory,
        compile_error_messages: Vec<ErrorMessage>,
        cli_error_messages: Vec<String>,
    ) {
        self.build_state_map.write().await.insert(
            category,
            BuildState::Done {
                compile_error_messages,
                cli_error_messages,
            },
        );
        self.on_state_change().await;
    }
    pub async fn compile_error_messages(&self) -> Vec<ErrorMessage> {
        self.build_state_map
            .read()
            .await
            .iter()
            .flat_map(|(_, build_state)| {
                match build_state {
                    BuildState::Building => vec![],
                    BuildState::Done {
                        compile_error_messages,
                        ..
                    } => compile_error_messages.clone(),
                }
                .clone()
            })
            .collect()
    }
    async fn cli_error_messages(&self) -> Vec<String> {
        self.build_state_map
            .read()
            .await
            .iter()
            .flat_map(|(_, build_state)| {
                match build_state {
                    BuildState::Building => vec![],
                    BuildState::Done {
                        cli_error_messages, ..
                    } => cli_error_messages.clone(),
                }
                .clone()
            })
            .collect()
    }
    async fn building_categories(&self) -> Vec<String> {
        self.build_state_map
            .read()
            .await
            .iter()
            .filter_map(|(category, build_state)| match build_state {
                BuildState::Building => Some(format!("{category:?}")),
                BuildState::Done { .. } => None,
            })
            .collect()
    }
    async fn on_state_change(&self) {
        let building_categories = self.building_categories().await;
        match building_categories.is_empty() {
            true => print_build_result(
                &self.compile_error_messages().await,
                &self.cli_error_messages().await,
            ),
            false => {
                println!("building: {}", building_categories.join(", "));
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum BuildStatusCategory {
    Namui,
    WebRuntime,
}
