use core::panic;
use namui::{
    file::{bundle, local_storage},
    MediaHandle, Px,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::ErrorKind, ops::Mul};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MusicMetadata {
    pub id: String,
    pub title: String,
    pub artists: Vec<String>,
    pub groups: Vec<String>,
    pub length: f64,
    pub preview_start_at: f64,
    pub preview_end_at: f64,
}
impl MusicMetadata {
    pub fn thumbnail_url(&self) -> String {
        let Self { id, .. } = self;
        format!("bundle:musics/{id}/{id}.jpg")
    }
    pub fn load_video(&self) -> MediaHandle {
        let Self { id, .. } = self;
        let path =
            namui::system::file::bundle::to_real_path(format!("bundle:musics/{id}/{id}.mp4"))
                .unwrap();
        namui::system::media::new_media(&path).unwrap()
    }
    pub fn load_audio(&self) -> MediaHandle {
        let Self { id, .. } = self;
        let path =
            namui::system::file::bundle::to_real_path(format!("bundle:musics/{id}/{id}.opus"))
                .unwrap();
        namui::system::media::new_media(&path).unwrap()
    }
}
impl PartialEq for MusicMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
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
impl Default for Speed {
    fn default() -> Self {
        Self::X4
    }
}
impl std::fmt::Display for Speed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Speed::X1 => write!(f, "X1"),
            Speed::X2 => write!(f, "X2"),
            Speed::X3 => write!(f, "X3"),
            Speed::X4 => write!(f, "X4"),
            Speed::X5 => write!(f, "X5"),
            Speed::X6 => write!(f, "X6"),
            Speed::X7 => write!(f, "X7"),
            Speed::X8 => write!(f, "X8"),
            Speed::X9 => write!(f, "X9"),
            Speed::X10 => write!(f, "X10"),
        }
    }
}
impl Mul<Px> for Speed {
    type Output = Px;

    fn mul(self, rhs: Px) -> Self::Output {
        rhs * match self {
            Speed::X1 => 1.0,
            Speed::X2 => 2.0,
            Speed::X3 => 3.0,
            Speed::X4 => 4.0,
            Speed::X5 => 5.0,
            Speed::X6 => 6.0,
            Speed::X7 => 7.0,
            Speed::X8 => 8.0,
            Speed::X9 => 9.0,
            Speed::X10 => 10.0,
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
