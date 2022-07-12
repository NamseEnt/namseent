use super::{
    parse_response_as_json::{parse_response_as_json, ResponseParseError},
    types::{Content, Dirent, RequestBuilder},
    GithubAPiClient,
};

impl GithubAPiClient {
    pub async fn read_dir(&self, path: &str) -> Result<Vec<Dirent>, ReadDirError> {
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
            return Err(ReadDirError::ResponseNotOk(response.status()));
        }

        let response_body: ReadDirResponse = parse_response_as_json(response).await?;
        Ok(response_body
            .into_iter()
            .map(|content| {
                self.set_sha(content.path.clone(), content.sha.clone());
                Dirent::from(content)
            })
            .collect())
    }
}

#[derive(Debug)]
pub enum ReadDirError {
    ResponseNotOk(u16),
    ResponseParseError(ResponseParseError),
}

impl From<ResponseParseError> for ReadDirError {
    fn from(error: ResponseParseError) -> Self {
        Self::ResponseParseError(error)
    }
}

type ReadDirResponse = Vec<Content>;
