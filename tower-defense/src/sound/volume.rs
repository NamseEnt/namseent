use namui::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, State)]
pub enum VolumePreset {
    Maximum,
    High,
    Medium,
    Low,
    Minimum,
}

impl VolumePreset {
    pub fn as_f32(self) -> f32 {
        match self {
            Self::Maximum => 1.0,
            Self::High => 0.75,
            Self::Medium => 0.5,
            Self::Low => 0.1,
            Self::Minimum => 0.05,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, State)]
pub enum SoundGroup {
    Sfx,
    Ui,
    Ambient,
    Music,
}

#[derive(Clone, Debug, State)]
pub struct VolumeSettings {
    pub master: f32,
    pub sfx: f32,
    pub ui: f32,
    pub ambient: f32,
    pub music: f32,
}

impl Default for VolumeSettings {
    fn default() -> Self {
        Self {
            master: 1.0,
            sfx: 1.0,
            ui: 1.0,
            ambient: 1.0,
            music: 1.0,
        }
    }
}

impl VolumeSettings {
    pub fn subgroup_volume(&self, group: SoundGroup) -> f32 {
        match group {
            SoundGroup::Sfx => self.sfx,
            SoundGroup::Ui => self.ui,
            SoundGroup::Ambient => self.ambient,
            SoundGroup::Music => self.music,
        }
    }

    pub fn group_volume(&self, group: SoundGroup) -> f32 {
        self.master * self.subgroup_volume(group)
    }

    pub fn effective_volume(&self, group: SoundGroup, preset: VolumePreset) -> f32 {
        self.group_volume(group) * preset.as_f32()
    }
}

pub(crate) fn clamp01(value: f32) -> f32 {
    value.clamp(0.0, 1.0)
}
