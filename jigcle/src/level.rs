use namui::{file::bundle, Result, Wh};
use serde::{Deserialize, Serialize};

const LEVEL_LIST_PATH: &str = "bundle:level/list.yaml";

pub async fn load_level_list() -> Result<Vec<Level>> {
    let yaml_string = bundle::read(LEVEL_LIST_PATH).await?;
    let level_list: Vec<Level> = serde_yaml::from_slice(&yaml_string)?;
    Ok(level_list)
}

#[derive(Serialize, Deserialize)]
pub struct Level {
    pub name: String,
    pub puzzle_size: Wh<usize>,
    pub image_ratio: f32,
    pub image_filename: String,
    pub audio_filename: String,
}
