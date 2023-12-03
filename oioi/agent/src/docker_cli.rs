use anyhow::Result;

pub(crate) async fn pull_image(image: &str) -> Result<()> {
    crate::bash::run(format!("docker pull {image}")).await?;
    Ok(())
}
