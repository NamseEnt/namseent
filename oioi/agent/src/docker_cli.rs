use anyhow::Result;

pub(crate) async fn pull_image(image: &str) -> Result<()> {
    crate::command::run(format!("docker pull {image}")).await?;
    Ok(())
}
