use anyhow::Result;

pub(crate) async fn pull_image(image: &str) -> Result<()> {
    run_docker_command(&["pull", image]).await?;
    Ok(())
}

async fn run_docker_command(args: &[&str]) -> Result<Vec<u8>> {
    let output = tokio::process::Command::new("docker")
        .args(args)
        .output()
        .await
        .map_err(|e| {
            anyhow::anyhow!(
                "Failed to run docker {args}: {e}",
                args = args.join(" "),
                e = e
            )
        })?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Failed to run docker {args}: {stderr}",
            args = args.join(" "),
            stderr = String::from_utf8_lossy(&output.stderr),
        ));
    }

    Ok(output.stdout)
}
