mod kv_store;

use anyhow::Result;
use axum::{routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;
use std::path::PathBuf;

const CERT_DIR: &str = "/etc/letsencrypt/live/visual-novel.namseent.com";
fn cert_dir() -> PathBuf {
    PathBuf::from(CERT_DIR)
}

#[tokio::main]
async fn main() -> Result<()> {
    real_main().await
}

async fn real_main() -> Result<()> {
    start_server().await
}

async fn start_server() -> Result<()> {
    let app = Router::new()
        .route("/turn_on_memory_cache", get(|| async { "todo" }))
        .route("/turn_off_memory_cache", get(|| async { "todo" }))
        .route("/health", get(|| async { "Good" }));

    let port = if is_on_aws() {
        std::env::var("PORT").unwrap()
    } else {
        "8080".to_string()
    };

    let addr = format!("[::]:{port}").parse()?;

    if is_on_aws() {
        axum_server::bind(addr)
            .serve(app.into_make_service())
            .await?;
    } else {
        let config =
            RustlsConfig::from_pem_file(cert_dir().join("cert.pem"), cert_dir().join("key.pem"))
                .await?;

        keep_cert_updated(config.clone());

        axum_server::bind_rustls(addr, config)
            .serve(app.into_make_service())
            .await?;
    };

    Ok(())
}

fn keep_cert_updated(config: RustlsConfig) {
    tokio::task::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(24 * 3600)).await;
            if let Err(err) = config
                .reload_from_pem_file(cert_dir().join("cert.pem"), cert_dir().join("key.pem"))
                .await
            {
                eprintln!("Failed to reload cert: {}", err);
            }
        }
    });
}

fn is_on_aws() -> bool {
    std::env::var("IS_ON_AWS").is_ok()
}
