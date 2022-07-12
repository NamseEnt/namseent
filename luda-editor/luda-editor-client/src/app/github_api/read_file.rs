use super::{
    parse_response_as_json::{parse_response_as_json, ResponseParseError},
    types::{Content, Dirent, RequestBuilder},
    GithubAPiClient,
};

impl GithubAPiClient {
    pub async fn read_file(&self, path: &str) -> Result<Dirent, ReadFileError> {
        let url = format!(
            "{}/repos/{}/{}/contents/{}",
            self.get_base_url(),
            self.get_owner(),
            self.get_repo(),
            path
        );

        let response = RequestBuilder::new(url)
            .access_token(self.get_access_token().clone())
            .accept_json()
            .send()
            .await;

        if !response.ok() {
            return Err(ReadFileError::ResponseNotOk(response.status()));
        }

        let response_body: ReadFileResponse = parse_response_as_json(response).await?;
        self.set_sha(response_body.path.clone(), response_body.sha.clone());
        Ok(Dirent::from(response_body))
    }
}

#[derive(Debug)]
pub enum ReadFileError {
    ResponseNotOk(u16),
    ResponseParseError(ResponseParseError),
}

impl From<ResponseParseError> for ReadFileError {
    fn from(error: ResponseParseError) -> Self {
        Self::ResponseParseError(error)
    }
}

type ReadFileResponse = Content;
