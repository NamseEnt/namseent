use super::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone)]
pub struct Sequence {
    pub tracks: Arc<[Arc<Track>]>,
}

impl Sequence {
    pub fn get_clip(&self, id: &str) -> Option<Clip> {
        for track in self.tracks.iter() {
            match track.as_ref() {
                Track::Camera(track) => {
                    for clip in track.clips.iter() {
                        if clip.id == id {
                            return Some(Clip::Camera(clip.clone()));
                        }
                    }
                }
                Track::Subtitle(track) => {
                    for clip in track.clips.iter() {
                        if clip.id == id {
                            return Some(Clip::Subtitle(clip.clone()));
                        }
                    }
                }
            }
        }
        None
    }
    pub fn find_track(&self, callback: impl Fn(&Track) -> bool) -> Option<Arc<Track>> {
        for track in self.tracks.iter() {
            if callback(track.as_ref()) {
                return Some(track.clone());
            }
        }
        None
    }
    pub fn find_track_by_clip_id(&self, clip_id: &str) -> Option<Arc<Track>> {
        self.find_track(|track| match track {
            Track::Camera(track) => track.clips.iter().any(|clip| clip.id.eq(clip_id)),
            Track::Subtitle(track) => track.clips.iter().any(|clip| clip.id.eq(clip_id)),
        })
    }
    pub fn into_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

impl<'a> TryFrom<&'a str> for Sequence {
    fn try_from(value: &str) -> Result<Sequence, String> {
        match serde_json::from_str::<Sequence>(&value) {
            Ok(sequence) => Ok(sequence),
            Err(error) => Err(error.to_string()),
        }
    }

    type Error = String;
}

impl Default for Sequence {
    fn default() -> Self {
        Self {
            tracks: Arc::new([
                Arc::new(Track::Camera(CameraTrack {
                    id: "camera-track".to_string(),
                    clips: vec![].into(),
                })),
                Arc::new(Track::Subtitle(SubtitleTrack {
                    id: "subtitle-track".to_string(),
                    clips: vec![].into(),
                })),
            ]),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraClip {
    pub id: String,
    pub start_at: Time,
    pub end_at: Time,
    pub animation: namui::animation::Animation,
}
impl CameraClip {
    pub fn is_at_time(&self, time: &Time) -> bool {
        self.start_at <= time && time < self.end_at
    }
    pub fn clone_with_new_id(&self) -> CameraClip {
        let mut new = (*self).clone();
        new.id = CameraClip::get_new_id();
        new
    }
    pub fn get_new_id() -> String {
        format!("CameraClip-{}", namui::nanoid())
    }
}

#[derive(Debug, Clone)]
pub enum Clip {
    Camera(Arc<CameraClip>),
    Subtitle(Arc<SubtitleClip>),
}

impl Clip {
    pub fn get_id(&self) -> &str {
        match self {
            Clip::Camera(clip) => &clip.id,
            Clip::Subtitle(clip) => &clip.id,
        }
    }
    pub fn as_camera_clip(&self) -> Option<&CameraClip> {
        match self {
            Clip::Camera(clip) => Some(clip.as_ref()),
            _ => None,
        }
    }

    pub(crate) fn get_start_time(&self) -> Time {
        match self {
            Clip::Camera(clip) => clip.start_at,
            Clip::Subtitle(clip) => clip.start_at,
        }
    }
}
impl AsRef<CameraClip> for Clip {
    fn as_ref(&self) -> &CameraClip {
        match self {
            Clip::Camera(clip) => clip.as_ref(),
            _ => panic!("Clip is not CameraClip"),
        }
    }
}
impl From<CameraClip> for Clip {
    fn from(clip: CameraClip) -> Self {
        Clip::Camera(Arc::new(clip))
    }
}
impl AsRef<SubtitleClip> for Clip {
    fn as_ref(&self) -> &SubtitleClip {
        match self {
            Clip::Subtitle(clip) => clip.as_ref(),
            _ => panic!("Clip is not SubtitleClip"),
        }
    }
}
impl From<SubtitleClip> for Clip {
    fn from(clip: SubtitleClip) -> Self {
        Clip::Subtitle(Arc::new(clip))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleClip {
    pub id: String,
    pub start_at: Time,
    pub subtitle: Subtitle,
    #[serde(skip_serializing_if = "is_false")]
    #[serde(default)]
    pub is_needed_to_update_position: bool,
}

fn is_false(value: impl std::borrow::Borrow<bool>) -> bool {
    !value.borrow()
}

impl SubtitleClip {
    pub fn is_at_time(
        &self,
        time: &Time,
        language: Language,
        duration_measurer: &dyn SubtitlePlayDurationMeasure,
    ) -> bool {
        self.start_at <= time && time < self.end_at(language, duration_measurer)
    }

    pub(crate) fn end_at(
        &self,
        language: Language,
        duration_measurer: &dyn SubtitlePlayDurationMeasure,
    ) -> Time {
        self.start_at + duration_measurer.get_play_duration(&self.subtitle, &language)
    }
}

pub enum UpdateResult<T, Error> {
    Updated(T),
    NotUpdated,
    Err(Error),
}

impl<T, Error> UpdateResult<T, Error>
where
    Error: std::fmt::Display,
{
    pub fn unwrap(self) -> T {
        match self {
            UpdateResult::Updated(value) => value,
            UpdateResult::NotUpdated => panic!("Not updated"),
            UpdateResult::Err(error) => panic!("{}", error),
        }
    }
}

fn update_arcs<T, Error>(
    arcs: &mut Arc<[Arc<T>]>,
    callback: impl Fn(&T) -> UpdateResult<T, Error>,
) -> UpdateResult<(), Error> {
    let mut vec = arcs.to_vec();
    let mut is_updated = false;
    for arc in vec.iter_mut() {
        match callback(arc.as_ref()) {
            UpdateResult::Updated(updated_element) => {
                is_updated = true;
                let _ = std::mem::replace(arc, Arc::new(updated_element));
            }
            UpdateResult::NotUpdated => continue,
            UpdateResult::Err(error) => return UpdateResult::Err(error),
        }
    }
    if is_updated {
        *arcs = vec.into();
        UpdateResult::Updated(())
    } else {
        UpdateResult::NotUpdated
    }
}

pub trait ClipReplacer<Clip> {
    fn replace_clip(
        self,
        clip_id: &str,
        replace_callback: impl FnOnce(&Clip) -> Result<Clip, String> + Copy,
    ) -> UpdateResult<Self, String>
    where
        Self: Sized;
}

macro_rules! sequence_clip_replacer {
    ($track: ident, $clip: tt) => {
        impl ClipReplacer<$clip> for Sequence {
            fn replace_clip(
                mut self,
                clip_id: &str,
                replace_callback: impl FnOnce(&$clip) -> Result<$clip, String> + Copy,
            ) -> UpdateResult<Self, String> {
                match update_arcs(&mut self.tracks, |track| {
                    if let Track::$track(track) = track {
                        let track = track.clone();
                        match track.replace_clip(clip_id, replace_callback) {
                            UpdateResult::Updated(track) => {
                                UpdateResult::Updated(Track::$track(track))
                            }
                            UpdateResult::NotUpdated => UpdateResult::NotUpdated,
                            UpdateResult::Err(error) => UpdateResult::Err(error),
                        }
                    } else {
                        return UpdateResult::NotUpdated;
                    }
                }) {
                    UpdateResult::Updated(_) => UpdateResult::Updated(self),
                    UpdateResult::NotUpdated => UpdateResult::NotUpdated,
                    UpdateResult::Err(error) => UpdateResult::Err(error),
                }
            }
        }
    };
}

macro_rules! track_clip_replacer {
    ($track: tt, $clip: tt) => {
        impl ClipReplacer<$clip> for $track {
            fn replace_clip(
                mut self,
                clip_id: &str,
                replace_callback: impl FnOnce(&$clip) -> Result<$clip, String> + Copy,
            ) -> UpdateResult<Self, String> {
                match update_arcs(&mut self.clips, |clip| {
                    if clip.id == *clip_id {
                        match replace_callback(clip) {
                            Ok(clip) => UpdateResult::Updated(clip),
                            Err(error) => {
                                return UpdateResult::Err(error);
                            }
                        }
                    } else {
                        UpdateResult::NotUpdated
                    }
                }) {
                    UpdateResult::Updated(_) => UpdateResult::Updated(self),
                    UpdateResult::NotUpdated => UpdateResult::NotUpdated,
                    UpdateResult::Err(error) => UpdateResult::Err(error),
                }
            }
        }
    };
}

sequence_clip_replacer!(Camera, CameraClip);
track_clip_replacer!(CameraTrack, CameraClip);
sequence_clip_replacer!(Subtitle, SubtitleClip);
track_clip_replacer!(SubtitleTrack, SubtitleClip);

pub trait ClipFind<Clip> {
    fn find_clip(&self, clip_id: &str) -> Option<&Clip>
    where
        Self: Sized;
}

impl ClipFind<CameraClip> for Sequence {
    fn find_clip(&self, clip_id: &str) -> Option<&CameraClip> {
        self.tracks.iter().find_map(|track| {
            if let Track::Camera(track) = track.as_ref() {
                track
                    .clips
                    .iter()
                    .find(|clip| clip.id == *clip_id)
                    .map(|clip| clip.as_ref())
            } else {
                None
            }
        })
    }
}

pub trait TrackReplacer<TrackType> {
    fn replace_track(
        self,
        track_id: &str,
        replace_callback: impl FnOnce(TrackType) -> Result<TrackType, String> + Copy,
    ) -> UpdateResult<Self, String>
    where
        Self: Sized;
}

impl<TTrack> TrackReplacer<TTrack> for Sequence
where
    TTrack: From<Track>,
    Track: From<TTrack>,
{
    fn replace_track(
        mut self,
        track_id: &str,
        replace_callback: impl FnOnce(TTrack) -> Result<TTrack, String> + Copy,
    ) -> UpdateResult<Self, String> {
        match update_arcs(&mut self.tracks, |track| {
            if track.get_id() == track_id {
                match replace_callback(track.clone().into()) {
                    Ok(track) => UpdateResult::Updated(track.into()),
                    Err(error) => {
                        return UpdateResult::Err(error);
                    }
                }
            } else {
                return UpdateResult::NotUpdated;
            }
        }) {
            UpdateResult::Updated(_) => UpdateResult::Updated(self),
            UpdateResult::NotUpdated => UpdateResult::NotUpdated,
            UpdateResult::Err(error) => UpdateResult::Err(error),
        }
    }
}
