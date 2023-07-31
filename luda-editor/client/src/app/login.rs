use anyhow::{anyhow, Result};
use namui::prelude::*;

const DEV_CLIENT_ID: &str = "abd04a6aeba3e99f5b4b";
const CLIENT_ID: Option<&str> = option_env!("GITHUB_CLIENT_ID");

pub async fn get_session_id() -> Result<Uuid> {
    let session_id = namui::cache::get_serde::<Uuid>("SessionId").await?;
    if let Some(session_id) = session_id {
        if is_session_id_valid(session_id).await? {
            return Ok(session_id);
        }
    }

    let code = request_github_auth_code().await?;
    let session_id = login_with_oauth_code(code).await?;
    Ok(session_id)
}

async fn is_session_id_valid(session_id: Uuid) -> Result<bool> {
    crate::RPC.set_session_id(session_id);
    match crate::RPC
        .validate_session(rpc::validate_session::Request {})
        .await
    {
        Ok(_) => Ok(true),
        Err(error) => match error {
            rpc::validate_session::Error::InvalidSession => Ok(false),
            rpc::validate_session::Error::Unknown(error) => Err(anyhow!("{:?}", error)),
        },
    }
}

async fn request_github_auth_code() -> Result<String> {
    let client_id = CLIENT_ID.unwrap_or(DEV_CLIENT_ID);

    let redirect_uri: String = namui::web::execute_function_sync(
        "
    return window.location.href;",
    )
    .run();

    let url = format!(
        "
        https://github.com/login/oauth/authorize
        ?client_id={client_id}
        &redirect_uri=
        https://sslwiheugl5ojmqlecerzhng740cekqc.lambda-url.ap-northeast-2.on.aws/{redirect_uri}
    "
    )
    .replace(" ", "")
    .replace("\n", "");

    let code: String = namui::web::execute_async_function(
        "
        const authCodeWindow = window.open(url);

        while (true) {
            await new Promise(resolve => setTimeout(resolve, 100));

            const query = authCodeWindow.location.search;
            if (query?.startsWith('?code=')) {
                const code = query.slice(6);
                authCodeWindow.close();
                return code;
            }
        }
        ",
    )
    .arg("url", &url)
    .run()
    .await;

    namui::log!("code: {}", code);

    Ok(code)
}
async fn login_with_oauth_code(code: String) -> Result<Uuid> {
    let response = crate::RPC
        .log_in_with_github_oauth_code(rpc::log_in_with_github_oauth_code::Request { code })
        .await?;

    let session_id = response.session_id;
    namui::cache::set_serde("SessionId", &session_id).await?;

    Ok(session_id)
}
