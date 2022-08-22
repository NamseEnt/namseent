use super::*;

pub async fn get_access_token_with_oauth_code(
    client_id: &str,
    client_secret: &str,
    code: &str,
) -> Result<String, GithubClientError> {
    const URL: &str = "https://github.com/login/oauth/access_token";

    let body = serde_json::to_vec(&GetAccessTokenWithOauthCodeRequest {
        client_id: client_id.to_string(),
        client_secret: client_secret.to_string(),
        code: code.to_string(),
    })
    .map_err(|error| GithubClientError::BodySerializeError(error.into()))?;

    let response: GetAccessTokenWithOauthCodeResponse =
        namui::network::http::fetch_json(URL, namui::network::http::Method::POST, |builder| {
            builder
                .body(body)
                .header("Content-Type", "application/json")
                .header("Accept", "application/json")
                .header("User-Agent", "luda-editor")
                .fetch_credentials_include()
        })
        .await?;

    Ok(response.access_token)
}

#[derive(Serialize)]
struct GetAccessTokenWithOauthCodeRequest {
    client_id: String,
    client_secret: String,
    code: String,
}

#[derive(Deserialize)]
struct GetAccessTokenWithOauthCodeResponse {
    access_token: String,
}
