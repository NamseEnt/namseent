use crate::common::github;
use crate::route_generated::Redirect;
use forte_sdk::*;

pub type Props = Redirect;

pub async fn handler(req: ForteRequest<'_>) -> anyhow::Result<Props> {
    let nonce = crate::common::auth::prepare_oauth(req.jar).await;
    let client_id = std::env::var("GITHUB_CLIENT_ID")?;
    let redirect_uri = github::callback_url(req.uri_authority);

    let query = form_urlencoded::Serializer::new(String::new())
        .append_pair("client_id", &client_id)
        .append_pair("redirect_uri", &redirect_uri)
        .append_pair("scope", "read:user")
        .append_pair("state", &nonce)
        .finish();

    Ok(Redirect::External {
        url: format!("https://github.com/login/oauth/authorize?{query}"),
    })
}
