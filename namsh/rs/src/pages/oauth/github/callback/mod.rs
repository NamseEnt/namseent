use crate::common::{auth, github};
use crate::docs::*;
use crate::route_generated::Redirect;
use forte_sdk::*;
use serde::Deserialize;

pub struct SearchParams {
    pub code: String,
    pub state: String,
}

pub type Props = Redirect;

pub async fn handler(
    req: ForteRequest<'_>,
    search_params: SearchParams,
) -> anyhow::Result<Props> {
    if !auth::verify_oauth_state(req.jar, &search_params.state) {
        return Ok(Redirect::Login);
    }

    let client_id = std::env::var("GITHUB_CLIENT_ID")?;
    let client_secret = std::env::var("GITHUB_CLIENT_SECRET")?;
    let redirect_uri = github::callback_url(req.uri_authority);

    let access_token = match exchange_code(
        &client_id,
        &client_secret,
        &search_params.code,
        &redirect_uri,
    )
    .await?
    {
        Some(t) => t,
        None => return Ok(Redirect::Login),
    };

    let gh_user = match fetch_github_user(&access_token).await? {
        Some(u) => u,
        None => return Ok(Redirect::Login),
    };

    let db = doc_db::turso();
    let user = match (UserDocGet {
        github_id: gh_user.id,
    })
    .send_with(&db)
    .await?
    {
        Some(mut existing) => {
            if existing.github_login != gh_user.login {
                existing.github_login = gh_user.login.clone();
                UserDocPut(existing.clone()).send_with(&db).await?;
            }
            existing
        }
        None => return Ok(Redirect::Login),
    };

    auth::create_session(req.jar, user).await?;
    if let Some(pending) = auth::take_pending_cli_consent(req.jar) {
        return Ok(Redirect::External { url: pending });
    }
    Ok(Redirect::Index)
}

async fn exchange_code(
    client_id: &str,
    client_secret: &str,
    code: &str,
    redirect_uri: &str,
) -> anyhow::Result<Option<String>> {
    let body = form_urlencoded::Serializer::new(String::new())
        .append_pair("client_id", client_id)
        .append_pair("client_secret", client_secret)
        .append_pair("code", code)
        .append_pair("redirect_uri", redirect_uri)
        .finish();

    let resp = http::Client::new()
        .send(
            http::Request::builder()
                .uri("https://github.com/login/oauth/access_token")
                .method("POST")
                .header("Accept", "application/json")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(body)?,
        )
        .await?;

    if !resp.status().is_success() {
        return Ok(None);
    }

    #[derive(Deserialize)]
    struct TokenResp {
        access_token: Option<String>,
    }
    let parsed: TokenResp = resp.into_body().json().await?;
    Ok(parsed.access_token)
}

async fn fetch_github_user(access_token: &str) -> anyhow::Result<Option<GhUser>> {
    let resp = http::Client::new()
        .send(
            http::Request::builder()
                .uri("https://api.github.com/user")
                .method("GET")
                .header("Authorization", format!("Bearer {access_token}"))
                .header("Accept", "application/vnd.github+json")
                .header("User-Agent", "namsh")
                .body(Vec::<u8>::new())?,
        )
        .await?;

    if !resp.status().is_success() {
        return Ok(None);
    }

    let user: GhUser = resp.into_body().json().await?;
    Ok(Some(user))
}

#[derive(Deserialize)]
struct GhUser {
    id: i64,
    login: String,
}
