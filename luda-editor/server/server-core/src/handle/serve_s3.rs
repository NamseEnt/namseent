use hyper::{Body, Request, Response, StatusCode, http::response::Builder};
use lambda_web::{LambdaError, is_running_on_lambda};

pub async fn serve_s3(
    request: Request<Body>,
    response_builder: Builder,
) -> Result<Response<Body>, LambdaError> {
    if !is_running_on_lambda() {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("Not running on lambda"))
            .unwrap());
    }
    let serve_static_s3_bucket =
        std::env::var("SERVE_STATIC_S3_BUCKET").expect("SERVE_STATIC_S3_BUCKET is not set");
    let serve_static_s3_key_prefix =
        std::env::var("SERVE_STATIC_S3_KEY_PREFIX").expect("SERVE_STATIC_S3_KEY_PREFIX is not set");

    let mut path = request.uri().path();
    if path == "/" {
        path = "/index.html";
    }

    let key = crate::append_slash![serve_static_s3_key_prefix, path];

    let s3_location = crate::append_slash!(
        "https://s3.ap-northeast-2.amazonaws.com",
        serve_static_s3_bucket,
        key
    );

    Ok(response_builder
        .status(302)
        .header("Location", s3_location)
        .body(Body::empty())?)
}
