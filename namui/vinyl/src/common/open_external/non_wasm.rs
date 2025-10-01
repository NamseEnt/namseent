use anyhow::Result;

pub fn open_external(url: &str) -> Result<()> {
    opener::open(url)?;
    Ok(())
}
