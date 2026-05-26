use crate::common::auth;
use crate::route_generated::Redirect;
use forte_sdk::*;
use serde::Serialize;

pub struct SearchParams {
    pub redirect_uri: String,
    pub code_challenge: String,
    pub code_challenge_method: String,
    pub state: String,
    pub label: String,
}

#[derive(Serialize)]
pub struct Props {
    pub github_login: String,
    pub redirect_uri: String,
    pub code_challenge: String,
    pub code_challenge_method: String,
    pub state: String,
    pub default_label: String,
}

pub async fn handler(
    req: ForteRequest<'_>,
    search_params: SearchParams,
) -> anyhow::Result<Props> {
    if search_params.code_challenge_method != "S256" {
        anyhow::bail!("code_challenge_method must be S256");
    }
    if search_params.code_challenge.is_empty() {
        anyhow::bail!("code_challenge is required");
    }
    if !auth::is_loopback_redirect(&search_params.redirect_uri) {
        anyhow::bail!("redirect_uri must be http loopback (127.0.0.1, localhost, or [::1])");
    }
    if search_params.state.is_empty() {
        anyhow::bail!("state is required");
    }

    let Some(user) = auth::current_user(req.jar).await else {
        let query = form_urlencoded::Serializer::new(String::new())
            .append_pair("redirect_uri", &search_params.redirect_uri)
            .append_pair("code_challenge", &search_params.code_challenge)
            .append_pair("code_challenge_method", &search_params.code_challenge_method)
            .append_pair("state", &search_params.state)
            .append_pair("label", &search_params.label)
            .finish();
        let url = format!("/oauth/cli/authorize?{query}");
        auth::stash_pending_cli_consent(req.jar, &url);
        return Err(Redirect::Login.into());
    };

    Ok(Props {
        github_login: user.github_login,
        redirect_uri: search_params.redirect_uri,
        code_challenge: search_params.code_challenge,
        code_challenge_method: search_params.code_challenge_method,
        state: search_params.state,
        default_label: search_params.label,
    })
}
