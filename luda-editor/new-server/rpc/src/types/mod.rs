use namui_type::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Team {
    pub id: u128,
    pub name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Project {
    pub id: u128,
    pub name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Episode {
    pub id: u128,
    pub name: String,
    pub created_at: SystemTime,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TeamInviteCode {
    pub code: u128,
    pub expiration_time: SystemTime,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum EpisodeEditAction {
    AddScene {
        index: usize,
        scene: Scene,
    },
    RemoveScene {
        id: u128,
    },
    EditText {
        scene_id: u128,
        language_code: String,
        text: String,
    },
    UpdateScene {
        scene: Scene,
    },
    PutSpeaker {
        speaker: Speaker,
    },
    DeleteSpeaker {
        speaker_id: u128,
    },
    SaveSpeakerSlots {
        speaker_slots: Vec<u128>,
    },
}

pub use migration::schema::{
    AssetDoc, AssetKind, AssetSystemTag, AssetTag, Circumcircle, Scene, SceneSound, SceneSprite,
    Speaker,
};

/// Use this on the client side to get the S3 URL of an asset.
pub fn asset_s3_get_key(asset_id: u128, asset_kind: AssetKind) -> String {
    match asset_kind {
        AssetKind::Sprite => format!("sprite/{asset_id}"),
        AssetKind::Audio => format!("audio/after-transcode/{asset_id}"),
    }
}

/// I guess you don't need to use this on the client side.
pub fn asset_s3_put_key(asset_id: u128, asset_kind: AssetKind) -> String {
    match asset_kind {
        AssetKind::Sprite => format!("sprite/{asset_id}"),
        AssetKind::Audio => format!("audio/before-transcode/{asset_id}"),
    }
}
