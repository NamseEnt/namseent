// SPDX-License-Identifier: MIT
//! Run Actix Web on AWS Lambda
//!
//!
use crate::request::LambdaHttpEvent;
use core::convert::TryFrom;
use core::future::Future;
use lambda_runtime::{Error as LambdaError, LambdaEvent, Service as LambdaService};
use std::pin::Pin;

/// Run Actix web application on AWS Lambda
///
/// ```no_run
/// use lambda_web::actix_web::{self, get, App, HttpServer, Responder};
/// use lambda_web::{is_running_on_lambda, run_actix_on_lambda, LambdaError};
///
/// #[get("/")]
/// async fn hello() -> impl Responder {
///     format!("Hello")
/// }
///
/// #[actix_web::main]
/// async fn main() -> Result<(),LambdaError> {
///     let factory = move || {
///         App::new().service(hello)
///     };
///     if is_running_on_lambda() {
///         // Run on AWS Lambda
///         run_actix_on_lambda(factory).await?;
///     } else {
///         // Run local server
///         HttpServer::new(factory).bind("127.0.0.1:8080")?.run().await?;
///     }
///     Ok(())
/// }
/// ```
///
pub async fn run_actix_on_lambda<F, I, S, B>(factory: F) -> Result<(), LambdaError>
where
    F: Fn() -> I + Send + Clone + 'static,
    I: actix_service::IntoServiceFactory<S, actix_http::Request>,
    S: actix_service::ServiceFactory<
            actix_http::Request,
            Config = actix_web::dev::AppConfig,
            Response = actix_web::dev::ServiceResponse<B>,
            Error = actix_web::Error,
        > + 'static,
    S::InitError: std::fmt::Debug,
    B: actix_web::body::MessageBody,
    B::Error: std::fmt::Display,
    <B as actix_web::body::MessageBody>::Error: std::fmt::Debug,
{
    // Prepare actix_service::Service
    let srv = factory().into_factory();
    let new_svc = srv
        .new_service(actix_web::dev::AppConfig::default())
        .await
        .unwrap();

    lambda_runtime::run(ActixHandler(new_svc)).await?;

    Ok(())
}

/// Lambda_runtime handler for Actix Web
struct ActixHandler<S, B>(S)
where
    S: actix_service::Service<
            actix_http::Request,
            Response = actix_web::dev::ServiceResponse<B>,
            Error = actix_web::Error,
        > + 'static,
    B: actix_web::body::MessageBody,
    B::Error: std::fmt::Display,
    <B as actix_web::body::MessageBody>::Error: std::fmt::Debug;

impl<S, B> LambdaService<LambdaEvent<LambdaHttpEvent<'_>>> for ActixHandler<S, B>
where
    S: actix_service::Service<
            actix_http::Request,
            Response = actix_web::dev::ServiceResponse<B>,
            Error = actix_web::Error,
        > + 'static,
    B: actix_web::body::MessageBody,
    B::Error: std::fmt::Display,
    <B as actix_web::body::MessageBody>::Error: std::fmt::Debug,
{
    type Response = serde_json::Value;
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<serde_json::Value, Self::Error>>>>;

    /// Returns Poll::Ready when servie can process more requrests.
    fn poll_ready(
        &mut self,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    /// Lambda handler function
    /// Parse Lambda event as Actix-web request,
    /// serialize Actix-web response to Lambda JSON response
    fn call(&mut self, req: LambdaEvent<LambdaHttpEvent<'_>>) -> Self::Future {
        use serde_json::json;

        let event = req.payload;
        let _context = req.context;

        // check if web client supports content-encoding: br
        let client_br = event.client_supports_brotli();
        // multi-value-headers response format
        let multi_value = event.multi_value();

        // Parse request
        let actix_request = actix_http::Request::try_from(event);

        // Call Actix service when request parsing succeeded
        let svc_call = actix_request.map(|req| self.0.call(req));

        let fut = async move {
            match svc_call {
                Ok(svc_fut) => {
                    // Request parsing succeeded
                    if let Ok(response) = svc_fut.await {
                        // Returns as API Gateway response
                        api_gateway_response_from_actix_web(response, client_br, multi_value)
                            .await
                            .or_else(|_err| {
                                Ok(json!({
                                    "isBase64Encoded": false,
                                    "statusCode": 500u16,
                                    "headers": { "content-type": "text/plain"},
                                    "body": "Internal Server Error"
                                }))
                            })
                    } else {
                        // Some Actix web error -> 500 Internal Server Error
                        Ok(json!({
                            "isBase64Encoded": false,
                            "statusCode": 500u16,
                            "headers": { "content-type": "text/plain"},
                            "body": "Internal Server Error"
                        }))
                    }
                }
                Err(_request_err) => {
                    // Request parsing error
                    Ok(json!({
                        "isBase64Encoded": false,
                        "statusCode": 400u16,
                        "headers": { "content-type": "text/plain"},
                        "body": "Bad Request"
                    }))
                }
            }
        };
        Box::pin(fut)
    }
}

impl TryFrom<LambdaHttpEvent<'_>> for actix_http::Request {
    type Error = LambdaError;

    /// Actix-web Request from API Gateway event
    fn try_from(event: LambdaHttpEvent) -> Result<Self, Self::Error> {
        use actix_web::http::Method;

        // Construct actix_web request
        let method = Method::try_from(event.method())?;
        let req = actix_web::test::TestRequest::with_uri(&event.path_query()).method(method);

        // Source IP
        let req = if let Some(source_ip) = event.source_ip() {
            req.peer_addr(std::net::SocketAddr::from((source_ip, 0u16)))
        } else {
            req
        };

        // Headers
        let req = event
            .headers()
            .into_iter()
            .fold(req, |req, (k, v)| req.insert_header((k, &v as &str)));

        // Body
        let req = req.set_payload(event.body()?);

        Ok(req.to_request())
    }
}

impl<B> crate::brotli::ResponseCompression for actix_web::dev::ServiceResponse<B> {
    /// Content-Encoding header value
    fn content_encoding<'a>(&'a self) -> Option<&'a str> {
        self.headers()
            .get(actix_web::http::header::CONTENT_ENCODING)
            .and_then(|val| val.to_str().ok())
    }

    /// Content-Type header value
    fn content_type<'a>(&'a self) -> Option<&'a str> {
        self.headers()
            .get(actix_web::http::header::CONTENT_TYPE)
            .and_then(|val| val.to_str().ok())
    }
}

/// API Gateway response from Actix-web response
async fn api_gateway_response_from_actix_web<B: actix_web::body::MessageBody>(
    response: actix_web::dev::ServiceResponse<B>,
    client_support_br: bool,
    multi_value: bool,
) -> Result<serde_json::Value, B::Error> {
    use crate::brotli::ResponseCompression;
    use actix_web::http::header::SET_COOKIE;
    use serde_json::json;

    // HTTP status
    let status_code = response.status().as_u16();

    // Convert header to JSON map
    let mut cookies = Vec::<String>::new();
    let mut headers = serde_json::Map::new();
    for (k, v) in response.headers() {
        if let Ok(value_str) = v.to_str() {
            if multi_value {
                // REST API format, returns multiValueHeaders
                if let Some(values) = headers.get_mut(k.as_str()) {
                    if let Some(value_ary) = values.as_array_mut() {
                        value_ary.push(json!(value_str));
                    }
                } else {
                    headers.insert(k.as_str().to_string(), json!([value_str]));
                }
            } else {
                // HTTP API v2 format, returns headers
                if k == SET_COOKIE {
                    cookies.push(value_str.to_string());
                } else {
                    headers.insert(k.as_str().to_string(), json!(value_str));
                }
            }
        }
    }

    // check if response should be compressed
    let compress = client_support_br && response.can_brotli_compress();
    let body_bytes = actix_web::body::to_bytes(response.into_body()).await?;
    let body_base64 = if compress {
        if multi_value {
            headers.insert("content-encoding".to_string(), json!(["br"]));
        } else {
            headers.insert("content-encoding".to_string(), json!("br"));
        }
        crate::brotli::compress_response_body(&body_bytes)
    } else {
        base64::encode(body_bytes)
    };

    if multi_value {
        Ok(json!({
            "isBase64Encoded": true,
            "statusCode": status_code,
            "multiValueHeaders": headers,
            "body": body_base64
        }))
    } else {
        Ok(json!({
            "isBase64Encoded": true,
            "statusCode": status_code,
            "cookies": cookies,
            "headers": headers,
            "body": body_base64
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{request::LambdaHttpEvent, test_consts::*};

    // Request JSON to actix_http::Request
    fn prepare_request(event_str: &str) -> actix_http::Request {
        let reqjson: LambdaHttpEvent = serde_json::from_str(event_str).unwrap();
        actix_http::Request::try_from(reqjson).unwrap()
    }

    #[test]
    fn test_path_decode() {
        let req = prepare_request(API_GATEWAY_V2_GET_ROOT_NOQUERY);
        assert_eq!(req.uri().path(), "/");
        let req = prepare_request(API_GATEWAY_REST_GET_ROOT_NOQUERY);
        assert_eq!(req.uri().path(), "/stage/");

        let req = prepare_request(API_GATEWAY_V2_GET_SOMEWHERE_NOQUERY);
        assert_eq!(req.uri().path(), "/somewhere");
        let req = prepare_request(API_GATEWAY_REST_GET_SOMEWHERE_NOQUERY);
        assert_eq!(req.uri().path(), "/stage/somewhere");

        let req = prepare_request(API_GATEWAY_V2_GET_SPACEPATH_NOQUERY);
        assert_eq!(req.uri().path(), "/path%20with/space");
        let req = prepare_request(API_GATEWAY_REST_GET_SPACEPATH_NOQUERY);
        assert_eq!(req.uri().path(), "/stage/path%20with/space");

        let req = prepare_request(API_GATEWAY_V2_GET_PERCENTPATH_NOQUERY);
        assert_eq!(req.uri().path(), "/path%25with/percent");
        let req = prepare_request(API_GATEWAY_REST_GET_PERCENTPATH_NOQUERY);
        assert_eq!(req.uri().path(), "/stage/path%25with/percent");

        let req = prepare_request(API_GATEWAY_V2_GET_UTF8PATH_NOQUERY);
        assert_eq!(
            req.uri().path(),
            "/%E6%97%A5%E6%9C%AC%E8%AA%9E/%E3%83%95%E3%82%A1%E3%82%A4%E3%83%AB%E5%90%8D"
        );
        let req = prepare_request(API_GATEWAY_REST_GET_UTF8PATH_NOQUERY);
        assert_eq!(
            req.uri().path(),
            "/stage/%E6%97%A5%E6%9C%AC%E8%AA%9E/%E3%83%95%E3%82%A1%E3%82%A4%E3%83%AB%E5%90%8D"
        );
    }

    #[test]
    fn test_query_decode() {
        let req = prepare_request(API_GATEWAY_V2_GET_ROOT_ONEQUERY);
        assert_eq!(req.uri().query(), Some("key=value"));
        let req = prepare_request(API_GATEWAY_REST_GET_ROOT_ONEQUERY);
        assert_eq!(req.uri().query(), Some("key=value"));

        let req = prepare_request(API_GATEWAY_V2_GET_SOMEWHERE_ONEQUERY);
        assert_eq!(req.uri().query(), Some("key=value"));
        let req = prepare_request(API_GATEWAY_REST_GET_SOMEWHERE_ONEQUERY);
        assert_eq!(req.uri().query(), Some("key=value"));

        let req = prepare_request(API_GATEWAY_V2_GET_SOMEWHERE_TWOQUERY);
        assert_eq!(req.uri().query(), Some("key1=value1&key2=value2"));
        let req = prepare_request(API_GATEWAY_REST_GET_SOMEWHERE_TWOQUERY);
        assert!(
            req.uri().query() == Some("key1=value1&key2=value2")
                || req.uri().query() == Some("key2=value2&key1=value1")
        );

        let req = prepare_request(API_GATEWAY_V2_GET_SOMEWHERE_SPACEQUERY);
        assert_eq!(req.uri().query(), Some("key=value1+value2"));
        let req = prepare_request(API_GATEWAY_REST_GET_SOMEWHERE_SPACEQUERY);
        assert_eq!(req.uri().query(), Some("key=value1%20value2"));

        let req = prepare_request(API_GATEWAY_V2_GET_SOMEWHERE_UTF8QUERY);
        assert_eq!(req.uri().query(), Some("key=%E6%97%A5%E6%9C%AC%E8%AA%9E"));
        let req = prepare_request(API_GATEWAY_REST_GET_SOMEWHERE_UTF8QUERY);
        assert_eq!(req.uri().query(), Some("key=%E6%97%A5%E6%9C%AC%E8%AA%9E"));
    }

    #[test]
    fn test_remote_ip_decode() {
        use std::net::IpAddr;
        use std::str::FromStr;

        let req = prepare_request(API_GATEWAY_V2_GET_ROOT_ONEQUERY);
        assert_eq!(
            req.peer_addr().unwrap().ip(),
            IpAddr::from_str("1.2.3.4").unwrap()
        );
        let req = prepare_request(API_GATEWAY_REST_GET_ROOT_ONEQUERY);
        assert_eq!(
            req.peer_addr().unwrap().ip(),
            IpAddr::from_str("1.2.3.4").unwrap()
        );

        let req = prepare_request(API_GATEWAY_V2_GET_REMOTE_IPV6);
        assert_eq!(
            req.peer_addr().unwrap().ip(),
            IpAddr::from_str("2404:6800:400a:80c::2004").unwrap()
        );
        let req = prepare_request(API_GATEWAY_REST_GET_REMOTE_IPV6);
        assert_eq!(
            req.peer_addr().unwrap().ip(),
            IpAddr::from_str("2404:6800:400a:80c::2004").unwrap()
        );
    }

    #[tokio::test]
    async fn test_form_post() {
        use actix_web::http::Method;

        let req = prepare_request(API_GATEWAY_V2_POST_FORM_URLENCODED);
        assert_eq!(req.method(), Method::POST);
        let req = prepare_request(API_GATEWAY_REST_POST_FORM_URLENCODED);
        assert_eq!(req.method(), Method::POST);

        // Base64 encoded
        let req = prepare_request(API_GATEWAY_V2_POST_FORM_URLENCODED_B64);
        assert_eq!(req.method(), Method::POST);
        let req = prepare_request(API_GATEWAY_REST_POST_FORM_URLENCODED_B64);
        assert_eq!(req.method(), Method::POST);
    }

    #[test]
    fn test_parse_header() {
        let req = prepare_request(API_GATEWAY_V2_GET_ROOT_NOQUERY);
        assert_eq!(req.head().headers.get("x-forwarded-port").unwrap(), &"443");
        assert_eq!(
            req.head().headers.get("x-forwarded-proto").unwrap(),
            &"https"
        );
        let req = prepare_request(API_GATEWAY_REST_GET_ROOT_NOQUERY);
        assert_eq!(req.head().headers.get("x-forwarded-port").unwrap(), &"443");
        assert_eq!(
            req.head().headers.get("x-forwarded-proto").unwrap(),
            &"https"
        );
    }

    #[test]
    fn test_parse_cookies() {
        let req = prepare_request(API_GATEWAY_V2_GET_ROOT_NOQUERY);
        assert_eq!(req.head().headers.get("cookie"), None);

        let req = prepare_request(API_GATEWAY_V2_GET_ONE_COOKIE);
        assert_eq!(req.head().headers.get("cookie").unwrap(), &"cookie1=value1");
        let req = prepare_request(API_GATEWAY_REST_GET_ONE_COOKIE);
        assert_eq!(req.head().headers.get("cookie").unwrap(), &"cookie1=value1");

        let req = prepare_request(API_GATEWAY_V2_GET_TWO_COOKIES);
        assert!(
            req.head().headers.get("cookie").unwrap() == &"cookie2=value2; cookie1=value1"
                || req.head().headers.get("cookie").unwrap() == &"cookie1=value1; cookie2=value2"
        );
        let req = prepare_request(API_GATEWAY_REST_GET_TWO_COOKIES);
        assert!(
            req.head().headers.get("cookie").unwrap() == &"cookie2=value2; cookie1=value1"
                || req.head().headers.get("cookie").unwrap() == &"cookie1=value1; cookie2=value2"
        );
    }
}
