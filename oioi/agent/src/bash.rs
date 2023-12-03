use anyhow::Result;

pub(crate) async fn run(command: impl AsRef<str>) -> Result<Vec<u8>> {
    let command = command.as_ref();

    let output = tokio::process::Command::new("bash")
        .args(["-c", command])
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run() {
        let output = run("echo hello").await.unwrap();
        assert_eq!(output, b"hello\n");
    }
}
