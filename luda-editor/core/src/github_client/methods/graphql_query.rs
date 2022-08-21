use super::*;

impl GithubClient {
    #![cfg_attr(test, allow(dead_code))]
    pub async fn graphql_query<T: serde::de::DeserializeOwned>(
        &self,
        query: String,
    ) -> Result<T, GithubClientError> {
        let url = format!("{base_url}/graphql", base_url = self.base_url(),);

        #[derive(serde::Serialize)]
        struct Body {
            query: String,
        }

        let body = serde_json::to_vec(&Body { query })
            .map_err(|error| GithubClientError::BodySerializeError(error.into()))?;

        Ok(
            namui::network::http::fetch_json(&url, namui::network::http::Method::POST, |builder| {
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
