mod get_or_create_user;
mod impls;
mod user_identity;

use crate::{session::SessionDocument, storage::dynamo_db::Document};
use get_or_create_user::*;
use lambda_web::is_running_on_lambda;
use user_identity::*;

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
    fn exchange_google_auth_code_to_access_token<'a>(
        &'a self,
        _session: Option<SessionDocument>,
        _req: rpc::exchange_google_auth_code_to_access_token::Request,
    ) -> std::pin::Pin<
        Box<
            dyn 'a
                + std::future::Future<
                    Output = Result<
                        rpc::exchange_google_auth_code_to_access_token::Response,
                        rpc::exchange_google_auth_code_to_access_token::Error,
                    >,
                >
                + Send,
        >,
    > {
        todo!("https://namseent.zulipchat.com/#narrow/stream/332914-.EA.B8.B0.EC.88.A0.EC.97.B0.EA.B5.AC.EC.86.8C/topic/CRDT.20Lock-free.20Editor/near/295555692")
        // Box::pin(async move {
        //     // NOTE: https://developers.google.com/identity/protocols/oauth2/web-server#exchange-authorization-code

        //     let result = self.reqwest_client
        //         .request(Method::POST, "https://oauth2.googleapis.com/token")
        //         .header("Content-Type", "application/x-www-form-urlencoded")
        //         .header("Accept", "application/json")
        //         .body(format!("code={code}&client_id={client_id}&client_secret={client_secret}&redirect_uri={redirect_uri}&grant_type=authorization_code",
        //             code=req.code,
        //             client_id=self.google_client_setting.client_id,
        //             client_secret=self.google_client_setting.client_secret,
        //             redirect_uri=self.google_client_setting.redirect_uri,
        //         )).send().await;

        //     match result {
        //         Ok(response) => {
        //             if !response.status().is_success() {
        //                 return Err(rpc::exchange_google_auth_code_to_access_token::Error::Unknown(format!("{:?}", response.status())));
        //             }
        //             let body = response.text().await.unwrap();

        //             struct ResponseBody {
        //                 "access_token": "1/fFAGRNJru1FTz70BzhT3Zg",
        //                 "expires_in": 3920,
        //                 "token_type": "Bearer",
        //                 "scope": "https://www.googleapis.com/auth/drive.metadata.readonly",
        //                 "refresh_token": "1//xEoDL4iW3cxlI7yDbSRFYNG01kVKM2C-259HOF2aQbI"
        //             }

        //         },
        //         Err(error) => {
        //             eprintln!(
        //                 "Fail to exchange google auth code to access token: {}",
        //                 error
        //             );
        //             Err(
        //                 rpc::exchange_google_auth_code_to_access_token::Error::Unknown(
        //                     error.to_string(),
        //                 ),
        //             )
        //         }
        //     }
        // })
    }

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

            let github_user_id = self.get_github_user_id(access_token).await;
            if let Err(error) = github_user_id {
                return Err(rpc::log_in_with_github_oauth_code::Error::Unknown(
                    error.to_string(),
                ));
            }
            let github_user_id = github_user_id.unwrap();

            let user = get_or_create_user(UserIdentity::Github { github_user_id }).await;
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
}

#[derive(serde::Serialize, serde::Deserialize)]
struct IdentityDocument {
    pub id: String,
    pub user_id: String,
}

impl Document for IdentityDocument {
    fn partition_key_prefix() -> &'static str {
        "identity"
    }

    fn partition_key_without_prefix(&self) -> String {
        self.id.clone()
    }

    fn sort_key(&self) -> Option<&str> {
        None
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct UserDocument {
    pub id: String,
    // TODO: Add User Name
}

impl Document for UserDocument {
    fn partition_key_prefix() -> &'static str {
        "user"
    }

    fn partition_key_without_prefix(&self) -> String {
        self.id.clone()
    }

    fn sort_key(&self) -> Option<&str> {
        None
    }
}
