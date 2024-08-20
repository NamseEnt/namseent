mod scene;

use namui_type::*;
pub use scene::*;

#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[archive(check_bytes)]
pub struct Team {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[archive(check_bytes)]
pub struct Project {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[archive(check_bytes)]
pub struct Episode {
    pub id: String,
    pub name: String,
    pub created_at: SystemTime,
}

#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[archive(check_bytes)]
pub struct TeamInviteCode {
    pub code: String,
    pub expiration_time: SystemTime,
}

#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[archive(check_bytes)]
pub enum EpisodeEditAction {
    AddScene {
        index: usize,
        scene: Scene,
    },
    RemoveScene {
        id: String,
    },
    EditText {
        scene_id: String,
        language_code: String,
        text: String,
    },
    UpdateScene {
        scene: Scene,
    },
}

pub type SceneSprite = migration::schema::SceneSprite;
pub type SpriteDoc = migration::schema::SpriteDoc;
pub type Sprite = migration::schema::Sprite;
pub type SpritePart = migration::schema::SpritePart;
pub type SpritePartOption = migration::schema::SpritePartOption;
pub type SystemTag = migration::schema::SystemTag;
pub type SpriteTag = migration::schema::SpriteTag;
