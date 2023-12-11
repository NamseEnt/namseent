use anyhow::Result;
use axum::{
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use futures_util::stream::TryStreamExt;
use std::{env::temp_dir, os::unix::fs::PermissionsExt};

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

async fn start_remote_run(request: Request) -> Result<(), AppError> {
    let executable_name = request
        .headers()
        .get("exetuable-name")
        .ok_or_else(|| anyhow::anyhow!("Missing header: executable-name"))?
        .to_str()?
        .to_string();

    let mut body_stream = request
        .into_body()
        .into_data_stream()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        .into_async_read();
    let mut gz = async_compression::futures::bufread::GzipDecoder::new(&mut body_stream);

    let archive = async_tar::Archive::new(&mut gz);

    let root = &temp_dir().join("remote-develop-agent");
    tokio::fs::remove_dir_all(root).await?;
    tokio::fs::create_dir_all(root).await?;

    archive.unpack(root).await?;

    let executable = root.join(executable_name);
    std::fs::set_permissions(&executable, std::fs::Permissions::from_mode(0o755))?;
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
