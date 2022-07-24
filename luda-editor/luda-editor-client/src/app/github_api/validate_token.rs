use super::{types::RequestBuilder, GithubApiClient};

impl GithubApiClient {
    pub async fn validate_token(&self) -> Result<(), ValidateTokenError> {
        let url = format!("{}/", self.get_base_url(),);

        let response = RequestBuilder::new(url)
            .get()
            .access_token(self.get_access_token().clone())
            .accept_json()
            .send()
            .await;

        if !response.ok() {
            return Err(ValidateTokenError::ValidationFailed);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum ValidateTokenError {
    ValidationFailed,
}
