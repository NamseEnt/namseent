use crate::app::github_api::GithubAPiClient;
use namui::prelude::*;

pub struct Storage {
    github_api_client: GithubAPiClient,
    client_id: String,
}
impl Storage {
    pub fn new(github_api_client: GithubAPiClient) -> Self {
        let client_id = nanoid();
        Self {
            github_api_client,
            client_id,
        }
    }

    pub(super) fn get_github_api_client(&self) -> &GithubAPiClient {
        &&self.github_api_client
    }

    pub(super) fn get_client_id(&self) -> &String {
        &self.client_id
    }
}
