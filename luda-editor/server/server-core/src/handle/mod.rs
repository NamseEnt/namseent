mod serve_s3;

use lambda_web::LambdaError;
use rpc::{
    hyper::{Body, Method, Request, Response, StatusCode},
    Uuid,
};
use serve_s3::serve_s3;

pub async fn handle_with_wrapped_error(
    request: Request<Body>,
) -> Result<Response<Body>, LambdaError> {
    let request_id = Uuid::new_v4();
    let method = request.method().clone();
    let path = request
        .uri()
        .query()
        .map(|query| query.to_string())
        .unwrap_or_default();
    log::info!("Request[{request_id}] {method} {path:?}");
    let response = handle(request).await;
    match response {
        Ok(response) => {
            log::info!(
                "Response[{request_id}] {method} {path} {status}",
                status = response.status()
            );
            Ok(response)
        }
        Err(error) => {
            eprintln!("{:#?}", error);
            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(error.to_string()))
                .unwrap())
        }
    }
}

async fn handle(request: Request<Body>) -> Result<Response<Body>, LambdaError> {
    let response_builder = Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header("Access-Control-Allow-Headers", "Content-Type, session")
        .header("Cache-Control", "no-cache, no-store, must-revalidate")
        .header("Pragma", "no-cache")
        .header("Expires", "0");

    if request.method() == Method::OPTIONS {
        return Ok(response_builder
            .status(StatusCode::OK)
            .body(Body::empty())
            .unwrap());
    }
    if request.method() == Method::GET {
        return serve_s3(request, response_builder).await;
    }
    let session = crate::session::get_session(&request).await;
    if let Err(error) = session {
        return Ok(response_builder
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from(format!("Fail to get session: {}", error)))
            .unwrap());
    }
    let session = session.unwrap();

    let services = crate::services();
    let response = rpc::handle_rpc(
        request,
        response_builder,
        &services.auth_service,
        &services.sequence_service,
        &services.image_service,
        &services.project_service,
        &services.cg_service,
        &services.memo_service,
        session,
    )
    .await?;

    Ok(response)
}
