use anyhow::Result;
use axum::{
    body::Bytes,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use std::env::temp_dir;

#[tokio::main]
async fn main() {
    real_main().await.unwrap();
}

async fn real_main() -> Result<()> {
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/start_remote_run", post(start_remote_run));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8986").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn start_remote_run(headers: HeaderMap, body: Bytes) -> Result<(), AppError> {
    let executable_name = headers
        .get("exetuable-name")
        .ok_or_else(|| anyhow::anyhow!("Missing header: executable-name"))?
        .to_str()?
        .to_string();

    let mut tar = tar::Archive::new(flate2::read::GzDecoder::new(&body[..]));

    let root = &temp_dir().join("remote-develop-agent");
    if root.exists() {
        tokio::fs::remove_dir_all(root).await?;
    }
    tokio::fs::create_dir_all(root).await?;

    tar.unpack(root)?;

    let executable = root.join(executable_name);
    tokio::process::Command::new(executable).spawn()?;

    Ok(())
}

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
