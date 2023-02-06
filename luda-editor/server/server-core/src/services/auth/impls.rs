use super::*;

impl AuthService {
    pub async fn exchange_github_auth_code_to_access_token(
        &self,
        code: String,
    ) -> Result<String, String> {
        #[derive(serde::Serialize)]
        struct RequestBody {
            client_id: String,
            client_secret: String,
            code: String,
        }

        #[derive(serde::Deserialize)]
        struct ResponseBody {
            access_token: String,
        }

        let body = serde_json::to_vec(&RequestBody {
            client_id: self.github_client_setting.client_id.clone(),
            client_secret: self.github_client_setting.client_secret.clone(),
            code,
        })
        .unwrap();

        let result = self
            .reqwest_client
            .post("https://github.com/login/oauth/access_token")
            .body(body)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("User-Agent", "luda-editor")
            .send()
            .await;

        match result {
            Ok(response) => {
                if response.status().is_success() {
                    let body = response.text().await.unwrap();
                    let response: ResponseBody = serde_json::from_str(&body).unwrap();
                    Ok(response.access_token)
                } else {
                    Err(format!(
                        "exchange_github_auth_code_to_access_token failed: {:?}\n{body}",
                        response.status(),
                        body = response.text().await.unwrap()
                    ))
                }
            }
            Err(error) => Err(error.to_string()),
        }
    }
    pub async fn get_github_user(&self, github_access_token: String) -> Result<GithubUser, String> {
        let result = self
            .reqwest_client
            .get("https://api.github.com/user")
            .header("Authorization", format!("token {github_access_token}"))
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", "luda-editor")
            .send()
            .await;

        #[derive(serde::Deserialize)]
        struct ResponseBody {
            id: u128,
            login: String,
        }
        match result {
            Ok(response) => {
                if response.status().is_success() {
                    let body = response.text().await.unwrap();
                    let response: ResponseBody = serde_json::from_str(&body).unwrap();
                    Ok(GithubUser {
                        id: response.id.to_string(),
                        username: response.login,
                    })
                } else {
                    Err(format!(
                        "get_github_user_id failed: {:?}\n{body}",
                        response.status(),
                        body = response.text().await.unwrap()
                    )
                    .into())
                }
            }
            Err(error) => Err(error.to_string()),
        }
    }
}

pub struct GithubUser {
    pub id: String,
    pub username: String,
}
