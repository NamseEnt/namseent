use aws_sdk_s3::presigning::config::PresigningConfig;
use lambda_web::{is_running_on_lambda, LambdaError};
use rpc::hyper::{http::response::Builder, Body, Request, Response, StatusCode};

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

    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);

    let mut path = request.uri().path();
    if path == "/" {
        path = "/index.html";
    }

    let key = format!("{}{}", serve_static_s3_key_prefix, path);

    if [".html", ".js"].iter().any(|ext| key.ends_with(ext)) {
        let response = client
            .get_object()
            .bucket(serve_static_s3_bucket)
            .key(key)
            .send()
            .await?;

        return Ok(response_builder
            .status(200)
            .header("Content-Type", response.content_type().unwrap())
            .body(Body::from(response.body.collect().await?.into_bytes()))?);
    }

    let response = client
        .get_object()
        .bucket(serve_static_s3_bucket)
        .key(key)
        .presigned(
            PresigningConfig::builder()
                .expires_in(std::time::Duration::from_secs(60))
                .build()?,
        )
        .await?;

    return Ok(response_builder
        .status(302)
        .header("Location", response.uri().to_string())
        .body(Body::empty())?);
}
