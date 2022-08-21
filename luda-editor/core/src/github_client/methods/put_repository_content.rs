use super::*;

#[derive(serde::Deserialize, Debug)]
pub struct PutRepositoryContentResponseBody;

impl GithubClient {
    #![cfg_attr(test, allow(dead_code))]
    pub async fn put_repository_content<'a>(
        &self,
        branch: &str,
        path: &str,
        sha: Option<&'a str>,
        base64_content: &str,
        commit_message: &str,
    ) -> Result<PutRepositoryContentResponseBody, GithubClientError> {
        let url = format!(
            "{base_url}/repos/{owner}/{repo}/contents/{path}",
            base_url = self.base_url(),
            owner = self.owner(),
            repo = self.repo(),
        );

        #[derive(serde::Serialize)]
        struct Body<'a> {
            message: &'a str,
            content: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            sha: Option<&'a str>,
            branch: &'a str,
        }

        let body = serde_json::to_vec(&Body {
            message: commit_message,
            content: base64_content,
            sha,
            branch,
        })
        .map_err(|error| GithubClientError::BodySerializeError(error.into()))?;

        Ok(
            namui::network::http::fetch_json(&url, namui::network::http::Method::PUT, |builder| {
                builder
                    .header("Authorization", format!("token {}", self.access_token()))
                    .header("Accept", "application/vnd.github+json")
                    .header("User-Agent", "luda-editor")
                    .fetch_credentials_include()
                    .body(body)
            })
            .await?,
        )
    }
}
