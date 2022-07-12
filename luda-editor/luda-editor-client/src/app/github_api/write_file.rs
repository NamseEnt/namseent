use super::{
    parse_response_as_json::{parse_response_as_json, ResponseParseError},
    types::{Content, Dirent, RequestBuilder},
    GithubAPiClient,
};
use base64::encode;
use serde::{Deserialize, Serialize};

impl GithubAPiClient {
    pub async fn write_file(
        &self,
        path: &str,
        content: impl AsRef<[u8]>,
    ) -> Result<Dirent, WriteFileError> {
        let url = format!(
            "{}/repos/{}/{}/contents/{}",
            self.get_base_url(),
            self.get_owner(),
            self.get_repo(),
            path
        );

        let sha = self.get_sha(path).map(|sha| sha.clone());
        let response = RequestBuilder::new(url)
            .put()
            .access_token(self.get_access_token().clone())
            .accept_json()
            .json_body(&WriteFileRequest {
                message: "Write file".to_string(),
                content: encode(content),
                sha,
            })
            .send()
            .await;

        if !response.ok() {
            let status = response.status();
            let error = match status {
                404 => WriteFileError::FileNotFound,
                409 => WriteFileError::Conflict,
                422 => WriteFileError::ValidationFailed,
                _ => WriteFileError::ResponseNotOk(status),
            };
            return Err(error);
        }

        let response_body: WriteFileResponse = parse_response_as_json(response).await?;
        self.set_sha(
            response_body.content.path.clone(),
            response_body.content.sha.clone(),
        );
        Ok(Dirent::from(response_body.content))
    }
}

#[derive(Debug)]
pub enum WriteFileError {
    ResponseNotOk(u16),
    ResponseParseError(ResponseParseError),
    FileNotFound,
    Conflict,
    ValidationFailed,
}
impl From<ResponseParseError> for WriteFileError {
    fn from(error: ResponseParseError) -> Self {
        Self::ResponseParseError(error)
    }
}

#[derive(Serialize)]
struct WriteFileRequest {
    message: String,
    content: String,
    sha: Option<String>,
}

#[derive(Deserialize)]
struct WriteFileResponse {
    content: Content,
}
