use crate::{MapCoordF32, PresentationInstant};
use namui::*;

use super::{SoundGroup, VolumePreset};

pub type SoundId = u64;

#[derive(Clone)]
pub enum SpatialMode {
    Spatial { position: MapCoordF32 },
    NonSpatial,
}

#[derive(Clone)]
pub struct SoundEvent {
    pub id: SoundId,
    pub asset: AudioAsset,
    pub group: SoundGroup,
    pub volume_preset: VolumePreset,
    pub spatial: SpatialMode,
    pub repeat: bool,
    pub play_at: PresentationInstant,
    pub created_at: PresentationInstant,
    pub max_duration: Option<Duration>,
}

impl SoundEvent {
    pub fn is_ready(&self, presentation_instant: PresentationInstant) -> bool {
        presentation_instant >= self.play_at
    }

    pub fn is_expired(&self, presentation_instant: PresentationInstant) -> bool {
        let Some(max_duration) = self.max_duration else {
            return false;
        };

        if presentation_instant < self.play_at {
            return false;
        }

        presentation_instant - self.play_at >= max_duration
    }
}

#[derive(Clone)]
pub struct EmitSoundParams {
    pub asset: AudioAsset,
    pub group: SoundGroup,
    pub volume_preset: VolumePreset,
    pub spatial: SpatialMode,
    pub repeat: bool,
    pub max_duration: Option<Duration>,
}

impl EmitSoundParams {
    pub fn one_shot(
        asset: AudioAsset,
        group: SoundGroup,
        volume_preset: VolumePreset,
        spatial: SpatialMode,
    ) -> Self {
        Self {
            asset,
            group,
            volume_preset,
            spatial,
            repeat: false,
            max_duration: None,
        }
    }

    pub fn looping(
        asset: AudioAsset,
        group: SoundGroup,
        volume_preset: VolumePreset,
        spatial: SpatialMode,
    ) -> Self {
        Self {
            asset,
            group,
            volume_preset,
            spatial,
            repeat: true,
            max_duration: None,
        }
    }

    pub fn with_max_duration(mut self, max_duration: Duration) -> Self {
        self.max_duration = Some(max_duration);
        self
    }
}
