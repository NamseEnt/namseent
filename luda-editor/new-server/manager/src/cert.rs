use crate::*;
use std::time::Duration;

pub fn cron_cert_update() {
    tokio::task::spawn(async move {
        loop {
            if let Err(error) = run_sync().await {
                eprintln!("Failed to update cert: {}", error);
            }

            tokio::time::sleep(Duration::from_secs(24 * 3600)).await;
        }
    });
}

async fn run_sync() -> Result<()> {
    tokio::process::Command::new("aws")
        .args([
            "s3",
            "sync",
            format!("s3://{}/certs", std::env::var("S3_BUCKET").unwrap()).as_ref(),
            cert_dir().to_str().unwrap(),
        ])
        .spawn()?
        .wait()
        .await?;

    Ok(())
}
