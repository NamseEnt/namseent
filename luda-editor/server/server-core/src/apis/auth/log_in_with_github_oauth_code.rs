use super::shared::*;
use crate::documents::*;
use lambda_web::is_running_on_lambda;
use rpc::log_in_with_github_oauth_code::{Error, Request, Response};
use std::sync::OnceLock;

pub async fn log_in_with_github_oauth_code(
    session: Option<SessionDocument>,
    req: Request,
) -> rpc::log_in_with_github_oauth_code::Result {
    if session.is_some() {
        return Err(Error::AlreadyLoggedIn);
    }

    let access_token = exchange_github_auth_code_to_access_token(req.code).await;
    if let Err(error) = access_token {
        return Err(Error::Unknown(error.to_string()));
    }
    let access_token = access_token.unwrap();

    let github_user = get_github_user(access_token).await;
    if let Err(error) = github_user {
        return Err(Error::Unknown(error.to_string()));
    }
    let github_user = github_user.unwrap();

    let user = get_or_create_user(UserIdentity::Github {
        github_user_id: github_user.id,
        username: github_user.username,
    })
    .await;
    if let Err(error) = user {
        return Err(Error::Unknown(error.to_string()));
    }
    let user = user.unwrap();

    let session = crate::session::create_session(user.id).await;
    if let Err(error) = session {
        return Err(Error::Unknown(error.to_string()));
    }
    let session = session.unwrap();

    Ok(Response {
        session_id: session.id,
    })
}

async fn exchange_github_auth_code_to_access_token(code: String) -> Result<String, String> {
    #[derive(serde::Serialize)]
    struct RequestBody {
        client_id: String,
        client_secret: String,
        code: String,
    }

    #[derive(serde::Deserialize)]
    struct ResponseBody {
        access_token: String,
    }

    #[derive(Debug)]
    struct GithubAuthSetting {
        client_id: String,
        client_secret: String,
    }

    static GITHUB_CLIENT_SETTING: OnceLock<GithubAuthSetting> = OnceLock::new();
    let github_client_setting = GITHUB_CLIENT_SETTING.get_or_init(|| {
        if is_running_on_lambda() {
            GithubAuthSetting {
                client_id: std::env::var("GITHUB_CLIENT_ID")
                    .expect("Fail to get GITHUB_CLIENT_ID from Environment Variable"),
                client_secret: std::env::var("GITHUB_CLIENT_SECRET")
                    .expect("Fail to get GITHUB_CLIENT_SECRET from Environment Variable"),
            }
        } else {
            GithubAuthSetting {
                // NOTE: https://github.com/organizations/NamseEnt/settings/applications/1968967
                // This is for testing. Do not use in production environment.
                client_id: "abd04a6aeba3e99f5b4b".to_string(),
                client_secret: "501a915ca627e24d2088cf01416fe836db470dba".to_string(),
            }
        }
    });

    let body = serde_json::to_vec(&RequestBody {
        client_id: github_client_setting.client_id.clone(),
        client_secret: github_client_setting.client_secret.clone(),
        code,
    })
    .unwrap();

    let result = reqwest::Client::new()
        .post("https://github.com/login/oauth/access_token")
        .body(body)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("User-Agent", "luda-editor")
        .send()
        .await;

    match result {
        Ok(response) => {
            if response.status().is_success() {
                let body = response.text().await.unwrap();
                let response: ResponseBody = serde_json::from_str(&body).unwrap();
                Ok(response.access_token)
            } else {
                Err(format!(
                    "exchange_github_auth_code_to_access_token failed: {:?}\n{body}",
                    response.status(),
                    body = response.text().await.unwrap()
                ))
            }
        }
        Err(error) => Err(error.to_string()),
    }
}

async fn get_github_user(github_access_token: String) -> Result<GithubUser, String> {
    let result = reqwest::Client::new()
        .get("https://api.github.com/user")
        .header("Authorization", format!("token {github_access_token}"))
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "luda-editor")
        .send()
        .await;

    #[derive(serde::Deserialize)]
    struct ResponseBody {
        id: u128,
        login: String,
    }
    match result {
        Ok(response) => {
            if response.status().is_success() {
                let body = response.text().await.unwrap();
                let response: ResponseBody = serde_json::from_str(&body).unwrap();
                Ok(GithubUser {
                    id: response.id.to_string(),
                    username: response.login,
                })
            } else {
                Err(format!(
                    "get_github_user_id failed: {:?}\n{body}",
                    response.status(),
                    body = response.text().await.unwrap()
                ))
            }
        }
        Err(error) => Err(error.to_string()),
    }
}

struct GithubUser {
    id: String,
    username: String,
}
