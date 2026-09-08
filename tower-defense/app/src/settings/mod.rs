use crate::sound::VolumeSettings;
use namui::*;
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

const SETTINGS_STORAGE_KEY: &str = "tower-defense-settings";
const SETTINGS_VERSION: u32 = 1;

static SETTINGS_ATOM: Atom<Settings> = Atom::uninitialized();

#[derive(Clone, Debug, PartialEq, SerdeSerialize, SerdeDeserialize, State)]
pub struct Settings {
    pub version: u32,
    #[serde(default)]
    pub audio: AudioSettings,
}

#[derive(Clone, Debug, PartialEq, SerdeSerialize, SerdeDeserialize, Default, State)]
pub struct AudioSettings {
    #[serde(default)]
    pub volume: VolumeSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            version: SETTINGS_VERSION,
            audio: AudioSettings::default(),
        }
    }
}

impl Settings {
    pub fn init<'a>(ctx: &'a RenderCtx) -> Sig<'a, Settings> {
        ctx.init_atom(&SETTINGS_ATOM, Settings::default).0
    }

    pub async fn load_async() -> Self {
        let raw = namui::system::kv_store::get(SETTINGS_STORAGE_KEY).await;
        raw.and_then(|bytes| String::from_utf8(bytes).ok())
            .and_then(|raw_string| Self::from_storage_str(&raw_string))
            .unwrap_or_default()
            .sanitize()
    }

    pub fn set_settings(self) {
        SETTINGS_ATOM.mutate(|settings| {
            *settings = self;
        });
    }

    pub fn save(&self) {
        let sanitized = self.clone().sanitize();
        let Ok(serialized) = serde_json::to_string(&sanitized) else {
            return;
        };
        spawn(async move {
            namui::system::kv_store::put(SETTINGS_STORAGE_KEY, Some(serialized.as_bytes())).await;
        });
    }

    pub fn replace_volume_settings(volume: VolumeSettings) {
        SETTINGS_ATOM.mutate(move |settings| {
            settings.audio.volume = volume.clamped();
            settings.version = SETTINGS_VERSION;
            settings.save();
        });
    }

    fn from_storage_str(raw: &str) -> Option<Self> {
        let value = serde_json::from_str::<serde_json::Value>(raw).ok()?;
        Self::from_value(value)
    }

    fn from_value(value: serde_json::Value) -> Option<Self> {
        let version = value
            .get("version")
            .and_then(serde_json::Value::as_u64)
            .map(|version| version as u32)
            .unwrap_or(0);

        match version {
            SETTINGS_VERSION => serde_json::from_value(value).ok().map(Self::sanitize),
            0 => Some(Self::migrate_v0(value)),
            _ => None,
        }
    }

    fn migrate_v0(value: serde_json::Value) -> Self {
        let mut settings = Self::default();
        settings.audio.volume.master = read_volume(&value, "master").unwrap_or(1.0);
        settings.audio.volume.sfx = read_volume(&value, "sfx").unwrap_or(1.0);
        settings.audio.volume.ui = read_volume(&value, "ui").unwrap_or(1.0);
        settings.audio.volume.ambient = read_volume(&value, "ambient").unwrap_or(1.0);
        settings.audio.volume.music = read_volume(&value, "music").unwrap_or(1.0);
        settings.sanitize()
    }

    fn sanitize(mut self) -> Self {
        self.version = SETTINGS_VERSION;
        self.audio.volume = self.audio.volume.clamped();
        self
    }
}

fn read_volume(value: &serde_json::Value, key: &str) -> Option<f32> {
    value
        .get(key)
        .and_then(serde_json::Value::as_f64)
        .map(|value| value as f32)
}

#[cfg(test)]
mod tests {
    use super::Settings;

    #[test]
    fn migrates_unversioned_volume_settings() {
        let raw = r#"{"master":0.3,"sfx":0.4,"ui":0.5,"ambient":0.6,"music":0.7}"#;

        let settings = Settings::from_storage_str(raw).expect("settings should migrate");

        assert_eq!(settings.version, 1);
        assert_eq!(settings.audio.volume.master, 0.3);
        assert_eq!(settings.audio.volume.sfx, 0.4);
        assert_eq!(settings.audio.volume.ui, 0.5);
        assert_eq!(settings.audio.volume.ambient, 0.6);
        assert_eq!(settings.audio.volume.music, 0.7);
    }

    #[test]
    fn clamps_loaded_values() {
        let raw = r#"{"version":1,"audio":{"volume":{"master":1.4,"sfx":-0.1,"ui":0.5,"ambient":2.0,"music":0.2}}}"#;

        let settings = Settings::from_storage_str(raw).expect("settings should parse");

        assert_eq!(settings.audio.volume.master, 1.0);
        assert_eq!(settings.audio.volume.sfx, 0.0);
        assert_eq!(settings.audio.volume.ui, 0.5);
        assert_eq!(settings.audio.volume.ambient, 1.0);
        assert_eq!(settings.audio.volume.music, 0.2);
    }
}
