use anyhow::Result;

pub(crate) async fn run(command: impl AsRef<str>) -> Result<Vec<u8>> {
    let command = command.as_ref();
    let mut args = command.split_whitespace();
    let program = args.next().unwrap();
    let args = args.collect::<Vec<_>>();

    let output = tokio::process::Command::new(program)
        .args(&args)
        .output()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to run {command}: {e}"))?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Failed to run {command}: {stderr}",
            stderr = String::from_utf8_lossy(&output.stderr),
        ));
    }

    Ok(output.stdout)
}
