use super::{
    parse_response_as_json::{parse_response_as_json, ResponseParseError},
    types::RequestBuilder,
    GithubApiClient, User,
};

impl GithubApiClient {
    pub async fn get_user_id(&self) -> Result<u32, GetUserIdError> {
        let url = format!("{}/user", self.get_base_url());

        let response = RequestBuilder::new(url)
            .get()
            .access_token(self.get_access_token().clone())
            .accept_json()
            .send()
            .await;

        if !response.ok() {
            return Err(GetUserIdError::ValidationFailed);
        }

        let response: User = parse_response_as_json(response).await?;
        Ok(response.id)
    }
}

#[derive(Debug)]
pub enum GetUserIdError {
    ValidationFailed,
    ResponseParseError(ResponseParseError),
}
impl From<ResponseParseError> for GetUserIdError {
    fn from(error: ResponseParseError) -> Self {
        GetUserIdError::ResponseParseError(error)
    }
}
