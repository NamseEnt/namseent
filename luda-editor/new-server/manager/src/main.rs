use anyhow::Result;
use axum::{routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;
use std::{path::PathBuf, sync::atomic::AtomicU16};

static SERVER_PORT: AtomicU16 = AtomicU16::new(0);
const CERT_DIR: &str = "/etc/letsencrypt/live/visual-novel.namseent.com";
pub fn cert_dir() -> PathBuf {
    PathBuf::from(CERT_DIR)
}

#[tokio::main]
async fn main() {
    real_main().await
}

async fn real_main() {
    tokio::task::spawn(start_port_server());
}

async fn start_port_server() -> Result<()> {
    let config =
        RustlsConfig::from_pem_file(cert_dir().join("cert.pem"), cert_dir().join("key.pem"))
            .await?;

    tokio::task::spawn({
        let config = config.clone();
        async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(24 * 3600)).await;
                if let Err(err) = config
                    .reload_from_pem_file(cert_dir().join("cert.pem"), cert_dir().join("key.pem"))
                    .await
                {
                    eprintln!("Failed to reload cert: {}", err);
                }
            }
        }
    });

    let app = Router::new().route(
        "/",
        get(|| async {
            SERVER_PORT
                .load(std::sync::atomic::Ordering::Relaxed)
                .to_string()
        }),
    );

    let addr = "[::]:444".parse()?;
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
