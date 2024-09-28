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

pub use migration::schema::{
    AssetDoc, AssetKind, AssetSystemTag, AssetTag, Circumcircle, SceneSprite,
};

/// Use this on the client side to get the S3 URL of an asset.
pub fn asset_s3_get_key(asset_id: &str, asset_kind: AssetKind) -> String {
    match asset_kind {
        AssetKind::Sprite => format!("sprite/{asset_id}"),
        AssetKind::Audio => format!("audio/after-transcode/{asset_id}"),
    }
}

/// I guess you don't need to use this on the client side.
pub fn asset_s3_put_key(asset_id: &str, asset_kind: AssetKind) -> String {
    match asset_kind {
        AssetKind::Sprite => format!("sprite/{asset_id}"),
        AssetKind::Audio => format!("audio/before-transcode/{asset_id}"),
    }
}
