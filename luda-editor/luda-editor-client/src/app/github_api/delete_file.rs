use super::{parse_response_as_json::ResponseParseError, types::RequestBuilder, GithubApiClient};
use serde::Serialize;

impl GithubApiClient {
    pub async fn delete_file(&self, path: &str) -> Result<(), DeleteFileError> {
        let url = format!(
            "{}/repos/{}/{}/contents/{}",
            self.get_base_url(),
            self.get_owner(),
            self.get_repo(),
            path
        );

        let sha = self.get_sha(path).map(|sha| sha.clone());
        let response = RequestBuilder::new(url)
            .delete()
            .access_token(self.get_access_token().clone())
            .accept_json()
            .json_body(&WriteFileRequest {
                message: "Delete file".to_string(),
                sha,
            })
            .send()
            .await;

        if !response.ok() {
            let status = response.status();
            let error = match status {
                404 => DeleteFileError::FileNotFound,
                409 => DeleteFileError::Conflict,
                422 => DeleteFileError::FailToGetLatestSha,
                _ => DeleteFileError::ResponseNotOk(status),
            };
            return Err(error);
        }

        self.remove_sha(path);
        Ok(())
    }
}

#[derive(Debug)]
pub enum DeleteFileError {
    ResponseNotOk(u16),
    ResponseParseError(ResponseParseError),
    FileNotFound,
    Conflict,
    FailToGetLatestSha,
}
impl From<ResponseParseError> for DeleteFileError {
    fn from(error: ResponseParseError) -> Self {
        Self::ResponseParseError(error)
    }
}

#[derive(Serialize)]
struct WriteFileRequest {
    message: String,
    sha: Option<String>,
}
