use super::{parse_response_as_json::ResponseParseError, types::RequestBuilder, GithubApiClient};
use crate::app::github_api::parse_response_as_json::parse_response_as_json;
use serde::{Deserialize, Serialize};

impl GithubApiClient {
    pub async fn get_access_token_with_oauth_code(
        client_id: &str,
        client_secret: &str,
        code: &str,
    ) -> Result<String, GetAccessTokenWithOauthCodeError> {
        const URL: &str = "https://github.com/login/oauth/access_token";

        let response = RequestBuilder::new(URL.to_string())
            .post()
            .accept_json()
            .json_body(&GetAccessTokenWithOauthCodeRequest {
                client_id: client_id.to_string(),
                client_secret: client_secret.to_string(),
                code: code.to_string(),
            })
            .send()
            .await;

        if !response.ok() {
            return Err(GetAccessTokenWithOauthCodeError::ValidationFailed);
        }

        let response: GetAccessTokenWithOauthCodeResponse =
            parse_response_as_json(response).await?;
        Ok(response.access_token)
    }
}

#[derive(Debug)]
pub enum GetAccessTokenWithOauthCodeError {
    ValidationFailed,
    ResponseParseError(ResponseParseError),
}
impl From<ResponseParseError> for GetAccessTokenWithOauthCodeError {
    fn from(error: ResponseParseError) -> Self {
        GetAccessTokenWithOauthCodeError::ResponseParseError(error)
    }
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
