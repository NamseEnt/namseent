use super::*;

#[derive(serde::Deserialize, Debug)]
pub struct PutRepositoryContentResponseBody {
    // pub content: Content,
    // pub commit: Commit,
}

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

// #[derive(serde::Deserialize, Debug)]
// pub struct Content {
//     pub name: String,
//     pub path: String,
//     pub sha: String,
//     pub size: u32,
//     pub r#type: String,
//     // pub url: String,
//     // pub html_url: String,
//     // pub git_url: String,
//     // pub download_url: String,
// }

// #[derive(serde::Deserialize, Debug)]
// pub struct Commit {
//     pub sha: String,
//     pub node_id: String,
//     pub url: String,
//     pub html_url: String,
// "author": {
//   "type": "object",
//   "properties": {
//     "date": {
//       "type": "string"
//     },
//     "name": {
//       "type": "string"
//     },
//     "email": {
//       "type": "string"
//     }
//   }
// },
// "committer": {
//   "type": "object",
//   "properties": {
//     "date": {
//       "type": "string"
//     },
//     "name": {
//       "type": "string"
//     },
//     "email": {
//       "type": "string"
//     }
//   }
// },
// "message": {
//   "type": "string"
// },
// "tree": {
//   "type": "object",
//   "properties": {
//     "url": {
//       "type": "string"
//     },
//     "sha": {
//       "type": "string"
//     }
//   }
// },
// "parents": {
//   "type": "array",
//   "items": {
//     "type": "object",
//     "properties": {
//       "url": {
//         "type": "string"
//       },
//       "html_url": {
//         "type": "string"
//       },
//       "sha": {
//         "type": "string"
//       }
//     }
//   }
// },
// "verification": {
//   "type": "object",
//   "properties": {
//     "verified": {
//       "type": "boolean"
//     },
//     "reason": {
//       "type": "string"
//     },
//     "signature": {
//       "type": [
//         "string",
//         "null"
//       ]
//     },
//     "payload": {
//       "type": [
//         "string",
//         "null"
//       ]
//     }
//   }
// }
// }
