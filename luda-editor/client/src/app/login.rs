use super::App;
use crate::storage::Storage;
use editor_core::{github_client::GithubClient, storage::GithubStorage};
use namui::prelude::*;
use std::sync::Arc;

const CLIENT_ID: &str = "abd04a6aeba3e99f5b4b";
const CLIENT_SECRET: &str = "501a915ca627e24d2088cf01416fe836db470dba";

pub enum Event {
    AccessToken(String),
    Error(String),
}

impl App {
    pub fn update_login(&mut self, event: &dyn std::any::Any) {
        if let Some(namui::NamuiEvent::DeepLinkOpened(DeepLinkOpenedEvent { url })) =
            event.downcast_ref()
        {
            match Url::parse(url) {
                Ok(url) => {
                    let code = url.query_pairs().find(|(key, _)| key == "code");
                    if let Some((_, code)) = code {
                        login_with_oauth_code(code.to_string());
                    }
                }
                Err(error) => {
                    namui::log!("Error for deep link {url}: {error}");
                }
            }
        } else if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::AccessToken(access_token) => {
                    let github_client = GithubClient::new(
                        access_token.clone(),
                        "https://api.github.com".to_string(),
                        "namseent".to_string(),
                        "luda-editor-storage".to_string(),
                    );
                    let storage = Storage::new(Arc::new(GithubStorage::new(
                        github_client,
                        "master".to_string(),
                    )));
                    namui::event::send(super::Event::LoggedIn(storage));
                }
                Event::Error(error) => namui::log!("error: {}", error),
            }
        }
    }
}

pub fn check_token() {
    namui::spawn_local(async {
        match namui::cache::get("AccessToken").await.unwrap() {
            Some(token) => {
                namui::event::send(Event::AccessToken(String::from_utf8(token.into()).unwrap()));
            }
            None => {
                let url = format!("https://github.com/login/oauth/authorize?client_id={CLIENT_ID}&scope=public_repo");
                namui::open_external(url.as_str());
            }
        }
    });
}
fn login_with_oauth_code(code: String) {
    spawn_local(async move {
        match editor_core::github_client::get_access_token_with_oauth_code(
            CLIENT_ID,
            CLIENT_SECRET,
            &code,
        )
        .await
        {
            Ok(access_token) => {
                namui::event::send(Event::AccessToken(access_token.clone()));
                namui::cache::set("AccessToken", access_token.as_bytes())
                    .await
                    .unwrap();
            }
            Err(error) => {
                namui::event::send(Event::Error(error.to_string()));
            }
        };
    })
}
