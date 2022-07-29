mod methods;
mod types;

pub use methods::*;
pub use types::*;

pub struct GithubClient {
    access_token: String,
    base_url: String,
    owner: String,
    repo: String,
}

impl GithubClient {
    #![cfg_attr(test, allow(dead_code))]
    pub fn new(access_token: String, base_url: String, owner: String, repo: String) -> Self {
        Self {
            access_token,
            base_url,
            repo,
            owner,
        }
    }

    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn repo(&self) -> &str {
        &self.repo
    }
}

mockall::mock! {
    pub GithubClient {
        pub async fn get_repository_content(
            &self,
            branch: &str,
            path: &str,
        ) -> Result<crate::github_client::GetRepositoryContentResponseBody, GithubClientError>;
        pub async fn get_repository_content_raw(
            &self,
            branch: &str,
            path: &str,
        ) -> Result<Box<[u8]>, GithubClientError>;
        pub async fn put_repository_content<'a>(
            &self,
            branch: &str,
            path: &str,
            sha: Option<&'a str>,
            base64_content: &str,
            commit_message: &str,
        ) -> Result<PutRepositoryContentResponseBody, GithubClientError>;
        pub async fn graphql_query<T: serde::de::DeserializeOwned + 'static>(
            &self,
            query: String,
        ) -> Result<T, GithubClientError>;
        pub fn owner(&self) -> &str;
        pub fn repo(&self) -> &str;
    }
}
