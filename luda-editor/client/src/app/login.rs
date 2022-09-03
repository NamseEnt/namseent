use super::App;
use namui::prelude::*;

const DEV_CLIENT_ID: &str = "abd04a6aeba3e99f5b4b";
const CLIENT_ID: Option<&str> = option_env!("GITHUB_CLIENT_ID");

pub enum Event {
    SessionId(String),
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
                Event::SessionId(session_id) => {
                    crate::RPC.set_session_id(session_id.clone());
                    namui::event::send(super::Event::LoggedIn);
                }
                Event::Error(error) => namui::log!("error: {}", error),
            }
        }
    }
}

pub fn check_token() {
    namui::spawn_local(async {
        match namui::cache::get("SessionId").await.unwrap() {
            Some(token) => {
                namui::event::send(Event::SessionId(String::from_utf8(token.into()).unwrap()));
            }
            None => match request_github_auth_code().await {
                Ok(code) => login_with_oauth_code(code),
                Err(error) => {
                    namui::event::send(Event::Error(error.to_string()));
                }
            },
        }
    });
}

async fn request_github_auth_code() -> Result<String, Box<dyn std::error::Error>> {
    let client_id = CLIENT_ID.unwrap_or(DEV_CLIENT_ID);
    let redirect_uri = web_sys::window().unwrap().location().href().unwrap();
    let url = format!("https://github.com/login/oauth/authorize?client_id={client_id}&redirect_uri=https://sslwiheugl5ojmqlecerzhng740cekqc.lambda-url.ap-northeast-2.on.aws/{redirect_uri}");

    let auth_code_window = web_sys::window()
        .unwrap()
        .open_with_url(&url)
        .unwrap()
        .unwrap();

    loop {
        namui::time::delay(100.ms()).await;
        match auth_code_window.location().search() {
            Ok(query) => {
                if query.starts_with("?code=") {
                    auth_code_window.close().unwrap();
                    return Ok(query[6..].to_string());
                }
            }
            Err(_) => continue,
        }
    }
}
fn login_with_oauth_code(code: String) {
    spawn_local(async move {
        let result = crate::RPC
            .log_in_with_github_oauth_code(rpc::log_in_with_github_oauth_code::Request { code })
            .await;

        match result {
            Ok(response) => {
                let session_id = response.session_id;
                namui::event::send(Event::SessionId(session_id.clone()));
                namui::cache::set("SessionId", session_id.as_bytes())
                    .await
                    .unwrap();
            }
            Err(error) => {
                namui::event::send(Event::Error(error.to_string()));
            }
        }
    })
}
