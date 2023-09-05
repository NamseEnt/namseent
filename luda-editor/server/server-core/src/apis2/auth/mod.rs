pub mod documents;
mod get_or_create_user;
mod impls;
mod user_identity;

use crate::session::SessionDocument;
pub use get_or_create_user::*;
use lambda_web::is_running_on_lambda;
pub use user_identity::*;

#[derive(Debug)]
pub struct AuthService {
    reqwest_client: reqwest::Client,
    // google_client_setting: GoogleAuthSetting,
    github_client_setting: GithubAuthSetting,
}

// #[derive(Debug)]
// pub struct GoogleAuthSetting {
//     client_id: String,
//     client_secret: String,
//     redirect_uri: String,
// }

#[derive(Debug)]
pub struct GithubAuthSetting {
    client_id: String,
    client_secret: String,
}

impl AuthService {
    pub fn new(// google_client_setting: GoogleAuthSetting,
    ) -> Self {
        let github_client_setting = {
            if is_running_on_lambda() {
                GithubAuthSetting {
                    client_id: std::env::var("GITHUB_CLIENT_ID")
                        .expect("Fail to get GITHUB_CLIENT_ID from Environment Variable"),
                    client_secret: std::env::var("GITHUB_CLIENT_SECRET")
                        .expect("Fail to get GITHUB_CLIENT_SECRET from Environment Variable"),
                }
            } else {
                GithubAuthSetting {
                    // NOTE: https://github.com/organizations/NamseEnt/settings/applications/1968967
                    // This is for testing. Do not use in production environment.
                    client_id: "abd04a6aeba3e99f5b4b".to_string(),
                    client_secret: "501a915ca627e24d2088cf01416fe836db470dba".to_string(),
                }
            }
        };

        AuthService {
            reqwest_client: reqwest::Client::new(),
            // google_client_setting,
            github_client_setting,
        }
    }
}

impl rpc::AuthService<SessionDocument> for AuthService {
    fn log_in_with_github_oauth_code<'a>(
        &'a self,
        session: Option<SessionDocument>,
        req: rpc::log_in_with_github_oauth_code::Request,
    ) -> std::pin::Pin<
        Box<
            dyn 'a
                + std::future::Future<
                    Output = Result<
                        rpc::log_in_with_github_oauth_code::Response,
                        rpc::log_in_with_github_oauth_code::Error,
                    >,
                >
                + Send,
        >,
    > {
        Box::pin(async move {
            if session.is_some() {
                return Err(rpc::log_in_with_github_oauth_code::Error::AlreadyLoggedIn);
            }

            let access_token = self
                .exchange_github_auth_code_to_access_token(req.code)
                .await;
            if let Err(error) = access_token {
                return Err(rpc::log_in_with_github_oauth_code::Error::Unknown(
                    error.to_string(),
                ));
            }
            let access_token = access_token.unwrap();

            let github_user = self.get_github_user(access_token).await;
            if let Err(error) = github_user {
                return Err(rpc::log_in_with_github_oauth_code::Error::Unknown(
                    error.to_string(),
                ));
            }
            let github_user = github_user.unwrap();

            let user = get_or_create_user(UserIdentity::Github {
                github_user_id: github_user.id,
                username: github_user.username,
            })
            .await;
            if let Err(error) = user {
                return Err(rpc::log_in_with_github_oauth_code::Error::Unknown(
                    error.to_string(),
                ));
            }
            let user = user.unwrap();

            let session = crate::session::create_session(user.id).await;
            if let Err(error) = session {
                return Err(rpc::log_in_with_github_oauth_code::Error::Unknown(
                    error.to_string(),
                ));
            }
            let session = session.unwrap();

            Ok(rpc::log_in_with_github_oauth_code::Response {
                session_id: session.id,
            })
        })
    }

    fn validate_session<'a>(
        &'a self,
        session: Option<SessionDocument>,
        _req: rpc::validate_session::Request,
    ) -> std::pin::Pin<
        Box<dyn 'a + std::future::Future<Output = rpc::validate_session::Result> + Send>,
    > {
        Box::pin(async move {
            match session {
                Some(_) => Ok(rpc::validate_session::Response {}),
                None => Err(rpc::validate_session::Error::InvalidSession),
            }
        })
    }
}
