#[path = "pages/builds/mod.rs"]
mod pages_builds;
#[path = "pages/index/mod.rs"]
mod pages_index;
#[allow(non_snake_case)]
#[path = "pages/issues/[hash]/mod.rs"]
mod pages_issues__hash_;
#[path = "pages/login/mod.rs"]
mod pages_login;
#[path = "pages/oauth/cli/authorize/mod.rs"]
mod pages_oauth_cli_authorize;
#[path = "pages/oauth/github/callback/mod.rs"]
mod pages_oauth_github_callback;
#[path = "pages/tokens/mod.rs"]
mod pages_tokens;
use forte_sdk::anyhow::Result;
use forte_sdk::http::{HeaderMap, Request, Response, StatusCode, body::Body};
use forte_sdk::http_header::{COOKIE, LOCATION, SET_COOKIE};
use forte_sdk::*;
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub enum Redirect {
    External { url: String },
    OauthCliAuthorize,
    OauthGithubCallback,
    Builds,
    Index,
    Issues_hash_ { hash: String },
    Tokens,
    Login,
}
impl Redirect {
    pub fn to_path(&self) -> String {
        match self {
            Redirect::External { url } => url.clone(),
            Redirect::OauthCliAuthorize => "/oauth/cli/authorize".to_string(),
            Redirect::OauthGithubCallback => "/oauth/github/callback".to_string(),
            Redirect::Builds => "/builds".to_string(),
            Redirect::Index => "/".to_string(),
            Redirect::Issues_hash_ { hash } => {
                format!("/{}", ["issues".to_string(), hash.to_string()].join("/"))
            }
            Redirect::Tokens => "/tokens".to_string(),
            Redirect::Login => "/login".to_string(),
        }
    }
}
impl std::fmt::Display for Redirect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Redirect to {}", self.to_path())
    }
}
impl std::error::Error for Redirect {}
#[allow(clippy::crate_in_macro_def)]
mod proxy {
    forte_sdk::wit_bindgen::generate!(
        { inline :
        "package forte:user; world service-export { import wasi:http/types@0.3.0-rc-2026-03-15; export wasi:http/handler@0.3.0-rc-2026-03-15; }",
        path :
        "/Users/namse/namseent/namsh/rs/target/wasm32-wasip2/release/build/namsh-b97b353f6635a933/out",
        world : "service-export", default_bindings_module :
        "crate::route_generated::proxy", pub_export_macro : true, async : true, features
        : ["clocks-timezone"], with : { "wasi:http/handler@0.3.0-rc-2026-03-15" :
        generate, "wasi:http/types@0.3.0-rc-2026-03-15" :
        forte_sdk::bindings::wasi::http::types, "wasi:clocks/types@0.3.0-rc-2026-03-15" :
        forte_sdk::bindings::wasi::clocks::types, }, runtime_path :
        "forte_sdk::wit_bindgen::rt", }
    );
}
struct Server;
impl proxy::exports::wasi::http::handler::Guest for Server {
    async fn handle(
        req: forte_sdk::bindings::wasi::http::types::Request,
    ) -> core::result::Result<
        forte_sdk::bindings::wasi::http::types::Response,
        forte_sdk::bindings::wasi::http::types::ErrorCode,
    > {
        forte_sdk::serve::serve(req, |request| async move { dispatch(request).await }).await
    }
}
proxy::export!(Server);
async fn dispatch(request: Request<Vec<u8>>) -> Result<Response<Body>> {
    let path_for_route = request.uri().path().to_string();
    let key = classify_route(&path_for_route);
    let result = dispatch_inner(request).await;
    result.map(|mut resp| {
        if let Ok(v) = forte_sdk::http::HeaderValue::from_str(&key) {
            resp.headers_mut()
                .insert("x-fn0-execution-time-metric-key", v);
        }
        resp
    })
}
fn classify_route(path: &str) -> String {
    if path.strip_prefix("/__forte_action/").is_some() {
        return "/__forte_action/[name]".to_string();
    }
    if path.strip_prefix("/__forte_admin/").is_some() {
        return "/__forte_admin/[name]".to_string();
    }
    if path == "/__fn0_queue_task/execute" {
        return "/__fn0_queue_task/execute".to_string();
    }
    if path.strip_prefix("/__self_invoke/").is_some() {
        return "/__self_invoke/[name]".to_string();
    }
    let path_segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();
    if path == "/oauth/cli/authorize" {
        return "/oauth/cli/authorize".to_string();
    }
    if path == "/oauth/github/callback" {
        return "/oauth/github/callback".to_string();
    }
    if path == "/builds" {
        return "/builds".to_string();
    }
    if path == "/" {
        return "/".to_string();
    }
    if path_segments.len() == 2usize && path_segments[0usize] == "issues" {
        return "/issues/[hash]".to_string();
    }
    if path == "/tokens" {
        return "/tokens".to_string();
    }
    if path == "/login" {
        return "/login".to_string();
    }
    "unknown".to_string()
}
async fn dispatch_inner(request: Request<Vec<u8>>) -> Result<Response<Body>> {
    let (parts, body_bytes) = request.into_parts();
    let headers = parts.headers;
    let path = parts.uri.path().to_string();
    let method = parts.method;
    let mut cookie_jar = make_cookie_jar(&headers);
    let Some(uri_authority) = parts.uri.authority() else {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("Missing authority in request URI"))
            .unwrap());
    };
    let uri_authority = uri_authority.as_str();
    let path = path.as_str();
    if let Some(hook_name) = path.strip_prefix("/__self_invoke/") {
        return handle_hook(
            hook_name,
            uri_authority,
            &method,
            &headers,
            &mut cookie_jar,
            &body_bytes,
        )
        .await;
    }
    if let Some(action_name) = path.strip_prefix("/__forte_action/") {
        return handle_action(
            action_name,
            uri_authority,
            &method,
            &headers,
            &mut cookie_jar,
            &body_bytes,
        )
        .await;
    }
    if path == "/__fn0_queue_task/execute" {
        return handle_queue_task_execute(&body_bytes).await;
    }
    if let Some(task_name) = path.strip_prefix("/__forte_admin/") {
        return handle_admin_task(task_name, &headers, &body_bytes).await;
    }
    let path_segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();
    if path == "/oauth/cli/authorize" {
        use std::collections::HashMap;
        let query = parts.uri.query().unwrap_or("");
        let query_params: HashMap<String, String> =
            forte_sdk::form_urlencoded::parse(query.as_bytes())
                .map(|(k, v)| (k.into_owned(), v.into_owned()))
                .collect();
        let Some(redirect_uri) = query_params.get("redirect_uri").cloned() else {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(format!(
                    "Missing required query parameter: {}",
                    "redirect_uri"
                )))
                .unwrap());
        };
        let Some(code_challenge) = query_params.get("code_challenge").cloned() else {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(format!(
                    "Missing required query parameter: {}",
                    "code_challenge"
                )))
                .unwrap());
        };
        let Some(code_challenge_method) = query_params.get("code_challenge_method").cloned() else {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(format!(
                    "Missing required query parameter: {}",
                    "code_challenge_method"
                )))
                .unwrap());
        };
        let Some(state) = query_params.get("state").cloned() else {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(format!(
                    "Missing required query parameter: {}",
                    "state"
                )))
                .unwrap());
        };
        let Some(label) = query_params.get("label").cloned() else {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(format!(
                    "Missing required query parameter: {}",
                    "label"
                )))
                .unwrap());
        };
        let search_params = pages_oauth_cli_authorize::SearchParams {
            redirect_uri,
            code_challenge,
            code_challenge_method,
            state,
            label,
        };
        let req = ForteRequest {
            uri_authority,
            method: &method,
            headers: &headers,
            jar: &mut cookie_jar,
            raw_body: &body_bytes,
            body: (),
        };
        match pages_oauth_cli_authorize::handler(req, search_params).await {
            Ok(props) => {
                let body_bytes = forte_json::to_vec(&props);
                Ok(build_response_with_cookies(
                    Response::builder()
                        .status(StatusCode::OK)
                        .header("x-fn0-next", "js")
                        .body(Body::from(body_bytes))
                        .unwrap(),
                    &cookie_jar,
                ))
            }
            Err(e) => {
                if let Some(redirect) = e.downcast_ref::<Redirect>() {
                    Ok(build_response_with_cookies(
                        Response::builder()
                            .status(StatusCode::FOUND)
                            .header(LOCATION, redirect.to_path())
                            .body(Body::empty())
                            .unwrap(),
                        &cookie_jar,
                    ))
                } else {
                    eprintln!("Error at {}: {:?}", path, e);
                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from("Internal Server Error"))
                        .unwrap())
                }
            }
        }
    } else if path == "/oauth/github/callback" {
        use std::collections::HashMap;
        let query = parts.uri.query().unwrap_or("");
        let query_params: HashMap<String, String> =
            forte_sdk::form_urlencoded::parse(query.as_bytes())
                .map(|(k, v)| (k.into_owned(), v.into_owned()))
                .collect();
        let Some(code) = query_params.get("code").cloned() else {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(format!(
                    "Missing required query parameter: {}",
                    "code"
                )))
                .unwrap());
        };
        let Some(state) = query_params.get("state").cloned() else {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(format!(
                    "Missing required query parameter: {}",
                    "state"
                )))
                .unwrap());
        };
        let search_params = pages_oauth_github_callback::SearchParams { code, state };
        let req = ForteRequest {
            uri_authority,
            method: &method,
            headers: &headers,
            jar: &mut cookie_jar,
            raw_body: &body_bytes,
            body: (),
        };
        match pages_oauth_github_callback::handler(req, search_params).await {
            Ok(redirect) => Ok(build_response_with_cookies(
                Response::builder()
                    .status(StatusCode::FOUND)
                    .header(LOCATION, redirect.to_path())
                    .body(Body::empty())
                    .unwrap(),
                &cookie_jar,
            )),
            Err(e) => {
                eprintln!("Error at {}: {:?}", path, e);
                Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("Internal Server Error"))
                    .unwrap())
            }
        }
    } else if path == "/builds" {
        let req = ForteRequest {
            uri_authority,
            method: &method,
            headers: &headers,
            jar: &mut cookie_jar,
            raw_body: &body_bytes,
            body: (),
        };
        match pages_builds::handler(req).await {
            Ok(props) => {
                let body_bytes = forte_json::to_vec(&props);
                Ok(build_response_with_cookies(
                    Response::builder()
                        .status(StatusCode::OK)
                        .header("x-fn0-next", "js")
                        .body(Body::from(body_bytes))
                        .unwrap(),
                    &cookie_jar,
                ))
            }
            Err(e) => {
                if let Some(redirect) = e.downcast_ref::<Redirect>() {
                    Ok(build_response_with_cookies(
                        Response::builder()
                            .status(StatusCode::FOUND)
                            .header(LOCATION, redirect.to_path())
                            .body(Body::empty())
                            .unwrap(),
                        &cookie_jar,
                    ))
                } else {
                    eprintln!("Error at {}: {:?}", path, e);
                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from("Internal Server Error"))
                        .unwrap())
                }
            }
        }
    } else if path == "/" {
        let req = ForteRequest {
            uri_authority,
            method: &method,
            headers: &headers,
            jar: &mut cookie_jar,
            raw_body: &body_bytes,
            body: (),
        };
        match pages_index::handler(req).await {
            Ok(props) => {
                let body_bytes = forte_json::to_vec(&props);
                Ok(build_response_with_cookies(
                    Response::builder()
                        .status(StatusCode::OK)
                        .header("x-fn0-next", "js")
                        .body(Body::from(body_bytes))
                        .unwrap(),
                    &cookie_jar,
                ))
            }
            Err(e) => {
                if let Some(redirect) = e.downcast_ref::<Redirect>() {
                    Ok(build_response_with_cookies(
                        Response::builder()
                            .status(StatusCode::FOUND)
                            .header(LOCATION, redirect.to_path())
                            .body(Body::empty())
                            .unwrap(),
                        &cookie_jar,
                    ))
                } else {
                    eprintln!("Error at {}: {:?}", path, e);
                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from("Internal Server Error"))
                        .unwrap())
                }
            }
        }
    } else if path_segments.len() == 2usize && path_segments.first() == Some(&"issues") {
        let hash: String = path_segments[1usize].to_string();
        let path_params = pages_issues__hash_::PathParams { hash };
        let req = ForteRequest {
            uri_authority,
            method: &method,
            headers: &headers,
            jar: &mut cookie_jar,
            raw_body: &body_bytes,
            body: (),
        };
        match pages_issues__hash_::handler(req, path_params).await {
            Ok(props) => {
                let body_bytes = forte_json::to_vec(&props);
                Ok(build_response_with_cookies(
                    Response::builder()
                        .status(StatusCode::OK)
                        .header("x-fn0-next", "js")
                        .body(Body::from(body_bytes))
                        .unwrap(),
                    &cookie_jar,
                ))
            }
            Err(e) => {
                if let Some(redirect) = e.downcast_ref::<Redirect>() {
                    Ok(build_response_with_cookies(
                        Response::builder()
                            .status(StatusCode::FOUND)
                            .header(LOCATION, redirect.to_path())
                            .body(Body::empty())
                            .unwrap(),
                        &cookie_jar,
                    ))
                } else {
                    eprintln!("Error at {}: {:?}", path, e);
                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from("Internal Server Error"))
                        .unwrap())
                }
            }
        }
    } else if path == "/tokens" {
        let req = ForteRequest {
            uri_authority,
            method: &method,
            headers: &headers,
            jar: &mut cookie_jar,
            raw_body: &body_bytes,
            body: (),
        };
        match pages_tokens::handler(req).await {
            Ok(props) => {
                let body_bytes = forte_json::to_vec(&props);
                Ok(build_response_with_cookies(
                    Response::builder()
                        .status(StatusCode::OK)
                        .header("x-fn0-next", "js")
                        .body(Body::from(body_bytes))
                        .unwrap(),
                    &cookie_jar,
                ))
            }
            Err(e) => {
                if let Some(redirect) = e.downcast_ref::<Redirect>() {
                    Ok(build_response_with_cookies(
                        Response::builder()
                            .status(StatusCode::FOUND)
                            .header(LOCATION, redirect.to_path())
                            .body(Body::empty())
                            .unwrap(),
                        &cookie_jar,
                    ))
                } else {
                    eprintln!("Error at {}: {:?}", path, e);
                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from("Internal Server Error"))
                        .unwrap())
                }
            }
        }
    } else if path == "/login" {
        let req = ForteRequest {
            uri_authority,
            method: &method,
            headers: &headers,
            jar: &mut cookie_jar,
            raw_body: &body_bytes,
            body: (),
        };
        match pages_login::handler(req).await {
            Ok(redirect) => Ok(build_response_with_cookies(
                Response::builder()
                    .status(StatusCode::FOUND)
                    .header(LOCATION, redirect.to_path())
                    .body(Body::empty())
                    .unwrap(),
                &cookie_jar,
            )),
            Err(e) => {
                eprintln!("Error at {}: {:?}", path, e);
                Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("Internal Server Error"))
                    .unwrap())
            }
        }
    } else {
        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap())
    }
}
async fn handle_hook(
    hook_name: &str,
    _uri_authority: &str,
    _method: &http::Method,
    _headers: &HeaderMap,
    _cookie_jar: &mut cookie::CookieJar,
    _body_bytes: &[u8],
) -> Result<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from(format!("Hook '{}' not found", hook_name)))
        .unwrap())
}
#[forte_sdk::tracing::instrument(
    name = "handle_action",
    skip_all,
    fields(action = action_name),
)]
async fn handle_action(
    action_name: &str,
    uri_authority: &str,
    method: &http::Method,
    headers: &HeaderMap,
    cookie_jar: &mut cookie::CookieJar,
    body_bytes: &[u8],
) -> Result<Response<Body>> {
    match action_name {
        "revoke_token" => {
            let input: crate::actions::revoke_token::Input =
                match forte_json::from_slice(body_bytes) {
                    Ok(v) => v,
                    Err(e) => {
                        return Ok(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Body::from(format!("invalid request body: {}", e)))
                            .unwrap());
                    }
                };
            let req = ForteRequest {
                uri_authority,
                method,
                headers,
                jar: cookie_jar,
                raw_body: body_bytes,
                body: input,
            };
            let output = crate::actions::revoke_token::handler(req).await;
            let json = forte_json::to_vec(&output);
            Ok(build_response_with_cookies(
                Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "application/json")
                    .body(Body::from(json))
                    .unwrap(),
                cookie_jar,
            ))
        }
        "request_pdb_download" => {
            let input: crate::actions::request_pdb_download::Input =
                match forte_json::from_slice(body_bytes) {
                    Ok(v) => v,
                    Err(e) => {
                        return Ok(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Body::from(format!("invalid request body: {}", e)))
                            .unwrap());
                    }
                };
            let req = ForteRequest {
                uri_authority,
                method,
                headers,
                jar: cookie_jar,
                raw_body: body_bytes,
                body: input,
            };
            let output = crate::actions::request_pdb_download::handler(req).await;
            let json = forte_json::to_vec(&output);
            Ok(build_response_with_cookies(
                Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "application/json")
                    .body(Body::from(json))
                    .unwrap(),
                cookie_jar,
            ))
        }
        "intake_crash" => {
            let input: crate::actions::intake_crash::Input =
                match forte_json::from_slice(body_bytes) {
                    Ok(v) => v,
                    Err(e) => {
                        return Ok(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Body::from(format!("invalid request body: {}", e)))
                            .unwrap());
                    }
                };
            let req = ForteRequest {
                uri_authority,
                method,
                headers,
                jar: cookie_jar,
                raw_body: body_bytes,
                body: input,
            };
            let output = crate::actions::intake_crash::handler(req).await;
            let json = forte_json::to_vec(&output);
            Ok(build_response_with_cookies(
                Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "application/json")
                    .body(Body::from(json))
                    .unwrap(),
                cookie_jar,
            ))
        }
        "add_user" => {
            let input: crate::actions::add_user::Input = match forte_json::from_slice(body_bytes) {
                Ok(v) => v,
                Err(e) => {
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from(format!("invalid request body: {}", e)))
                        .unwrap());
                }
            };
            let req = ForteRequest {
                uri_authority,
                method,
                headers,
                jar: cookie_jar,
                raw_body: body_bytes,
                body: input,
            };
            let output = crate::actions::add_user::handler(req).await;
            let json = forte_json::to_vec(&output);
            Ok(build_response_with_cookies(
                Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "application/json")
                    .body(Body::from(json))
                    .unwrap(),
                cookie_jar,
            ))
        }
        "remove_user" => {
            let input: crate::actions::remove_user::Input = match forte_json::from_slice(body_bytes)
            {
                Ok(v) => v,
                Err(e) => {
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from(format!("invalid request body: {}", e)))
                        .unwrap());
                }
            };
            let req = ForteRequest {
                uri_authority,
                method,
                headers,
                jar: cookie_jar,
                raw_body: body_bytes,
                body: input,
            };
            let output = crate::actions::remove_user::handler(req).await;
            let json = forte_json::to_vec(&output);
            Ok(build_response_with_cookies(
                Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "application/json")
                    .body(Body::from(json))
                    .unwrap(),
                cookie_jar,
            ))
        }
        "list_stack_groups" => {
            let input: crate::actions::list_stack_groups::Input =
                match forte_json::from_slice(body_bytes) {
                    Ok(v) => v,
                    Err(e) => {
                        return Ok(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Body::from(format!("invalid request body: {}", e)))
                            .unwrap());
                    }
                };
            let req = ForteRequest {
                uri_authority,
                method,
                headers,
                jar: cookie_jar,
                raw_body: body_bytes,
                body: input,
            };
            let output = crate::actions::list_stack_groups::handler(req).await;
            let json = forte_json::to_vec(&output);
            Ok(build_response_with_cookies(
                Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "application/json")
                    .body(Body::from(json))
                    .unwrap(),
                cookie_jar,
            ))
        }
        "request_dump_download" => {
            let input: crate::actions::request_dump_download::Input =
                match forte_json::from_slice(body_bytes) {
                    Ok(v) => v,
                    Err(e) => {
                        return Ok(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Body::from(format!("invalid request body: {}", e)))
                            .unwrap());
                    }
                };
            let req = ForteRequest {
                uri_authority,
                method,
                headers,
                jar: cookie_jar,
                raw_body: body_bytes,
                body: input,
            };
            let output = crate::actions::request_dump_download::handler(req).await;
            let json = forte_json::to_vec(&output);
            Ok(build_response_with_cookies(
                Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "application/json")
                    .body(Body::from(json))
                    .unwrap(),
                cookie_jar,
            ))
        }
        "approve_cli_authorization" => {
            let input: crate::actions::approve_cli_authorization::Input =
                match forte_json::from_slice(body_bytes) {
                    Ok(v) => v,
                    Err(e) => {
                        return Ok(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Body::from(format!("invalid request body: {}", e)))
                            .unwrap());
                    }
                };
            let req = ForteRequest {
                uri_authority,
                method,
                headers,
                jar: cookie_jar,
                raw_body: body_bytes,
                body: input,
            };
            let output = crate::actions::approve_cli_authorization::handler(req).await;
            let json = forte_json::to_vec(&output);
            Ok(build_response_with_cookies(
                Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "application/json")
                    .body(Body::from(json))
                    .unwrap(),
                cookie_jar,
            ))
        }
        "oauth_cli_exchange" => {
            let input: crate::actions::oauth_cli_exchange::Input =
                match forte_json::from_slice(body_bytes) {
                    Ok(v) => v,
                    Err(e) => {
                        return Ok(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Body::from(format!("invalid request body: {}", e)))
                            .unwrap());
                    }
                };
            let req = ForteRequest {
                uri_authority,
                method,
                headers,
                jar: cookie_jar,
                raw_body: body_bytes,
                body: input,
            };
            let output = crate::actions::oauth_cli_exchange::handler(req).await;
            let json = forte_json::to_vec(&output);
            Ok(build_response_with_cookies(
                Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "application/json")
                    .body(Body::from(json))
                    .unwrap(),
                cookie_jar,
            ))
        }
        "list_users" => {
            let input: crate::actions::list_users::Input = match forte_json::from_slice(body_bytes)
            {
                Ok(v) => v,
                Err(e) => {
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from(format!("invalid request body: {}", e)))
                        .unwrap());
                }
            };
            let req = ForteRequest {
                uri_authority,
                method,
                headers,
                jar: cookie_jar,
                raw_body: body_bytes,
                body: input,
            };
            let output = crate::actions::list_users::handler(req).await;
            let json = forte_json::to_vec(&output);
            Ok(build_response_with_cookies(
                Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "application/json")
                    .body(Body::from(json))
                    .unwrap(),
                cookie_jar,
            ))
        }
        "request_pdb_upload" => {
            let input: crate::actions::request_pdb_upload::Input =
                match forte_json::from_slice(body_bytes) {
                    Ok(v) => v,
                    Err(e) => {
                        return Ok(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Body::from(format!("invalid request body: {}", e)))
                            .unwrap());
                    }
                };
            let req = ForteRequest {
                uri_authority,
                method,
                headers,
                jar: cookie_jar,
                raw_body: body_bytes,
                body: input,
            };
            let output = crate::actions::request_pdb_upload::handler(req).await;
            let json = forte_json::to_vec(&output);
            Ok(build_response_with_cookies(
                Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "application/json")
                    .body(Body::from(json))
                    .unwrap(),
                cookie_jar,
            ))
        }
        "list_tokens" => {
            let input: crate::actions::list_tokens::Input = match forte_json::from_slice(body_bytes)
            {
                Ok(v) => v,
                Err(e) => {
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from(format!("invalid request body: {}", e)))
                        .unwrap());
                }
            };
            let req = ForteRequest {
                uri_authority,
                method,
                headers,
                jar: cookie_jar,
                raw_body: body_bytes,
                body: input,
            };
            let output = crate::actions::list_tokens::handler(req).await;
            let json = forte_json::to_vec(&output);
            Ok(build_response_with_cookies(
                Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "application/json")
                    .body(Body::from(json))
                    .unwrap(),
                cookie_jar,
            ))
        }
        "issue_token" => {
            let input: crate::actions::issue_token::Input = match forte_json::from_slice(body_bytes)
            {
                Ok(v) => v,
                Err(e) => {
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from(format!("invalid request body: {}", e)))
                        .unwrap());
                }
            };
            let req = ForteRequest {
                uri_authority,
                method,
                headers,
                jar: cookie_jar,
                raw_body: body_bytes,
                body: input,
            };
            let output = crate::actions::issue_token::handler(req).await;
            let json = forte_json::to_vec(&output);
            Ok(build_response_with_cookies(
                Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "application/json")
                    .body(Body::from(json))
                    .unwrap(),
                cookie_jar,
            ))
        }
        "get_stack_group" => {
            let input: crate::actions::get_stack_group::Input =
                match forte_json::from_slice(body_bytes) {
                    Ok(v) => v,
                    Err(e) => {
                        return Ok(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Body::from(format!("invalid request body: {}", e)))
                            .unwrap());
                    }
                };
            let req = ForteRequest {
                uri_authority,
                method,
                headers,
                jar: cookie_jar,
                raw_body: body_bytes,
                body: input,
            };
            let output = crate::actions::get_stack_group::handler(req).await;
            let json = forte_json::to_vec(&output);
            Ok(build_response_with_cookies(
                Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "application/json")
                    .body(Body::from(json))
                    .unwrap(),
                cookie_jar,
            ))
        }
        "list_builds" => {
            let input: crate::actions::list_builds::Input = match forte_json::from_slice(body_bytes)
            {
                Ok(v) => v,
                Err(e) => {
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from(format!("invalid request body: {}", e)))
                        .unwrap());
                }
            };
            let req = ForteRequest {
                uri_authority,
                method,
                headers,
                jar: cookie_jar,
                raw_body: body_bytes,
                body: input,
            };
            let output = crate::actions::list_builds::handler(req).await;
            let json = forte_json::to_vec(&output);
            Ok(build_response_with_cookies(
                Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "application/json")
                    .body(Body::from(json))
                    .unwrap(),
                cookie_jar,
            ))
        }
        "confirm_pdb_uploaded" => {
            let input: crate::actions::confirm_pdb_uploaded::Input =
                match forte_json::from_slice(body_bytes) {
                    Ok(v) => v,
                    Err(e) => {
                        return Ok(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Body::from(format!("invalid request body: {}", e)))
                            .unwrap());
                    }
                };
            let req = ForteRequest {
                uri_authority,
                method,
                headers,
                jar: cookie_jar,
                raw_body: body_bytes,
                body: input,
            };
            let output = crate::actions::confirm_pdb_uploaded::handler(req).await;
            let json = forte_json::to_vec(&output);
            Ok(build_response_with_cookies(
                Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "application/json")
                    .body(Body::from(json))
                    .unwrap(),
                cookie_jar,
            ))
        }
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(format!("Action '{}' not found", action_name)))
            .unwrap()),
    }
}
async fn handle_queue_task_execute(_body_bytes: &[u8]) -> Result<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("No queue tasks defined"))
        .unwrap())
}
async fn handle_admin_task(
    _task_name: &str,
    _headers: &HeaderMap,
    _body_bytes: &[u8],
) -> Result<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("No admin tasks defined"))
        .unwrap())
}
fn make_cookie_jar(headers: &HeaderMap) -> cookie::CookieJar {
    let mut jar = cookie::CookieJar::new();
    let Some(cookie) = headers.get(COOKIE) else {
        return jar;
    };
    let Ok(cookie_str) = cookie.to_str() else {
        return jar;
    };
    for cookie in cookie::Cookie::split_parse_encoded(cookie_str) {
        let Ok(cookie) = cookie else { continue };
        jar.add_original(cookie.into_owned());
    }
    jar
}
fn build_response_with_cookies(
    mut response: Response<Body>,
    cookie_jar: &cookie::CookieJar,
) -> Response<Body> {
    for cookie in cookie_jar.delta() {
        if let Ok(value) = cookie.encoded().to_string().parse() {
            response.headers_mut().append(SET_COOKIE, value);
        }
    }
    response
}
