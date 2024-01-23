use core::panic;
use namui::{
    file::{bundle, local_storage},
    Url,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::ErrorKind};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MusicMetadata {
    pub id: String,
    pub title: String,
    pub artists: Vec<String>,
    pub groups: Vec<String>,
    pub length: f64,
}
impl MusicMetadata {
    pub fn thumbnail_url(&self) -> Url {
        let Self { id, .. } = self;
        Url::parse(&format!("bundle:musics/{id}/{id}.jpg")).unwrap()
    }
}

pub async fn load_music_metadata() -> Vec<MusicMetadata> {
    let metadata_file = bundle::read("bundle:musics/music_metadata.yml")
        .await
        .unwrap();
    let mut metadata: Vec<MusicMetadata> = serde_yaml::from_slice(&metadata_file).unwrap();
    metadata.sort_by(|a, b| a.title.cmp(&b.title));
    metadata
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Speed {
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
}
impl AsRef<f32> for Speed {
    fn as_ref(&self) -> &f32 {
        match self {
            Self::X1 => &1.0,
            Self::X2 => &2.0,
            Self::X3 => &3.0,
            Self::X4 => &4.0,
            Self::X5 => &5.0,
            Self::X6 => &6.0,
            Self::X7 => &7.0,
            Self::X8 => &8.0,
            Self::X9 => &9.0,
            Self::X10 => &10.0,
        }
    }
}
impl Default for Speed {
    fn default() -> Self {
        Self::X4
    }
}
impl ToString for Speed {
    fn to_string(&self) -> String {
        match self {
            Speed::X1 => "X1".to_string(),
            Speed::X2 => "X2".to_string(),
            Speed::X3 => "X3".to_string(),
            Speed::X4 => "X4".to_string(),
            Speed::X5 => "X5".to_string(),
            Speed::X6 => "X6".to_string(),
            Speed::X7 => "X7".to_string(),
            Speed::X8 => "X8".to_string(),
            Speed::X9 => "X9".to_string(),
            Speed::X10 => "X10".to_string(),
        }
    }
}
pub const SPEEDS: [Speed; 10] = [
    Speed::X1,
    Speed::X2,
    Speed::X3,
    Speed::X4,
    Speed::X5,
    Speed::X6,
    Speed::X7,
    Speed::X8,
    Speed::X9,
    Speed::X10,
];

const MUSIC_SPEED_MAP_PATH: &str = "music_speed_map.yml";

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MusicSpeedMap {
    map: HashMap<String, Speed>,
}
impl MusicSpeedMap {
    pub fn get(&self, id: &str) -> Speed {
        self.map.get(id).copied().unwrap_or_default()
    }
    pub fn set(&mut self, id: String, speed: Speed) {
        self.map.insert(id, speed);
    }
    pub async fn save(&self) {
        // Ensure dir
        local_storage::make_dir("").await.unwrap();
        local_storage::write(MUSIC_SPEED_MAP_PATH, serde_yaml::to_string(self).unwrap())
            .await
            .unwrap();
    }
}

pub async fn load_music_speed_map() -> MusicSpeedMap {
    match local_storage::read(MUSIC_SPEED_MAP_PATH)
        .await
        .map(|file| serde_yaml::from_slice::<MusicSpeedMap>(&file).unwrap())
    {
        Ok(music_speed_map) => music_speed_map,
        Err(error) => {
            if !matches!(error.kind(), ErrorKind::NotFound) {
                panic!("{error:?}");
            }
            MusicSpeedMap::default()
        }
    }
}

const MUSIC_BEST_SCORE_MAP_PATH: &str = "music_best_score_map.yml";

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MusicBestScoreMap {
    map: HashMap<String, usize>,
}
impl MusicBestScoreMap {
    pub fn get(&self, id: &str) -> usize {
        self.map.get(id).copied().unwrap_or_default()
    }
    pub fn set(&mut self, id: String, score: usize) {
        self.map.insert(id, score);
    }
    pub async fn save(&self) {
        // Ensure dir
        local_storage::make_dir("").await.unwrap();
        local_storage::write(
            MUSIC_BEST_SCORE_MAP_PATH,
            serde_yaml::to_string(self).unwrap(),
        )
        .await
        .unwrap();
    }
}

pub async fn load_music_best_score_map() -> MusicBestScoreMap {
    match local_storage::read(MUSIC_BEST_SCORE_MAP_PATH)
        .await
        .map(|file| serde_yaml::from_slice::<MusicBestScoreMap>(&file).unwrap())
    {
        Ok(music_speed_map) => music_speed_map,
        Err(error) => {
            if !matches!(error.kind(), ErrorKind::NotFound) {
                panic!("{error:?}");
            }
            MusicBestScoreMap::default()
        }
    }
}
